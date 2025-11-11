//! Agent Service Implementation
//!
//! Core service for managing AI agent conversations, coordinating between
//! LLM providers, context management, and data persistence.

use super::context::AgentContext;
use super::error::{AgentError, Result};
use crate::llm::provider::{
    ContentBlock, LLMRequest, LLMResponse, Message, Provider, ProviderStream, StopReason,
};
use crate::llm::tools::{ToolExecutionContext, ToolRegistry};
use crate::services::{MessageService, ServiceContext, SessionService};
use serde_json::Value;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use uuid::Uuid;

/// Tool approval request information
#[derive(Debug, Clone)]
pub struct ToolApprovalInfo {
    /// Tool name
    pub tool_name: String,
    /// Tool description
    pub tool_description: String,
    /// Tool input parameters
    pub tool_input: Value,
    /// Tool capabilities
    pub capabilities: Vec<String>,
}

/// Type alias for approval callback function
/// Returns true if approved, false if denied
pub type ApprovalCallback = Arc<
    dyn Fn(ToolApprovalInfo) -> Pin<Box<dyn Future<Output = Result<bool>> + Send>> + Send + Sync,
>;

/// Agent Service for managing AI conversations
pub struct AgentService {
    /// LLM provider
    provider: Arc<dyn Provider>,

    /// Service context for database operations
    context: ServiceContext,

    /// Tool registry for executing tools
    tool_registry: Arc<ToolRegistry>,

    /// Maximum tool execution iterations
    max_tool_iterations: usize,

    /// System prompt template
    default_system_prompt: Option<String>,

    /// Whether to auto-approve tool execution
    auto_approve_tools: bool,

    /// Callback for requesting tool approval from user
    approval_callback: Option<ApprovalCallback>,
}

impl AgentService {
    /// Create a new agent service
    pub fn new(provider: Arc<dyn Provider>, context: ServiceContext) -> Self {
        Self {
            provider,
            context,
            tool_registry: Arc::new(ToolRegistry::new()),
            max_tool_iterations: 10,
            default_system_prompt: None,
            auto_approve_tools: false,
            approval_callback: None,
        }
    }

    /// Set the default system prompt
    pub fn with_system_prompt(mut self, prompt: String) -> Self {
        self.default_system_prompt = Some(prompt);
        self
    }

    /// Set maximum tool iterations
    pub fn with_max_tool_iterations(mut self, max: usize) -> Self {
        self.max_tool_iterations = max;
        self
    }

    /// Set the tool registry
    pub fn with_tool_registry(mut self, registry: Arc<ToolRegistry>) -> Self {
        self.tool_registry = registry;
        self
    }

    /// Set whether to auto-approve tool execution
    pub fn with_auto_approve_tools(mut self, auto_approve: bool) -> Self {
        self.auto_approve_tools = auto_approve;
        self
    }

    /// Set the approval callback for interactive tool approval
    pub fn with_approval_callback(mut self, callback: Option<ApprovalCallback>) -> Self {
        self.approval_callback = callback;
        self
    }

    /// Send a message and get a response
    ///
    /// This will:
    /// 1. Load conversation context from the database
    /// 2. Add the new user message
    /// 3. Send to the LLM provider
    /// 4. Save the response to the database
    /// 5. Update token usage
    pub async fn send_message(
        &self,
        session_id: Uuid,
        user_message: String,
        model: Option<String>,
    ) -> Result<AgentResponse> {
        // Get or create session
        let session_service = SessionService::new(self.context.clone());
        let _session = session_service
            .get_session(session_id)
            .await
            .map_err(|e| AgentError::Database(e.to_string()))?
            .ok_or(AgentError::SessionNotFound(session_id))?;

        // Load conversation context
        let message_service = MessageService::new(self.context.clone());
        let db_messages = message_service
            .list_messages_for_session(session_id)
            .await
            .map_err(|e| AgentError::Database(e.to_string()))?;

        let model_name = model.unwrap_or_else(|| self.provider.default_model().to_string());
        let context_window = self.provider.context_window(&model_name).unwrap_or(4096);

        let mut context =
            AgentContext::from_db_messages(session_id, db_messages, context_window as usize);

        // Add system prompt if available
        if let Some(system_prompt) = &self.default_system_prompt {
            context.system_prompt = Some(system_prompt.clone());
        }

        // Add user message
        let user_msg = Message::user(user_message.clone());
        context.add_message(user_msg.clone());

        // Save user message to database
        let _user_db_msg = message_service
            .create_message(session_id, "user".to_string(), user_message)
            .await
            .map_err(|e| AgentError::Database(e.to_string()))?;

        // Build LLM request
        let request =
            LLMRequest::new(model_name.clone(), context.messages.clone()).with_max_tokens(4096);

        let request = if let Some(system) = context.system_prompt {
            request.with_system(system)
        } else {
            request
        };

        // Send to provider
        let response = self
            .provider
            .complete(request)
            .await
            .map_err(AgentError::Provider)?;

        // Extract text from response
        let assistant_text = Self::extract_text_from_response(&response);

        // Save assistant response to database
        let assistant_db_msg = message_service
            .create_message(session_id, "assistant".to_string(), assistant_text.clone())
            .await
            .map_err(|e| AgentError::Database(e.to_string()))?;

        // Calculate total tokens and cost for this message
        let total_tokens = response.usage.input_tokens + response.usage.output_tokens;
        let cost = self.provider.calculate_cost(
            &response.model,
            response.usage.input_tokens,
            response.usage.output_tokens,
        );

        // Update message with usage info
        message_service
            .update_message_usage(assistant_db_msg.id, total_tokens as i32, cost)
            .await
            .map_err(|e| AgentError::Database(e.to_string()))?;

        // Update session token usage
        session_service
            .update_session_usage(session_id, total_tokens as i32, cost)
            .await
            .map_err(|e| AgentError::Database(e.to_string()))?;

        Ok(AgentResponse {
            message_id: assistant_db_msg.id,
            content: assistant_text,
            stop_reason: response.stop_reason,
            usage: response.usage,
            cost,
            model: response.model,
        })
    }

    /// Send a message and get a streaming response
    ///
    /// Returns a stream of response chunks that can be consumed incrementally.
    pub async fn send_message_streaming(
        &self,
        session_id: Uuid,
        user_message: String,
        model: Option<String>,
    ) -> Result<AgentStreamResponse> {
        // Get or create session
        let session_service = SessionService::new(self.context.clone());
        let _session = session_service
            .get_session(session_id)
            .await
            .map_err(|e| AgentError::Database(e.to_string()))?
            .ok_or(AgentError::SessionNotFound(session_id))?;

        // Load conversation context
        let message_service = MessageService::new(self.context.clone());
        let db_messages = message_service
            .list_messages_for_session(session_id)
            .await
            .map_err(|e| AgentError::Database(e.to_string()))?;

        let model_name = model.unwrap_or_else(|| self.provider.default_model().to_string());
        let context_window = self.provider.context_window(&model_name).unwrap_or(4096);

        let mut context =
            AgentContext::from_db_messages(session_id, db_messages, context_window as usize);

        // Add system prompt if available
        if let Some(system_prompt) = &self.default_system_prompt {
            context.system_prompt = Some(system_prompt.clone());
        }

        // Add user message
        let user_msg = Message::user(user_message.clone());
        context.add_message(user_msg);

        // Save user message to database
        message_service
            .create_message(session_id, "user".to_string(), user_message)
            .await
            .map_err(|e| AgentError::Database(e.to_string()))?;

        // Build LLM request
        let request = LLMRequest::new(model_name.clone(), context.messages.clone())
            .with_max_tokens(4096)
            .with_streaming();

        let request = if let Some(system) = context.system_prompt {
            request.with_system(system)
        } else {
            request
        };

        // Get streaming response from provider
        let stream = self
            .provider
            .stream(request)
            .await
            .map_err(AgentError::Provider)?;

        Ok(AgentStreamResponse {
            session_id,
            message_id: Uuid::new_v4(),
            stream,
            model: model_name,
        })
    }

    /// Send a message with automatic tool execution
    ///
    /// This method implements a tool execution loop:
    /// 1. Send message to LLM
    /// 2. If LLM requests tool use, execute the tool
    /// 3. Send tool results back to LLM
    /// 4. Repeat until LLM finishes or max iterations reached
    pub async fn send_message_with_tools(
        &self,
        session_id: Uuid,
        user_message: String,
        model: Option<String>,
    ) -> Result<AgentResponse> {
        self.send_message_with_tools_and_mode(session_id, user_message, model, false)
            .await
    }

    /// Send a message with automatic tool execution and explicit read-only mode control
    pub async fn send_message_with_tools_and_mode(
        &self,
        session_id: Uuid,
        user_message: String,
        model: Option<String>,
        read_only_mode: bool,
    ) -> Result<AgentResponse> {
        // Get or create session
        let session_service = SessionService::new(self.context.clone());
        let _session = session_service
            .get_session(session_id)
            .await
            .map_err(|e| AgentError::Database(e.to_string()))?
            .ok_or(AgentError::SessionNotFound(session_id))?;

        // Load conversation context
        let message_service = MessageService::new(self.context.clone());
        let db_messages = message_service
            .list_messages_for_session(session_id)
            .await
            .map_err(|e| AgentError::Database(e.to_string()))?;

        let model_name = model.unwrap_or_else(|| self.provider.default_model().to_string());
        let context_window = self.provider.context_window(&model_name).unwrap_or(4096);

        let mut context =
            AgentContext::from_db_messages(session_id, db_messages, context_window as usize);

        // Add system prompt if available
        if let Some(system_prompt) = &self.default_system_prompt {
            context.system_prompt = Some(system_prompt.clone());
        }

        // Add user message
        let user_msg = Message::user(user_message.clone());
        context.add_message(user_msg);

        // Save user message to database
        let _user_db_msg = message_service
            .create_message(session_id, "user".to_string(), user_message)
            .await
            .map_err(|e| AgentError::Database(e.to_string()))?;

        // Create tool execution context
        let tool_context = ToolExecutionContext::new(session_id)
            .with_auto_approve(self.auto_approve_tools)
            .with_working_directory(std::env::current_dir().unwrap_or_default())
            .with_read_only_mode(read_only_mode);

        // Tool execution loop
        let mut iteration = 0;
        let mut total_input_tokens = 0u32;
        let mut total_output_tokens = 0u32;
        let mut final_response: Option<LLMResponse> = None;

        while iteration < self.max_tool_iterations {
            iteration += 1;

            // Build LLM request with tools if available
            let mut request =
                LLMRequest::new(model_name.clone(), context.messages.clone()).with_max_tokens(4096);

            if let Some(system) = &context.system_prompt {
                request = request.with_system(system.clone());
            }

            // Add tools if registry has any
            let tool_count = self.tool_registry.count();
            tracing::debug!("Tool registry contains {} tools", tool_count);
            if tool_count > 0 {
                let tool_defs = self.tool_registry.get_tool_definitions();
                tracing::debug!("Adding {} tool definitions to request", tool_defs.len());
                request = request.with_tools(tool_defs);
            } else {
                tracing::warn!("No tools registered in tool registry!");
            }

            // Send to provider
            let response = self
                .provider
                .complete(request)
                .await
                .map_err(AgentError::Provider)?;

            // Track token usage
            total_input_tokens += response.usage.input_tokens;
            total_output_tokens += response.usage.output_tokens;

            // Check if response contains tool use
            tracing::debug!("Response has {} content blocks", response.content.len());
            for (i, block) in response.content.iter().enumerate() {
                match block {
                    ContentBlock::Text { text } => {
                        tracing::debug!(
                            "Block {}: Text ({}...)",
                            i,
                            &text.chars().take(50).collect::<String>()
                        );
                    }
                    ContentBlock::ToolUse { id, name, input: _ } => {
                        tracing::debug!("Block {}: ToolUse {{ name: {}, id: {} }}", i, name, id);
                    }
                    _ => {
                        tracing::debug!("Block {}: Other content block", i);
                    }
                }
            }

            let tool_uses: Vec<_> = response
                .content
                .iter()
                .filter_map(|block| {
                    if let ContentBlock::ToolUse { id, name, input } = block {
                        Some((id.clone(), name.clone(), input.clone()))
                    } else {
                        None
                    }
                })
                .collect();

            tracing::debug!("Found {} tool uses to execute", tool_uses.len());

            if tool_uses.is_empty() {
                // No tool use - we're done
                tracing::debug!("No tool uses found, completing with final response");
                final_response = Some(response);
                break;
            }

            // Execute tools and build response message
            let mut tool_results = Vec::new();

            for (tool_id, tool_name, tool_input) in tool_uses {
                tracing::info!(
                    "Executing tool '{}' (iteration {}/{})",
                    tool_name,
                    iteration,
                    self.max_tool_iterations
                );

                // Check if approval is needed
                let needs_approval = if let Some(tool) = self.tool_registry.get(&tool_name) {
                    tool.requires_approval()
                        && !self.auto_approve_tools
                        && !tool_context.auto_approve
                } else {
                    false
                };

                // Request approval if needed
                if needs_approval {
                    if let Some(ref approval_callback) = self.approval_callback {
                        // Get tool details for approval request
                        let tool_info = if let Some(tool) = self.tool_registry.get(&tool_name) {
                            ToolApprovalInfo {
                                tool_name: tool_name.clone(),
                                tool_description: tool.description().to_string(),
                                tool_input: tool_input.clone(),
                                capabilities: tool
                                    .capabilities()
                                    .iter()
                                    .map(|c| format!("{:?}", c))
                                    .collect(),
                            }
                        } else {
                            // Tool not found, skip approval
                            tool_results.push(ContentBlock::ToolResult {
                                tool_use_id: tool_id,
                                content: format!("Tool not found: {}", tool_name),
                                is_error: Some(true),
                            });
                            continue;
                        };

                        // Call approval callback
                        tracing::info!("Requesting user approval for tool '{}'", tool_name);
                        match approval_callback(tool_info).await {
                            Ok(approved) => {
                                if !approved {
                                    tracing::warn!("User denied approval for tool '{}'", tool_name);
                                    tool_results.push(ContentBlock::ToolResult {
                                        tool_use_id: tool_id,
                                        content: "User denied permission to execute this tool"
                                            .to_string(),
                                        is_error: Some(true),
                                    });
                                    continue;
                                }
                                tracing::info!("User approved tool '{}'", tool_name);
                                // Create approved context for this tool execution
                                let approved_tool_context = ToolExecutionContext {
                                    session_id: tool_context.session_id,
                                    working_directory: tool_context.working_directory.clone(),
                                    env_vars: tool_context.env_vars.clone(),
                                    auto_approve: true, // User approved this execution
                                    timeout_secs: tool_context.timeout_secs,
                                    read_only_mode: tool_context.read_only_mode,
                                };

                                // Execute the tool with approved context
                                match self
                                    .tool_registry
                                    .execute(&tool_name, tool_input, &approved_tool_context)
                                    .await
                                {
                                    Ok(result) => {
                                        tool_results.push(ContentBlock::ToolResult {
                                            tool_use_id: tool_id,
                                            content: if result.success {
                                                result.output
                                            } else {
                                                result.error.unwrap_or_else(|| {
                                                    "Tool execution failed".to_string()
                                                })
                                            },
                                            is_error: Some(!result.success),
                                        });
                                    }
                                    Err(e) => {
                                        tool_results.push(ContentBlock::ToolResult {
                                            tool_use_id: tool_id,
                                            content: format!("Tool execution error: {}", e),
                                            is_error: Some(true),
                                        });
                                    }
                                }
                                continue; // Skip the normal execution path below
                            }
                            Err(e) => {
                                tracing::error!("Approval callback error: {}", e);
                                tool_results.push(ContentBlock::ToolResult {
                                    tool_use_id: tool_id,
                                    content: format!("Approval request failed: {}", e),
                                    is_error: Some(true),
                                });
                                continue;
                            }
                        }
                    } else {
                        // No approval callback configured, deny execution
                        tracing::warn!(
                            "Tool '{}' requires approval but no approval callback configured",
                            tool_name
                        );
                        tool_results.push(ContentBlock::ToolResult {
                            tool_use_id: tool_id,
                            content: "Tool requires approval but no approval mechanism configured"
                                .to_string(),
                            is_error: Some(true),
                        });
                        continue;
                    }
                }

                // Execute the tool
                match self
                    .tool_registry
                    .execute(&tool_name, tool_input, &tool_context)
                    .await
                {
                    Ok(result) => {
                        tool_results.push(ContentBlock::ToolResult {
                            tool_use_id: tool_id,
                            content: if result.success {
                                result.output
                            } else {
                                result
                                    .error
                                    .unwrap_or_else(|| "Tool execution failed".to_string())
                            },
                            is_error: Some(!result.success),
                        });
                    }
                    Err(e) => {
                        tool_results.push(ContentBlock::ToolResult {
                            tool_use_id: tool_id,
                            content: format!("Tool execution error: {}", e),
                            is_error: Some(true),
                        });
                    }
                }
            }

            // Add assistant message with tool use to context
            let assistant_msg = Message {
                role: crate::llm::provider::Role::Assistant,
                content: response.content.clone(),
            };
            context.add_message(assistant_msg);

            // Add user message with tool results to context
            let tool_result_msg = Message {
                role: crate::llm::provider::Role::User,
                content: tool_results,
            };
            context.add_message(tool_result_msg);

            // Check if we've hit max iterations
            if iteration >= self.max_tool_iterations {
                return Err(AgentError::MaxIterationsExceeded(self.max_tool_iterations));
            }
        }

        let response = final_response.ok_or_else(|| {
            AgentError::Internal("Tool loop completed without final response".to_string())
        })?;

        // Extract text from final response
        let assistant_text = Self::extract_text_from_response(&response);

        // Save final assistant response to database
        let assistant_db_msg = message_service
            .create_message(session_id, "assistant".to_string(), assistant_text.clone())
            .await
            .map_err(|e| AgentError::Database(e.to_string()))?;

        // Calculate total cost
        let total_tokens = total_input_tokens + total_output_tokens;
        let cost =
            self.provider
                .calculate_cost(&response.model, total_input_tokens, total_output_tokens);

        // Update message with usage info
        message_service
            .update_message_usage(assistant_db_msg.id, total_tokens as i32, cost)
            .await
            .map_err(|e| AgentError::Database(e.to_string()))?;

        // Update session token usage
        session_service
            .update_session_usage(session_id, total_tokens as i32, cost)
            .await
            .map_err(|e| AgentError::Database(e.to_string()))?;

        Ok(AgentResponse {
            message_id: assistant_db_msg.id,
            content: assistant_text,
            stop_reason: response.stop_reason,
            usage: crate::llm::provider::TokenUsage {
                input_tokens: total_input_tokens,
                output_tokens: total_output_tokens,
            },
            cost,
            model: response.model,
        })
    }

    /// Extract text content from an LLM response
    fn extract_text_from_response(response: &LLMResponse) -> String {
        let mut text = String::new();

        for content in &response.content {
            match content {
                ContentBlock::Text { text: t } => {
                    text.push_str(t);
                }
                ContentBlock::ToolUse { name, input, .. } => {
                    // Format tool use for display
                    text.push_str(&format!("\n[Tool: {}]\n{}\n", name, input));
                }
                _ => {}
            }
        }

        text
    }
}

/// Response from the agent
#[derive(Debug, Clone)]
pub struct AgentResponse {
    /// Message ID in database
    pub message_id: Uuid,

    /// Response content
    pub content: String,

    /// Stop reason
    pub stop_reason: Option<StopReason>,

    /// Token usage
    pub usage: crate::llm::provider::TokenUsage,

    /// Cost in USD
    pub cost: f64,

    /// Model used
    pub model: String,
}

/// Streaming response from the agent
pub struct AgentStreamResponse {
    /// Session ID
    pub session_id: Uuid,

    /// Message ID that will be created
    pub message_id: Uuid,

    /// Stream of events
    pub stream: ProviderStream,

    /// Model being used
    pub model: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::Database;
    use crate::llm::provider::{LLMRequest, LLMResponse, TokenUsage};
    use async_trait::async_trait;

    /// Mock provider for testing
    struct MockProvider;

    #[async_trait]
    impl Provider for MockProvider {
        async fn complete(
            &self,
            _request: LLMRequest,
        ) -> crate::llm::provider::Result<LLMResponse> {
            Ok(LLMResponse {
                id: "test-response-1".to_string(),
                model: "mock-model".to_string(),
                content: vec![ContentBlock::Text {
                    text: "This is a test response".to_string(),
                }],
                stop_reason: Some(StopReason::EndTurn),
                usage: TokenUsage {
                    input_tokens: 10,
                    output_tokens: 20,
                },
            })
        }

        async fn stream(
            &self,
            _request: LLMRequest,
        ) -> crate::llm::provider::Result<ProviderStream> {
            unimplemented!("Streaming not needed for basic tests")
        }

        fn name(&self) -> &str {
            "mock"
        }

        fn default_model(&self) -> &str {
            "mock-model"
        }

        fn supported_models(&self) -> Vec<String> {
            vec!["mock-model".to_string()]
        }

        fn context_window(&self, _model: &str) -> Option<u32> {
            Some(4096)
        }

        fn calculate_cost(&self, _model: &str, _input: u32, _output: u32) -> f64 {
            0.001 // Mock cost
        }
    }

    async fn create_test_service() -> (AgentService, Uuid) {
        let db = Database::connect_in_memory().await.unwrap();
        db.run_migrations().await.unwrap();
        let pool = db.pool().clone();

        let context = ServiceContext::new(pool);
        let provider = Arc::new(MockProvider);

        let agent_service = AgentService::new(provider, context.clone());

        // Create a test session
        let session_service = SessionService::new(context);
        let session = session_service
            .create_session(Some("Test Session".to_string()))
            .await
            .unwrap();

        (agent_service, session.id)
    }

    #[tokio::test]
    async fn test_agent_service_creation() {
        let (agent_service, _) = create_test_service().await;
        assert_eq!(agent_service.max_tool_iterations, 10);
    }

    #[tokio::test]
    async fn test_send_message() {
        let (agent_service, session_id) = create_test_service().await;

        let response = agent_service
            .send_message(session_id, "Hello, world!".to_string(), None)
            .await
            .unwrap();

        assert!(!response.content.is_empty());
        assert_eq!(response.model, "mock-model");
        assert!(response.cost > 0.0);
    }

    #[tokio::test]
    async fn test_send_message_with_system_prompt() {
        let (agent_service, session_id) = create_test_service().await;

        let agent_service =
            agent_service.with_system_prompt("You are a helpful assistant.".to_string());

        let response = agent_service
            .send_message(session_id, "Hello!".to_string(), None)
            .await
            .unwrap();

        assert!(!response.content.is_empty());
    }

    /// Mock provider that simulates tool use
    struct MockProviderWithTools {
        call_count: std::sync::Mutex<usize>,
    }

    impl MockProviderWithTools {
        fn new() -> Self {
            Self {
                call_count: std::sync::Mutex::new(0),
            }
        }
    }

    #[async_trait]
    impl Provider for MockProviderWithTools {
        async fn complete(
            &self,
            _request: LLMRequest,
        ) -> crate::llm::provider::Result<LLMResponse> {
            let mut count = self.call_count.lock().unwrap();
            *count += 1;
            let call_num = *count;

            if call_num == 1 {
                // First call: request tool use
                Ok(LLMResponse {
                    id: "test-response-1".to_string(),
                    model: "mock-model".to_string(),
                    content: vec![
                        ContentBlock::Text {
                            text: "I'll use the test tool.".to_string(),
                        },
                        ContentBlock::ToolUse {
                            id: "tool-1".to_string(),
                            name: "test_tool".to_string(),
                            input: serde_json::json!({"message": "test"}),
                        },
                    ],
                    stop_reason: Some(StopReason::ToolUse),
                    usage: TokenUsage {
                        input_tokens: 10,
                        output_tokens: 20,
                    },
                })
            } else {
                // Second call: final response after tool execution
                Ok(LLMResponse {
                    id: "test-response-2".to_string(),
                    model: "mock-model".to_string(),
                    content: vec![ContentBlock::Text {
                        text: "Tool execution completed successfully.".to_string(),
                    }],
                    stop_reason: Some(StopReason::EndTurn),
                    usage: TokenUsage {
                        input_tokens: 15,
                        output_tokens: 25,
                    },
                })
            }
        }

        async fn stream(
            &self,
            _request: LLMRequest,
        ) -> crate::llm::provider::Result<ProviderStream> {
            unimplemented!("Streaming not needed for tool tests")
        }

        fn name(&self) -> &str {
            "mock-with-tools"
        }

        fn default_model(&self) -> &str {
            "mock-model"
        }

        fn supported_models(&self) -> Vec<String> {
            vec!["mock-model".to_string()]
        }

        fn context_window(&self, _model: &str) -> Option<u32> {
            Some(4096)
        }

        fn calculate_cost(&self, _model: &str, _input: u32, _output: u32) -> f64 {
            0.001
        }
    }

    /// Mock tool for testing
    struct MockTool;

    #[async_trait]
    impl crate::llm::tools::Tool for MockTool {
        fn name(&self) -> &str {
            "test_tool"
        }

        fn description(&self) -> &str {
            "A test tool"
        }

        fn input_schema(&self) -> serde_json::Value {
            serde_json::json!({
                "type": "object",
                "properties": {
                    "message": {"type": "string"}
                }
            })
        }

        fn capabilities(&self) -> Vec<crate::llm::tools::ToolCapability> {
            vec![]
        }

        fn requires_approval(&self) -> bool {
            false
        }

        async fn execute(
            &self,
            _input: serde_json::Value,
            _context: &crate::llm::tools::ToolExecutionContext,
        ) -> crate::llm::tools::Result<crate::llm::tools::ToolResult> {
            Ok(crate::llm::tools::ToolResult::success(
                "Tool executed successfully".to_string(),
            ))
        }
    }

    #[tokio::test]
    async fn test_send_message_with_tool_execution() {
        let db = Database::connect_in_memory().await.unwrap();
        db.run_migrations().await.unwrap();
        let pool = db.pool().clone();

        let context = ServiceContext::new(pool);
        let provider = Arc::new(MockProviderWithTools::new());

        // Create tool registry and register our test tool
        let mut registry = ToolRegistry::new();
        registry.register(Arc::new(MockTool));

        let agent_service = AgentService::new(provider, context.clone())
            .with_tool_registry(Arc::new(registry))
            .with_auto_approve_tools(true);

        // Create a test session
        let session_service = SessionService::new(context);
        let session = session_service
            .create_session(Some("Test Session".to_string()))
            .await
            .unwrap();

        // Send message with tool execution
        let response = agent_service
            .send_message_with_tools(session.id, "Use the test tool".to_string(), None)
            .await
            .unwrap();

        assert!(!response.content.is_empty());
        assert!(response.content.contains("completed successfully"));
        assert_eq!(response.model, "mock-model");
        // Should have tokens from both calls
        assert!(response.usage.input_tokens >= 25); // 10 + 15
        assert!(response.usage.output_tokens >= 45); // 20 + 25
    }
}
