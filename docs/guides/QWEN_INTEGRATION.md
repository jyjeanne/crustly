# Qwen Integration Guide

Crustly now supports **native Qwen LLM integration** with advanced features like Hermes-style tool calling and Qwen3 thinking mode.

## Overview

The QwenProvider offers:
- **Hermes-style tool calling** for optimal function calling performance
- **Qwen3 thinking mode** with visible reasoning process
- Support for **local deployment** (vLLM, LM Studio, Ollama)
- Support for **DashScope cloud API** (International & China regions)
- **OpenAI-compatible API** format

## Quick Start

### Local Qwen (vLLM / LM Studio)

```bash
# Set environment variable
export QWEN_BASE_URL="http://localhost:8000/v1/chat/completions"

# Optional: Enable thinking mode
export QWEN_ENABLE_THINKING=true

# Start Crustly
cargo run
```

### DashScope Cloud API

```bash
# Set your DashScope API key
export DASHSCOPE_API_KEY="your-api-key-here"

# Start Crustly
cargo run
```

## Configuration

### Environment Variables

| Variable | Description | Example |
|----------|-------------|---------|
| `QWEN_BASE_URL` | Local Qwen endpoint | `http://localhost:8000/v1/chat/completions` |
| `DASHSCOPE_API_KEY` | DashScope cloud API key | `sk-...` |
| `QWEN_ENABLE_THINKING` | Enable Qwen3 thinking mode | `true` |

### Configuration File (crustly.toml)

```toml
[providers.qwen]
enabled = true

# For local deployment (vLLM, LM Studio)
base_url = "http://localhost:8000/v1/chat/completions"
default_model = "qwen3-8b"

# Tool call parser: "hermes" (recommended) or "openai"
tool_parser = "hermes"

# Qwen3 thinking mode
enable_thinking = true
thinking_budget = 5000  # Optional: limit thinking tokens

# For DashScope cloud (alternative to base_url)
# api_key = "your-dashscope-api-key"
# region = "intl"  # or "cn" for China
```

## Tool Call Parsing Modes

### Hermes Style (Recommended for Qwen3 via vLLM)

Hermes-style parsing uses XML tags for structured tool calls:

```xml
<tool_call>
{"name": "read_file", "arguments": {"path": "/home/user/file.txt"}}
</tool_call>
```

**Benefits:**
- Optimized for Qwen3 models via vLLM
- Better structured reasoning
- Higher tool call accuracy

### Native Qwen Style (Official Qwen-Agent Format)

Uses the official Qwen-Agent Unicode markers:

```
✿FUNCTION✿: read_file
✿ARGS✿: {"path": "/home/user/file.txt"}
```

**Benefits:**
- Official format from Qwen-Agent repository
- Direct compatibility with Qwen's internal function calling
- Supports parallel function calls natively
- Stop word handling for better parsing

**Configuration:**
```toml
tool_parser = "native"  # or "qwen"
```

### OpenAI Style

Standard OpenAI-compatible format with `tool_calls` array in response.

**Best for:**
- DashScope cloud API
- Compatibility with existing tools
- LM Studio with auto-parsing enabled

## Qwen3 Thinking Mode

When enabled, Qwen3 models use `<think>` tags to show their reasoning process:

```
<think>
The user wants me to analyze this code. Let me examine the function signature first...
I notice this is using async/await patterns. The error handling could be improved...
</think>

Here's my analysis of the code...
```

**Benefits:**
- Visible reasoning process
- Better debugging of model behavior
- More transparent decision making

**Configuration:**

```toml
[providers.qwen]
enable_thinking = true
thinking_budget = 5000  # Optional: limit thinking tokens
```

## Supported Models

### Qwen3 Series
- `qwen3-235b-a22b` - Flagship MoE model (131K context)
- `qwen3-32b` - High performance (131K context)
- `qwen3-14b` - Balanced (131K context)
- `qwen3-8b` - Fast & efficient (131K context)

### Qwen2.5 Coder Series
- `qwen2.5-coder-32b-instruct` - Large coder (131K context)
- `qwen2.5-coder-14b-instruct` - Medium coder (131K context)
- `qwen2.5-coder-7b-instruct` - Small coder (131K context)

### Qwen2.5 Base Series
- `qwen2.5-72b-instruct`
- `qwen2.5-32b-instruct`
- `qwen2.5-14b-instruct`
- `qwen2.5-7b-instruct`

### DashScope Cloud Models
- `qwen-max` - Premium tier (32K context)
- `qwen-plus` - Standard tier (131K context)
- `qwen-turbo` - Economy tier (131K context)

## Local Deployment with vLLM

### Install vLLM

```bash
pip install vllm
```

### Start vLLM Server with Tool Calling

```bash
vllm serve Qwen/Qwen3-8B \
    --enable-auto-tool-choice \
    --tool-call-parser hermes \
    --port 8000
```

### Configure Crustly

```toml
[providers.qwen]
enabled = true
base_url = "http://localhost:8000/v1/chat/completions"
default_model = "Qwen/Qwen3-8B"
tool_parser = "hermes"
enable_thinking = true
```

## Local Deployment with LM Studio

1. Download and install [LM Studio](https://lmstudio.ai/)
2. Download a Qwen model (e.g., `Qwen2.5-Coder-14B-Instruct`)
3. Start the local server (default: `http://localhost:1234`)
4. Configure Crustly:

```toml
[providers.qwen]
enabled = true
base_url = "http://localhost:1234/v1/chat/completions"
default_model = "qwen2.5-coder-14b-instruct"
tool_parser = "hermes"  # or "openai" if LM Studio handles parsing
```

## DashScope Cloud Configuration

### Regional Endpoints

- **International (Singapore)**: Default, use `region = "intl"`
- **China (Beijing)**: Use `region = "cn"`

```toml
[providers.qwen]
enabled = true
api_key = "your-dashscope-api-key"
region = "intl"  # or "cn"
default_model = "qwen-plus"
```

## Pricing (DashScope Cloud)

| Model | Input ($/M tokens) | Output ($/M tokens) |
|-------|-------------------|---------------------|
| qwen-max | $2.40 | $9.60 |
| qwen-plus | $0.80 | $2.00 |
| qwen-turbo | $0.30 | $0.60 |

**Note:** Local deployments have no API costs.

## Best Practices

### For Code Generation

```toml
[providers.qwen]
default_model = "qwen2.5-coder-14b-instruct"
tool_parser = "hermes"
enable_thinking = false  # Faster responses
```

### For Complex Reasoning

```toml
[providers.qwen]
default_model = "qwen3-32b"
tool_parser = "hermes"
enable_thinking = true
thinking_budget = 10000  # Allow more thinking
```

### For Cost-Effective Usage

```toml
[providers.qwen]
# Use local model for free
base_url = "http://localhost:8000/v1/chat/completions"
default_model = "qwen2.5-coder-7b-instruct"
```

## Troubleshooting

### Tool Calls Not Being Parsed

1. **Check tool parser setting**: Use `tool_parser = "hermes"` for Qwen3
2. **Verify vLLM settings**: Ensure `--tool-call-parser hermes` is set
3. **Enable debug mode**: `crustly -d` to see detailed logs

### Thinking Mode Not Working

1. Ensure `enable_thinking = true` in config
2. Use Qwen3 models (not Qwen2.5)
3. Check logs for thinking extraction

### Connection Issues

1. Verify base URL is correct
2. Check if server is running: `curl http://localhost:8000/v1/models`
3. Ensure no firewall blocking

## Example Usage

```bash
# Start Crustly with Qwen
$ cargo run

🏠 Using local Qwen at: http://localhost:8000/v1/chat/completions
🧠 Thinking mode: enabled
📦 Model: qwen3-8b

You: Read src/main.rs and explain the architecture

Crustly: 💭 *Thinking:* Let me examine the main entry point of this Rust application...

<tool_call>
{"name": "read_file", "arguments": {"path": "src/main.rs"}}
</tool_call>

[File contents displayed]

Based on my analysis, this application uses...
```

## API Compatibility

The QwenProvider uses OpenAI-compatible API format, making it work with:
- vLLM with `--enable-auto-tool-choice`
- LM Studio with OpenAI API mode
- Ollama with OpenAI API compatibility
- DashScope's OpenAI-compatible endpoint

## Additional Resources

- [Qwen Documentation](https://qwen.readthedocs.io/)
- [vLLM Tool Calling](https://qwen.readthedocs.io/en/latest/deployment/vllm.html)
- [DashScope API](https://www.alibabacloud.com/help/en/model-studio/)
- [Hermes Tool Calling Format](https://qwen.readthedocs.io/en/latest/framework/function_call.html)
