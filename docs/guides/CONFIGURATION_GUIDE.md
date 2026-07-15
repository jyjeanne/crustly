# 📝 Local Configuration with crustly.toml

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

