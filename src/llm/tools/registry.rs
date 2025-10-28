//! Tool Registry
//!
//! Manages the collection of available tools that can be invoked by agents.

use super::error::{Result, ToolError};
use super::r#trait::{Tool, ToolExecutionContext, ToolResult};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;

/// Registry of available tools
pub struct ToolRegistry {
    tools: HashMap<String, Arc<dyn Tool>>,
}

impl ToolRegistry {
    /// Create a new empty tool registry
    pub fn new() -> Self {
        Self {
            tools: HashMap::new(),
        }
    }

    /// Register a tool
    pub fn register(&mut self, tool: Arc<dyn Tool>) {
        let name = tool.name().to_string();
        tracing::debug!("Registered tool: {}", name);
        self.tools.insert(name, tool);
    }

    /// Get a tool by name
    pub fn get(&self, name: &str) -> Option<Arc<dyn Tool>> {
        self.tools.get(name).cloned()
    }

    /// Check if a tool is registered
    pub fn has_tool(&self, name: &str) -> bool {
        self.tools.contains_key(name)
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

    /// Execute a tool by name
    pub async fn execute(
        &self,
        name: &str,
        input: Value,
        context: &ToolExecutionContext,
    ) -> Result<ToolResult> {
        let tool = self
            .get(name)
            .ok_or_else(|| ToolError::NotFound(name.to_string()))?;

        // Validate input
        tool.validate_input(&input)?;

        // Check if approval is required
        if tool.requires_approval() && !context.auto_approve {
            return Err(ToolError::ApprovalRequired(format!(
                "Tool '{}' requires approval before execution",
                name
            )));
        }

        // Execute the tool
        tracing::info!("Executing tool: {}", name);
        let result = tool.execute(input, context).await?;

        if result.success {
            tracing::info!("Tool '{}' executed successfully", name);
        } else {
            tracing::warn!(
                "Tool '{}' failed: {:?}",
                name,
                result.error.as_deref().unwrap_or("unknown error")
            );
        }

        Ok(result)
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

        let result = registry.execute("test_tool", input, &context).await.unwrap();
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
}
