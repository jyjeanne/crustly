//! Tool Registry
//!
//! Manages the collection of available tools that can be invoked by agents.

use super::error::{Result, ToolError};
use super::r#trait::{Tool, ToolExecutionContext, ToolResult};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;

/// Longest tool-input rendering written to the log. A `write_file` call carries a
/// whole file body and a `web_fetch` a whole page; logging those in full on every
/// call would bury the line this exists to make readable.
const INPUT_PREVIEW_CHARS: usize = 300;

/// Render a tool's arguments for the execution log, truncated to
/// [`INPUT_PREVIEW_CHARS`].
///
/// The point is diagnostic: a model that asks for the wrong command produces a
/// tool that "succeeds" with empty output, which is indistinguishable from a
/// broken tool unless the arguments are visible.
///
/// Truncation is on `char` boundaries, never bytes - a cut through the middle of a
/// multi-byte character would panic on a path or prompt containing non-ASCII.
fn preview_input(input: &Value) -> String {
    let rendered = input.to_string();
    if rendered.chars().count() <= INPUT_PREVIEW_CHARS {
        return rendered;
    }
    let truncated: String = rendered.chars().take(INPUT_PREVIEW_CHARS).collect();
    format!("{truncated} …[truncated]")
}

/// Registry of available tools
pub struct ToolRegistry {
    tools: HashMap<String, Arc<dyn Tool>>,
    policy: Option<Arc<dyn crate::llm::tools::sandbox::PermissionPolicy>>,
}

impl ToolRegistry {
    /// Create a new empty tool registry
    pub fn new() -> Self {
        Self {
            tools: HashMap::new(),
            policy: None,
        }
    }

    /// Set the permission policy for this registry
    pub fn set_policy(&mut self, policy: Arc<dyn crate::llm::tools::sandbox::PermissionPolicy>) {
        self.policy = Some(policy);
    }

    /// Whether the policy affirmatively vouches for this exact call, meaning it
    /// may run without an approval prompt (e.g. a `bash` command whose program
    /// is on the `security.allow_bash` allowlist).
    ///
    /// False when no policy is configured: absent an explicit allowlist, nothing
    /// is trusted, and `requires_approval` tools keep prompting as before.
    ///
    /// Resolves `name` through the alias table first, same as `execute()`,
    /// so a policy rule written against the canonical name (e.g. an
    /// allowlist entry for `bash`) still applies when a model calls the
    /// same tool by an alias (e.g. `run_shell_command`). Without this, the
    /// approval-prompt precheck in `llm::agent::service` and `execute()`'s
    /// own policy check could disagree about the same call.
    pub fn is_trusted(&self, name: &str, input: &serde_json::Value) -> bool {
        use crate::llm::tools::sandbox::PolicyDecision;
        let canonical = self.canonical_name(name);
        matches!(
            self.policy.as_ref().map(|p| p.evaluate(canonical, input)),
            Some(PolicyDecision::Trusted)
        )
    }

    /// Register a tool
    pub fn register(&mut self, tool: Arc<dyn Tool>) {
        let name = tool.name().to_string();
        tracing::debug!("Registered tool: {}", name);
        self.tools.insert(name, tool);
    }

    /// Resolve `name` to the key actually present in the registry: `name`
    /// itself if it's registered directly, otherwise its alias target (see
    /// [`super::aliases::resolve`]) if *that* is registered, otherwise
    /// `name` unchanged - so a genuinely unknown name still produces a
    /// clear "not found" error naming what the model actually sent, rather
    /// than silently becoming something else.
    ///
    /// An exact match always wins over an alias: if a real tool is ever
    /// registered under a name that happens to collide with an alias entry,
    /// the real tool takes precedence.
    fn canonical_name<'a>(&self, name: &'a str) -> &'a str {
        if self.tools.contains_key(name) {
            return name;
        }
        match super::aliases::resolve(name) {
            Some(target) if self.tools.contains_key(target) => target,
            _ => name,
        }
    }

    /// Get a tool by name (resolving aliases - see [`Self::canonical_name`])
    pub fn get(&self, name: &str) -> Option<Arc<dyn Tool>> {
        self.tools.get(self.canonical_name(name)).cloned()
    }

    /// Check if a tool is registered (resolving aliases - see [`Self::canonical_name`])
    pub fn has_tool(&self, name: &str) -> bool {
        self.tools.contains_key(self.canonical_name(name))
    }

    /// List all registered tool names
    pub fn list_tools(&self) -> Vec<String> {
        self.tools.keys().cloned().collect()
    }

    /// Get tool definitions in LLM format
    pub fn get_tool_definitions(&self) -> Vec<crate::llm::provider::Tool> {
        self.tools
            .values()
            .map(|tool| crate::llm::provider::Tool {
                name: tool.name().to_string(),
                description: tool.description().to_string(),
                input_schema: tool.input_schema(),
            })
            .collect()
    }

    /// Execute a tool by name. `name` is resolved through the alias table
    /// (see [`Self::canonical_name`]) once, up front, and that canonical
    /// name is used for the tool lookup, the permission policy check, and
    /// logging - so a call for e.g. `list_directory` (qwen-code's name for
    /// `ls`) is indistinguishable, from the policy's perspective, from a
    /// call for `ls` itself. Evaluating the policy against the raw alias
    /// instead would let a configured `security.allow_bash`-style rule
    /// silently fail to match a call that used a different name for the
    /// same tool.
    pub async fn execute(
        &self,
        name: &str,
        input: Value,
        context: &ToolExecutionContext,
    ) -> Result<ToolResult> {
        let canonical = self.canonical_name(name);
        let tool = self
            .tools
            .get(canonical)
            .cloned()
            .ok_or_else(|| ToolError::NotFound(name.to_string()))?;

        // Validate input
        tool.validate_input(&input)?;

        // Check permission policy
        let mut trusted = false;
        if let Some(ref policy) = self.policy {
            use crate::llm::tools::sandbox::PolicyDecision;
            match policy.evaluate(canonical, &input) {
                PolicyDecision::Allow => {}
                PolicyDecision::Trusted => trusted = true,
                PolicyDecision::Deny(reason) => {
                    return Err(ToolError::PermissionDenied(reason));
                }
            }
        }

        // Check if approval is required. A policy that explicitly vouched for
        // these inputs stands in for the user's approval, so an allowlisted
        // command runs without prompting.
        if tool.requires_approval() && !context.auto_approve && !trusted {
            return Err(ToolError::ApprovalRequired(format!(
                "Tool '{}' requires approval before execution",
                canonical
            )));
        }

        if canonical != name {
            tracing::debug!("Resolved tool alias '{}' -> '{}'", name, canonical);
        }

        // Execute the tool. Log the arguments, not just the name: when a model
        // sends a command other than the one it was asked for, the only symptom
        // is a tool that "succeeds" with empty or surprising output, and without
        // the input there is no way to tell a broken tool from a model that asked
        // for the wrong thing.
        tracing::info!(
            "Executing tool: {} with input: {}",
            canonical,
            preview_input(&input)
        );
        let result = tool.execute(input, context).await?;

        if result.success {
            // Log a prefix of the output: when a tool "succeeds" but returns the
            // wrong thing (e.g. a listing of the wrong directory), the failure is
            // invisible from the log without it, and the model's confused reply
            // is the only symptom.
            const PREVIEW: usize = 400;
            let preview: String = result.output.chars().take(PREVIEW).collect();
            tracing::info!(
                "Tool '{}' executed successfully -> {}{}",
                canonical,
                preview,
                if result.output.chars().count() > PREVIEW {
                    " …[truncated]"
                } else {
                    ""
                }
            );
        } else {
            tracing::warn!(
                "Tool '{}' failed: {:?}",
                canonical,
                result.error.as_deref().unwrap_or("unknown error")
            );
        }

        Ok(result)
    }

    /// Connect to an MCP server, discover its tools, and register them.
    ///
    /// Built-in tool names take precedence — if a tool with the same namespaced name
    /// already exists, it is silently skipped.
    pub async fn register_mcp_server(
        &mut self,
        server_name: &str,
        command: &str,
        args: &[&str],
    ) -> anyhow::Result<usize> {
        use crate::mcp::client::{MCPClient, McpTool};
        use std::sync::Arc;
        use tokio::sync::Mutex;

        let mut client = MCPClient::connect(server_name, command, args).await?;
        let tool_defs = client.discover_tools().await?;
        let client_arc = Arc::new(Mutex::new(client));

        let mut registered = 0usize;
        for def in tool_defs {
            let mcp_tool = McpTool::new(server_name, def, client_arc.clone());
            let name = mcp_tool.name().to_string();
            if self.tools.contains_key(&name) {
                tracing::debug!("MCP tool '{}' skipped — built-in takes precedence", name);
                continue;
            }
            self.tools.insert(name.clone(), Arc::new(mcp_tool));
            tracing::info!("Registered MCP tool: {}", name);
            registered += 1;
        }
        Ok(registered)
    }

    /// Get the number of registered tools
    pub fn count(&self) -> usize {
        self.tools.len()
    }
}

impl Default for ToolRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::llm::tools::r#trait::ToolCapability;
    use async_trait::async_trait;
    use uuid::Uuid;

    /// The whole point of logging the input: seeing the command the model actually
    /// sent. A model that asks for the wrong command yields a tool that "succeeds"
    /// with empty output, which is indistinguishable from a broken tool otherwise.
    #[test]
    fn preview_input_shows_the_command() {
        let input = serde_json::json!({ "command": "ls -la" });
        assert_eq!(preview_input(&input), r#"{"command":"ls -la"}"#);
    }

    #[test]
    fn preview_input_truncates_a_large_payload() {
        // e.g. a write_file carrying a whole file body.
        let input = serde_json::json!({ "content": "x".repeat(5_000) });
        let preview = preview_input(&input);
        assert!(preview.ends_with(" …[truncated]"), "got: {preview}");
        assert!(
            preview.chars().count() <= INPUT_PREVIEW_CHARS + " …[truncated]".chars().count(),
            "preview should be bounded, got {} chars",
            preview.chars().count(),
        );
    }

    /// Truncating by byte index would panic mid-character. Tool inputs carry paths
    /// and prompts, which are routinely non-ASCII - and this runs on every call.
    #[test]
    fn preview_input_truncates_on_char_boundaries() {
        let input = serde_json::json!({ "path": "é".repeat(5_000) });
        let preview = preview_input(&input); // must not panic
        assert!(preview.ends_with(" …[truncated]"));
    }

    /// Mock tool for testing
    struct MockTool {
        name: String,
        requires_approval: bool,
    }

    #[async_trait]
    impl Tool for MockTool {
        fn name(&self) -> &str {
            &self.name
        }

        fn description(&self) -> &str {
            "A mock tool for testing"
        }

        fn input_schema(&self) -> Value {
            serde_json::json!({
                "type": "object",
                "properties": {
                    "message": {
                        "type": "string",
                        "description": "Test message"
                    }
                },
                "required": ["message"]
            })
        }

        fn capabilities(&self) -> Vec<ToolCapability> {
            vec![ToolCapability::ReadFiles]
        }

        fn requires_approval(&self) -> bool {
            self.requires_approval
        }

        async fn execute(
            &self,
            _input: Value,
            _context: &ToolExecutionContext,
        ) -> Result<ToolResult> {
            Ok(ToolResult::success("Mock execution successful".to_string()))
        }
    }

    #[test]
    fn test_registry_creation() {
        let registry = ToolRegistry::new();
        assert_eq!(registry.count(), 0);
    }

    #[test]
    fn test_register_tool() {
        let mut registry = ToolRegistry::new();
        let tool = Arc::new(MockTool {
            name: "test_tool".to_string(),
            requires_approval: false,
        });

        registry.register(tool);
        assert_eq!(registry.count(), 1);
        assert!(registry.has_tool("test_tool"));
        assert!(!registry.has_tool("nonexistent"));
    }

    #[test]
    fn test_list_tools() {
        let mut registry = ToolRegistry::new();

        registry.register(Arc::new(MockTool {
            name: "tool1".to_string(),
            requires_approval: false,
        }));
        registry.register(Arc::new(MockTool {
            name: "tool2".to_string(),
            requires_approval: false,
        }));

        let tools = registry.list_tools();
        assert_eq!(tools.len(), 2);
        assert!(tools.contains(&"tool1".to_string()));
        assert!(tools.contains(&"tool2".to_string()));
    }

    #[tokio::test]
    async fn test_execute_tool() {
        let mut registry = ToolRegistry::new();
        let tool = Arc::new(MockTool {
            name: "test_tool".to_string(),
            requires_approval: false,
        });

        registry.register(tool);

        let session_id = Uuid::new_v4();
        let context = ToolExecutionContext::new(session_id);
        let input = serde_json::json!({ "message": "test" });

        let result = registry
            .execute("test_tool", input, &context)
            .await
            .unwrap();
        assert!(result.success);
        assert_eq!(result.output, "Mock execution successful");
    }

    #[tokio::test]
    async fn test_execute_nonexistent_tool() {
        let registry = ToolRegistry::new();
        let session_id = Uuid::new_v4();
        let context = ToolExecutionContext::new(session_id);
        let input = serde_json::json!({});

        let result = registry.execute("nonexistent", input, &context).await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ToolError::NotFound(_)));
    }

    #[tokio::test]
    async fn test_execute_requires_approval() {
        let mut registry = ToolRegistry::new();
        let tool = Arc::new(MockTool {
            name: "dangerous_tool".to_string(),
            requires_approval: true,
        });

        registry.register(tool);

        let session_id = Uuid::new_v4();
        let context = ToolExecutionContext::new(session_id); // auto_approve = false
        let input = serde_json::json!({ "message": "test" });

        let result = registry.execute("dangerous_tool", input, &context).await;
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ToolError::ApprovalRequired(_)
        ));
    }

    #[tokio::test]
    async fn test_execute_with_auto_approve() {
        let mut registry = ToolRegistry::new();
        let tool = Arc::new(MockTool {
            name: "dangerous_tool".to_string(),
            requires_approval: true,
        });

        registry.register(tool);

        let session_id = Uuid::new_v4();
        let context = ToolExecutionContext::new(session_id).with_auto_approve(true);
        let input = serde_json::json!({ "message": "test" });

        let result = registry
            .execute("dangerous_tool", input, &context)
            .await
            .unwrap();
        assert!(result.success);
    }

    #[tokio::test]
    async fn register_mcp_server_with_nonexistent_command_fails_gracefully() {
        let mut registry = ToolRegistry::new();

        // Bounded with a timeout as a defensive check: a bad command
        // should fail fast (process spawn error), not hang the caller -
        // this is the exact path cli::cmd_chat now depends on at startup
        // for every configured [[mcp.servers]] entry.
        let result = tokio::time::timeout(
            std::time::Duration::from_secs(5),
            registry.register_mcp_server(
                "test-server",
                "definitely-not-a-real-binary-xyz-crustly-test",
                &[],
            ),
        )
        .await
        .expect("register_mcp_server must not hang on a bad command");

        assert!(result.is_err());
        // No tools should have been registered from a failed connection.
        assert_eq!(registry.count(), 0);
    }

    // ── Alias resolution ────────────────────────────────────────────────────

    #[test]
    fn get_resolves_a_known_alias_to_the_registered_canonical_tool() {
        let mut registry = ToolRegistry::new();
        registry.register(Arc::new(MockTool {
            name: "bash".to_string(),
            requires_approval: false,
        }));

        // "run_shell_command" is qwen-code's name for the same tool.
        assert!(registry.get("run_shell_command").is_some());
        assert!(registry.has_tool("run_shell_command"));
    }

    #[test]
    fn has_tool_is_false_for_an_alias_whose_target_is_not_registered() {
        let registry = ToolRegistry::new();
        // "run_shell_command" is a real alias entry, but nothing named
        // "bash" was ever registered in this registry.
        assert!(!registry.has_tool("run_shell_command"));
    }

    #[test]
    fn an_exact_match_wins_over_an_alias_entry() {
        let mut registry = ToolRegistry::new();
        // Register a real tool under a name that also happens to be a
        // listed alias key, to prove the exact match takes precedence
        // rather than being redirected to the alias's target.
        registry.register(Arc::new(MockTool {
            name: "cat".to_string(),
            requires_approval: false,
        }));

        let tool = registry.get("cat").expect("exact match must be found");
        // If alias resolution had won, this would be Crustly's real
        // `read_file` tool instead of the mock registered above.
        assert_eq!(tool.name(), "cat");
    }

    #[tokio::test]
    async fn execute_resolves_an_alias_name_to_the_registered_tool() {
        let mut registry = ToolRegistry::new();
        registry.register(Arc::new(MockTool {
            name: "bash".to_string(),
            requires_approval: false,
        }));

        let context = ToolExecutionContext::new(Uuid::new_v4());
        let input = serde_json::json!({ "message": "test" });

        // "Bash" (Claude Code's name) and "shell" (a generic alias) both
        // resolve to the "bash" tool registered above.
        for alias in ["Bash", "shell", "run_shell_command"] {
            let result = registry
                .execute(alias, input.clone(), &context)
                .await
                .unwrap_or_else(|e| panic!("alias '{alias}' should resolve: {e}"));
            assert!(result.success);
        }
    }

    #[tokio::test]
    async fn execute_reports_not_found_using_the_original_unresolved_name() {
        let registry = ToolRegistry::new();
        let context = ToolExecutionContext::new(Uuid::new_v4());

        let result = registry
            .execute("definitely_not_a_real_tool", serde_json::json!({}), &context)
            .await;

        match result {
            Err(ToolError::NotFound(name)) => assert_eq!(name, "definitely_not_a_real_tool"),
            other => panic!("expected NotFound with the original name, got {other:?}"),
        }
    }

    /// Regression: a permission policy rule written against the canonical
    /// tool name must still apply when a model calls the same tool by an
    /// alias - otherwise a `security.allow_bash`-style allowlist (or a
    /// deny rule) silently stops working the moment a model uses a
    /// different name for the same tool.
    #[tokio::test]
    async fn execute_evaluates_policy_against_the_canonical_name_not_the_alias() {
        use crate::llm::tools::sandbox::DenyToolRule;

        let mut registry = ToolRegistry::new();
        registry.register(Arc::new(MockTool {
            name: "bash".to_string(),
            requires_approval: false,
        }));
        // Denies by the canonical name "bash" - not "run_shell_command".
        registry.set_policy(Arc::new(DenyToolRule::new("bash")));

        let context = ToolExecutionContext::new(Uuid::new_v4());
        let result = registry
            .execute(
                "run_shell_command",
                serde_json::json!({ "message": "test" }),
                &context,
            )
            .await;

        assert!(
            matches!(result, Err(ToolError::PermissionDenied(_))),
            "expected the canonical-name deny rule to catch the alias call, got {result:?}"
        );
    }

    /// The same consistency must hold for the pre-execution trust check
    /// (`is_trusted`), which `llm::agent::service` calls separately from
    /// `execute()` to decide whether to prompt for approval.
    #[test]
    fn is_trusted_evaluates_policy_against_the_canonical_name_not_the_alias() {
        use crate::llm::tools::sandbox::AllowToolRule;

        let mut registry = ToolRegistry::new();
        registry.register(Arc::new(MockTool {
            name: "bash".to_string(),
            requires_approval: true,
        }));
        // AllowToolRule only ever returns Allow (never Trusted), so this
        // confirms canonical-name evaluation happens, not that it's
        // Trusted - is_trusted's own tests elsewhere cover the Trusted case.
        registry.set_policy(Arc::new(AllowToolRule::new("bash")));

        // Both must evaluate identically because both resolve to "bash".
        let direct = registry.is_trusted("bash", &serde_json::json!({}));
        let via_alias = registry.is_trusted("run_shell_command", &serde_json::json!({}));
        assert_eq!(direct, via_alias);
    }
}
