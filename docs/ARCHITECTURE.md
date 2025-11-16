# Crustly Architecture Documentation

```
   ___             _   _
  / __|_ _ _  _ __| |_| |_  _
 | (__| '_| || (_-<  _| | || |
  \___|_|  \_,_/__/\__|_|\_, |
                         |__/
        🥐 Flaky & Fast

        by Jeremy JEANNE
```

## Executive Summary

Crustly is a high-performance terminal AI assistant built in Rust, featuring:
- **Multi-LLM Support**: Anthropic, OpenAI, and local LLMs
- **Extensible Tool System**: 14 tools for file operations, code execution, and workflows
- **Interactive TUI**: Ratatui-based terminal interface with plan mode
- **Local-First Storage**: SQLite database for privacy and persistence
- **Intelligent Prompt Analysis**: Automatic tool hint detection

---

## Table of Contents

1. [System Overview](#1-system-overview)
2. [Module Architecture](#2-module-architecture)
3. [Core Components](#3-core-components)
4. [Data Flow](#4-data-flow)
5. [Tool System](#5-tool-system)
6. [Database Layer](#6-database-layer)
7. [Service Layer](#7-service-layer)
8. [Configuration](#8-configuration)
9. [Error Handling](#9-error-handling)
10. [Design Patterns](#10-design-patterns)
11. [Class Diagrams](#11-class-diagrams)

---

## 1. System Overview

### High-Level Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                        USER INTERFACE                           │
├─────────────────────────────────────────────────────────────────┤
│   TUI (Ratatui)              │        CLI (Clap)               │
│   - Interactive chat         │        - Single commands         │
│   - Plan mode                │        - Batch processing        │
│   - File picker              │        - Configuration           │
│   - Tool approval            │        - Log management          │
└─────────────────────┬────────┴──────────┬───────────────────────┘
                      │                   │
                      ▼                   ▼
┌─────────────────────────────────────────────────────────────────┐
│                     APPLICATION LAYER                           │
├─────────────────────────────────────────────────────────────────┤
│   AgentService                │       PromptAnalyzer            │
│   - Conversation management   │       - Keyword detection        │
│   - Tool execution loop       │       - Tool hint injection      │
│   - Cost tracking             │       - Intent recognition       │
└─────────────────────┬────────┴──────────┬───────────────────────┘
                      │                   │
                      ▼                   ▼
┌─────────────────────────────────────────────────────────────────┐
│                       PROVIDER LAYER                            │
├─────────────────────────────────────────────────────────────────┤
│   Provider Trait              │       Tool Registry              │
│   ├─ AnthropicProvider        │       ├─ ReadTool (read_file)   │
│   ├─ OpenAIProvider           │       ├─ WriteTool (write_file) │
│   ├─ GeminiProvider           │       ├─ EditTool (edit_file)   │
│   ├─ BedrockProvider          │       ├─ BashTool (bash)        │
│   ├─ AzureProvider            │       ├─ GlobTool (glob)        │
│   └─ VertexAIProvider         │       ├─ GrepTool (grep)        │
│                               │       ├─ LsTool (ls)            │
│                               │       ├─ WebSearchTool          │
│                               │       ├─ CodeExecTool           │
│                               │       ├─ NotebookEditTool       │
│                               │       ├─ PlanTool               │
│                               │       ├─ TaskTool               │
│                               │       ├─ ContextTool            │
│                               │       └─ HttpClientTool         │
└─────────────────────┬────────┴──────────┬───────────────────────┘
                      │                   │
                      ▼                   ▼
┌─────────────────────────────────────────────────────────────────┐
│                      SERVICE LAYER                              │
├─────────────────────────────────────────────────────────────────┤
│   SessionService     │  MessageService  │  PlanService          │
│   - CRUD sessions    │  - CRUD messages │  - Plan management    │
│   - Token tracking   │  - Conversation  │  - Task orchestration │
│   - Cost aggregation │  - History       │  - Execution workflow │
└─────────────────────┬────────┴──────────┴───────────────────────┘
                      │
                      ▼
┌─────────────────────────────────────────────────────────────────┐
│                      DATABASE LAYER                             │
├─────────────────────────────────────────────────────────────────┤
│   SQLite Database                                               │
│   ├─ Sessions (chat history metadata)                          │
│   ├─ Messages (conversation content)                           │
│   ├─ Plans (structured task plans)                             │
│   ├─ PlanTasks (individual steps)                              │
│   ├─ Files (cached file contents)                              │
│   └─ ToolExecutions (audit trail)                              │
└─────────────────────────────────────────────────────────────────┘
```

### Key Characteristics

| Aspect | Description |
|--------|-------------|
| **Language** | Rust 2021 Edition (1.75+) |
| **Async Runtime** | Tokio (full features) |
| **TUI Framework** | Ratatui + Crossterm |
| **Database** | SQLite via SQLx |
| **HTTP Client** | Reqwest |
| **Configuration** | TOML-based |
| **License** | FSL-1.1-MIT |

---

## 2. Module Architecture

### Module Hierarchy

```
src/
├── lib.rs                 # Library entry point & module declarations
├── main.rs                # Binary entry point
├── error.rs               # Global error types
│
├── app/                   # Application lifecycle
│   └── mod.rs
│
├── cli/                   # Command-line interface
│   └── mod.rs             # CLI parsing, commands, handlers
│
├── config/                # Configuration management
│   ├── mod.rs             # Config struct, loading, validation
│   ├── secrets.rs         # Secret management (API keys)
│   ├── crabrace.rs        # Crabrace provider registry
│   └── update.rs          # Configuration updater
│
├── db/                    # Database layer
│   ├── mod.rs             # Database connection, pool
│   ├── models.rs          # Data models (Session, Message, Plan)
│   ├── retry.rs           # Retry logic for DB operations
│   └── repository/        # Repository pattern implementation
│       ├── mod.rs
│       ├── session.rs     # SessionRepository
│       ├── message.rs     # MessageRepository
│       ├── file.rs        # FileRepository
│       └── plan.rs        # PlanRepository
│
├── llm/                   # LLM abstraction layer
│   ├── mod.rs
│   ├── agent/             # Agent service
│   │   ├── service.rs     # AgentService (core logic)
│   │   ├── context.rs     # Conversation context management
│   │   └── error.rs       # Agent-specific errors
│   ├── provider/          # LLM provider abstraction
│   │   ├── trait.rs       # Provider trait definition
│   │   ├── types.rs       # LLM request/response types
│   │   ├── anthropic.rs   # Anthropic Claude provider
│   │   ├── openai.rs      # OpenAI/Local LLM provider
│   │   ├── error.rs       # Provider errors
│   │   └── retry.rs       # Retry logic
│   ├── tools/             # Tool system
│   │   ├── mod.rs         # Tool module exports
│   │   ├── trait.rs       # Tool trait definition
│   │   ├── registry.rs    # ToolRegistry
│   │   ├── error.rs       # Tool errors
│   │   └── [14 tool implementations...]
│   └── prompt/            # Prompt formatting
│       └── mod.rs
│
├── logging.rs             # Conditional debug logging
│
├── services/              # Business logic layer
│   ├── mod.rs             # ServiceContext, ServiceManager
│   ├── session.rs         # SessionService
│   ├── message.rs         # MessageService
│   ├── file.rs            # FileService
│   └── plan.rs            # PlanService
│
├── tui/                   # Terminal user interface
│   ├── mod.rs
│   ├── app.rs             # App state management
│   ├── runner.rs          # TUI event loop
│   ├── render.rs          # Rendering logic
│   ├── events.rs          # Event handling
│   ├── prompt_analyzer.rs # Keyword detection & hints
│   ├── plan.rs            # Plan document structure
│   ├── splash.rs          # Splash screen
│   ├── highlight.rs       # Syntax highlighting
│   ├── markdown.rs        # Markdown rendering
│   ├── styles/            # UI styling
│   ├── components/        # Reusable UI components
│   ├── pages/             # UI pages
│   └── utils/             # TUI utilities
│
├── events/                # Global event definitions
│   └── mod.rs
│
├── message/               # Message types
│   └── mod.rs
│
├── lsp/                   # Language Server Protocol
│   └── mod.rs
│
├── mcp/                   # Model Context Protocol
│   └── mod.rs
│
├── sync/                  # Synchronization utilities
│   └── mod.rs
│
└── utils/                 # Utility functions
    └── mod.rs
```

### Module Dependencies

```
main.rs
  └─► cli::run()
       └─► tui::run() / cmd_*()
            └─► App::new()
                 ├─► AgentService
                 │    ├─► Provider (trait)
                 │    │    ├─► AnthropicProvider
                 │    │    └─► OpenAIProvider
                 │    ├─► ToolRegistry
                 │    │    └─► Tool (trait) x 14
                 │    └─► ServiceContext
                 │         └─► Database::pool()
                 ├─► SessionService
                 ├─► MessageService
                 ├─► PlanService
                 └─► PromptAnalyzer
```

---

## 3. Core Components

### 3.1 AgentService

The central orchestrator for AI conversations.

```rust
pub struct AgentService {
    provider: Arc<dyn Provider>,           // LLM provider
    context: ServiceContext,               // Database access
    tool_registry: Arc<ToolRegistry>,      // Available tools
    max_tool_iterations: usize,            // Loop protection (default: 10)
    default_system_prompt: Option<String>, // System prompt
    auto_approve_tools: bool,              // Skip approval dialogs
    approval_callback: Option<ApprovalCallback>,
    working_directory: PathBuf,            // Tool execution directory
}
```

**Key Methods:**

| Method | Purpose |
|--------|---------|
| `send_message()` | Simple message without tools |
| `send_message_with_tools()` | Message with tool execution |
| `send_message_with_tools_and_mode()` | With read-only mode support |
| `send_message_streaming()` | Streaming response support |

**Tool Execution Loop:**

```
┌─────────────────┐
│ User Message    │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│ Send to LLM     │
│ (with tools)    │
└────────┬────────┘
         │
         ▼
    ┌────────────┐
    │ Tool Use?  │──NO──► Return Response
    └────┬───────┘
         │YES
         ▼
┌─────────────────┐
│ Approval Check  │
├─────────────────┤
│ requires_approval? │
│ auto_approve?   │
│ user callback?  │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│ Execute Tool    │
│ via Registry    │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│ Add Result to   │
│ Conversation    │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│ Back to LLM     │
│ (iteration++)   │
└────────┬────────┘
         │
    (max 10 iterations)
```

### 3.2 Provider Abstraction

Unified interface for all LLM providers.

```rust
pub trait Provider: Send + Sync {
    async fn complete(&self, request: LLMRequest) -> Result<LLMResponse>;
    async fn stream(&self, request: LLMRequest) -> Result<ProviderStream>;

    fn supports_streaming(&self) -> bool;
    fn supports_tools(&self) -> bool;
    fn supports_vision(&self) -> bool;

    fn name(&self) -> &str;
    fn default_model(&self) -> &str;
    fn supported_models(&self) -> Vec<String>;
    fn validate_model(&self, model: &str) -> bool;
    fn context_window(&self, model: &str) -> Option<u32>;
    fn calculate_cost(&self, model: &str, input: u32, output: u32) -> f64;
}
```

**Implementations:**

| Provider | Key Features |
|----------|--------------|
| **AnthropicProvider** | Claude models, tool use, vision |
| **OpenAIProvider** | GPT models, local LLMs (Ollama, LM Studio) |
| **GeminiProvider** | Google Gemini models |
| **BedrockProvider** | AWS Bedrock |
| **AzureProvider** | Azure OpenAI Service |
| **VertexAIProvider** | Google Vertex AI |

### 3.3 TUI Application

Interactive terminal interface managing user experience.

```rust
pub struct App {
    // Core state
    current_session: Option<Session>,
    messages: Vec<DisplayMessage>,
    mode: AppMode,

    // Processing
    is_processing: bool,
    streaming_response: Option<String>,

    // Plan mode
    current_plan: Option<PlanDocument>,
    executing_plan: bool,

    // Tool approval
    pending_approval: Option<ToolApprovalRequest>,

    // Services
    agent_service: Arc<AgentService>,
    prompt_analyzer: PromptAnalyzer,
}
```

**Application Modes:**

| Mode | Purpose | Key Actions |
|------|---------|-------------|
| `Splash` | Startup screen | Wait 3s or press any key |
| `Chat` | Main conversation | Send messages, use tools |
| `Plan` | Plan review (read-only) | Approve/Reject/Revise plans |
| `Sessions` | Session management | Switch/Create sessions |
| `ToolApproval` | Permission dialog | Approve/Deny tool execution |
| `FilePicker` | File selection | Browse and select files |
| `Help` | Help screen | View keyboard shortcuts |
| `Settings` | Configuration | Modify settings |

**Keyboard Shortcuts:**

| Shortcut | Action |
|----------|--------|
| `Ctrl+Enter` | Submit message |
| `Ctrl+C` | Quit |
| `Ctrl+N` | New session |
| `Ctrl+L` | List sessions |
| `Ctrl+P` | Toggle plan mode |
| `Ctrl+A` | Approve plan (Plan mode) |
| `Ctrl+R` | Reject plan (Plan mode) |
| `Ctrl+I` | Request revision (Plan mode) |
| `@` | Open file picker |
| `Esc` | Cancel/Back |

### 3.4 PromptAnalyzer

Automatically detects user intent and adds tool hints.

```rust
pub struct PromptAnalyzer {
    plan_regex: Regex,
    read_file_regex: Regex,
    search_regex: Regex,
    write_file_regex: Regex,
    edit_file_regex: Regex,
    bash_regex: Regex,
    web_search_regex: Regex,
}
```

**Keyword Detection:**

| Tool | Example Keywords |
|------|-----------------|
| **plan** | "make a plan", "create a plan", "plan for" |
| **read_file** | "read file", "show me file", "view file" |
| **grep** | "search for", "find", "grep", "locate" |
| **write_file** | "create file", "write file", "new file" |
| **edit_file** | "edit file", "modify file", "update file" |
| **bash** | "run command", "execute command", "shell command" |
| **web_search** | "search online", "google", "search the web" |

**Example Transformation:**

```
Input:  "make a plan for implementing JWT authentication"
Output: "make a plan for implementing JWT authentication

**TOOL HINT**: Use the `plan` tool to create a structured plan with
tasks, dependencies, and implementation steps."
```

---

## 4. Data Flow

### 4.1 User Message Flow

```
User Types Message (TUI)
         │
         ▼
┌─────────────────────┐
│ App.handle_chat_key │
│ - Collect input     │
│ - Detect Ctrl+Enter │
└─────────┬───────────┘
          │
          ▼
┌─────────────────────┐
│ App.send_message()  │
│ - Analyze prompt    │
│ - Transform w/ hints│
└─────────┬───────────┘
          │
          ▼
┌─────────────────────────────────┐
│ AgentService.send_message_*()  │
│ - Load conversation context    │
│ - Build LLMRequest with tools  │
│ - Send to provider             │
└─────────┬───────────────────────┘
          │
          ▼
┌─────────────────────┐
│ Provider.complete() │
│ - Call LLM API      │
│ - Parse response    │
└─────────┬───────────┘
          │
          ▼
┌─────────────────────┐
│ Tool Execution Loop │
│ (if tool_use block) │
│ - Approval check    │
│ - Execute tool      │
│ - Format result     │
│ - Continue loop     │
└─────────┬───────────┘
          │
          ▼
┌─────────────────────┐
│ Save to Database    │
│ - Message content   │
│ - Token usage       │
│ - Cost calculation  │
└─────────┬───────────┘
          │
          ▼
┌─────────────────────┐
│ Return AgentResponse│
│ - content           │
│ - usage             │
│ - cost              │
│ - model             │
└─────────────────────┘
```

### 4.2 Plan Creation Flow

```
User: "make a plan for implementing login"
         │
         ▼
┌─────────────────────┐
│ AgentService        │
│ - LLM recognizes    │
│   plan intent       │
└─────────┬───────────┘
          │
          ▼
┌─────────────────────┐
│ PlanTool.execute()  │
│ operation: "create" │
│ - Create PlanDoc    │
│ - Save to DB        │
└─────────┬───────────┘
          │
          ▼
┌─────────────────────┐
│ PlanTool.execute()  │
│ operation: "add_task"│
│ - Add task 1        │
│ - Add task 2        │
│ - Add task N        │
└─────────┬───────────┘
          │
          ▼
┌─────────────────────┐
│ PlanTool.execute()  │
│ operation: "finalize"│
│ - Set status to     │
│   PendingApproval   │
└─────────┬───────────┘
          │
          ▼
┌─────────────────────┐
│ App.check_and_load_ │
│ plan()              │
│ - Load plan from DB │
│ - Show notification │
│ - Wait for user     │
│   approval          │
└─────────┬───────────┘
          │
    ┌─────┴─────┐
    │   User    │
    │  Action   │
    └─────┬─────┘
          │
    ┌─────┼─────┬─────────┐
    │     │     │         │
    ▼     ▼     ▼         ▼
Ctrl+A  Ctrl+R  Ctrl+I    Esc
Approve  Reject  Revise   Cancel
    │     │       │         │
    ▼     ▼       ▼         ▼
Execute Clear   Prefill   Return
  Plan   Plan    Input    to Chat
```

### 4.3 Tool Approval Flow

```
┌─────────────────────┐
│ Tool requires       │
│ approval?           │
└─────────┬───────────┘
          │
     ┌────┴────┐
     │   YES   │
     └────┬────┘
          │
          ▼
┌─────────────────────┐
│ context.auto_approve│
│ enabled?            │
└─────────┬───────────┘
          │
     ┌────┴────┐     ┌────┐
     │   NO    │     │ YES│
     └────┬────┘     └──┬─┘
          │             │
          ▼             ▼
┌─────────────────┐ ┌─────────────┐
│ Create Approval │ │ Execute     │
│ Request         │ │ Immediately │
│ - tool_name     │ └─────────────┘
│ - tool_input    │
│ - capabilities  │
└─────────┬───────┘
          │
          ▼
┌─────────────────────┐
│ Send to TUI via     │
│ mpsc channel        │
└─────────┬───────────┘
          │
          ▼
┌─────────────────────┐
│ TUI: ToolApproval   │
│ Mode                │
│ - Display tool info │
│ - Show capabilities │
│ - 5-minute timeout  │
└─────────┬───────────┘
          │
     ┌────┴────┐
     │  User   │
     │  Input  │
     └────┬────┘
          │
    ┌─────┼─────┐
    ▼     ▼     ▼
   'A'   'D'  Timeout
 Approve Deny  (5min)
    │     │     │
    ▼     ▼     ▼
Execute Return Return
  Tool   Error   Error
```

---

## 5. Tool System

### 5.1 Tool Trait

```rust
#[async_trait]
pub trait Tool: Send + Sync {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn input_schema(&self) -> Value;  // JSON Schema
    fn capabilities(&self) -> Vec<ToolCapability>;
    fn requires_approval(&self) -> bool;
    async fn execute(&self, input: Value, context: &ToolExecutionContext)
        -> Result<ToolResult>;
    fn validate_input(&self, _input: &Value) -> Result<()>;
}
```

### 5.2 Tool Capabilities

```rust
pub enum ToolCapability {
    ReadFiles,           // Can read file contents
    WriteFiles,          // Can modify/create files
    ExecuteShell,        // Can run shell commands
    Network,             // Can access network
    SystemModification,  // Can modify system state
    PlanManagement,      // Can manage plans/tasks
}
```

### 5.3 All 14 Tools

| # | Tool Name | File | Capabilities | Approval | Description |
|---|-----------|------|--------------|----------|-------------|
| 1 | `read_file` | read.rs | ReadFiles | No | Read file contents with line ranges |
| 2 | `write_file` | write.rs | WriteFiles, SystemMod | **Yes** | Create or overwrite files |
| 3 | `edit_file` | edit.rs | WriteFiles, SystemMod | **Yes** | Edit files (replace, insert, delete, regex) |
| 4 | `bash` | bash.rs | ExecuteShell, SystemMod | **Yes** | Execute shell commands |
| 5 | `ls` | ls.rs | ReadFiles | No | List directory contents |
| 6 | `glob` | glob.rs | ReadFiles | No | Find files by pattern |
| 7 | `grep` | grep.rs | ReadFiles | No | Search file contents (literal/regex) |
| 8 | `web_search` | web_search.rs | Network | No | Internet search (DuckDuckGo) |
| 9 | `execute_code` | code_exec.rs | ExecuteShell, SystemMod | **Yes** | Run Python/JS/Rust/Shell code |
| 10 | `notebook_edit` | notebook.rs | WriteFiles, SystemMod | **Yes** | Edit Jupyter notebooks |
| 11 | `task` | task.rs | PlanManagement | No | Task tracking and management |
| 12 | `context` | context.rs | PlanManagement | No | Session context/variables |
| 13 | `http_request` | http.rs | Network | No | Make HTTP API requests |
| 14 | `plan` | plan_tool.rs | PlanManagement | No | Create and manage structured plans |

### 5.4 Tool Execution Context

```rust
pub struct ToolExecutionContext {
    pub session_id: Uuid,
    pub working_directory: PathBuf,
    pub env_vars: HashMap<String, String>,
    pub auto_approve: bool,
    pub timeout_secs: u64,
    pub read_only_mode: bool,  // Plan mode restriction
}
```

**Read-Only Mode Restrictions:**

When `read_only_mode = true`:
- ❌ `write_file`: Blocked
- ❌ `edit_file`: Blocked
- ❌ `execute_code`: Blocked
- ❌ `notebook_edit`: Blocked
- ⚠️ `bash`: Filters unsafe commands (>, >>, | tee, rm, mv, etc.)
- ✅ All other tools: Normal operation

### 5.5 ToolRegistry

```rust
pub struct ToolRegistry {
    tools: HashMap<String, Arc<dyn Tool>>,
}

impl ToolRegistry {
    pub fn new() -> Self;
    pub fn register(&mut self, tool: Arc<dyn Tool>);
    pub fn get(&self, name: &str) -> Option<Arc<dyn Tool>>;
    pub fn has_tool(&self, name: &str) -> bool;
    pub fn list_tools(&self) -> Vec<String>;
    pub fn get_tool_definitions(&self) -> Vec<Tool>;  // For LLM
    pub async fn execute(&self, name: &str, input: Value,
        context: &ToolExecutionContext) -> Result<ToolResult>;
    pub fn count(&self) -> usize;
}
```

**Registration in CLI:**

```rust
let mut tool_registry = ToolRegistry::new();

// Phase 1: Essential file operations
tool_registry.register(Arc::new(ReadTool));
tool_registry.register(Arc::new(WriteTool));
tool_registry.register(Arc::new(EditTool));
tool_registry.register(Arc::new(BashTool));
tool_registry.register(Arc::new(LsTool));
tool_registry.register(Arc::new(GlobTool));
tool_registry.register(Arc::new(GrepTool));

// Phase 2: Advanced features
tool_registry.register(Arc::new(WebSearchTool));
tool_registry.register(Arc::new(CodeExecTool));
tool_registry.register(Arc::new(NotebookEditTool));

// Phase 3: Workflow & integration
tool_registry.register(Arc::new(TaskTool));
tool_registry.register(Arc::new(ContextTool));
tool_registry.register(Arc::new(HttpClientTool));
tool_registry.register(Arc::new(PlanTool));
```

---

## 6. Database Layer

### 6.1 Connection Management

```rust
pub struct Database {
    pool: SqlitePool,
}

impl Database {
    pub async fn connect<P: AsRef<Path>>(path: P) -> Result<Self>;
    pub async fn connect_in_memory() -> Result<Self>;
    pub fn pool(&self) -> &SqlitePool;
    pub fn is_connected(&self) -> bool;
    pub async fn run_migrations(&self) -> Result<()>;
    pub async fn close(self) -> Result<()>;
}
```

**Connection Configuration:**
- Max connections: 5
- Busy timeout: 5 seconds
- WAL mode for concurrency
- SQLx migrations

### 6.2 Data Models

**Session:**
```rust
pub struct Session {
    pub id: Uuid,
    pub title: Option<String>,
    pub model: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub archived_at: Option<DateTime<Utc>>,
    pub token_count: i32,
    pub total_cost: f64,
}
```

**Message:**
```rust
pub struct Message {
    pub id: Uuid,
    pub session_id: Uuid,
    pub role: String,
    pub content: String,
    pub sequence: i32,
    pub created_at: DateTime<Utc>,
    pub token_count: Option<i32>,
    pub cost: Option<f64>,
}
```

**Plan:**
```rust
pub struct Plan {
    pub id: Uuid,
    pub session_id: Uuid,
    pub title: String,
    pub description: String,
    pub context: String,
    pub risks: String,           // JSON array
    pub test_strategy: String,
    pub technical_stack: String, // JSON array
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub approved_at: Option<DateTime<Utc>>,
}
```

**PlanTask:**
```rust
pub struct PlanTask {
    pub id: Uuid,
    pub plan_id: Uuid,
    pub task_order: i32,
    pub title: String,
    pub description: String,
    pub task_type: String,
    pub dependencies: String,     // JSON array
    pub complexity: i32,
    pub acceptance_criteria: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
```

### 6.3 Repository Pattern

```
Service Layer
     │
     ▼
Repository (trait)
     │
     ├─► SessionRepository
     │     - create(session)
     │     - find_by_id(id)
     │     - list(options)
     │     - update(session)
     │     - delete(id)
     │
     ├─► MessageRepository
     │     - create(message)
     │     - find_by_session(session_id)
     │     - find_by_id(id)
     │     - update(message)
     │     - delete_by_session(session_id)
     │
     └─► PlanRepository
           - create(plan)
           - find_by_id(id)
           - find_by_session(session_id)
           - update(plan)
           - delete(id)
```

---

## 7. Service Layer

### 7.1 ServiceContext

Shared dependency injection container.

```rust
pub struct ServiceContext {
    pub pool: Arc<Pool>,
}

impl ServiceContext {
    pub fn new(pool: Arc<Pool>) -> Self;
    pub fn clone(&self) -> Self;
}
```

### 7.2 Services

**SessionService:**
- Session CRUD operations
- Token and cost tracking
- Archive management

**MessageService:**
- Message CRUD operations
- Conversation history management
- Usage tracking per message

**PlanService:**
- Plan lifecycle management
- JSON import/export
- Status transitions

**FileService:**
- Cached file operations
- Content versioning
- File metadata

### 7.3 Service Pattern

```rust
// Example: SessionService
pub struct SessionService {
    context: ServiceContext,
}

impl SessionService {
    pub fn new(context: ServiceContext) -> Self {
        Self { context }
    }

    pub async fn create_session(&self, title: Option<String>) -> Result<Session> {
        let session = Session {
            id: Uuid::new_v4(),
            title,
            model: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            archived_at: None,
            token_count: 0,
            total_cost: 0.0,
        };

        SessionRepository::create(self.context.pool(), &session).await?;
        Ok(session)
    }

    // ... other methods
}
```

---

## 8. Configuration

### 8.1 Config Structure

```rust
pub struct Config {
    pub crabrace: CrabraceConfig,
    pub database: DatabaseConfig,
    pub logging: LoggingConfig,
    pub debug: DebugConfig,
    pub providers: ProviderConfigs,
}

pub struct ProviderConfigs {
    pub anthropic: Option<ProviderConfig>,
    pub openai: Option<ProviderConfig>,
    pub gemini: Option<ProviderConfig>,
    pub bedrock: Option<ProviderConfig>,
    pub azure: Option<ProviderConfig>,
    pub vertex: Option<ProviderConfig>,
}

pub struct ProviderConfig {
    pub enabled: bool,
    pub api_key: Option<String>,
    pub base_url: Option<String>,
    pub default_model: Option<String>,
}

pub struct DatabaseConfig {
    pub path: PathBuf,
    pub max_connections: u32,
    pub busy_timeout_secs: u64,
}
```

### 8.2 Loading Priority

1. Default configuration
2. Config file (`~/.config/crustly/config.toml`)
3. Environment variables
4. CLI arguments

### 8.3 Environment Variables

| Variable | Description |
|----------|-------------|
| `ANTHROPIC_API_KEY` | Anthropic Claude API key |
| `OPENAI_API_KEY` | OpenAI API key |
| `OPENAI_BASE_URL` | Custom OpenAI-compatible endpoint |
| `CRUSTLY_CONFIG` | Custom config file path |
| `RUST_LOG` | Log level filter |

---

## 9. Error Handling

### 9.1 Error Hierarchy

```rust
pub enum CrustlyError {
    Database(sqlx::Error),
    Io(std::io::Error),
    Config { message: String, code: ErrorCode },
    Provider { provider: String, message: String, code: ErrorCode },
    ToolExecution { tool: String, message: String, code: ErrorCode },
    PermissionDenied(String),
}

pub enum ErrorCode {
    // Configuration (1000-1999)
    ConfigNotFound = 1000,
    ConfigInvalid = 1001,
    ConfigMergeError = 1002,

    // Provider (2000-2999)
    ProviderNotFound = 2000,
    ProviderAuthFailed = 2001,
    ProviderRateLimit = 2002,
    ProviderTimeout = 2003,

    // Tool (3000-3999)
    ToolNotFound = 3000,
    ToolExecutionFailed = 3001,
    ToolTimeout = 3002,

    // Permission (4000-4999)
    PermissionDenied = 4000,
    PermissionNotGranted = 4001,
}
```

### 9.2 Tool Errors

```rust
pub enum ToolError {
    NotFound(String),
    InvalidInput(String),
    Execution(String),
    ApprovalRequired(String),
    Io(io::Error),
    Timeout,
}
```

### 9.3 Error Propagation

```
Tool → ToolError
  │
  ▼
ToolRegistry → Result<ToolResult>
  │
  ▼
AgentService → AgentError
  │
  ▼
App → TuiEvent::Error(String)
  │
  ▼
User sees error message in UI
```

---

## 10. Design Patterns

### 10.1 Trait-Based Abstraction

- **Provider Trait**: Unified LLM interface
- **Tool Trait**: Extensible tool system
- **Repository Pattern**: Database abstraction

### 10.2 Builder Pattern

```rust
// AgentService configuration
AgentService::new(provider, context)
    .with_system_prompt("...")
    .with_tool_registry(registry)
    .with_auto_approve_tools(false)
    .with_approval_callback(Some(callback))
    .with_max_tool_iterations(10)
    .with_working_directory(dir)

// ToolExecutionContext
ToolExecutionContext::new(session_id)
    .with_auto_approve(false)
    .with_timeout(30)
    .with_read_only_mode(false)
    .with_working_directory(dir)

// LogConfig
LogConfig::new()
    .with_debug_mode(true)
    .with_log_level(Level::DEBUG)
    .with_log_dir(path)
```

### 10.3 Registry Pattern

- **ToolRegistry**: Dynamic tool management
- Runtime registration and lookup
- Tool definition generation for LLM

### 10.4 Service Layer Pattern

- **ServiceContext**: Dependency injection
- **ServiceManager**: Facade
- Individual services (Session, Message, Plan, File)

### 10.5 Event-Driven Architecture

```rust
pub enum TuiEvent {
    Key(KeyEvent),
    MessageSubmitted(String),
    ResponseComplete(AgentResponse),
    ToolApprovalRequested(ToolApprovalRequest),
    ToolApprovalResponse(ToolApprovalResponse),
    // ...
}

// Event loop
loop {
    match app.next_event().await {
        Some(event) => app.handle_event(event).await?,
        None => break,
    }
}
```

### 10.6 Concurrency with Arc/Mutex

```rust
provider: Arc<dyn Provider>,
tool_registry: Arc<ToolRegistry>,
agent_service: Arc<AgentService>,
pool: Arc<SqlitePool>,
```

---

## 11. Class Diagrams

### 11.1 PlantUML Diagram

See `docs/architecture.puml` for the complete PlantUML class diagram.

**To render:**
```bash
# Install PlantUML
brew install plantuml  # macOS
apt install plantuml   # Linux

# Generate diagram
plantuml docs/architecture.puml

# Or use online renderer
# https://www.plantuml.com/plantuml/
```

### 11.2 Core Class Relationships

```
┌─────────────────┐         ┌─────────────────┐
│      App        │         │  AgentService   │
├─────────────────┤    uses ├─────────────────┤
│ agent_service   │────────▶│ provider        │
│ prompt_analyzer │         │ tool_registry   │
│ session_service │         │ context         │
│ message_service │         └─────────────────┘
│ plan_service    │                 │
└─────────────────┘                 │uses
                                    ▼
                            ┌─────────────────┐
                            │  ToolRegistry   │
                            ├─────────────────┤
                            │ tools: HashMap  │
                            │ register()      │
                            │ execute()       │
                            └─────────────────┘
                                    │manages
                                    ▼
                            ┌─────────────────┐
                            │   Tool (trait)  │
                            ├─────────────────┤
                            │ name()          │
                            │ execute()       │
                            │ capabilities()  │
                            └─────────────────┘
                                    △
                    ┌───────────────┼───────────────┐
                    │               │               │
            ┌───────┴───────┐ ┌─────┴─────┐ ┌──────┴──────┐
            │   ReadTool    │ │ WriteTool │ │   ...       │
            └───────────────┘ └───────────┘ └─────────────┘
```

### 11.3 Provider Abstraction

```
┌─────────────────────┐
│   Provider (trait)  │
├─────────────────────┤
│ +complete()         │
│ +stream()           │
│ +calculate_cost()   │
│ +context_window()   │
└─────────────────────┘
          △
          │implements
    ┌─────┴─────┐
    │           │
┌───┴───┐   ┌───┴───┐
│Anthropic│  │OpenAI │
│Provider │  │Provider│
└─────────┘  └─────────┘
```

### 11.4 Database Layer

```
┌─────────────────┐
│ ServiceContext  │
├─────────────────┤
│ pool: Arc<Pool> │
└─────────────────┘
         │
    wraps│
         ▼
┌─────────────────┐
│    Database     │
├─────────────────┤
│ pool: SqlitePool│
│ connect()       │
│ run_migrations()│
└─────────────────┘
         │
         ▼
┌─────────────────┐    operates on    ┌─────────────┐
│   Repository    │──────────────────▶│    Model    │
├─────────────────┤                   ├─────────────┤
│ create()        │                   │ Session     │
│ find_by_id()    │                   │ Message     │
│ update()        │                   │ Plan        │
│ delete()        │                   │ PlanTask    │
└─────────────────┘                   └─────────────┘
```

---

## Appendix A: File Structure Summary

```
Total files: ~150+
Total lines of code: ~25,000+
Primary language: Rust (100%)
Dependencies: 652 crates
```

## Appendix B: Performance Characteristics

| Operation | Expected Performance |
|-----------|---------------------|
| Tool execution | <100ms (local tools) |
| LLM request | 1-30s (network dependent) |
| Database query | <10ms |
| TUI rendering | 60 FPS |
| Build time (debug) | ~90s |
| Binary size | ~50MB |

## Appendix C: Security Considerations

1. **Tool Approval System**: Dangerous operations require user consent
2. **Read-Only Mode**: Plan mode restricts write operations
3. **API Key Management**: Secure storage via environment variables
4. **Input Validation**: All tool inputs validated before execution
5. **Command Filtering**: Bash tool filters unsafe commands in read-only mode

## Appendix D: Future Enhancements

- [ ] RAG (Retrieval-Augmented Generation) support
- [ ] Vector store integration
- [ ] More LLM providers
- [ ] Plugin system for custom tools
- [ ] Web interface
- [ ] Multi-user support
- [ ] Enhanced LSP integration

---

**Document Version:** 1.0
**Last Updated:** November 2024
**Author:** Jeremy JEANNE
**License:** FSL-1.1-MIT
