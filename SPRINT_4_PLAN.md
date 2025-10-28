# Sprint 4: LLM Integration - Implementation Plan

## Overview
Implement the LLM integration layer with provider abstraction, agent service, and tool execution framework.

## Architecture

```
┌─────────────────────────────────────────────────┐
│           Application Layer                      │
│  (CLI, TUI - uses AgentService)                 │
└─────────────────┬───────────────────────────────┘
                  │
┌─────────────────▼───────────────────────────────┐
│         Agent Service Layer                      │
│  - Message orchestration                         │
│  - Tool execution approval                       │
│  - Response streaming                            │
│  - Session management                            │
└─────────────────┬───────────────────────────────┘
                  │
┌─────────────────▼───────────────────────────────┐
│       Provider Abstraction Layer                │
│  - Provider trait                                │
│  - Request/Response types                        │
│  - Streaming support                             │
│  - Error handling                                │
└─────────────────┬───────────────────────────────┘
                  │
     ┌────────────┼────────────┬─────────────┐
     │            │            │             │
┌────▼────┐  ┌───▼────┐  ┌───▼────┐   ┌───▼────┐
│Anthropic│  │OpenAI  │  │Gemini  │   │Bedrock │
│Provider │  │Provider│  │Provider│   │Provider│
└─────────┘  └────────┘  └────────┘   └────────┘
```

## Components to Implement

### 1. Provider Layer (`src/llm/provider/`)

**Files:**
- `mod.rs` - Module exports
- `types.rs` - Common types (Message, Role, ToolCall, etc.)
- `trait.rs` - Provider trait definition
- `anthropic.rs` - Anthropic/Claude implementation
- `openai.rs` - OpenAI implementation (future)
- `error.rs` - Provider-specific errors

**Key Types:**
```rust
pub struct LLMRequest {
    pub model: String,
    pub messages: Vec<Message>,
    pub system: Option<String>,
    pub tools: Option<Vec<Tool>>,
    pub temperature: Option<f32>,
    pub max_tokens: Option<u32>,
    pub stream: bool,
}

pub struct LLMResponse {
    pub id: String,
    pub model: String,
    pub content: Vec<ContentBlock>,
    pub stop_reason: Option<StopReason>,
    pub usage: TokenUsage,
}

pub enum ContentBlock {
    Text { text: String },
    ToolUse { id: String, name: String, input: Value },
}

#[async_trait]
pub trait Provider: Send + Sync {
    async fn complete(&self, request: LLMRequest) -> Result<LLMResponse>;
    async fn stream(&self, request: LLMRequest) -> Result<impl Stream<Item = Result<StreamEvent>>>;
    fn supports_streaming(&self) -> bool;
    fn supports_tools(&self) -> bool;
    fn name(&self) -> &str;
}
```

### 2. Agent Service (`src/llm/agent/`)

**Files:**
- `mod.rs` - AgentService implementation
- `types.rs` - Agent-specific types
- `context.rs` - Agent context management

**Responsibilities:**
- Manage conversation context
- Handle tool execution approval
- Stream responses to user
- Track token usage
- Save messages to database

**Key Methods:**
```rust
impl AgentService {
    pub async fn send_message(&self, session_id: Uuid, content: String) -> Result<Response>;
    pub async fn stream_message(&self, session_id: Uuid, content: String) -> Result<ResponseStream>;
    pub async fn execute_tool(&self, tool_call: ToolCall) -> Result<ToolResult>;
    pub async fn approve_tool(&self, tool_id: String) -> Result<()>;
    pub async fn deny_tool(&self, tool_id: String) -> Result<()>;
}
```

### 3. Tool Framework (`src/llm/tools/`)

**Files:**
- `mod.rs` - Tool registry and execution
- `bash.rs` - Bash command execution
- `read.rs` - File reading
- `write.rs` - File writing
- `edit.rs` - File editing
- `grep.rs` - Code search

**Tool Interface:**
```rust
#[async_trait]
pub trait Tool: Send + Sync {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn parameters(&self) -> ToolParameters;
    async fn execute(&self, args: Value) -> Result<ToolResult>;
    fn requires_approval(&self) -> bool;
}

pub struct ToolRegistry {
    tools: HashMap<String, Box<dyn Tool>>,
}
```

### 4. Prompt Management (`src/llm/prompt/`)

**Files:**
- `mod.rs` - Prompt templates and management
- `system.rs` - System prompt generation

**System Prompt:**
```
You are Crustly, a helpful AI coding assistant built with Rust.

You help developers by:
- Understanding and explaining code
- Writing high-quality code
- Debugging issues
- Suggesting improvements

You have access to tools for file operations and command execution.
Always ask for permission before executing potentially destructive operations.
```

## Implementation Phases

### Phase 1: Provider Abstraction (Day 1-2)
- [x] Define core types (Message, Role, ContentBlock)
- [ ] Implement Provider trait
- [ ] Create AnthropicProvider
- [ ] Add streaming support
- [ ] Error handling

### Phase 2: Agent Service (Day 2-3)
- [ ] AgentService structure
- [ ] Message handling
- [ ] Context management
- [ ] Database integration
- [ ] Token usage tracking

### Phase 3: Tool Framework (Day 3-4)
- [ ] Tool trait definition
- [ ] Tool registry
- [ ] Implement core tools (bash, read, write)
- [ ] Tool approval system
- [ ] Tool result handling

### Phase 4: Integration & Testing (Day 4-5)
- [ ] End-to-end tests
- [ ] Mock provider for testing
- [ ] Streaming tests
- [ ] Tool execution tests
- [ ] Error handling tests

## Dependencies

**Already Available:**
- `reqwest` - HTTP client
- `tokio-stream` - Streaming support
- `serde_json` - JSON handling
- `async-trait` - Async traits

**To Add:**
- Nothing new required

## Testing Strategy

1. **Unit Tests:**
   - Provider implementations
   - Tool execution
   - Message formatting

2. **Integration Tests:**
   - Agent service with mock provider
   - Tool approval workflow
   - Streaming responses

3. **Mock Provider:**
   - For testing without API calls
   - Simulates streaming
   - Simulates tool calls

## Success Criteria

✅ Provider abstraction working
✅ Anthropic provider functional
✅ Streaming responses working
✅ Tool execution framework operational
✅ Agent service integrated with services layer
✅ All tests passing
✅ Token usage tracked
✅ Error handling comprehensive

## Timeline

**Total: 5 days**
- Day 1: Provider layer (8 hours)
- Day 2: Agent service (8 hours)
- Day 3: Tool framework (8 hours)
- Day 4: Integration (8 hours)
- Day 5: Testing & refinement (8 hours)

## Next Sprint

Sprint 5: CLI Implementation
- Command parsing
- Interactive mode
- Auto-approve mode
- Output formatting
