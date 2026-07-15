# 🦙 Using Crustly with Ollama

[Ollama](https://ollama.com/) is a lightweight, always-on LLM runtime that exposes an OpenAI-compatible API. It is the **recommended local backend** for Crustly because it runs as a background service, connects instantly, and uses short easy-to-remember model names.

> 💡 **Why Ollama over LM Studio?** Ollama runs as a daemon — no GUI to accidentally close. Crustly's 10-second connect timeout and 90-second connection pool are a perfect fit for a persistent service. See [OLLAMA_VS_LM_STUDIO.md](../OLLAMA_VS_LM_STUDIO.md) for the full comparison.

---

### Prerequisites

- **Rust 1.75+** (already required for Crustly)
- **16 GB RAM** recommended (8 GB minimum for small models)
- **NVIDIA/AMD GPU** optional but strongly recommended for speed
- **Internet connection** for the initial model download only

---

### Step 1: Install Ollama

**Linux / macOS:**
```bash
curl -fsSL https://ollama.com/install.sh | sh
```

**Windows:**
Download and run the installer from [https://ollama.com/download](https://ollama.com/download).

After installation, Ollama automatically starts as a **background service** on `http://localhost:11434`. You do not need to start it manually after the first install.

**Verify it's running:**
```bash
curl http://localhost:11434/api/tags
# Expected: { "models": [...] }
```

---

### Step 2: Pull a Model

```bash
# ⭐ Recommended for code (best tool-call reliability, 16 GB RAM)
ollama pull qwen2.5-coder:7b

# Lightweight option (8 GB RAM, CPU-friendly)
ollama pull llama3.2:3b

# Balanced general-purpose (16 GB RAM)
ollama pull llama3.1:8b

# Higher quality code (24 GB RAM)
ollama pull qwen2.5-coder:14b

# Reasoning + code review (20 GB RAM)
ollama pull gemma3:12b
```

**List installed models at any time:**
```bash
ollama list
```

**Example output:**
```
NAME                       ID              SIZE    MODIFIED
qwen2.5-coder:7b           abc123def456    4.7 GB  2 minutes ago
llama3.1:8b                def456abc123    4.9 GB  5 days ago
```

The `NAME` column is exactly what you set as `default_model` in Crustly's config.

---

### Step 3: Verify Ollama's OpenAI Compatibility

Crustly uses the OpenAI-compatible endpoint that Ollama exposes at `/v1`:

```bash
curl http://localhost:11434/v1/models
```

**Expected output:**
```json
{
  "object": "list",
  "data": [
    {
      "id": "qwen2.5-coder:7b",
      "object": "model",
      "created": 1234567890,
      "owned_by": "library"
    }
  ]
}
```

---

### Step 4: Configure Crustly for Ollama

#### Option A: Configuration File (Recommended)

Initialize Crustly's config directory if you haven't already:
```bash
cargo run -- init
```

Then open the config file:
```bash
# Linux/macOS
nano ~/.config/crustly/config.toml

# Windows (PowerShell)
notepad $env:APPDATA\crustly\config.toml
```

Add or replace the `[providers.openai]` section:

```toml
[providers.openai]
enabled = true
base_url = "http://localhost:11434/v1/chat/completions"
default_model = "qwen2.5-coder:7b"   # Must match output of: ollama list
```

> ⚠️ **Important:** The `default_model` value must **exactly match** the model name shown by `ollama list` — including the `:tag` suffix.

#### Option B: Environment Variables (Quick Start)

```bash
# Linux/macOS
export OPENAI_API_KEY="ollama"           # Any non-empty value
export OPENAI_BASE_URL="http://localhost:11434/v1"

# Windows PowerShell
$env:OPENAI_API_KEY = "ollama"
$env:OPENAI_BASE_URL = "http://localhost:11434/v1"

# Windows Command Prompt
set OPENAI_API_KEY=ollama
set OPENAI_BASE_URL=http://localhost:11434/v1
```

---

### Step 5: Test the Connection

```bash
# Quick single-shot test
cargo run -- run "Hello! Which model are you?"
```

**Expected output:**
```
🏠 Using local LLM at: http://localhost:11434/v1/chat/completions
📦 Model: qwen2.5-coder:7b

Hello! I'm Qwen2.5-Coder, a 7B parameter language model...

📊 Tokens: 52
💰 Cost: $0.000000 (Local - FREE!)
```

**Launch the full TUI:**
```bash
cargo run
```

---

### Step 6: Using Crustly with Ollama

Once connected, Crustly works identically to cloud mode — all tools, plan mode, streaming, and approval dialogs are fully functional.

```bash
$ cargo run

You: Read src/main.rs and explain the entry point
Crustly: [reads file using read_file tool, streams response]

You: Add error handling to the database connection
Crustly: [writes file — approval dialog appears]
⚠️ PERMISSION REQUIRED: write_file → src/db/connection.rs
[A]pprove  [D]eny  [V]iew Details

You: Run cargo test
Crustly: [executes bash tool] ✅ 145 tests passed
```

---

### Recommended Models for Crustly + Ollama

| Model | Pull Command | RAM | Code | Tool Calling | Best For |
|-------|-------------|-----|------|-------------|---------|
| **Qwen2.5-Coder 7B** ⭐ | `ollama pull qwen2.5-coder:7b` | 16 GB | ★★★★★ | ✅ Excellent | Code gen, tool use, default pick |
| **Llama 3.1 8B** ⭐ | `ollama pull llama3.1:8b` | 16 GB | ★★★★☆ | ✅ Full | General-purpose coding |
| **Ornith 9B** ⭐ | `ollama pull ornith:9b` | 16 GB | ★★★★★ | ✅ Excellent | Agentic coding, 256K context |
| Llama 3.2 3B | `ollama pull llama3.2:3b` | 8 GB | ★★★☆☆ | ✅ Yes | Low-RAM / CPU-only machines |
| Gemma 3 12B | `ollama pull gemma3:12b` | 20 GB | ★★★★☆ | ✅ Yes | Code review & explanation |
| Qwen2.5-Coder 14B | `ollama pull qwen2.5-coder:14b` | 24 GB | ★★★★★ | ✅ Excellent | Higher quality code generation |
| Qwen2.5-Coder 32B | `ollama pull qwen2.5-coder:32b` | 48 GB | ★★★★★ | ✅ Excellent | Near-GPT-4 code quality |
| Llama 3.3 70B | `ollama pull llama3.3:70b` | 64 GB | ★★★★★ | ✅ Full | Complex multi-file tasks |
| **Gemma 4 26B A4B (MoE)** ⭐ | `ollama pull gemma4:26b` | 32 GB | ★★★★★ | ✅ Excellent | 256K context, repo-wide analysis, agentic workflows |

> ⚠️ **Tool calling is required** for Crustly's plan mode and file operations. All models in the table above support it. **Avoid DeepSeek Coder V2** — a tokenizer bug causes empty tool-call responses in both Ollama and LM Studio.

---

### Switching Between Models

You can change the active model any time by editing `config.toml`:

```toml
[providers.openai]
enabled = true
base_url = "http://localhost:11434/v1/chat/completions"
default_model = "llama3.1:8b"   # Changed from qwen2.5-coder:7b
```

Ollama will **automatically download the model layers** on first use if not already pulled.

To pre-download without changing config:
```bash
ollama pull llama3.1:8b
```

---

### Running Multiple Models

Ollama can serve multiple pulled models. Switch between them in `config.toml` without restarting Ollama — it hot-loads the requested model automatically.

```bash
# Pull several models once
ollama pull qwen2.5-coder:7b
ollama pull llama3.1:8b

# Then switch in config.toml between sessions
```

---

### Troubleshooting Ollama

#### "Connection refused" at `localhost:11434`

```bash
# Check if Ollama is running
ollama list

# If not, start it manually
ollama serve
```

On Linux, if Ollama was installed via the script, it registers as a systemd service:
```bash
sudo systemctl status ollama
sudo systemctl start ollama
```

---

#### "Model not found" error

```bash
# List available models
ollama list

# The NAME column is what goes in default_model
# Example output: qwen2.5-coder:7b
# Correct config:  default_model = "qwen2.5-coder:7b"
```

---

#### Slow responses (CPU-only)

```bash
# Check GPU usage
ollama ps  # Shows loaded models and device

# If running on CPU, try a smaller model
ollama pull llama3.2:3b   # 3B vs 7B = ~2× faster on CPU
```

GPU acceleration is automatic if CUDA (NVIDIA) or ROCm (AMD) drivers are installed. No extra configuration needed.

---

#### Increasing context window

By default Ollama models load with a 2048-token context. For long Crustly sessions, increase it via a `Modelfile`:

```bash
# Create a custom Modelfile
cat > Modelfile << 'EOF'
FROM qwen2.5-coder:7b
PARAMETER num_ctx 8192
EOF

# Build a local variant
ollama create qwen2.5-coder:7b-ctx8k -f Modelfile

# Use the new variant in Crustly config
# default_model = "qwen2.5-coder:7b-ctx8k"
```

---

#### Quick Troubleshooting Checklist

1. ✅ **Ollama running?** → `ollama list` (shows output) or `curl http://localhost:11434/api/tags`
2. ✅ **Model pulled?** → `ollama list` shows model name
3. ✅ **Model name in config matches `ollama list` exactly?** → `cargo run -- config`
4. ✅ **Using `:tag` suffix in `default_model`?** → e.g., `qwen2.5-coder:7b` not `qwen2.5-coder`
5. ✅ **Config file in correct location?** → `~/.config/crustly/config.toml` (Linux/macOS) or `%APPDATA%\crustly\config.toml` (Windows)

---

**🎉 You're now running Crustly with Ollama — fully private, zero cost, and always available!**

> 💡 **Pro Tip:** Ollama starts automatically with your OS after installation. Just open a terminal and run `cargo run` — Crustly will connect immediately without any manual setup.

---

