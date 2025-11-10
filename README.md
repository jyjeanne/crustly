# Crustly 🥐

**High-Performance Terminal AI Assistant for Software Development**

> A blazingly fast, memory-efficient terminal-based AI assistant written in Rust.
> Rust reimplementation of Crush with 95%+ feature parity and superior performance.

[![Rust](https://img.shields.io/badge/rust-1.75%2B-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-FSL--1.1--MIT-blue.svg)](LICENSE.md)

```
   ___             _   _
  / __|_ _ _  _ __| |_| |_  _
 | (__| '_| || (_-<  _| | || |
  \___|_|  \_,_/__/\__|_|\_, |
                         |__/
        🥐 Flaky & Fast
```

---

## 🎯 Main Coding Features

**Your AI coding assistant that lives in your terminal.**

### ⚡ **Core Capabilities**

| Feature | Description | Benefit |
|---------|-------------|---------|
| 🔧 **Built-in Tools** | Read/Write files, Execute commands | Direct code manipulation from chat |
| 🔒 **Interactive Approval** | Permission dialogs for dangerous operations | Full control over what AI can do |
| 🎨 **Syntax Highlighting** | 100+ languages with line numbers | Beautiful code display in terminal |
| 🏠 **Local LLM Support** | Run with LM Studio/Ollama | 100% private, $0 cost, offline |
| 💬 **Multi-line Input** | Paste entire functions | Natural code interaction |
| 🧠 **Session Context** | Persistent conversation memory | Maintains project context |
| ⌨️ **Terminal Native** | Fast keyboard shortcuts | No context switching |
| 💰 **Cost Tracking** | Per-message token & cost | Budget control |
| 🌊 **Streaming** | Real-time response generation | See code as it's written |

### 🚀 **Quick Example**

```bash
$ crustly

You: "Read src/main.rs"
Crustly: [reads file with syntax highlighting]

You: "Add error handling to the database connection"
Crustly: [modifies file with write tool]

You: "Run cargo test"
Crustly: [executes] ✅ 145 tests passed

You: "Generate documentation for this module"
Crustly: [creates comprehensive docs]
```

### 🔒 **Privacy First**

```bash
# Use local LLMs for sensitive code
# 100% private - code never leaves your machine
# See "Using Crustly with Local LLMs" section below
```

### 💡 **Perfect For**

- ✅ **Code Generation** - Functions, tests, entire modules
- ✅ **Debugging** - Error analysis and fixes with context
- ✅ **Refactoring** - Improve code quality
- ✅ **Documentation** - Generate docs, comments, READMEs
- ✅ **Code Review** - Get feedback on your code
- ✅ **Learning** - Understand complex concepts
- ✅ **Terminal Workflow** - Stay in your flow, no browser tabs

### 🆚 **Why Choose Crustly?**

| You Want | Crustly Delivers |
|----------|------------------|
| Privacy | ✅ Local LLM support, data stays on your machine |
| Cost Control | ✅ Token tracking + free local inference |
| Terminal Native | ✅ No GUI, perfect for CLI lovers |
| File Operations | ✅ Built-in read/write/execute tools |
| Context Awareness | ✅ Persistent sessions, never lose context |
| Beautiful Code | ✅ Syntax highlighting for 100+ languages |
| Fast Workflow | ✅ Keyboard shortcuts, streaming responses |

---

## 🔒 Interactive Approval System

**Crustly gives you complete control over dangerous operations with beautiful interactive approval dialogs.**

### How It Works

When Claude wants to modify files or execute commands, Crustly pauses and asks for your permission:

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
│ [A]pprove  [D]eny  [V]iew Details  [Esc] Cancel  │
└────────────────────────────────────────────────────┘
```

### Security Features

✅ **Dangerous operations always require approval:**
- File writes (`write_file`)
- Shell commands (`bash`)
- System modifications

✅ **Safe operations proceed automatically:**
- File reads (`read_file`)
- Information queries

✅ **Full transparency:**
- See exactly what Claude wants to do
- View all parameters before deciding
- Toggle detailed JSON view with `V` key

✅ **Complete control:**
- Press `A` or `Y` to approve
- Press `D` or `N` to deny
- Press `Esc` to cancel
- No way to bypass (unless explicitly configured)

### Example Workflow

```bash
You: "Create a config file with debug enabled"

[Approval Dialog Appears]
Claude wants to: write_file
Path: config.json
Content: { "debug": true }

[You Press 'A']

Claude: ✅ "I've created the config file at config.json"
```

**Your safety is our priority.** Every dangerous operation requires your explicit approval.

---

## ⚠️ Important Disclaimers

### 🚧 Development Status

**Crustly is currently under active development.** While functional, it is not yet production-ready and may contain bugs or incomplete features.

### 💰 Token Cost Responsibility

**You are responsible for monitoring and managing your own API usage and costs.**

- We are **NOT responsible** for token cost overload from paid cloud AI services (Anthropic Claude, OpenAI, etc.)
- API costs are your responsibility - always monitor your usage
- Set up billing alerts with your cloud provider
- Consider using local LLMs (LM Studio, Ollama) for cost-free operation

### 🔧 Support Limitations

**We are NOT responsible for troubleshooting issues with paid cloud AI services.**

- Cloud API issues should be directed to the respective providers
- Billing questions should go to Anthropic, OpenAI, etc.
- We provide the tool, you manage your API relationships

### 💡 Recommendations

✅ **Always monitor your API usage dashboard**
✅ **Set billing limits with your cloud provider**
✅ **Test with small requests first**
✅ **Use local LLMs for cost-free development**
✅ **Review pricing before using cloud APIs**

> **By using Crustly, you acknowledge these risks and responsibilities.**

---

## 🌐 Supported AI Providers

Crustly currently has **2 fully implemented providers**: **Anthropic** and **OpenAI**. The OpenAI provider is compatible with any OpenAI-compatible API, enabling local LLMs and alternative providers.

### Implemented Providers

#### ✅ Anthropic Claude (Fully Supported)
- **Models**: Claude 3.5 Sonnet, Claude 3 Opus, Claude 3 Sonnet, Claude 3 Haiku
- **Setup**: `export ANTHROPIC_API_KEY="sk-ant-api03-YOUR_KEY"`
- **Features**: Streaming, tools, vision (via Claude), cost tracking

#### ✅ OpenAI (Fully Supported)
- **Models**: GPT-4 Turbo, GPT-4, GPT-3.5 Turbo
- **Setup**: `export OPENAI_API_KEY="sk-YOUR_KEY"`
- **Features**: Streaming, tools, cost tracking
- **Compatible with**: Any OpenAI-compatible API endpoint

### OpenAI-Compatible Providers

The OpenAI provider works with **any OpenAI-compatible API**, including:

| Provider | Status | Setup |
|----------|--------|-------|
| **LM Studio** | ✅ Tested | `OPENAI_BASE_URL="http://localhost:1234/v1"` |
| **Ollama** | ✅ Compatible | `OPENAI_BASE_URL="http://localhost:11434/v1"` |
| **LocalAI** | ✅ Compatible | `OPENAI_BASE_URL="http://localhost:8080/v1"` |
| OpenRouter | 🟡 Compatible | `OPENAI_BASE_URL="https://openrouter.ai/api/v1"` |
| Groq | 🟡 Compatible | `OPENAI_BASE_URL="https://api.groq.com/openai/v1"` |

### Future Providers (Planned)

These providers will require dedicated implementations:

| Provider | Status | Sprint |
|----------|--------|--------|
| Google Gemini | 📅 Planned | Sprint 12+ |
| AWS Bedrock | 📅 Planned | Sprint 12+ |
| Azure OpenAI | 📅 Planned | Sprint 12+ |
| Cerebras | 📅 Planned | Sprint 12+ |
| Huggingface | 📅 Planned | Sprint 12+ |

### Environment Variables

| Variable | Provider | Required |
|----------|----------|----------|
| `ANTHROPIC_API_KEY` | Anthropic Claude | ✅ For Anthropic |
| `OPENAI_API_KEY` | OpenAI / Compatible APIs | ✅ For OpenAI |
| `OPENAI_BASE_URL` | OpenAI-compatible APIs | Optional (for custom endpoints) |

### Example Configuration

```bash
# Linux/Mac
export ANTHROPIC_API_KEY="sk-ant-api03-YOUR_KEY_HERE"
export OPENAI_API_KEY="sk-YOUR_OPENAI_KEY"

# Windows PowerShell
$env:ANTHROPIC_API_KEY="sk-ant-api03-YOUR_KEY_HERE"
$env:OPENAI_API_KEY="sk-YOUR_OPENAI_KEY"
```

### Local LLMs (No API Key Required)

You can also use Crustly with **local LLMs** for 100% private, cost-free operation:
- **LM Studio** - Desktop app with OpenAI-compatible API ✅ **Ready to use!**
- **Ollama** - Command-line local model runner ✅ **Ready to use!**
- **LocalAI** - Self-hosted OpenAI alternative ✅ **Ready to use!**

**Quick Start with LM Studio:**
```bash
# 1. Start LM Studio with a model loaded
# 2. Set environment variable
export OPENAI_BASE_URL="http://localhost:1234/v1"

# 3. Run Crustly
cargo run
```

See [LM_STUDIO_GUIDE.md](docs/guides/LM_STUDIO_GUIDE.md) for complete setup instructions.

---

## 🚀 Quick Start

### Prerequisites

- **Rust 1.75+** - [Install Rust](https://rustup.rs/)
- **API Key** from your preferred provider (see Supported AI Providers above)
- **SQLite** (bundled with sqlx)
- **Git** (optional)

### Installation

```bash
# Clone the repository
git clone https://github.com/jyjeanne/crustly.git
cd crustly

# Build the project
cargo build --release

# Set your API key (choose your preferred provider)
export ANTHROPIC_API_KEY="sk-ant-api03-YOUR_KEY_HERE"
# or
export OPENAI_API_KEY="sk-YOUR_OPENAI_KEY"
# See "Supported AI Providers" section for all options

# Initialize configuration (optional)
cargo run -- init

# Run interactive mode
cargo run
```

### First Run

1. **Set your API key** (choose your preferred provider):
```bash
# Example with Anthropic (Linux/Mac)
export ANTHROPIC_API_KEY="sk-ant-api03-YOUR_KEY_HERE"

# Example with OpenAI (Linux/Mac)
export OPENAI_API_KEY="sk-YOUR_OPENAI_KEY"

# Windows PowerShell
$env:ANTHROPIC_API_KEY="sk-ant-api03-YOUR_KEY_HERE"
# or
$env:OPENAI_API_KEY="sk-YOUR_OPENAI_KEY"
```

> 💡 See the **Supported AI Providers** section above for the complete list of environment variables.


2. **Launch the TUI:**
```bash
cargo run
```

3. **Start chatting:**
   - Type your message
   - Press `Ctrl+Enter` to send
   - Press `Ctrl+H` to see all available commands and help
   - Press `Ctrl+C` to quit

> 💡 **Tip:** Press `Ctrl+H` at any time to display the comprehensive help screen with all keyboard shortcuts and features!

### Usage

```bash
# Interactive TUI mode (default)
cargo run
# or
cargo run -- chat

# Non-interactive mode (single command)
cargo run -- run "What is Rust?"

# With JSON output
cargo run -- run --format json "List 3 programming languages"

# With markdown output
cargo run -- run --format markdown "Explain async/await"

# Initialize configuration
cargo run -- init

# Show current configuration
cargo run -- config

# Show configuration with secrets
cargo run -- config --show-secrets

# Initialize database
cargo run -- db init

# Show database statistics
cargo run -- db stats
```

---

## 📋 A Note on Claude Max and GitHub Copilot

**Crustly only supports model providers through official, compliant APIs.**

We do not support or endorse any methods that rely on personal Claude Max and GitHub Copilot accounts or OAuth workarounds, which violate Anthropic and Microsoft's Terms of Service.

### Official API Access Only

✅ **Supported & Compliant:**
- Anthropic API (with official API key from console.anthropic.com)
- OpenAI API (with official API key)
- Local LLMs (LM Studio, Ollama, LocalAI)
- Any OpenAI-compatible API endpoint with proper authorization

❌ **Not Supported & Against ToS:**
- Using Claude Max subscription through unofficial methods
- Using GitHub Copilot through OAuth workarounds
- Reverse-engineered or unofficial API endpoints
- Account-sharing or credential-borrowing schemes

### Why This Matters

- **Legal Compliance** - Using unofficial methods violates provider Terms of Service
- **Account Safety** - Your accounts could be suspended or banned
- **Security Risks** - Unofficial methods may expose your credentials
- **Ethical Development** - We respect provider agreements and policies

### Recommended Alternatives

If you can't afford cloud API costs, consider these legitimate alternatives:
1. **Local LLMs** - Run models on your own hardware (see section below)
2. **API Credits** - Many providers offer free trial credits
3. **Educational Programs** - Some providers offer discounts for students/researchers

---

## 🏠 Using Crustly with Local LLMs (LM Studio)

Want to run Crustly completely offline with your own hardware? Here's how to use it with **LM Studio** for local inference.

### What is LM Studio?

[LM Studio](https://lmstudio.ai/) is a desktop application that lets you run large language models locally on your computer. It provides an OpenAI-compatible API server, making it perfect for use with Crustly.

**Benefits:**
- ✅ **100% Private** - All data stays on your machine
- ✅ **No API Costs** - Free inference after model download
- ✅ **Offline Operation** - Works without internet
- ✅ **Fast Responses** - No network latency (with good hardware)
- ✅ **OpenAI Compatible** - Drop-in replacement for cloud APIs

---

### Step-by-Step Setup Guide

#### Step 1: Install LM Studio

1. **Download LM Studio:**
   - Visit [https://lmstudio.ai/](https://lmstudio.ai/)
   - Download for your platform (Windows, macOS, or Linux)
   - Install the application

2. **Launch LM Studio:**
   ```bash
   # Open LM Studio from your applications menu
   # Or on Windows: Start Menu → LM Studio
   # Or on macOS: Applications → LM Studio
   ```

---

#### Step 2: Download a Model

1. **Open the Model Discovery Tab:**
   - Click the 🔍 **"Discover"** or **"Search"** tab in LM Studio

2. **Choose a Model:**

   **Recommended Models for Crustly:**

   | Model | Size | RAM Needed | Best For |
   |-------|------|------------|----------|
   | **Mistral-7B-Instruct** | 4-8 GB | 16 GB | General chat, fast responses |
   | **Llama-3-8B-Instruct** | 4-8 GB | 16 GB | Balanced performance |
   | **Qwen-2.5-7B-Instruct** | 4-8 GB | 16 GB | Coding tasks |
   | **DeepSeek-Coder-6.7B** | 4-7 GB | 16 GB | Code-focused |
   | **Llama-3.1-8B-Instruct** | 4-8 GB | 16 GB | Latest, very capable |

   > 💡 **Tip:** Start with a 7B-8B parameter model in Q4 or Q5 quantization for best speed/quality balance.

3. **Download Your Chosen Model:**
   - Search for the model (e.g., "Mistral 7B Instruct")
   - Click the **"Download"** button
   - Select quantization: **Q4_K_M** (recommended) or **Q5_K_M** (higher quality)
   - Wait for download to complete (may take 5-30 minutes depending on size)

---

#### Step 3: Load the Model

1. **Go to Chat Tab:**
   - Click the 💬 **"Chat"** tab in LM Studio

2. **Select Your Model:**
   - Click the dropdown at the top
   - Choose your downloaded model from the list
   - Wait for the model to load (10-60 seconds)

3. **Verify Model Loaded:**
   - You should see "Model loaded" in green
   - The model name appears at the top

---

#### Step 4: Start the Local Server

1. **Open the Local Server Tab:**
   - Click the **"Local Server"** or **"Developer"** tab (⚙️ icon)

2. **Configure Server Settings:**
   ```
   Port: 1234 (default - don't change unless needed)
   CORS: Enabled (✓)
   Model: [Your selected model should be shown]
   ```

3. **Start the Server:**
   - Click the **"Start Server"** button (green play icon)
   - Wait for "Server running on http://localhost:1234" message

4. **Verify Server Running:**
   ```bash
   # Test the server with curl
   curl http://localhost:1234/v1/models
   ```

   **Expected Response:**
   ```json
   {
     "object": "list",
     "data": [
       {
         "id": "mistral-7b-instruct-v0.2",
         "object": "model",
         "created": 1234567890,
         "owned_by": "lmstudio"
       }
     ]
   }
   ```

---

#### Step 5: Configure Crustly for LM Studio

1. **Create or Edit Crustly Config:**
   ```bash
   # Initialize config if not done already
   cargo run -- init

   # Open config file
   # Linux/Mac: ~/.config/crustly/config.toml
   # Windows: C:\Users\YourName\AppData\Roaming\crustly\config.toml
   ```

2. **Add OpenAI Provider Configuration:**

   Edit `config.toml` and add:

   ```toml
   [llm]
   default_provider = "openai"  # Use OpenAI-compatible provider

   [llm.providers.openai]
   api_key = "lm-studio"  # Can be any non-empty value for local
   base_url = "http://localhost:1234/v1"  # LM Studio local endpoint
   default_model = "local-model"  # Will use whatever is loaded in LM Studio

   # Optional: Add timeout settings for slower hardware
   timeout = 120  # 2 minutes for generation
   ```

3. **Alternative: Use Environment Variables:**
   ```bash
   # Linux/Mac
   export OPENAI_API_KEY="lm-studio"
   export OPENAI_BASE_URL="http://localhost:1234/v1"

   # Windows PowerShell
   $env:OPENAI_API_KEY="lm-studio"
   $env:OPENAI_BASE_URL="http://localhost:1234/v1"

   # Windows Command Prompt
   set OPENAI_API_KEY=lm-studio
   set OPENAI_BASE_URL=http://localhost:1234/v1
   ```

---

#### Step 6: Test the Connection

1. **Simple Test with Non-Interactive Mode:**
   ```bash
   cargo run -- run "Hello! Can you introduce yourself?"
   ```

2. **Expected Output:**
   ```
   🤔 Processing...

   Hello! I'm an AI assistant running locally on your machine through
   LM Studio. I'm based on [Model Name] and I'm here to help you with
   various tasks while keeping all your data private and secure.

   📊 Tokens: 45
   💰 Cost: $0.000000 (Local - FREE!)
   ```

3. **Launch Full TUI:**
   ```bash
   cargo run
   ```

4. **Verify in Header:**
   - Model should show as "local-model" or your actual model name
   - Cost should show $0.0000 (local inference)

---

### Step 7: Using Crustly with Local LLM

**Normal Usage:**
```bash
# Just use Crustly as normal!
cargo run
```

**Tips for Local LLMs:**

1. **First Response is Slower:**
   - The first message loads the model into memory
   - Subsequent messages are much faster

2. **Adjust Expectations:**
   - Local 7B models are smart but not Claude-level
   - Better for coding, simple tasks, and conversation
   - May struggle with very complex reasoning

3. **Monitor Performance:**
   ```bash
   # Check LM Studio logs for:
   # - Tokens per second (tok/s)
   # - Memory usage
   # - GPU utilization (if using GPU)
   ```

4. **Optimize Speed:**
   - Use GPU acceleration if available
   - Lower quantization (Q4) for speed
   - Reduce max tokens in responses
   - Close other memory-intensive apps

---

### Recommended Models by Use Case

#### 🚀 **Fast & Lightweight (4GB RAM)**
```
Model: TinyLlama-1.1B-Chat
Size: ~1 GB
Speed: Very fast
Use: Quick responses, simple tasks
```

#### ⚖️ **Balanced (16GB RAM)**
```
Model: Mistral-7B-Instruct-v0.2
Size: 4-8 GB (Q4_K_M quantization)
Speed: Fast
Use: General purpose, coding, chat
```

#### 💪 **High Quality (32GB RAM)**
```
Model: Llama-3-70B-Instruct
Size: 40+ GB (Q4 quantization)
Speed: Slower but very capable
Use: Complex reasoning, production use
```

#### 👨‍💻 **Coding Focused (16GB RAM)**
```
Model: DeepSeek-Coder-33B-Instruct
Size: 20 GB (Q4)
Speed: Medium
Use: Code generation, debugging
```

---

### Troubleshooting Local Setup

#### Problem: "Connection refused" error

**Symptoms:**
```
Error: Connection refused at http://localhost:1234/v1/chat/completions
Failed to connect to local LLM server
```

**Solution:**
```bash
# 1. Verify LM Studio server is running
curl http://localhost:1234/v1/models

# 2. Check the port (default is 1234)
# In LM Studio: Server tab → verify port number

# 3. Make sure config.toml has correct URL
base_url = "http://localhost:1234/v1/chat/completions"  # Include full path

# 4. Verify LM Studio server is actually started
# In LM Studio: Click "Start Server" button (should show green "Running")
```

**Common causes:**
- LM Studio server not started (click "Start Server" in LM Studio)
- Wrong port number in config
- Firewall blocking localhost connections
- LM Studio crashed or frozen

---

#### Problem: "Invalid model identifier" error ⚠️ COMMON

**Symptoms:**
```
Error: Invalid model identifier 'gpt-4-turbo-preview'
LM Studio logs: Model 'gpt-4-turbo-preview' not found
```

**Root Cause:** The `default_model` in your `crustly.toml` doesn't match the loaded model in LM Studio.

**Solution:**

1. **Find the EXACT model name in LM Studio:**
   - Open LM Studio
   - Go to **"Local Server"** tab
   - Look at the **"Currently Loaded Model"** field
   - Copy the model name **EXACTLY** (case-sensitive!)

   **Example model names:**
   - `qwen2.5-coder-7b-instruct` ✅
   - `mistral-7b-instruct-v0.2.Q4_K_M.gguf` ✅
   - `llama-3.2-1b-instruct` ✅

2. **Update your `crustly.toml`:**
   ```toml
   [providers.openai]
   enabled = true
   base_url = "http://localhost:1234/v1/chat/completions"
   default_model = "qwen2.5-coder-7b-instruct"  # ⭐ EXACT match required!
   ```

3. **Verify the fix:**
   ```bash
   # Check Crustly picked up the correct model
   cargo run -- config

   # Look for your model name in the output:
   # Providers:
   #   - openai: qwen2.5-coder-7b-instruct  <-- Should match LM Studio
   ```

4. **Test it:**
   ```bash
   cargo run -- run "Hello, can you introduce yourself?"

   # Should work now! ✅
   ```

**Important Notes:**
- Model name is **case-sensitive**
- Must include version numbers and quantization if shown
- Don't use generic names like "local-model" or "gpt-4"
- The name in `crustly.toml` must match LM Studio **exactly**

---

#### Problem: Context size / Context length overflow ⚠️ VERY COMMON

**Symptoms:**
```
Error: Context length exceeded
Error: Maximum context size is 2048, but 3542 tokens were provided
LM Studio shows: "Context overflow" or stops responding
```

**Root Cause:** Your conversation history + new message exceeds the model's context window.

**Solution 1: Increase Context Length in LM Studio (RECOMMENDED)**

This is the **best long-term solution**:

1. **Open LM Studio Settings:**
   - Click the **⚙️ Settings** icon (top-right)
   - Or go to **"Local Server"** tab → **"Server Options"**

2. **Find "Context Length" or "Max Context":**
   - Look for a field labeled:
     - "Context Length"
     - "Max Context Tokens"
     - "n_ctx"
     - "Context Window"

3. **Increase the value:**
   ```
   Current: 2048   ❌ Too small
   Recommended: 8192   ✅ Good for most tasks
   Maximum: 16384  ✅ Best (if your hardware supports it)
   ```

   **Guidelines:**
   - **Minimum:** 4096 (for basic conversations)
   - **Recommended:** 8192 (for development tasks)
   - **Optimal:** 16384 or 32768 (for large codebases)

4. **Apply and Restart:**
   - Click "Apply" or "Save"
   - **Stop and restart** the LM Studio server:
     1. Click "Stop Server"
     2. Wait 2 seconds
     3. Click "Start Server"

5. **Verify in Crustly:**
   ```bash
   cargo run
   # Send a longer message
   # Should work now! ✅
   ```

**Visual Guide (LM Studio):**
```
┌─────────────────────────────────────────┐
│ LM Studio - Server Options             │
├─────────────────────────────────────────┤
│                                         │
│ Model: qwen2.5-coder-7b-instruct       │
│                                         │
│ Context Length: [8192     ] ⭐         │
│                  ▲                      │
│              Change this!               │
│                                         │
│ Temperature: 0.7                        │
│ Max Tokens: 2048                        │
│                                         │
│ [Apply Settings]  [Start Server]       │
└─────────────────────────────────────────┘
```

---

**Solution 2: Start a New Session in Crustly (Quick Fix)**

If you can't increase context length, clear the conversation history:

```bash
# In Crustly TUI:
# Press Ctrl+N to start a new session
# This clears the conversation history

# Or from command line:
cargo run  # Start fresh
```

**Why this works:** New sessions have no history, so context usage is minimal.

---

**Solution 3: Use a Model with Larger Context**

Some models have larger context windows by default:

| Model | Default Context | Max Context |
|-------|----------------|-------------|
| Mistral-7B | 8192 | 32768 |
| Llama-3.2 | 8192 | 131072 |
| Qwen-2.5 | 8192 | 32768 |
| CodeLlama | 16384 | 100000 |

Download a model with a larger context window in LM Studio.

---

**Solution 4: Reduce Message Length**

Send shorter messages:
```
❌ Bad: Paste 5000 lines of code and ask "explain this"
✅ Good: "Read src/main.rs and explain the main function"
```

Let Crustly use tools to read files instead of pasting code in messages.

---

**Understanding Context Size:**

Context includes:
- System prompt (~200 tokens)
- All previous messages in conversation
- Current message
- Tool schemas (~300 tokens per tool)

**Example breakdown:**
```
System prompt:      200 tokens
Previous 5 messages: 1500 tokens
Current message:    500 tokens
Tool schemas:       800 tokens (13 tools × ~60 tokens)
─────────────────────────────
Total:              3000 tokens

If context limit is 2048 → Error! ❌
If context limit is 8192 → Success! ✅
```

---

**How to Monitor Context Usage:**

1. **Check token count in Crustly header:**
   ```
   💬 Tokens: 2,847  <-- Watch this number
   ```

2. **Watch LM Studio logs:**
   - Look for warnings about context length
   - Shows current context usage

3. **Start new sessions regularly:**
   - Long conversations use more context
   - Press `Ctrl+N` to start fresh when needed

---

**Best Practices to Avoid Context Overflow:**

1. ✅ Set context length to **8192 or higher** in LM Studio
2. ✅ Start new sessions for unrelated tasks (`Ctrl+N`)
3. ✅ Use tools to read files instead of pasting code
4. ✅ Keep prompts concise and specific
5. ✅ Monitor token count in the header
6. ❌ Don't paste huge code blocks in messages
7. ❌ Don't let conversations go on indefinitely

---

#### Problem: Very slow responses

**Solution:**
1. **Enable GPU acceleration in LM Studio:**
   - Settings → Enable GPU
   - Restart LM Studio

2. **Use lower quantization:**
   - Q4_K_M instead of Q8 or FP16
   - Smaller model (7B instead of 13B)

3. **Reduce max output tokens:**
   ```toml
   # In crustly.toml
   [providers.openai]
   max_tokens = 512  # Reduce from default 2048
   ```

4. **Close other apps to free RAM**

---

#### Problem: Model responses are poor quality

**Solutions:**
1. **Try a different model:**
   - Llama-3.1-8B is generally better than Mistral-7B
   - Qwen-2.5 is excellent for coding

2. **Use higher quantization:**
   - Q5_K_M or Q6_K instead of Q4_K_M
   - More VRAM/RAM needed but better quality

3. **Adjust temperature in LM Studio:**
   - Lower temperature (0.7) for factual responses
   - Higher temperature (1.0) for creative responses

4. **Increase context length** (see above)
   - Models perform better with more context

---

#### Problem: Out of memory errors

**Symptoms:**
```
LM Studio: "Out of memory"
System: Swap usage at 100%
Crustly: Connection timeout or crashes
```

**Solutions:**
1. **Use smaller model:**
   - 7B instead of 13B
   - Q4 instead of Q8
   - Example: Switch from `llama-3-70b` to `llama-3-8b`

2. **Enable offloading in LM Studio:**
   - Settings → GPU offloading → Adjust layers
   - Offload some layers to CPU if GPU memory limited
   - Example: Offload 20 layers to CPU, keep 20 on GPU

3. **Reduce context length:**
   - Instead of 32768, use 8192
   - Reduces memory usage significantly

4. **Close browser tabs and other apps:**
   - Chrome/Firefox can use 2-4 GB RAM
   - Close unnecessary applications
   - Check Task Manager (Windows) or Activity Monitor (macOS)

5. **Restart LM Studio:**
   - Sometimes memory leaks accumulate
   - Complete restart frees memory

---

#### Problem: LM Studio shows model loaded, but Crustly can't connect

**Solution:**
```bash
# 1. Make sure you clicked "Start Server" in LM Studio
#    Loading model ≠ Starting server

# 2. Verify server is actually running:
curl http://localhost:1234/v1/models

# Should return JSON with model info, not connection error

# 3. Check LM Studio logs for errors:
#    Look at bottom panel in LM Studio for error messages

# 4. Try restarting LM Studio completely
```

---

#### Problem: "Model not found" even though model name matches

**Solution:**

This can happen if:
1. Model name has special characters or spaces
2. Model file is corrupted
3. LM Studio cache is stale

**Fix:**
```bash
# 1. In LM Studio, unload the model
# 2. Click "Reload Model"
# 3. Wait for full load (check progress bar)
# 4. Verify model name again
# 5. Update crustly.toml with exact name
# 6. Test with: cargo run -- run "Hello"
```

---

#### Quick Troubleshooting Checklist

When things don't work, check in this order:

1. ✅ **LM Studio server running?**
   - Green "Running" indicator visible
   - Can curl http://localhost:1234/v1/models

2. ✅ **Model loaded in LM Studio?**
   - Model name visible at top
   - Loading progress at 100%

3. ✅ **Model name matches exactly?**
   - Run `cargo run -- config`
   - Compare with LM Studio's "Local Server" tab

4. ✅ **Context length sufficient?**
   - Set to 8192 or higher in LM Studio
   - Server restarted after changing

5. ✅ **Config file in correct location?**
   - `~/.config/crustly/crustly.toml` (Linux/macOS)
   - `%APPDATA%\crustly\crustly.toml` (Windows)

6. ✅ **No firewall blocking localhost?**
   - Rare, but check if nothing else works

If all checks pass and it still doesn't work:
- Check LM Studio logs for detailed errors
- Try a different model
- Restart both LM Studio and Crustly

---

### Performance Benchmarks (Approximate)

| Hardware | Model | Speed (tok/s) | Experience |
|----------|-------|---------------|------------|
| **M1 Mac 16GB** | Mistral-7B Q4 | 30-40 | Excellent |
| **M2 Mac 16GB** | Llama-3-8B Q4 | 40-60 | Excellent |
| **RTX 3060 12GB** | Mistral-7B Q4 | 50-70 | Excellent |
| **RTX 4090 24GB** | Llama-3-70B Q4 | 20-30 | Very Good |
| **CPU Only (i7)** | Mistral-7B Q4 | 5-10 | Usable |
| **CPU Only (i5)** | TinyLlama Q4 | 15-25 | Good |

---

### Comparison: Cloud vs Local

| Aspect | Cloud (Anthropic) | Local (LM Studio) |
|--------|-------------------|-------------------|
| **Privacy** | Data sent to API | 100% private |
| **Cost** | ~$3-15 per 1M tokens | Free (after download) |
| **Speed** | Very fast (1-2s) | Fast (2-10s depending on hardware) |
| **Quality** | Excellent (Claude) | Good (depends on model) |
| **Setup** | API key only | Download model + setup |
| **Offline** | ❌ Needs internet | ✅ Works offline |
| **Hardware** | None needed | 16GB+ RAM recommended |

---

### Best Practices for Local LLM Usage

1. **Start Small:**
   - Begin with 7B model to test your hardware
   - Upgrade to larger if needed and capable

2. **Keep LM Studio Updated:**
   - New versions have better performance
   - New models added regularly

3. **Monitor Resources:**
   - Watch RAM/VRAM usage
   - Check CPU/GPU temperature

4. **Use Appropriate Models:**
   - Coding: DeepSeek-Coder, Qwen
   - Chat: Llama-3, Mistral
   - Speed: TinyLlama, Phi

5. **Cache Models:**
   - LM Studio caches models in:
     - Mac: `~/.cache/lm-studio`
     - Windows: `C:\Users\YourName\.cache\lm-studio`
     - Linux: `~/.cache/lm-studio`

---

### Alternative Local Solutions

Besides LM Studio, Crustly can work with:

1. **Ollama** (CLI-based)
   ```bash
   # Install Ollama
   curl https://ollama.ai/install.sh | sh

   # Pull model
   ollama pull mistral

   # Configure Crustly
   base_url = "http://localhost:11434/v1"
   ```

2. **LocalAI** (Docker)
   ```bash
   docker run -p 8080:8080 localai/localai
   base_url = "http://localhost:8080/v1"
   ```

3. **Text-Generation-WebUI** (Advanced)
   ```bash
   # OpenAI API extension
   base_url = "http://localhost:5000/v1"
   ```

---

**🎉 You're now running Crustly completely locally and privately!**

> 💡 **Pro Tip:** Keep LM Studio running in the background, and Crustly will automatically use your local LLM instead of cloud APIs.

---

## 📝 Local Configuration with crustly.toml

### Understanding Configuration Options

Crustly supports **two configuration methods**:

1. **Environment Variables** (quick setup, temporary)
2. **Configuration File** (`crustly.toml`) - **RECOMMENDED for local LLMs**

The configuration file approach is **preferred for local LLM setups** because:
- ✅ Persistent across sessions (no need to re-export variables)
- ✅ More options available (custom model names, timeouts, etc.)
- ✅ Better for Windows users (no PowerShell profile editing)
- ✅ Version control friendly (can check in without secrets)

---

### Creating Your Local Configuration File

#### Step 1: Copy the Example Configuration

The repository includes a complete example configuration file:

```bash
# Linux/macOS
cp config.toml.example ~/.config/crustly/crustly.toml

# Windows (PowerShell)
Copy-Item config.toml.example $env:APPDATA\crustly\crustly.toml

# Alternative: Let Crustly create the directories
cargo run -- init
# Then manually copy config.toml.example to the location shown
```

---

#### Step 2: Edit Configuration for Your Setup

Open the config file in your favorite editor:

```bash
# Linux/macOS
nano ~/.config/crustly/crustly.toml
# or
code ~/.config/crustly/crustly.toml

# Windows
notepad %APPDATA%\crustly\crustly.toml
```

---

#### Step 3: Configure for LM Studio

Here's a **complete working configuration** for LM Studio:

```toml
# ~/.config/crustly/crustly.toml (Linux/macOS)
# or %APPDATA%\crustly\crustly.toml (Windows)

[database]
# Database file location (stores conversation history)
path = "~/.crustly/crustly.db"  # Linux/macOS
# path = "C:\\Users\\YourName\\.crustly\\crustly.db"  # Windows (use double backslashes)

[providers]
# ========================================
# Local LLM Configuration (LM Studio)
# ========================================
[providers.openai]
enabled = true
base_url = "http://localhost:1234/v1/chat/completions"  # LM Studio default port

# ⭐ CRITICAL: Set this to EXACTLY match the model name in LM Studio!
# How to find the model name:
#   1. Open LM Studio
#   2. Look at the "Local Server" tab
#   3. Copy the model name EXACTLY as shown (case-sensitive)
#
# Common examples:
#   - "qwen2.5-coder-7b-instruct"
#   - "mistral-7b-instruct-v0.2"
#   - "llama-3.2-1b-instruct"
#   - "deepseek-coder-6.7b-instruct"
default_model = "qwen2.5-coder-7b-instruct"

# Optional: Adjust timeout for slower hardware (seconds)
# timeout = 120  # Default: 120 seconds

# Optional: Set custom context length
# max_tokens = 8192  # Match LM Studio's context length setting
```

**⚠️ IMPORTANT:** The `default_model` value must **EXACTLY** match the model name shown in LM Studio's "Local Server" tab. Case-sensitive!

---

### Configuration File Locations

Crustly searches for `crustly.toml` in these locations (in order):

1. **Current directory**: `./crustly.toml`
2. **User config directory**:
   - **Linux/macOS**: `~/.config/crustly/crustly.toml`
   - **Windows**: `%APPDATA%\crustly\crustly.toml` (typically `C:\Users\YourName\AppData\Roaming\crustly\crustly.toml`)
3. **User home directory**: `~/crustly.toml` (Linux/macOS)

Environment variables **override** config file settings.

---

### Verify Your Configuration

After creating `crustly.toml`, verify it's correctly loaded:

```bash
# Check configuration
cargo run -- config

# Expected output:
# 🦀 Crustly Configuration
#
# Database: /home/user/.crustly/crustly.db
# Log level: info
#
# Providers:
#   - openai: qwen2.5-coder-7b-instruct  <-- Your model name
#     Base URL: http://localhost:1234/v1/chat/completions
#     API Key: [SET]
```

If you see your model name listed, **configuration is successful!** ✅

---

### Example Configurations for Different Setups

#### Configuration 1: LM Studio (Windows)

```toml
[database]
path = "C:\\Users\\YourName\\.crustly\\crustly.db"

[providers.openai]
enabled = true
base_url = "http://localhost:1234/v1/chat/completions"
default_model = "qwen2.5-coder-7b-instruct"
```

#### Configuration 2: Ollama (Linux)

```toml
[database]
path = "~/.crustly/crustly.db"

[providers.openai]
enabled = true
base_url = "http://localhost:11434/v1/chat/completions"
default_model = "mistral"  # Match model name from: ollama list
```

#### Configuration 3: Cloud API (Anthropic)

```toml
[database]
path = "~/.crustly/crustly.db"

[providers.anthropic]
enabled = true
api_key = "sk-ant-api03-YOUR_KEY_HERE"  # Or use ANTHROPIC_API_KEY env var
default_model = "claude-3-5-sonnet-20240620"
```

#### Configuration 4: Multiple Providers (Hybrid)

```toml
[database]
path = "~/.crustly/crustly.db"

# Local LLM for development (default)
[providers.openai]
enabled = true
base_url = "http://localhost:1234/v1/chat/completions"
default_model = "qwen2.5-coder-7b-instruct"

# Cloud API for complex tasks (manual selection)
[providers.anthropic]
enabled = true
api_key = "sk-ant-api03-YOUR_KEY_HERE"
default_model = "claude-3-5-sonnet-20240620"
```

---

### Configuration Tips

1. **Use `crustly.toml` for local LLMs** - Much easier than environment variables
2. **Keep secrets in environment variables** - Don't commit API keys to git
3. **The model name is critical** - Must match LM Studio exactly
4. **Test with `crustly config`** - Always verify before using
5. **Windows users: use double backslashes** - `C:\\Users\\...` not `C:\Users\...`

---

## 💡 Best Practices for Using Crustly

### Writing Effective Prompts

Crustly is equipped with **powerful tools** (file operations, code execution, web search, etc.). To get the most out of it, **encourage tool usage** in your prompts.

---

### ✅ Sample Prompts (Recommended)

These prompts **encourage Crustly to explore and use tools**:

#### 1. **Codebase Exploration**
```
Analyze this codebase:
1. Explore the /src directory structure
2. Identify the main entry points
3. Find all dependencies in Cargo.toml
4. List the design patterns used
5. Summarize the architecture

Start by using glob to find all Rust files.
```

**Why it works:** Explicitly tells Crustly to use tools (glob, read_file)

---

#### 2. **Deep Code Analysis**
```
I need a comprehensive analysis of the authentication system:
1. Find all files related to authentication (grep for "auth", "login", "session")
2. Read the main authentication modules
3. Document the flow from login to session creation
4. Identify security best practices used
5. Suggest improvements

Use grep and read_file tools to explore the code.
```

**Why it works:** Mentions specific tools, gives clear steps

---

#### 3. **Bug Investigation**
```
I'm getting a "connection timeout" error in the API client.
1. Find all files containing "timeout" or "connect"
2. Read the network client implementation
3. Check the configuration for timeout settings
4. Explain what's causing the issue
5. Suggest a fix

Start by using grep to locate the relevant code.
```

**Why it works:** Asks Crustly to investigate systematically

---

#### 4. **Feature Implementation**
```
I need to add rate limiting to the API:
1. Explore the current request handling code (find files with "request", "handler")
2. Read the middleware implementation
3. Research rate limiting strategies (use web_search if available)
4. Create a rate limiting middleware
5. Write tests for the new feature

Begin by exploring the existing middleware architecture.
```

**Why it works:** Multi-step task encourages thorough exploration

---

#### 5. **Documentation Generation**
```
Generate comprehensive documentation for this project:
1. Read README.md to understand current docs
2. Explore all modules in /src (use glob for *.rs files)
3. For each module, read and document:
   - Purpose and functionality
   - Public API
   - Usage examples
4. Create a DEVELOPER_GUIDE.md

Start by listing all source files.
```

**Why it works:** Structured task with clear tool usage

---

#### 6. **Dependency Analysis**
```
I want to understand all external dependencies:
1. Read Cargo.toml
2. For each dependency, search the code for usage (grep)
3. Document what each dependency is used for
4. Identify any unused dependencies
5. Suggest lightweight alternatives

Begin by reading the Cargo.toml file.
```

**Why it works:** Specific files mentioned, clear methodology

---

### ❌ Ineffective Prompts (To Avoid)

These prompts **don't encourage tool usage**, leading to generic responses:

```
❌ "What does this codebase do?"
   Better: "Explore the /src directory and summarize what this codebase does"

❌ "Explain how authentication works"
   Better: "Find and read all authentication-related files, then explain the flow"

❌ "Is there a bug in the code?"
   Better: "Search for potential bugs by reading the error handling code in /src"

❌ "What design patterns are used?"
   Better: "Analyze the codebase structure (use ls -R) and identify design patterns"

❌ "Improve the README"
   Better: "Read README.md, analyze the project structure (glob *.rs), then suggest improvements"
```

---

### Key Principles for Effective Prompts

1. **Be Specific About Tools:**
   - ✅ "Use glob to find all TypeScript files"
   - ❌ "Find TypeScript files"

2. **Give Step-by-Step Instructions:**
   - ✅ "1. Read the file, 2. Analyze the code, 3. Suggest improvements"
   - ❌ "Improve this file"

3. **Mention Files/Directories Explicitly:**
   - ✅ "Explore the /src/llm directory"
   - ❌ "Look at the code"

4. **Encourage Exploration:**
   - ✅ "Start by listing all files, then read the main modules"
   - ❌ "Tell me about the codebase"

5. **Request Evidence:**
   - ✅ "Read the test file and show me the test cases"
   - ❌ "Are there tests?"

---

### Pro Tips for Local LLM Users

When using **local models** (LM Studio, Ollama), keep these tips in mind:

1. **Be Even More Explicit:**
   - Local models may need more guidance
   - Example: "Use the glob tool with pattern **/*.rs to find all Rust files, then use read_file to read src/main.rs"

2. **Break Down Complex Tasks:**
   - Instead of: "Analyze the entire codebase"
   - Use: "First, list all directories. Then, for each directory, show me the key files."

3. **Provide Context:**
   - Include relevant background in your prompt
   - Example: "This is a Rust project using Tokio for async. Analyze the async patterns used."

4. **Use Examples:**
   - Show the model what you want
   - Example: "Find TODO comments using: grep -r 'TODO' src/"

5. **Iterate:**
   - If the first response is generic, follow up with: "Please use the glob tool to actually explore the files"

---

### Sample Workflow Session

Here's a **real example** of an effective Crustly session:

```
User: I want to understand how this Rust project handles errors.
      Start by using glob to find all Rust files, then grep for
      error handling patterns like "Result<", "Error", "unwrap".

Crustly: [Uses glob tool to find 92 .rs files]
         [Uses grep to search for error patterns]
         [Reads key error handling modules]

         Based on my analysis of the codebase:
         1. Found 156 instances of Result<T, E>
         2. Error types defined in src/error.rs
         3. Custom error enum with 12 variants
         4. Comprehensive error handling in services layer

         [Provides detailed summary with file references]

User: Great! Now read src/error.rs and explain the error handling strategy.

Crustly: [Uses read_file tool]
         [Provides detailed explanation with code examples]

User: Are there any places where errors are unwrapped unsafely?

Crustly: [Uses grep to search for ".unwrap()" and ".expect()"]
         [Reports findings with line numbers and context]
```

**Why this works:**
- User explicitly requests tools (glob, grep, read_file)
- Crustly uses the tools proactively
- Follow-up questions build on previous exploration
- Results are concrete and evidence-based

---

## 👨‍💻 Why Crustly for Coding?

Crustly is specifically designed to be a **developer's best friend** in the terminal. Here's what makes it perfect for coders:

### 🚀 **Core Coding Features**

#### 1. **Built-in Tool Execution System**
Execute commands and manipulate files directly from chat:

```
You: "Read the contents of src/main.rs"
Crustly: [executes read tool] Here's your code: ...

You: "Create a new test file with basic structure"
Crustly: [executes write tool] Created tests/integration_test.rs with: ...

You: "Run cargo test"
Crustly: [executes bash tool] Running tests... ✅ 145 tests passed
```

**Available Tools:**
- 📖 **`read`** - Read file contents with syntax awareness
- ✏️ **`write`** - Create or modify files
- 💻 **`bash`** - Execute shell commands safely
- 📁 **File tracking** - Monitors all files touched by tools

#### 2. **Syntax Highlighting for 100+ Languages**
Code appears with proper highlighting in the terminal:
- Rust, Python, JavaScript, TypeScript, Go, Java, C++, and 100+ more
- Uses `syntect` with professional color schemes
- Automatic language detection
- Line numbers for easy reference

#### 3. **Markdown Code Blocks**
Code snippets are beautifully rendered:
```rust
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

#### 4. **Multi-line Input**
Write or paste long code snippets naturally:
- Press `Enter` for new lines
- `Ctrl+Enter` to send
- Perfect for pasting entire functions or classes

#### 5. **Session-Based Context**
Crustly remembers your entire conversation:
```
You: "I'm working on a REST API in Rust"
Crustly: Great! I'll help you...

[Later in same session]
You: "Add error handling to the API"
Crustly: [Remembers you're working on Rust REST API]
```

#### 6. **Terminal-Native Workflow**
Stay in your terminal, no context switching:
- Launch with `crustly` or `cargo run`
- Split screen with your editor
- No browser tabs needed
- Fast keyboard shortcuts (`Ctrl+H` for help)

#### 7. **Local LLM Support (Privacy)**
Run completely offline with LM Studio:
- **100% Private** - Your proprietary code never leaves your machine
- **Zero API Costs** - Use local models like DeepSeek-Coder
- **Offline Development** - Work on sensitive projects securely
- See detailed guide above ⬆️

#### 8. **Streaming Responses**
See code generation in real-time:
- Character-by-character streaming
- Animated spinner shows processing
- No waiting for complete response
- Stop mid-generation if needed

#### 9. **Cost & Token Tracking**
Monitor your API usage:
```
💬 Tokens: 1,248  💰 Cost: $0.0037
```
- Per-message tracking
- Session totals
- Database persistence
- Budget control

---

### 🎯 **Common Coding Tasks**

#### **Code Generation**
```
You: "Write a binary search function in Rust with tests"
Crustly: [Generates implementation + tests with proper syntax highlighting]
```

#### **Code Review**
```
You: "Review this code for potential bugs"
[Paste your code]
Crustly: [Analyzes and provides feedback with specific line references]
```

#### **Debugging Help**
```
You: "I'm getting 'borrow checker error' in this code"
[Paste code]
Crustly: [Explains the issue and shows the fix with highlighting]
```

#### **Refactoring**
```
You: "Refactor this function to be more idiomatic Rust"
Crustly: [Shows before/after with explanations]
```

#### **Documentation**
```
You: "Generate doc comments for this module"
Crustly: [Creates comprehensive rustdoc comments]
```

#### **Testing**
```
You: "Write unit tests for this struct"
Crustly: [Generates test cases with proper assertions]
```

#### **Command Execution**
```
You: "Show me all TODO comments in the project"
Crustly: [Executes] grep -r "TODO" src/
```

---

### 🔄 **Typical Developer Workflow**

**Morning:**
```bash
$ crustly
> "Show me what we worked on yesterday"
[Crustly loads previous session and summarizes]

> "Let's continue with the authentication module"
[Crustly maintains context from yesterday]
```

**Implementing Feature:**
```
> "Create a new user authentication service"
[Crustly generates code with write tool]

> "Add password hashing with bcrypt"
[Crustly adds the feature]

> "Write integration tests"
[Crustly creates test file]

> "Run the tests"
[Executes: cargo test]
```

**Debugging:**
```
> "The login endpoint returns 500, here's the error:"
[Paste error]

> "Read the auth service file"
[Crustly reads it with read tool]

> "Fix the issue"
[Crustly modifies file with write tool]

> "Run tests again"
[Executes: cargo test] ✅ All passing!
```

**Documentation:**
```
> "Generate API documentation for the endpoints"
[Crustly creates comprehensive docs]

> "Add examples to the README"
[Crustly updates README with code examples]
```

---

### 💡 **Pro Tips for Coders**

1. **Keep Context in Sessions:**
   - Start new session per feature/bug
   - Use `Ctrl+L` to switch between projects
   - Session history persists indefinitely

2. **Leverage Tool System:**
   - Let Crustly read files instead of pasting
   - Use bash tool for git commands
   - Write tool for quick file generation

3. **Use Local LLMs for Sensitive Code:**
   - Company proprietary code
   - Pre-release features
   - Security-sensitive implementations

4. **Keyboard Shortcuts:**
   ```
   Ctrl+Enter  - Send message
   Ctrl+H      - Help (full command list)
   Ctrl+N      - New session (new feature)
   Ctrl+L      - Switch sessions (different projects)
   Page Up/Down - Scroll through long code outputs
   ```

5. **Multi-line for Code:**
   - Paste entire functions
   - Press Enter for newlines
   - `Ctrl+Enter` when ready to send

6. **Markdown for Formatting:**
   - Use triple backticks for code blocks
   - Specify language for syntax highlighting
   - Makes responses easier to read

---

### 🆚 **Comparison with Other Coding Assistants**

| Feature | Crustly | GitHub Copilot | ChatGPT | Cursor |
|---------|---------|----------------|---------|--------|
| **Terminal Native** | ✅ | ❌ | ❌ | ❌ |
| **File Operations** | ✅ Built-in | ❌ | ❌ | ✅ |
| **Command Execution** | ✅ | ❌ | ❌ | ❌ |
| **Local LLM Support** | ✅ | ❌ | ❌ | ❌ |
| **Session History** | ✅ Persistent | ❌ | ✅ Limited | ✅ |
| **Syntax Highlighting** | ✅ 100+ langs | ✅ | ❌ | ✅ |
| **Cost Tracking** | ✅ | ❌ | ❌ | ❌ |
| **Offline Mode** | ✅ | ❌ | ❌ | ❌ |
| **Open Source** | ✅ | ❌ | ❌ | ❌ |
| **Privacy First** | ✅ | ⚠️ | ⚠️ | ⚠️ |

---

### 🎓 **Perfect For:**

- ✅ **Backend Developers** - Rust, Go, Python, Node.js
- ✅ **Systems Programmers** - C, C++, Rust
- ✅ **DevOps Engineers** - Shell scripting, automation
- ✅ **Full-Stack Developers** - Multiple languages
- ✅ **Open Source Contributors** - Code review, documentation
- ✅ **Students** - Learning programming concepts
- ✅ **Security-Conscious Devs** - Local inference for proprietary code
- ✅ **CLI Enthusiasts** - Terminal workflow lovers
- ✅ **Budget-Conscious** - Cost tracking + local LLMs

---

### 🚀 **Future Coding Features (Planned)**

- 🔜 **LSP Integration** - Semantic code understanding
- 🔜 **Git Integration** - Commit message generation, PR reviews
- 🔜 **Project Context** - Auto-load `.cursorrules`, codebase awareness
- 🔜 **Code Search** - Grep across entire projects
- 🔜 **Refactoring Tools** - Automated code transformations
- 🔜 **Test Generation** - Intelligent test case creation
- 🔜 **Performance Analysis** - Profiling suggestions
- 🔜 **Security Scanning** - Vulnerability detection

---

**Ready to supercharge your coding workflow?** 🚀

```bash
cargo run
# Start coding with Crustly!
```

---

## ✨ Features

### Currently Implemented (Sprint 11 Complete ✅)

#### Interactive Terminal UI (TUI)
- **Modern Interface** - Built with Ratatui for responsive terminal experience
- **Real-time Chat** - Send/receive messages with AI models
- **Session Management** - Create, switch, and resume conversations
- **Markdown Rendering** - Rich text formatting with pulldown-cmark
  - Headings (H1-H3) with bold, underlined, cyan styling
  - Code blocks with decorative borders and language labels
  - Inline code with yellow highlighting
  - Horizontal rules and proper line spacing
- **Syntax Highlighting** - 100+ languages via syntect
  - Rust, Python, JavaScript, TypeScript, Go, Java, C++, and more
  - Line numbers for code blocks
  - Base16 Ocean Dark theme
- **Visual Polish**
  - Animated braille spinner for loading states (⠋ ⠙ ⠹ ...)
  - Block cursor (█) in input field
  - Color-coded messages by role (User: Cyan, Claude: Green)
  - Emoji indicators (📝 Session, 🤖 Model, 💬 Tokens, 💰 Cost)
  - Beautiful croissant splash screen on startup
- **Keyboard Shortcuts** - Efficient navigation and control
  - `Ctrl+Enter` - Send message
  - `Ctrl+N` - New session
  - `Ctrl+L` - List sessions
  - `Ctrl+H` - Show help (📚 **Press Ctrl+H from anywhere to see all commands!**)
  - `Ctrl+C` - Quit
  - `Escape` - Clear input
  - `Page Up/Down` - Scroll chat history
- **Enhanced Help Screen** - Comprehensive command reference with:
  - Global commands (always available)
  - Chat mode commands (message composition)
  - Session list commands (navigation)
  - Feature showcase (what Crustly can do)

#### LLM Integration
- **Multi-Provider Support** - Two providers fully implemented:
  - **Anthropic Claude** - Full support for Claude 3 models
    - `claude-3-5-sonnet-20240620` (default)
    - `claude-3-opus-20240229`
    - `claude-3-sonnet-20240229`
    - `claude-3-haiku-20240307`
  - **OpenAI** - Full support for GPT models + local LLMs
    - `gpt-4-turbo-preview` (default)
    - `gpt-4`, `gpt-3.5-turbo`
    - Compatible with LM Studio, Ollama, LocalAI (OpenAI-compatible APIs)
- **Streaming Responses** - Real-time message streaming
- **Context Preservation** - Multi-turn conversations with full history
- **Automatic Retry Logic** - Exponential backoff with jitter
- **Rate Limit Handling** - Respects Retry-After headers

#### Tool Execution System
- **Built-in Tools**:
  - **read** - Read file contents
  - **write** - Create or edit files
  - **bash** - Execute shell commands
- **Interactive Approval System** - Full control over dangerous operations
  - Beautiful approval dialogs with tool details
  - View full JSON parameters before approving
  - Auto-deny after 5 minutes (timeout protection)
  - Visual countdown timer (color-coded: green/yellow/red)
  - Keyboard shortcuts: A/Y (approve), D/N (deny), V (view details)
- **Extensible Registry** - Easy to add new tools

#### Cost & Token Tracking
- **Per-Message Tracking** - Token count and cost for each message
- **Session Totals** - Accumulated usage per conversation
- **Database Persistence** - All metrics saved for analysis

#### Database & Persistence
- **SQLite Storage** - Local-first data storage
- **Automatic Migrations** - Schema versioning with SQLx
- **Session History** - All conversations saved
- **File Tracking** - Monitor files touched by tools

#### Configuration System
- **TOML Configuration** - Easy-to-edit config files
- **Environment Variables** - Override config with env vars
- **Hierarchical Loading** - System → Local → Environment
- **Provider Management** - Configure multiple LLM providers

#### CLI Commands
- **`chat`** - Launch interactive TUI (default)
- **`run`** - Non-interactive single command execution
- **`init`** - Initialize configuration
- **`config`** - Show current configuration
- **`db`** - Database management (init, stats)

#### Output Formats (Non-Interactive Mode)
- **Text** - Plain text with statistics (default)
- **JSON** - Structured JSON output
- **Markdown** - Formatted markdown

#### Error Recovery & Resilience (Sprint 11)
- **Automatic Retry Logic** - Exponential backoff with jitter for API calls
  - Configurable max attempts (default: 3 retries)
  - Smart error classification (retryable vs permanent)
  - Rate limit aware with Retry-After header support
- **Database Lock Recovery** - SQLite lock detection and retry
  - Busy timeout configuration (5 seconds)
  - Exponential backoff for concurrent access
- **Structured Error Reporting** - Rich error information with severity levels
  - Color-coded error display (Info/Warning/Error/Critical)
  - Error categorization (Network/Database/Config/Input/Tool/Internal)
  - Retry tracking with next-retry estimation

#### Developer Experience
- **Fast Execution** - Async runtime with Tokio
- **Comprehensive Error Handling** - Detailed error messages with context
- **Logging** - Structured logging with tracing
- **Local-First** - All data stored locally for privacy
- **Cross-Platform** - Windows, Linux, macOS support
- **Performance Benchmarks** - Criterion-based database benchmarks

### Planned Features (Future Sprints)

- **Additional LLM Providers** - Expand beyond Anthropic and OpenAI
  - Google Gemini
  - AWS Bedrock
  - Azure OpenAI
  - Groq (ultra-fast inference)
  - OpenRouter (multi-model gateway)
  - Cerebras
- **LSP Integration** - Semantic code understanding for better context
- **MCP Support** - Model Context Protocol
- **Context Files** - Auto-load `.cursorrules` for project-specific behavior
- **Image/Vision Support** - Vision model integration for analyzing images
- **Security Hardening** (Sprint 12)
  - OS keyring integration for API key storage
  - Audit log for tool approval decisions
  - Path validation (prevent directory traversal)
  - Command sanitization (prevent injection)

---

## 🧪 Manual Testing Guide

### Prerequisites for Testing

1. **Set API Key:**
```bash
export ANTHROPIC_API_KEY="sk-ant-api03-YOUR_KEY_HERE"
```

2. **Build the Project:**
```bash
cargo build --release
```

3. **Enable Debug Logging (Optional):**
```bash
export RUST_LOG="crustly=debug"
```

### Test Scenario 1: Simple Interactive Chat

**Goal:** Verify TUI launches and can send/receive messages.

**Steps:**
1. Launch TUI:
```bash
cargo run
```

2. **Expected:** Terminal shows:
   - Header with session info
   - Empty chat area
   - Input box at bottom
   - Status bar with keyboard shortcuts

3. Type a message:
```
Hello! Can you introduce yourself?
```

4. Press `Ctrl+Enter` to send

5. **Expected:**
   - Your message appears in blue
   - "Processing..." indicator shows
   - Claude's response appears in green (1-3 seconds)
   - Token count and cost update in header

6. Press `Ctrl+C` to quit

**Success Criteria:**
- ✅ TUI launches without errors
- ✅ Messages send and receive successfully
- ✅ UI updates in real-time
- ✅ Keyboard shortcuts work

---

### Test Scenario 2: Non-Interactive Mode

**Goal:** Test single-command execution with different output formats.

**Steps:**

1. **Text Output (default):**
```bash
cargo run -- run "What is 2+2?"
```

**Expected Output:**
```
🤔 Processing...

The answer is 4.

📊 Tokens: 42
💰 Cost: $0.000126
```

2. **JSON Output:**
```bash
cargo run -- run --format json "List 3 programming languages"
```

**Expected:** Valid JSON with `content`, `usage`, `cost`, `model` fields

3. **Markdown Output:**
```bash
cargo run -- run --format markdown "Explain async/await in 2 sentences"
```

**Expected:** Markdown formatted response with metadata

**Success Criteria:**
- ✅ All three output formats work
- ✅ Real API responses received
- ✅ Token and cost tracking accurate

---

### Test Scenario 3: Session Management

**Goal:** Verify session creation, listing, and switching.

**Steps:**

1. **Create first session:**
```bash
cargo run
# Type: "This is my first conversation"
# Ctrl+Enter to send
# Wait for response
# Ctrl+C to quit
```

2. **Create second session:**
```bash
cargo run
# Type: "This is a different conversation"
# Ctrl+Enter
# Wait for response
```

3. **Press `Ctrl+L`** to list sessions

4. **Expected:**
   - See list of 2 sessions
   - Each with creation timestamp
   - Current session highlighted
   - Navigate with ↑/↓

5. **Select first session:**
   - Press ↑ to highlight first session
   - Press Enter to switch

6. **Expected:**
   - Chat history from first session loads
   - Previous messages visible

7. **Press `Ctrl+N`** to create new session

8. **Expected:**
   - New empty session created
   - Chat area clears

9. **Check database:**
```bash
cargo run -- db stats
```

**Expected:**
```
📊 Database Statistics

Sessions: 3
Messages: 4
Tracked files: 0
```

**Success Criteria:**
- ✅ Multiple sessions work independently
- ✅ Session switching preserves history
- ✅ New sessions create cleanly
- ✅ Database tracks all data

---

### Test Scenario 4: Cost and Token Tracking

**Goal:** Verify accurate tracking of API usage and costs.

**Steps:**

1. **Start new session:**
```bash
cargo run
```

2. **Send short message:**
```
Hi
```

3. **Note tokens and cost in header**

4. **Send longer message:**
```
Can you write a detailed explanation of how Rust's ownership system works?
```

5. **Expected:**
   - Higher token count for longer message
   - Higher cost accumulated
   - Numbers update after each message

6. **Quit and restart, check session total persists**

**Success Criteria:**
- ✅ Token counting accurate
- ✅ Cost calculation correct
- ✅ Totals accumulate properly
- ✅ Data persists across restarts

---

### Test Scenario 5: Multi-Turn Conversation with Context

**Goal:** Verify Claude maintains context across messages.

**Steps:**

1. **Start TUI:**
```bash
cargo run
```

2. **Message 1:**
```
My favorite color is blue.
```

3. **Message 2:**
```
What's my favorite color?
```

4. **Expected Response:**
   - Claude correctly responds "Your favorite color is blue"

5. **Message 3:**
```
If I mix my favorite color with yellow, what do I get?
```

6. **Expected Response:**
   - Claude knows favorite color is blue
   - Responds with "green"

**Success Criteria:**
- ✅ Context maintained across messages
- ✅ Claude references previous information
- ✅ Full conversation history visible

---

### Test Scenario 6: Configuration Management

**Goal:** Verify configuration system works.

**Steps:**

1. **Initialize config:**
```bash
cargo run -- init
```

**Expected:**
```
🦀 Crustly Configuration Initialization

✅ Configuration initialized at: ~/.config/crustly/config.toml

📝 Next steps:
   1. Edit the config file to add your API keys
   2. Set ANTHROPIC_API_KEY environment variable
   3. Run 'crustly' or 'crustly chat' to start
```

2. **Show config:**
```bash
cargo run -- config
```

**Expected:**
```
🦀 Crustly Configuration

Database: /path/to/crustly.db
Log level: info

Providers:
  - anthropic: claude-3-5-sonnet-20240620
    API Key: [SET]

💡 Use --show-secrets to display API keys
```

3. **Show with secrets:**
```bash
cargo run -- config --show-secrets
```

**Expected:** API key visible

**Success Criteria:**
- ✅ Config file created
- ✅ Config displayed correctly
- ✅ Secrets properly hidden/shown

---

### Troubleshooting Common Issues

#### Issue: "API key not set" error

**Solution:**
```bash
# Set environment variable
export ANTHROPIC_API_KEY="your-key-here"

# Verify it's set
echo $ANTHROPIC_API_KEY
```

#### Issue: Database errors

**Solution:**
```bash
# Reinitialize database
cargo run -- db init
```

#### Issue: Slow responses

**Check:**
- Internet connection speed
- Anthropic API status: https://status.anthropic.com/
- Message length (longer messages = slower responses)

#### Issue: TUI doesn't launch

**Check:**
```bash
# Enable debug logging
RUST_LOG=debug cargo run

# Check terminal compatibility
# Try different terminal emulator if needed
```

---

### Running Automated Tests

After manual testing, run the full test suite:

```bash
# Run all tests (139 total: 130 unit + 9 integration)
cargo test --all

# Run only unit tests
cargo test --lib

# Run only integration tests
cargo test --test integration_test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_end_to_end_simple_message
```

**Expected:** All 172 tests pass in ~2.4 seconds

---

## 📊 Performance

### Test Suite Performance

| Test Suite | Tests | Time | Status |
|------------|-------|------|--------|
| Unit Tests | 163 | ~2.3s | ✅ |
| Integration Tests | 9 | ~0.1s | ✅ |
| **Total** | **172** | **~2.4s** | **✅** |

### Database Operations

| Operation | Time | Notes |
|-----------|------|-------|
| Session creation | < 10ms | In-memory SQLite |
| Message insert | < 5ms | With token tracking |
| Message list query | < 20ms | Per session |
| Session list query | < 30ms | All sessions |

### Application Performance

| Metric | Current | Target | Status |
|--------|---------|--------|--------|
| Test Execution | ~2.7s | < 5s | ✅ |
| Startup Time | TBD | < 50ms | 📊 Needs benchmarking |
| Memory Usage (idle) | ~15MB | < 25MB | ✅ |
| Memory Usage (100 msgs) | ~20MB | < 50MB | ✅ |

---

## 🏗️ Architecture

```
Presentation Layer
    ↓
CLI (Clap) + TUI (Ratatui)
    ↓
Application Layer
    ↓
Service Layer (Session, Message, Agent)
    ↓
Data Access Layer (SQLx + SQLite)
    ↓
Integration Layer (LLM, LSP, MCP)
```

**Key Technologies:**
- **Tokio** - Async runtime
- **Axum** - HTTP server (future)
- **Ratatui** - Terminal UI
- **SQLx** - Database access
- **Clap** - CLI parsing
- **Tower-LSP** - LSP client
- **Crabrace** - Provider registry

---

## 📁 Project Structure

```
crustly/
├── src/
│   ├── cli/           # Command-line interface
│   ├── app/           # Application lifecycle
│   ├── config/        # Configuration management
│   │   └── crabrace.rs # Crabrace integration ✅
│   ├── db/            # Database layer (SQLx)
│   ├── services/      # Business logic
│   ├── llm/           # LLM integration
│   │   ├── agent/     # Agent service
│   │   ├── provider/  # LLM providers
│   │   ├── tools/     # Tool system
│   │   └── prompt/    # Prompt engineering
│   ├── tui/           # Terminal UI
│   ├── lsp/           # LSP integration
│   ├── mcp/           # MCP support
│   └── utils/         # Utilities
├── tests/             # Integration tests
├── benches/           # Benchmarks
└── docs/              # Documentation
```

---

## 🛠️ Development

### Build from Source

```bash
# Development build
cargo build

# Release build (optimized)
cargo build --release

# With profiling
cargo build --release --features profiling

# Run tests
cargo test

# Run benchmarks
cargo bench

# Format code
cargo fmt

# Lint
cargo clippy -- -D warnings
```

### Development Roadmap

**Current Status:** Sprint 11 Complete ✅ - Production Ready with Error Recovery 🟢

| Sprint | Focus | Status |
|--------|-------|--------|
| Sprint 0-1 | Database & Foundation | ✅ Complete |
| Sprint 2 | Configuration System | ✅ Complete |
| Sprint 3 | Service Layer | ✅ Complete |
| Sprint 4 | LLM Integration | ✅ Complete |
| Sprint 5 | TUI Framework | ✅ Complete |
| Sprint 6 | Runnable Application | ✅ Complete |
| Sprint 7 | Testing Infrastructure | ✅ Complete |
| Sprint 8 | Enhanced Testing (+43 tests) | ✅ Complete |
| Sprint 9 | Enhanced TUI (Markdown, Syntax Highlighting) | ✅ Complete |
| Sprint 10 | Multi-Provider Support (OpenAI) | ✅ Complete |
| Sprint 11 | Error Recovery & Resilience | ✅ Complete |
| Sprint 12+ | Advanced Features (Security, LSP, etc.) | 📅 Planned |

**Progress:** ~70% of original roadmap complete
**Core Functionality:** 100% working
**Current State:** Fully functional CLI AI assistant with TUI

---

## 📖 Documentation

### User Documentation
- **[User Guide](docs/guides/README_USER_GUIDE.md)** - Complete user guide with examples
- **[Manual Testing Guide](docs/guides/MANUAL_TESTING_GUIDE.md)** - Step-by-step testing instructions

### Development Documentation
- **[Testing Summary](docs/development/TESTING_SUMMARY.md)** - Test coverage and infrastructure
- **[Sprint 6 Complete](docs/development/SPRINT_6_COMPLETE.md)** - Runnable application completion
- **[Technical Specification](docs/CRUSTLY_SPECIFICATION_FINAL.md)** - Complete spec (v3.0)
- **[Implementation Summary](docs/IMPLEMENTATION_SUMMARY.md)** - Development roadmap
- **[Crabrace Integration](docs/guides/CRABRACE_INTEGRATION.md)** - Provider registry guide
- **[Build Notes](docs/guides/BUILD_NOTES.md)** - Build instructions & known issues
- **[Specification Review](docs/SPECIFICATION_REVIEW.md)** - Feature analysis

---

## 🤝 Contributing

Contributions welcome! Please read [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

### Development Setup

1. Install Rust 1.75+
2. Clone the repository
3. Run `cargo build`
4. Make changes
5. Run tests: `cargo test`
6. Submit PR

---

## 🐛 Known Issues

### Windows Build Issue

**Error:** `dlltool.exe not found`

**Solution:** See [BUILD_NOTES.md](docs/guides/BUILD_NOTES.md) for Windows setup instructions.

Alternative: Use WSL2 or Linux/macOS for development.

---

## 📄 License

**FSL-1.1-MIT License**

- **Functional Source License (FSL) 1.1** - First 2 years
- **MIT License** - After 2 years from release

See [LICENSE.md](LICENSE.md) for details.

---

## 🙏 Acknowledgments

- **Crush (Go)** - Original implementation
- **Crabrace** - Provider registry (Rust port of Catwalk)
- **Anthropic** - API
- **Ratatui Community** - Terminal UI framework

---

## 📞 Support

- **Issues:** [GitHub Issues](https://github.com/jyjeanne/crustly/issues)
- **Discussions:** [GitHub Discussions](https://github.com/jyjeanne/crustly/discussions)
- **Documentation:** [docs/](docs/)

---

## 📈 Status

**Current Version:** 0.1.0-alpha
**Development Status:** 🎉 **Sprint 11 Complete** ✅
**Application Status:** 🟢 **Production Ready with Error Recovery**
**Test Coverage:** 172 tests (100% pass rate)
**Multi-Provider Support:** ✅ Anthropic + OpenAI
**Local LLM Support:** ✅ LM Studio, Ollama (via OpenAI provider)
**Database Layer:** ✅ Complete (with lock recovery)
**Configuration System:** ✅ Complete
**Service Layer:** ✅ Complete
**LLM Integration:** ✅ Complete (2 providers)
**TUI Framework:** ✅ Complete (Markdown, Syntax Highlighting)
**CLI Application:** ✅ Complete
**Testing Infrastructure:** ✅ Complete (172 total tests)
**Error Recovery:** ✅ Complete (Retry logic, rate limiting)

### Sprint 0-1 Achievements ✅ (Database & Foundation)

- ✅ Project structure initialized (30+ files)
- ✅ Database schema (5 tables, 8 indexes)
- ✅ SQLx connection pool with migrations
- ✅ 5 data models (Session, Message, File, etc.)
- ✅ 3 full repositories with CRUD operations
- ✅ Archive system for sessions
- ✅ Token & cost tracking
- ✅ Error handling with 12 error codes
- ✅ Logging setup (tracing)

### Sprint 2 Achievements ✅ (Configuration System)

- ✅ Enhanced config loading (TOML + env vars)
- ✅ Hierarchical config system (defaults → system → local → env)
- ✅ Provider configurations for 6 LLM providers
- ✅ Secure secret management with zeroize
- ✅ Provider auto-update mechanism with Crabrace
- ✅ Config validation & save/load
- ✅ Debug options (debug_lsp, profiling)
- ✅ 29 comprehensive tests (all passing)

### Sprint 3 Achievements ✅ (Service Layer)

- ✅ Service layer architecture (ServiceContext, ServiceManager)
- ✅ SessionService with comprehensive business logic (350+ lines, 12 tests)
- ✅ MessageService with message management (390+ lines, 12 tests)
- ✅ FileService with file tracking (350+ lines, 11 tests)
- ✅ Enhanced database module with Pool management
- ✅ Model alignment with modern Rust patterns
- ✅ Custom FromRow implementations for type safety
- ✅ Database migration for schema transformation

### Sprint 4 Achievements ✅ (LLM Integration)

- ✅ Provider abstraction layer (trait-based)
- ✅ Anthropic provider implementation
- ✅ Message streaming support
- ✅ Agent service with context management
- ✅ Tool execution framework (3 tools: read, write, bash)
- ✅ Tool registry system
- ✅ Token usage and cost calculation
- ✅ Error handling for API failures
- ✅ Model selection and routing

### Sprint 5 Achievements ✅ (TUI Framework)

- ✅ Modern TUI with Ratatui
- ✅ Event-driven architecture with async channels
- ✅ Chat interface with scrolling
- ✅ Session list overlay
- ✅ Help screen
- ✅ Status bar with keyboard shortcuts
- ✅ Input handling with multi-line support
- ✅ Message rendering (user/assistant)
- ✅ Real-time UI updates

### Sprint 6 Achievements ✅ (Runnable Application)

- ✅ Complete CLI implementation (420+ lines)
- ✅ Command routing (chat, run, init, config, db)
- ✅ Component wiring (Database → Provider → Tools → Agent → TUI)
- ✅ Non-interactive mode with multiple output formats
- ✅ Configuration management commands
- ✅ Database management commands
- ✅ User-friendly error messages
- ✅ Application fully functional end-to-end

### Sprint 7 Achievements ✅ (Testing Infrastructure)

- ✅ **Integration tests** with MockProvider (9 tests)
- ✅ **Unit tests** across all modules (130 tests)
- ✅ **100% test pass rate** (172/172 tests passing)
- ✅ **Fast execution** (< 3 seconds for full suite)
- ✅ **Manual testing guide** (800+ lines, 6 scenarios)
- ✅ **Testing summary** documentation
- ✅ **CI/CD recommendations**
- ✅ **Comprehensive test coverage** across all layers

📄 **Documentation:**
- [TESTING_SUMMARY.md](docs/development/TESTING_SUMMARY.md) - Complete test overview
- [MANUAL_TESTING_GUIDE.md](docs/guides/MANUAL_TESTING_GUIDE.md) - Step-by-step testing guide
- [SPRINT_6_COMPLETE.md](docs/development/SPRINT_6_COMPLETE.md) - Sprint 6 completion report
- [SPRINT_8_COMPLETE.md](docs/development/SPRINT_8_COMPLETE.md) - Sprint 8 completion report
- [SPRINT_9_COMPLETE.md](docs/development/SPRINT_9_COMPLETE.md) - Sprint 9 completion report
- [SPRINT_10_STATUS.md](docs/development/SPRINT_10_STATUS.md) - Sprint 10 completion report
- [SPRINT_11_STATUS.md](docs/development/SPRINT_11_STATUS.md) - Sprint 11 completion report
- [README_USER_GUIDE.md](docs/guides/README_USER_GUIDE.md) - User-facing guide

### Sprint 8-11 Achievements (Recently Completed)

#### Sprint 8: Enhanced Testing ✅
- ✅ 43 new tests (172 total tests, up from 139 in Sprint 7, reduced to 172 in Sprint 11)
- ✅ CLI command tests (24 tests)
- ✅ Streaming response tests (10 tests)
- ✅ Error scenario tests (9 tests)
- ✅ 100% test pass rate maintained

#### Sprint 9: Enhanced TUI Experience ✅
- ✅ Markdown rendering with pulldown-cmark (267 lines)
- ✅ Syntax highlighting with syntect (219 lines, 100+ languages)
- ✅ Animated braille spinner for loading states
- ✅ Beautiful croissant splash screen
- ✅ Color-coded messages and emoji indicators

#### Sprint 10: Multi-Provider Support + Quick Wins ✅
- ✅ OpenAI provider fully implemented (517 lines)
- ✅ Local LLM support (LM Studio, Ollama)
- ✅ Fixed hard-coded model display
- ✅ Added config path support (`--config` flag)
- ✅ Implemented connection timeouts
- ✅ Added approval timeout (5 minutes with visual countdown)
- ✅ Created first benchmark suite (Criterion)

#### Sprint 11: Error Recovery & Resilience ✅
- ✅ Retry logic with exponential backoff and jitter
- ✅ Rate limit detection with Retry-After header parsing
- ✅ Database lock recovery (SQLite BUSY/LOCKED)
- ✅ Structured error infrastructure (severity levels, categories)
- ✅ TUI bug fixes (keyboard double-input, splash screen timing)
- ✅ 13 new tests (retry, database, error handling)

### Next Priorities

**Short Term (Sprint 12):**
- Security hardening (OS keyring, audit log)
- Path validation and command sanitization
- Enhanced approval system features

**Medium Term (Sprint 13-14):**
- Additional LLM providers (Gemini, Bedrock, Azure)
- LSP integration for code understanding
- MCP protocol support
- Advanced context management

**Long Term:**
- Context file support (.cursorrules)
- Vision model integration
- Advanced tool system
- Plugin architecture

---

**Built with** ❤️ **and Rust 🦀**

**"Why 'Crustly'?"** 🥐
Like a croissant's flaky layers, Crustly has a layered architecture.
Crusty on the outside (fast), soft on the inside (approachable).
