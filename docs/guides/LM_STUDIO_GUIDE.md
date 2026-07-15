# 🏠 Using Crustly with Local LLMs (LM Studio)

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

   | Model | Size (Q4) | RAM | Best For |
   |-------|-----------|-----|---------|
   | **Qwen2.5-Coder-7B-Instruct** ⭐ | ~5 GB | 16 GB | Code generation, tool use |
   | **Llama-3.1-8B-Instruct** ⭐ | ~5 GB | 16 GB | General-purpose coding |
   | **Gemma-3-12B-IT** | ~7 GB | 20 GB | Code review & explanation |
   | Qwen2.5-Coder-32B-Instruct | ~20 GB | 48 GB | Near-GPT-4 code quality |
   | Llama-3.3-70B-Instruct | ~40 GB | 64 GB | Complex reasoning |

   > 💡 See the **Recommended Local Models for Coding** section below for full hardware requirements, Ollama commands, and per-model config snippets.

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

| Category | Model | RAM | Notes |
|----------|-------|-----|-------|
| 🚀 **Lightweight** | Llama-3.2-3B-Instruct | 8 GB | Runs on any machine, CPU-only |
| ⚖️ **Balanced** ⭐ | Qwen2.5-Coder-7B-Instruct | 16 GB | Best default for coding |
| ⚖️ **Balanced** ⭐ | Llama-3.1-8B-Instruct | 16 GB | Best default for general work |
| 🧠 **Reasoning** | Gemma-3-12B-IT | 20 GB | Code review, explanation |
| 💪 **High Quality** | Qwen2.5-Coder-32B-Instruct | 48 GB | Near-GPT-4 code quality |
| 💪 **High Quality** | Llama-3.3-70B-Instruct | 64 GB | Complex multi-file tasks |

> See the **Recommended Local Models for Coding** section below for exact versions, Ollama commands, GPU requirements, and per-model `config.toml` snippets.

---

### 🤖 Recommended Local Models for Coding

These three model families are the best choices for software development tasks with Crustly. Requirements below assume **Q4_K_M** quantization (best speed/quality balance).

---

#### Qwen Code (Recommended for coding)

Alibaba's code-first model family. Best tool-call reliability and strongest code completion of any sub-10B model tested.

| Version | VRAM / RAM | Context | Notes |
|---------|-----------|---------|-------|
| **Qwen2.5-Coder-7B-Instruct** ⭐ | 6 GB VRAM / 16 GB RAM | 128K | Best balance — recommended default |
| Qwen2.5-Coder-14B-Instruct | 10 GB VRAM / 24 GB RAM | 128K | Higher quality, needs more memory |
| Qwen2.5-Coder-32B-Instruct | 22 GB VRAM / 48 GB RAM | 128K | Near-GPT-4 code quality |
| Qwen3-8B (non-Coder) | 6 GB VRAM / 16 GB RAM | 128K | Latest Qwen gen, strong reasoning |

**LM Studio search term:** `Qwen2.5-Coder-7B-Instruct`
**Ollama:** `ollama pull qwen2.5-coder:7b`

**Minimum system requirements:**
- CPU: Any modern x86-64 (AVX2 recommended)
- RAM: **16 GB** (7B model); 24 GB (14B); 48 GB (32B)
- GPU (optional): NVIDIA RTX 3060 12 GB+ for 7B GPU offload
- Storage: ~5 GB (7B Q4), ~9 GB (14B Q4), ~20 GB (32B Q4)

**Config for LM Studio:**
```toml
[llm.providers.openai]
api_key = "lm-studio"
base_url = "http://localhost:1234/v1"
default_model = "qwen2.5-coder-7b-instruct"
```

---

#### Gemma 4 (Google)

Google's latest generation. Excellent instruction following and reasoning; strong on code review and explanation tasks.

| Version | VRAM / RAM | Context | Notes |
|---------|-----------|---------|-------|
| **Gemma-3-4B-IT** ⭐ | 4 GB VRAM / 8 GB RAM | 128K | Runs on almost any machine |
| Gemma-3-12B-IT | 9 GB VRAM / 20 GB RAM | 128K | Best Gemma balance for coding |
| Gemma-3-27B-IT | 20 GB VRAM / 40 GB RAM | 128K | Highest quality in the family |
| **Gemma 4 26B A4B (MoE)** ⭐ | 12 GB VRAM / 32 GB RAM | 256K | 128 experts, 8 active (~3.8B active params/token); native tool calling, vision, and thinking mode |

> Gemma 4 26B A4B is a Mixture-of-Experts model: 25.2B total parameters but only ~3.8B active per token (8 active / 128 total experts + 1 shared), giving dense-26B-class quality at a fraction of the compute/VRAM cost. Apache 2.0 licensed. Also available in `12b`, `31b` (dense), `e2b`/`e4b` (edge-optimized), and `31b-cloud` (Ollama-hosted) variants — see [`ollama.com/library/gemma4:26b`](https://ollama.com/library/gemma4:26b). Full architecture, tool-calling, MoE routing, JSON output, and Crustly integration reference: [`docs/models/gemma-4-26b-a4b/`](../models/gemma-4-26b-a4b/).

**LM Studio search term:** `gemma-3-12b-it` (Gemma-3) / check HuggingFace for Gemma 4 26B A4B GGUF quants
**Ollama:** `ollama pull gemma3:12b` (Gemma-3) / `ollama pull gemma4:26b` (Gemma 4 26B A4B MoE)

**Minimum system requirements:**
- RAM: **8 GB** (4B model); 20 GB (12B); 40 GB (27B); 32 GB (26B A4B MoE)
- GPU (optional): Any NVIDIA/AMD with 4 GB+ VRAM for 4B model; RTX 3060 12 GB+ recommended for 26B A4B MoE
- Storage: ~3 GB (4B Q4), ~7 GB (12B Q4), ~17 GB (27B Q4), ~18 GB (26B A4B MoE Q4_K_M, Ollama's default quant)

**Config for LM Studio:**
```toml
[llm.providers.openai]
api_key = "lm-studio"
base_url = "http://localhost:1234/v1"
default_model = "gemma-3-12b-it"
```

**Config for Ollama (Gemma 4 26B A4B MoE):**
```toml
[providers.ollama]
enabled = true
default_model = "gemma4:26b"
num_ctx = 65536      # good latency/quality balance; model supports up to 262144
temperature = 1.0    # Google/Ollama's standardized recommendation
top_p = 0.95
top_k = 64
```

---

#### Llama (Meta)

Meta's flagship open-weight series. Excellent generalist performance; Llama 3.1/3.2/3.3 all support tool calling natively.

| Version | VRAM / RAM | Context | Notes |
|---------|-----------|---------|-------|
| **Llama-3.2-3B-Instruct** | 3 GB VRAM / 8 GB RAM | 128K | Ultra-light, fast on CPU |
| **Llama-3.1-8B-Instruct** ⭐ | 6 GB VRAM / 16 GB RAM | 128K | Best Llama balance — recommended |
| Llama-3.3-70B-Instruct | 45 GB VRAM / 64 GB RAM | 128K | Near-GPT-4o quality |
| Llama-3.1-405B-Instruct | 250+ GB RAM | 128K | Research / multi-GPU only |

**LM Studio search term:** `Llama-3.1-8B-Instruct`
**Ollama:** `ollama pull llama3.1:8b`

**Minimum system requirements:**
- RAM: **8 GB** (3B); **16 GB** (8B); **64 GB** (70B)
- GPU (optional): NVIDIA RTX 3060 8 GB+ for 8B full GPU offload
- Storage: ~2 GB (3B Q4), ~5 GB (8B Q4), ~40 GB (70B Q4)

**Config for LM Studio:**
```toml
[llm.providers.openai]
api_key = "lm-studio"
base_url = "http://localhost:1234/v1"
default_model = "meta-llama-3.1-8b-instruct"
```

---

#### Ornith 9B (agentic coding)

Ornith AI's Qwen-3.5-based model, post-trained specifically for agentic software development: repo navigation, planning, refactoring, bug fixing, and tool calling. MIT licensed, with a 256K-token context window — the largest of any model in this section.

| Version | VRAM / RAM | Context | Notes |
|---------|-----------|---------|-------|
| **Ornith 9B (Q4_K_M)** ⭐ | 8 GB VRAM / 16 GB RAM | 256K | Recommended quant — best speed/quality balance |
| Ornith 9B (Q5_K_M) | 10 GB VRAM / 20 GB RAM | 256K | Slightly higher quality |
| Ornith 9B (FP16) | 24 GB VRAM / 32 GB RAM | 256K | Full precision, rarely needed locally |

> Q8 is also available (very high quality, heavier than Q5_K_M) but Ornith's documentation doesn't publish an exact memory figure for it — measure with `ollama ps` after pulling if you need one.

**Ollama:** `ollama pull ornith:9b`

**Minimum system requirements:**
- GPU: 8 GB VRAM (RTX 3060 12 GB or better recommended)
- RAM: **16 GB** minimum; 32 GB recommended; 64 GB for the largest contexts
- Storage: ~5.6 GB (Q4_K_M), ~6.5 GB (Q5_K_M), ~18 GB (FP16)

**Recommended sampling** (per Ornith's documentation — use `providers.ollama` for native `/api/chat`, not the OpenAI-compatible shim, so `temperature`/`top_p`/`top_k`/`num_ctx` apply):
```toml
[providers.ollama]
enabled = true
default_model = "ornith:9b"
num_ctx = 65536      # up to 262144 supported; 65536 balances context vs. memory/latency
temperature = 0.10   # 0.05 for refactoring, 0.30 for brainstorming
top_p = 0.90
top_k = 20
```

---

#### Quick Comparison

| Model | RAM | Code | Reasoning | Speed | Best For |
|-------|-----|------|-----------|-------|---------|
| Qwen2.5-Coder-7B ⭐ | 16 GB | ★★★★★ | ★★★★☆ | Fast | Code generation, tool use |
| Gemma-3-12B | 20 GB | ★★★★☆ | ★★★★★ | Medium | Code review, explanation |
| Llama-3.1-8B ⭐ | 16 GB | ★★★★☆ | ★★★★☆ | Fast | General-purpose coding |
| Ornith 9B | 16 GB | ★★★★★ | ★★★★☆ | Fast | Agentic coding, 256K context |
| Qwen2.5-Coder-32B | 48 GB | ★★★★★ | ★★★★★ | Slow | Production-quality code |
| Llama-3.3-70B | 64 GB | ★★★★★ | ★★★★★ | Very slow | Complex multi-file tasks |

> 💡 **No GPU?** Qwen2.5-Coder-7B and Llama-3.1-8B both run acceptably on CPU-only at 3–6 tokens/sec with 16 GB RAM.

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

