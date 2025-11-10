# Crustly Codebase Analysis: AI Assistant Capabilities & Local LLM Integration

**Date:** 2025-11-10
**Version:** 0.2.0
**Analysis Type:** Architecture & Integration Guide

---

## Executive Summary

**Crustly** is a high-performance, terminal-based AI assistant written in Rust designed for software development workflows. It provides a privacy-first approach with full support for local LLMs while maintaining compatibility with cloud-based AI providers (Anthropic Claude, OpenAI GPT).

### Key Highlights
- ✅ **100% Local Operation** - Works with LM Studio, Ollama, LocalAI
- ✅ **Built-in Tool System** - Read/write files, execute bash commands
- ✅ **Interactive Approval** - Full control over dangerous operations
- ✅ **Session Context** - Persistent conversation memory
- ✅ **Cost Tracking** - Monitor API usage and costs
- ✅ **Syntax Highlighting** - 100+ languages supported

---

## Table of Contents

1. [Architecture Overview](#architecture-overview)
2. [AI Assistant Capabilities](#ai-assistant-capabilities)
3. [Local LLM Integration](#local-llm-integration)
4. [Tool System Deep Dive](#tool-system-deep-dive)
5. [Configuration Guide](#configuration-guide)
6. [Security & Approval System](#security--approval-system)
7. [Usage Examples](#usage-examples)
8. [Performance Considerations](#performance-considerations)

---

## Architecture Overview

### High-Level Architecture

```
┌─────────────────────────────────────────────────────────┐
│                    User Interface                        │
│  ┌──────────────┐        ┌──────────────┐              │
│  │   CLI Mode   │        │   TUI Mode   │              │
│  │  (clap)      │        │  (ratatui)   │              │
│  └──────────────┘        └──────────────┘              │
└─────────────────────────────────────────────────────────┘
                           │
                           ▼
┌─────────────────────────────────────────────────────────┐
│                   Agent Service                          │
│  • Context Management                                    │
│  • Message Orchestration                                 │
│  • Tool Execution Coordination                           │
└─────────────────────────────────────────────────────────┘
                           │
              ┌────────────┼────────────┐
              ▼            ▼            ▼
┌──────────────────┐ ┌──────────┐ ┌─────────────┐
│  LLM Provider    │ │  Tool    │ │  Service    │
│  • Anthropic     │ │  Registry│ │  Layer      │
│  • OpenAI        │ │  • Read  │ │  • Session  │
│  • Local (LM     │ │  • Write │ │  • Message  │
│    Studio/Ollama)│ │  • Bash  │ │  • File     │
└──────────────────┘ └──────────┘ └─────────────┘
              │                         │
              └────────────┬────────────┘
                           ▼
              ┌────────────────────────┐
              │   SQLite Database      │
              │  • Sessions            │
              │  • Messages            │
              │  • Token/Cost Tracking │
              └────────────────────────┘
```

### Key Components

#### 1. **Provider Abstraction Layer** (`src/llm/provider/`)
- **Trait-based Design**: Common `Provider` trait for all LLM providers
- **OpenAI Provider**: Fully implemented with local LLM support
- **Anthropic Provider**: Fully implemented for Claude models
- **Retry Logic**: Built-in exponential backoff with jitter
- **Rate Limiting**: Automatic handling of `Retry-After` headers

**File:** `src/llm/provider/openai.rs` (517 lines)

Key features:
```rust
pub struct OpenAIProvider {
    api_key: String,
    base_url: String,  // Configurable for local LLMs
    client: Client,
}

// Three factory methods:
OpenAIProvider::new(api_key)              // Official OpenAI API
OpenAIProvider::local(base_url)           // Local LLMs (no auth)
OpenAIProvider::with_base_url(key, url)   // Custom endpoint
```

#### 2. **Tool System** (`src/llm/tools/`)

**Architecture:**
```rust
pub trait Tool: Send + Sync {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn input_schema(&self) -> Value;  // JSON Schema
    fn capabilities(&self) -> Vec<ToolCapability>;
    fn requires_approval(&self) -> bool;
    async fn execute(&self, input: Value, context: &ToolExecutionContext)
        -> Result<ToolResult>;
}
```

**Built-in Tools:**
- **read_file** - Read file contents (safe, no approval needed)
- **write_file** - Create/modify files (requires approval)
- **bash** - Execute shell commands (requires approval)

#### 3. **Agent Service** (`src/llm/agent/service.rs`)

Orchestrates the conversation flow:
1. Load session context from database
2. Add user message to context
3. Send request to LLM provider
4. Handle tool calls (with approval)
5. Save responses to database
6. Track token usage and costs

#### 4. **Configuration System** (`src/config/mod.rs`)

**Hierarchical Loading:**
```
1. Default values (hardcoded)
2. System config (~/.config/crustly/config.toml)
3. Local config (./crustly.toml)
4. Environment variables (highest priority)
```

**Key Configuration:**
```rust
pub struct Config {
    pub database: DatabaseConfig,
    pub logging: LoggingConfig,
    pub providers: ProviderConfigs {
        anthropic: Option<ProviderConfig>,
        openai: Option<ProviderConfig>,
        gemini: Option<ProviderConfig>,
        // ... more providers
    }
}
```

---

## AI Assistant Capabilities

### 1. **Built-in Tools**

#### Read Tool (`read_file`)
- **Purpose**: Read file contents from filesystem
- **Approval Required**: ❌ No (safe operation)
- **Capabilities**: `ReadFiles`

**Input Schema:**
```json
{
  "path": "src/main.rs",          // Required
  "start_line": 10,                // Optional
  "line_count": 50                 // Optional
}
```

**Use Cases:**
- Code review
- Understanding project structure
- Extracting specific code sections
- Reading configuration files

#### Write Tool (`write_file`)
- **Purpose**: Create or modify files
- **Approval Required**: ✅ Yes (dangerous operation)
- **Capabilities**: `WriteFiles`, `SystemModification`

**Input Schema:**
```json
{
  "path": "src/new_module.rs",
  "content": "pub fn hello() { println!(\"Hello\"); }"
}
```

**Use Cases:**
- Code generation
- Creating new files
- Refactoring existing code
- Generating documentation

#### Bash Tool (`bash`)
- **Purpose**: Execute shell commands
- **Approval Required**: ✅ Yes (dangerous operation)
- **Capabilities**: `ExecuteShell`, `SystemModification`

**Input Schema:**
```json
{
  "command": "cargo test",
  "working_directory": "./project"  // Optional
}
```

**Use Cases:**
- Running tests
- Building projects
- Git operations
- System administration tasks

### 2. **Session Management**

**Features:**
- ✅ Persistent conversation history
- ✅ Context window management
- ✅ Multi-session support
- ✅ Session switching (Ctrl+L)
- ✅ Session archiving

**Database Schema:**
```sql
-- sessions table
CREATE TABLE sessions (
    id TEXT PRIMARY KEY,
    title TEXT,
    created_at TIMESTAMP,
    updated_at TIMESTAMP,
    archived BOOLEAN DEFAULT 0
);

-- messages table
CREATE TABLE messages (
    id TEXT PRIMARY KEY,
    session_id TEXT,
    role TEXT,  -- 'user', 'assistant', 'system'
    content TEXT,
    created_at TIMESTAMP,
    FOREIGN KEY (session_id) REFERENCES sessions(id)
);
```

### 3. **Streaming Responses**

**Implementation:**
- Real-time character-by-character streaming
- Server-Sent Events (SSE) parsing
- Animated braille spinner during processing
- Progress indication

**Code Reference:** `src/llm/provider/openai.rs:323-399`

### 4. **Cost & Token Tracking**

**Tracked Metrics:**
- Input tokens
- Output tokens
- Cost per message
- Session totals
- Database persistence

**Pricing (as of 2024):**
- GPT-4 Turbo: $10/$30 per 1M tokens (input/output)
- GPT-3.5 Turbo: $0.5/$1.5 per 1M tokens
- Local LLMs: $0.00 (free after download)

### 5. **Syntax Highlighting**

**Library:** `syntect` (v5.2)
**Languages:** 100+ supported
**Theme:** Base16 Ocean Dark

**Rendered Code Blocks:**
```
╭─ rust ─────────────────╮
│  1 │ fn fibonacci(n: u32) -> u32 {
│  2 │     match n {
│  3 │         0 => 0,
│  4 │         1 => 1,
│  5 │         _ => fibonacci(n-1) + fibonacci(n-2)
│  6 │     }
│  7 │ }
╰────────────────────────╯
```

---

## Local LLM Integration

### Supported Local Solutions

| Solution | Status | Setup Difficulty | Performance |
|----------|--------|------------------|-------------|
| **LM Studio** | ✅ Tested | Easy (GUI) | Excellent |
| **Ollama** | ✅ Compatible | Easy (CLI) | Excellent |
| **LocalAI** | ✅ Compatible | Medium (Docker) | Good |
| **Text-Gen-WebUI** | ✅ Compatible | Medium | Good |

### How OpenAI Compatibility Works

Crustly's `OpenAIProvider` is designed to work with any OpenAI-compatible API:

**Key Code (`src/llm/provider/openai.rs`):**
```rust
/// Create provider for local LLM (LM Studio, Ollama, etc.)
pub fn local(base_url: String) -> Self {
    let client = Client::builder()
        .timeout(DEFAULT_TIMEOUT)
        .connect_timeout(DEFAULT_CONNECT_TIMEOUT)
        .build()
        .expect("Failed to create HTTP client");

    Self {
        api_key: "not-needed".to_string(),  // No auth for local
        base_url,
        client,
    }
}
```

**Authentication Logic:**
```rust
fn headers(&self) -> reqwest::header::HeaderMap {
    let mut headers = reqwest::header::HeaderMap::new();

    // Only add authorization if not using local
    if self.api_key != "not-needed" {
        headers.insert(
            reqwest::header::AUTHORIZATION,
            format!("Bearer {}", self.api_key).parse().unwrap(),
        );
    }

    headers.insert(
        reqwest::header::CONTENT_TYPE,
        "application/json".parse().unwrap(),
    );

    headers
}
```

### Configuration Methods

#### Method 1: Environment Variables (Recommended)

**For LM Studio:**
```bash
# Linux/macOS
export OPENAI_API_KEY="lm-studio"
export OPENAI_BASE_URL="http://localhost:1234/v1/chat/completions"

# Windows PowerShell
$env:OPENAI_API_KEY="lm-studio"
$env:OPENAI_BASE_URL="http://localhost:1234/v1/chat/completions"

# Run Crustly
cargo run
```

**For Ollama:**
```bash
export OPENAI_API_KEY="ollama"
export OPENAI_BASE_URL="http://localhost:11434/v1/chat/completions"
cargo run
```

**For LocalAI:**
```bash
export OPENAI_API_KEY="local"
export OPENAI_BASE_URL="http://localhost:8080/v1/chat/completions"
cargo run
```

#### Method 2: Configuration File

**Create/Edit:** `~/.config/crustly/config.toml`

```toml
[providers.openai]
enabled = true
api_key = "lm-studio"
base_url = "http://localhost:1234/v1/chat/completions"
default_model = "local-model"

# Optional: Timeout settings for slower hardware
[providers.openai.advanced]
timeout = 120  # 2 minutes for generation
```

#### Method 3: Custom Config Path

```bash
# Create custom config
cat > my-local-llm.toml << 'EOF'
[providers.openai]
enabled = true
api_key = "not-needed"
base_url = "http://localhost:1234/v1/chat/completions"
EOF

# Use custom config
cargo run -- --config my-local-llm.toml
```

### Environment Variable Priority

**Configuration Loading Order** (from `src/config/mod.rs:177-309`):
```rust
pub fn load() -> Result<Self> {
    // 1. Start with defaults
    let mut config = Self::default();

    // 2. Load system config (~/.config/crustly/config.toml)
    if let Some(system_config_path) = Self::system_config_path() {
        if system_config_path.exists() {
            config = Self::merge_from_file(config, &system_config_path)?;
        }
    }

    // 3. Load local config (./crustly.toml)
    let local_config_path = Self::local_config_path();
    if local_config_path.exists() {
        config = Self::merge_from_file(config, &local_config_path)?;
    }

    // 4. Apply environment variables (HIGHEST PRIORITY)
    config = Self::apply_env_overrides(config)?;

    Ok(config)
}
```

**Key Environment Variables:**
- `OPENAI_API_KEY` - API key (or dummy value for local)
- `OPENAI_BASE_URL` - Custom API endpoint
- `ANTHROPIC_API_KEY` - Anthropic Claude API key
- `CRUSTLY_DB_PATH` - Database file path
- `CRUSTLY_LOG_LEVEL` - Logging level (trace/debug/info/warn/error)

---

## Tool System Deep Dive

### Tool Execution Flow

```
User sends message
      │
      ▼
Agent Service receives message
      │
      ▼
Send to LLM Provider
      │
      ▼
LLM responds with tool call
      │
      ▼
┌─────────────────────────────┐
│  Check if approval needed   │
│  (based on capabilities)    │
└─────────────────────────────┘
      │
      ├─ Yes: Dangerous tool
      │       │
      │       ▼
      │  ┌────────────────────┐
      │  │ Show approval dialog│
      │  │ • Tool name         │
      │  │ • Description       │
      │  │ • Parameters        │
      │  │ • Capabilities      │
      │  └────────────────────┘
      │       │
      │       ▼
      │  User approves/denies
      │       │
      └───────┘
      │
      ▼
Execute tool
      │
      ▼
Return result to LLM
      │
      ▼
LLM generates final response
```

### Tool Registry System

**Location:** `src/llm/tools/registry.rs`

```rust
pub struct ToolRegistry {
    tools: Arc<DashMap<String, Arc<dyn Tool>>>,
}

impl ToolRegistry {
    pub fn new() -> Self {
        let registry = Self {
            tools: Arc::new(DashMap::new()),
        };

        // Register built-in tools
        registry.register(Arc::new(ReadTool));
        registry.register(Arc::new(WriteTool));
        registry.register(Arc::new(BashTool));

        registry
    }

    pub fn register(&self, tool: Arc<dyn Tool>) {
        self.tools.insert(tool.name().to_string(), tool);
    }

    pub fn get_tool(&self, name: &str) -> Option<Arc<dyn Tool>> {
        self.tools.get(name).map(|t| Arc::clone(&*t))
    }
}
```

### Tool Capabilities System

**Definition:** `src/llm/tools/trait.rs:103-116`

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToolCapability {
    ReadFiles,           // Can read files
    WriteFiles,          // Can write files
    ExecuteShell,        // Can execute shell commands
    Network,             // Can access network
    SystemModification,  // Can modify system state
}
```

**Approval Logic:**
```rust
fn requires_approval(&self) -> bool {
    let dangerous_capabilities = [
        ToolCapability::WriteFiles,
        ToolCapability::ExecuteShell,
        ToolCapability::SystemModification,
    ];

    self.capabilities()
        .iter()
        .any(|cap| dangerous_capabilities.contains(cap))
}
```

### Tool Execution Context

```rust
pub struct ToolExecutionContext {
    pub session_id: Uuid,
    pub working_directory: PathBuf,
    pub env_vars: HashMap<String, String>,
    pub auto_approve: bool,
    pub timeout_secs: u64,
}
```

**Example Usage:**
```rust
let context = ToolExecutionContext::new(session_id)
    .with_working_directory(PathBuf::from("/home/user/project"))
    .with_timeout(60);

let result = tool.execute(input, &context).await?;
```

---

## Configuration Guide

### Complete Configuration Example

**File:** `~/.config/crustly/config.toml`

```toml
# ============================================================================
# Crustly Configuration File
# ============================================================================

# ----------------------------------------------------------------------------
# Database Configuration
# ----------------------------------------------------------------------------
[database]
path = "/home/user/.local/share/crustly/crustly.db"

# ----------------------------------------------------------------------------
# Logging Configuration
# ----------------------------------------------------------------------------
[logging]
level = "info"  # Options: trace, debug, info, warn, error
file = "/home/user/.local/share/crustly/logs/crustly.log"  # Optional

# ----------------------------------------------------------------------------
# Debug Options
# ----------------------------------------------------------------------------
[debug]
debug_lsp = false
profiling = false

# ----------------------------------------------------------------------------
# Anthropic Provider (Claude)
# ----------------------------------------------------------------------------
[providers.anthropic]
enabled = true
api_key = "sk-ant-api03-YOUR_KEY_HERE"
default_model = "claude-3-5-sonnet-20240620"

# ----------------------------------------------------------------------------
# OpenAI Provider (Cloud)
# ----------------------------------------------------------------------------
[providers.openai]
enabled = true
api_key = "sk-YOUR_OPENAI_KEY"
base_url = "https://api.openai.com/v1/chat/completions"
default_model = "gpt-4-turbo-preview"

# ----------------------------------------------------------------------------
# Local LLM via LM Studio
# ----------------------------------------------------------------------------
# [providers.openai]
# enabled = true
# api_key = "lm-studio"
# base_url = "http://localhost:1234/v1/chat/completions"
# default_model = "local-model"

# ----------------------------------------------------------------------------
# Local LLM via Ollama
# ----------------------------------------------------------------------------
# [providers.openai]
# enabled = true
# api_key = "ollama"
# base_url = "http://localhost:11434/v1/chat/completions"
# default_model = "mistral"

# ----------------------------------------------------------------------------
# Crabrace Integration (Provider Registry)
# ----------------------------------------------------------------------------
[crabrace]
enabled = true
base_url = "https://api.anthropic.com/v1/providers"
auto_update = true
cache_ttl = 3600
```

### Minimal Local LLM Configuration

**For Quick Setup:**

```toml
[providers.openai]
enabled = true
api_key = "local"
base_url = "http://localhost:1234/v1/chat/completions"
```

### Environment Variable Override Examples

```bash
# Override log level
export CRUSTLY_LOG_LEVEL="debug"

# Override database path
export CRUSTLY_DB_PATH="/tmp/crustly-test.db"

# Use local LLM
export OPENAI_API_KEY="local"
export OPENAI_BASE_URL="http://localhost:1234/v1/chat/completions"

# Disable Crabrace
export CRUSTLY_CRABRACE_ENABLED="false"
```

---

## Security & Approval System

### Interactive Approval Dialog

**When a dangerous tool is called, Crustly displays:**

```
┌────────────────────────────────────────────────────┐
│ ⚠️  PERMISSION REQUIRED                            │
├────────────────────────────────────────────────────┤
│ 🔒 Permission Request                              │
│                                                    │
│ Claude wants to use the tool: write_file          │
│                                                    │
│ Description: Write content to a file...            │
│                                                    │
│ ⚠️  Capabilities:                                   │
│    • WriteFiles                                    │
│    • SystemModification                            │
│                                                    │
│ Parameters:                                        │
│    path: "config.json"                             │
│    content: "{ \"debug\": true }"                  │
│                                                    │
│ ⏱️  Timeout: 4m 23s remaining                      │
│                                                    │
│ [A]pprove  [D]eny  [V]iew Details  [Esc] Cancel  │
└────────────────────────────────────────────────────┘
```

### Approval System Features

**Implemented in:** `src/tui/components/dialogs/mod.rs`

1. **Automatic Timeout** - 5 minutes (300 seconds)
   - Visual countdown with color coding:
     - 🟢 Green: > 2 minutes remaining
     - 🟡 Yellow: 1-2 minutes remaining
     - 🔴 Red: < 1 minute remaining

2. **Detailed View** - Press `V` to see full JSON parameters

3. **Keyboard Shortcuts**:
   - `A` or `Y` - Approve
   - `D` or `N` - Deny
   - `V` - View full details
   - `Esc` - Cancel

4. **Capability Display** - Shows what the tool can do

### Bypass Approval (Development Only)

**Method 1: Auto-approve via code**
```rust
let agent = AgentService::new(provider, context)
    .with_auto_approve_tools(true);
```

**Method 2: Environment variable (not implemented yet)**
```bash
export CRUSTLY_AUTO_APPROVE_TOOLS="true"  # Planned feature
```

⚠️ **Warning:** Auto-approval bypasses all safety checks. Only use in trusted development environments.

---

## Usage Examples

### Example 1: Using Crustly with LM Studio

**Step 1: Start LM Studio**
```bash
# 1. Download and install LM Studio from https://lmstudio.ai/
# 2. Load a model (e.g., Mistral-7B-Instruct-v0.2)
# 3. Start the local server on port 1234
```

**Step 2: Configure Crustly**
```bash
export OPENAI_API_KEY="lm-studio"
export OPENAI_BASE_URL="http://localhost:1234/v1/chat/completions"
```

**Step 3: Run Crustly**
```bash
cd crustly
cargo run
```

**Step 4: Interact**
```
You: Can you read the main.rs file and explain what it does?

Crustly: [Calls read_file tool with path="src/main.rs"]
         [Reads file content]
         [Provides explanation]

You: Can you add error handling to the database connection?

Crustly: [Calls write_file tool]
         [Shows approval dialog]
         [You approve]
         [Modifies file]
         Done! I've added error handling...
```

### Example 2: Code Generation Workflow

```
You: Create a new module for user authentication with JWT tokens

Crustly: [Generates code]
         [Calls write_file with path="src/auth.rs"]

         ┌──────────────────────────────────────┐
         │ ⚠️  Write file approval needed       │
         │ Path: src/auth.rs                    │
         │ Size: 432 bytes                      │
         │ [A]pprove  [D]eny                   │
         └──────────────────────────────────────┘

[You press 'A']

Crustly: ✅ Created src/auth.rs

You: Now create tests for this module

Crustly: [Generates tests]
         [Calls write_file with path="tests/auth_test.rs"]
         [You approve]
         ✅ Created tests/auth_test.rs

You: Run the tests

Crustly: [Calls bash with command="cargo test auth"]

         ┌──────────────────────────────────────┐
         │ ⚠️  Execute command approval needed  │
         │ Command: cargo test auth             │
         │ [A]pprove  [D]eny                   │
         └──────────────────────────────────────┘

[You press 'A']

Crustly: [Executes command]
         ✅ 12 tests passed
```

### Example 3: Non-Interactive Mode

**Text Output:**
```bash
$ cargo run -- run "What is 2+2?"
🤔 Processing...

The answer is 4.

📊 Tokens: 42
💰 Cost: $0.000126
```

**JSON Output:**
```bash
$ cargo run -- run --format json "List 3 programming languages"
{
  "content": "1. Python\n2. JavaScript\n3. Rust",
  "model": "gpt-4-turbo-preview",
  "usage": {
    "input_tokens": 15,
    "output_tokens": 23
  },
  "cost": 0.000194
}
```

### Example 4: Session Management

```bash
# Start first session
$ cargo run
You: I'm working on a REST API in Rust
Crustly: Great! I'll help you...
[Ctrl+C to quit]

# Resume in new terminal
$ cargo run
[Previous conversation loaded]
You: Let's add authentication to the API
Crustly: [Remembers you're working on a Rust REST API]

# Switch sessions
[Press Ctrl+L]
┌────────────────────────────┐
│ Sessions                   │
├────────────────────────────┤
│ ▶ REST API Project        │
│   Documentation updates    │
│   Bug fix session          │
└────────────────────────────┘
[Use ↑↓ to navigate, Enter to select]
```

---

## Performance Considerations

### Local LLM Performance

**Factors Affecting Performance:**
1. **Hardware**
   - CPU: 8+ cores recommended
   - RAM: 16GB+ for 7B models, 32GB+ for 13B+
   - GPU: Optional but significantly improves speed

2. **Model Size**
   - 1B models: Fast but lower quality
   - 7B models: Good balance (recommended)
   - 13B+ models: Better quality, slower

3. **Quantization**
   - Q4: Faster, lower memory (recommended)
   - Q5: Balanced
   - Q8/FP16: Slower, higher quality

**Benchmark Examples (from README):**

| Hardware | Model | Speed (tok/s) | Experience |
|----------|-------|---------------|------------|
| M1 Mac 16GB | Mistral-7B Q4 | 30-40 | Excellent |
| M2 Mac 16GB | Llama-3-8B Q4 | 40-60 | Excellent |
| RTX 3060 12GB | Mistral-7B Q4 | 50-70 | Excellent |
| RTX 4090 24GB | Llama-3-70B Q4 | 20-30 | Very Good |
| CPU Only (i7) | Mistral-7B Q4 | 5-10 | Usable |

### Database Performance

**From benchmarks:** `benches/database.rs`

| Operation | Time | Notes |
|-----------|------|-------|
| Session creation | < 10ms | In-memory SQLite |
| Message insert | < 5ms | With token tracking |
| Message list query | < 20ms | Per session |
| Session list query | < 30ms | All sessions |

### Network Configuration

**Timeouts (from `src/llm/provider/openai.rs:28-30`):**
```rust
const DEFAULT_TIMEOUT: Duration = Duration::from_secs(120);        // 2 min
const DEFAULT_CONNECT_TIMEOUT: Duration = Duration::from_secs(10); // 10 sec
const DEFAULT_POOL_IDLE_TIMEOUT: Duration = Duration::from_secs(90); // 90 sec
```

**For slower local LLMs, increase timeouts in config:**
```toml
[providers.openai]
timeout = 300  # 5 minutes for very slow hardware
```

---

## Recommended Model Configurations

### For Coding Tasks (Best Balance)

**Recommended: Qwen 2.5 Coder 7B (Q4)**
```bash
# In LM Studio, search for: "Qwen2.5-Coder-7B-Instruct"
# Quantization: Q4_K_M
# RAM needed: 8-10GB
# Speed: 30-50 tok/s on modern CPU

export OPENAI_BASE_URL="http://localhost:1234/v1/chat/completions"
export OPENAI_API_KEY="local"
```

### For General Purpose

**Recommended: Mistral 7B Instruct v0.2 (Q4)**
```bash
# In LM Studio: "mistralai/Mistral-7B-Instruct-v0.2"
# Quantization: Q4_K_M
# RAM needed: 6-8GB
# Speed: 30-40 tok/s
```

### For Maximum Quality (Higher Resources)

**Recommended: Llama 3.1 70B (Q4)**
```bash
# In LM Studio: "meta-llama/Meta-Llama-3.1-70B-Instruct"
# Quantization: Q4_K_M
# RAM/VRAM needed: 40GB+
# Speed: 15-25 tok/s on high-end GPU
# Note: Requires powerful hardware
```

---

## Troubleshooting

### Issue: Connection Refused to Local LLM

**Solution:**
```bash
# 1. Verify LM Studio server is running
curl http://localhost:1234/v1/models

# 2. Check the port number (default is 1234)
# In LM Studio: Local Server tab → verify port

# 3. Update config with correct port
export OPENAI_BASE_URL="http://localhost:1234/v1/chat/completions"
```

### Issue: Slow Responses from Local LLM

**Solutions:**
1. Use smaller model (7B instead of 13B+)
2. Lower quantization (Q4 instead of Q8)
3. Enable GPU acceleration in LM Studio
4. Close memory-intensive applications
5. Increase timeout in config

### Issue: Out of Memory Errors

**Solutions:**
1. Use smaller model
2. Use Q4 quantization
3. Enable CPU offloading in LM Studio
4. Close other applications
5. Reduce context window

---

## Advanced Topics

### Custom Tool Development

**Create a new tool:**

```rust
// src/llm/tools/my_custom_tool.rs
use super::*;

pub struct MyCustomTool;

#[async_trait]
impl Tool for MyCustomTool {
    fn name(&self) -> &str {
        "my_custom_tool"
    }

    fn description(&self) -> &str {
        "Description of what this tool does"
    }

    fn input_schema(&self) -> Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "param1": {
                    "type": "string",
                    "description": "First parameter"
                }
            },
            "required": ["param1"]
        })
    }

    fn capabilities(&self) -> Vec<ToolCapability> {
        vec![ToolCapability::ReadFiles]
    }

    async fn execute(
        &self,
        input: Value,
        context: &ToolExecutionContext
    ) -> Result<ToolResult> {
        // Your implementation here
        Ok(ToolResult::success("Done!".to_string()))
    }
}
```

**Register the tool:**

```rust
// In src/llm/tools/registry.rs
impl ToolRegistry {
    pub fn new() -> Self {
        // ...
        registry.register(Arc::new(MyCustomTool));
        registry
    }
}
```

### Provider Retry Logic

**From `src/llm/provider/retry.rs`:**

```rust
pub struct RetryConfig {
    pub max_attempts: usize,      // Default: 3
    pub initial_backoff: Duration, // Default: 1 second
    pub max_backoff: Duration,     // Default: 60 seconds
    pub backoff_multiplier: f32,   // Default: 2.0
    pub jitter: bool,              // Default: true
}

pub async fn retry_with_backoff<F, Fut, T>(
    operation: F,
    config: &RetryConfig,
) -> Result<T>
where
    F: Fn() -> Fut,
    Fut: Future<Output = Result<T>>,
{
    // Exponential backoff with jitter
    // Automatically handles rate limits and transient errors
}
```

---

## Conclusion

Crustly provides a **complete AI assistant framework** with:

✅ **Privacy-First Design** - Full local LLM support
✅ **Safety Controls** - Interactive approval system
✅ **Extensible Architecture** - Easy to add new tools/providers
✅ **Production Ready** - 172 passing tests, robust error handling
✅ **Developer Friendly** - Terminal-native workflow

### Key Takeaways for Local LLM Usage

1. **Easy Setup**: Just set `OPENAI_BASE_URL` environment variable
2. **No Auth Required**: Uses dummy API key for local endpoints
3. **OpenAI Compatible**: Works with LM Studio, Ollama, LocalAI, etc.
4. **Full Feature Parity**: All tools work identically with local LLMs
5. **Cost-Free**: No API costs after model download

### Next Steps

1. **Start LM Studio** - Download and load a model
2. **Configure Crustly** - Set environment variables
3. **Run Interactive Mode** - `cargo run`
4. **Explore Tools** - Try file operations and bash commands
5. **Customize** - Add your own tools or models

---

**Generated:** 2025-11-10
**Crustly Version:** 0.2.0
**Architecture:** Rust + SQLx + Ratatui + Tokio
**License:** FSL-1.1-MIT
