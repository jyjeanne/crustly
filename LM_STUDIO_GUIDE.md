# LM Studio Quick Start Guide

## Overview

Crustly now supports **local LLMs via LM Studio** out of the box! No additional setup required - just set the `OPENAI_BASE_URL` environment variable and you're ready to go.

## Prerequisites

1. **LM Studio** installed and running
2. **A model loaded** in LM Studio (e.g., Mistral 7B, Llama 3.1, etc.)
3. **Local server running** in LM Studio

## Step-by-Step Setup

### 1. Install LM Studio

Download from: https://lmstudio.ai/

### 2. Download a Model

In LM Studio:
- Click the 🔍 **Search** tab
- Search for a model (recommended: "Mistral 7B Instruct" or "Llama 3.1 8B")
- Download the **Q4_K_M** or **Q5_K_M** quantization
- Wait for download to complete

### 3. Load the Model

- Go to the 💬 **Chat** tab
- Select your downloaded model from the dropdown
- Wait for "Model loaded" confirmation

### 4. Start the Local Server

- Go to the **Local Server** tab (⚙️ icon)
- Click **"Start Server"** (green play button)
- Verify you see: "Server running on http://localhost:1234"

### 5. Configure Crustly

Set the environment variable to point to your local LM Studio server:

```bash
# Linux/Mac
export OPENAI_BASE_URL="http://localhost:1234/v1"

# Windows PowerShell
$env:OPENAI_BASE_URL="http://localhost:1234/v1"

# Windows Command Prompt
set OPENAI_BASE_URL=http://localhost:1234/v1
```

**Note:** The `/v1` at the end is important!

### 6. Run Crustly

```bash
cargo run
```

You should see:
```
🦀 Starting Crustly AI Assistant...

🏠 Using local LLM at: http://localhost:1234/v1
```

That's it! You're now running Crustly with your local LLM.

## Quick Test

Test your local LLM connection:

```bash
# Non-interactive test
cargo run -- run "Hello! Can you introduce yourself in one sentence?"
```

## Usage Examples

### Example 1: Interactive Chat

```bash
export OPENAI_BASE_URL="http://localhost:1234/v1"
cargo run

# In Crustly:
You: "What is Rust?"
Local LLM: [responds with explanation...]
```

### Example 2: Code Generation

```bash
export OPENAI_BASE_URL="http://localhost:1234/v1"
cargo run -- run "Write a Rust function to calculate fibonacci"
```

### Example 3: Auto-Approve Tools

```bash
export OPENAI_BASE_URL="http://localhost:1234/v1"
cargo run -- run --auto-approve "Create a hello.txt file with 'Hello World'"
```

## Switching Between Providers

### Use Local LLM (LM Studio)
```bash
export OPENAI_BASE_URL="http://localhost:1234/v1"
cargo run
```

### Use OpenAI GPT
```bash
export OPENAI_API_KEY="sk-your-openai-key"
unset OPENAI_BASE_URL  # Remove local URL
cargo run
```

### Use Anthropic Claude
```bash
export ANTHROPIC_API_KEY="sk-ant-your-key"
unset OPENAI_BASE_URL
unset OPENAI_API_KEY
cargo run
```

## Provider Priority

Crustly selects providers in this order:

1. **Local LLM** (if `OPENAI_BASE_URL` is set)
2. **OpenAI** (if `OPENAI_API_KEY` is set)
3. **Anthropic** (if `ANTHROPIC_API_KEY` is set)

## Configuration File

You can also set the base URL in your config file:

```toml
# ~/.config/crustly/config.toml (Linux/Mac)
# or C:\Users\YourName\AppData\Roaming\crustly\config.toml (Windows)

[providers.openai]
enabled = true
base_url = "http://localhost:1234/v1"
```

Then just run:
```bash
cargo run
```

## Compatible Local LLM Servers

Crustly's OpenAI provider works with any OpenAI-compatible API:

| Server | Default URL | Notes |
|--------|-------------|-------|
| **LM Studio** | `http://localhost:1234/v1` | ✅ Tested |
| **Ollama** | `http://localhost:11434/v1` | ✅ Compatible |
| **LocalAI** | `http://localhost:8080/v1` | ✅ Compatible |
| **Text Generation WebUI** | `http://localhost:5000/v1` | ✅ Compatible (with OpenAI extension) |

## Recommended Models for Coding

| Model | Size | RAM Needed | Best For |
|-------|------|------------|----------|
| **Mistral 7B Instruct** | 4-8 GB | 16 GB | General coding, fast |
| **Qwen 2.5 7B Instruct** | 4-8 GB | 16 GB | Excellent for code |
| **DeepSeek Coder 6.7B** | 4-7 GB | 16 GB | Code-focused |
| **Llama 3.1 8B Instruct** | 4-8 GB | 16 GB | Latest, very capable |

## Troubleshooting

### Problem: "Connection refused"

**Check:**
1. LM Studio server is running (see "Server running" message)
2. URL is correct: `http://localhost:1234/v1` (note the `/v1`)
3. Port is 1234 (check LM Studio server tab)

**Test connection:**
```bash
curl http://localhost:1234/v1/models
```

### Problem: "No response" or slow responses

**Solutions:**
1. **Enable GPU** in LM Studio (Settings → GPU acceleration)
2. **Use smaller model** (7B instead of 13B)
3. **Lower quantization** (Q4 instead of Q8)
4. **Close other apps** to free RAM

### Problem: "Model not loaded"

**Solution:**
1. Go to LM Studio **Chat** tab
2. Select a model from dropdown
3. Wait for "Model loaded" message
4. Then start server in **Local Server** tab

### Problem: "API key not set"

If you see "Anthropic API key not set", it means `OPENAI_BASE_URL` wasn't detected.

**Fix:**
```bash
# Verify environment variable is set
echo $OPENAI_BASE_URL  # Linux/Mac
echo %OPENAI_BASE_URL%  # Windows CMD
$env:OPENAI_BASE_URL   # Windows PowerShell
```

## Performance Tips

1. **First response is slow** - Model loads into memory, subsequent responses are faster
2. **Use GPU** - Enable in LM Studio settings for 5-10x speedup
3. **Reduce token limit** - Shorter responses = faster generation
4. **Keep LM Studio open** - Leave it running in background for instant responses

## Benefits of Local LLMs

✅ **100% Private** - Code never leaves your machine
✅ **Zero Cost** - Free after model download
✅ **Offline** - Works without internet
✅ **No Rate Limits** - Use as much as you want
✅ **Fast** - No network latency (with good hardware)

## Comparison: Cloud vs Local

| Aspect | Cloud (Claude/GPT) | Local (LM Studio) |
|--------|-------------------|-------------------|
| **Privacy** | Data sent to API | 100% local |
| **Cost** | $3-15 per 1M tokens | Free |
| **Speed** | 1-2 seconds | 2-10 seconds* |
| **Quality** | Excellent | Good* |
| **Setup** | API key only | Download model + server |
| **Hardware** | None | 16GB+ RAM |

*Depends on model size and hardware

## Next Steps

1. ✅ LM Studio running → Set `OPENAI_BASE_URL` → Run Crustly
2. Try different models to find your favorite
3. Adjust LM Studio settings for performance
4. Use local LLM for sensitive/private code

## Support

- **LM Studio docs:** https://lmstudio.ai/docs
- **Crustly issues:** https://github.com/your-org/crustly/issues
- **Model recommendations:** Check HuggingFace leaderboard for coding

---

**Enjoy private, cost-free AI coding with Crustly + LM Studio!** 🦀🏠
