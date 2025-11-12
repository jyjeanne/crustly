# Local LLM Compatibility with Crustly

## Date: 2025-11-12

## Summary

Not all local LLMs support OpenAI-style tool calling required for Crustly's Plan Mode and other features.

## Hardware Context
- CPU: Intel i7-7700K @ 4.20GHz
- GPU: RTX 3060 12GB VRAM
- LLM Server: LM Studio

## Model Compatibility Matrix

| Model | Size | Tool Calling | Performance | Status | Notes |
|-------|------|--------------|-------------|--------|-------|
| **Qwen 2.5 Coder 14B** | ~8GB | ✅ Yes | Fast | **RECOMMENDED** | Excellent tool calling, 27 tok/s, cache reuse |
| **Hermes 3 Llama 3.1 8B** | ~5GB | ✅ Yes | Fast | **RECOMMENDED** | Best compatibility, no issues |
| Llama 3.1 8B Instruct | ~5GB | ✅ Yes | Fast | ✅ Working | Official Meta model |
| Llama 3.2 3B Instruct | ~2GB | ✅ Yes | Very Fast | ✅ Working | Smaller, faster alternative |
| DeepSeek Coder V2 Lite 16B | ~9GB | ❌ No | Slow | ❌ BROKEN | Double BOS token bug → empty responses |
| DeepSeek Coder V2 16B | ~10GB | ❌ No | Slow | ❌ BROKEN | Same tokenizer issue |
| Qwen 2.5 Coder 7B | ~4GB | ⚠️ Unknown | Fast | ⚠️ UNTESTED | May work like 14B with LM Studio |
| Hermes 2 Pro Mistral 7B | ~4GB | ⚠️ Partial | Fast | ⚠️ UNTESTED | Older version, may work |

## Tested Models - Detailed Results

### ✅ Hermes 3 Llama 3.1 8B (WORKING)
**Status**: Fully compatible

**Test Results**:
- Tool calling: ✅ Generates proper `tool_calls` array
- Plan Mode: ✅ Can create, finalize, approve plans
- Performance: Good (10-15s prompt processing)
- Issues: None

**Logs Example**:
```
Model generated tool calls: [plan(operation="create"...)]
```

**Recommendation**: **Use this model for Crustly**

---

### ❌ DeepSeek Coder V2 Lite 16B (BROKEN)
**Status**: Incompatible - Double BOS token bug

**Test Results**:
- Tool calling: ❌ Generates empty responses
- Plan Mode: ❌ Cannot create plans
- Performance: Very slow (150+ seconds)
- Issues: Double BOS token warning → empty output

**Logs Example**:
```
[ERROR] Added a BOS token to the prompt as specified by the model but
the prompt also starts with a BOS token. (x3)

Generated prediction: {
  "content": "",
  "tool_calls": [],
  "completion_tokens": 0
}
```

**Root Cause**:
- LM Studio adds BOS token to prompts
- DeepSeek model config also adds BOS token
- Result: Double BOS → tokenizer confusion → empty generation

**Workaround**: None. Model is fundamentally incompatible.

**Recommendation**: **Do NOT use. Switch to Hermes 3.**

---

### ✅ Qwen 2.5 Coder 14B (WORKING)
**Status**: Fully compatible - Excellent performance

**Test Results**:
- Tool calling: ✅ Generates proper `tool_calls` array via LM Studio
- Plan Mode: ✅ Can create, add tasks, finalize plans
- Performance: Excellent (4s total, 27 tokens/s generation)
- Cache reuse: 98.1% (very efficient for follow-up requests)
- Issues: None

**Logs Example**:
```
[INFO] [qwen/qwen2.5-coder-14b] Start to generate a tool call...
[INFO] Tool name generated: plan
[INFO] Model generated tool calls: [plan(operation="add_task"...)]

Generated prediction: {
  "content": "Next, let's add the task to write the \"Hello, World!\" code in main.rs.\n\n",
  "tool_calls": [
    {
      "type": "function",
      "id": "928943100",
      "function": {
        "name": "plan",
        "arguments": "{\"operation\":\"add_task\",\"title\":\"Write Hello, World! Code\"...}"
      }
    }
  ],
  "finish_reason": "tool_calls"  ← Proper finish reason!
}
```

**How It Works**:
- LM Studio uses special sampling switches for tool calling
- Triggered by `[TOOL_REQUEST]` marker in prompt
- Model generates proper OpenAI-format tool calls
- Switching sampling modes: `defaultToolsSamplingSwitch`

**Performance Stats**:
```
Total time: 4.0 seconds
Prompt eval: 191ms / 114 tokens (594 tokens/sec)
Token generation: 27 tokens/second
GPU memory: 8148 MiB (fully on GPU)
Cache reuse: 98.1% (very efficient)
```

**Use Case**:
- ✅ Plan Mode (fully working)
- ✅ File operations (read, write, edit)
- ✅ Code search and execution
- ✅ Task management
- ✅ General coding assistance

**Recommendation**: **Excellent choice for Crustly!** Best coding-focused model with tool calling support.

---

### ⚠️ Qwen 2.5 Coder 7B (UNTESTED)
**Status**: Unknown - Likely works like 14B version

**Expected**: Should work with LM Studio's tool calling support like the 14B version.

**Recommendation**: Worth testing if you need a smaller/faster model.

---

## Why Tool Calling Matters

Crustly uses OpenAI-style tool calling for:

1. **Plan Mode**: Create multi-step execution plans
   - Requires: `plan` tool with operations (create, add_task, finalize, etc.)

2. **File Operations**: Read, write, edit files
   - Requires: `read_file`, `write_file`, `edit_file` tools

3. **Code Search**: Find files and patterns
   - Requires: `glob`, `grep` tools

4. **Execution**: Run commands and code
   - Requires: `bash`, `execute_code` tools

5. **Task Management**: Track multi-step workflows
   - Requires: `task_manager` tool

6. **Web Access**: Search and fetch web content
   - Requires: `web_search`, `http_request` tools

**Without tool calling**, the model can only:
- Have conversations
- Generate code (but not save it)
- Explain concepts
- Answer questions

## OpenAI Tool Calling Format

**What Crustly expects**:
```json
{
  "role": "assistant",
  "content": "I'll create a plan for you.",
  "tool_calls": [
    {
      "id": "call_123",
      "type": "function",
      "function": {
        "name": "plan",
        "arguments": "{\"operation\":\"create\",\"title\":\"Print Hello World\"}"
      }
    }
  ]
}
```

**What broken models generate**:
```json
{
  "role": "assistant",
  "content": "I'll call the plan tool to create a plan...",
  "tool_calls": []  ← Empty!
}
```

## LM Studio Configuration

### Important: LM Studio Tool Calling Support

**LM Studio has built-in tool calling support** that enables models like Qwen 2.5 Coder to generate proper OpenAI-format tool calls.

How it works:
1. LM Studio inserts special markers like `[TOOL_REQUEST]` into prompts
2. Model recognizes these markers and enters "tool mode"
3. Special sampling switches activate: `defaultToolsSamplingSwitch`
4. Model generates structured tool calls instead of just text
5. LM Studio formats the output as proper OpenAI tool_calls array

**This is why Qwen 2.5 Coder 14B works perfectly with Crustly via LM Studio!**

### Optimal Settings for Tool Calling

**For Qwen 2.5 Coder 14B**:
```
Context Length: 8192 (excellent cache reuse)
Max Tokens: 2048 (tool responses are usually short)
Temperature: 0.8 (slightly higher for creativity)
Top P: 0.95
GPU Layers: 40 (all layers, or max available)
```

**For Hermes 3 Llama 3.1 8B**:
```
Context Length: 4096 (balance speed and context)
Max Tokens: 2048
Temperature: 0.7
Top P: 0.95
GPU Layers: 40 (all layers, or max available)
```

### Performance Tuning

**For faster responses (trade quality for speed)**:
```
Context Length: 2048 ↓
Max Tokens: 1024 ↓
Temperature: 0.6 ↓
Top P: 0.9 ↓
GPU Layers: 40 (keep maxed)
```

**For better quality (slower)**:
```
Context Length: 8192 or 16384
Max Tokens: 4096
Temperature: 0.8
Top P: 0.95
GPU Layers: 40
```

## How to Switch Models in LM Studio

1. **Stop current model**:
   - In LM Studio Server tab
   - Click "Unload Model"

2. **Download Hermes 3**:
   - Go to "Search" tab
   - Search: "Hermes 3 Llama 3.1 8B"
   - Download quantization: **Q4_K_M** (best balance)
   - Alternative: Q5_K_M (better quality, slower)

3. **Load new model**:
   - Click on "Hermes 3 Llama 3.1 8B Q4_K_M" in sidebar
   - Set GPU layers to 40
   - Click "Load Model"

4. **Test in Crustly**:
   - Press Ctrl+N (new session)
   - Type: "create a plan to print hello world"
   - Press Ctrl+Enter
   - Should see tool calls in logs

## Troubleshooting

### Model loads but generates slowly
- Check GPU layers offloaded (should be 40/40 or similar)
- Reduce context length from 8192 to 2048
- Reduce max tokens from 4096 to 1024

### Model generates text but no tool calls
- Model doesn't support tool calling
- Switch to Hermes 3 or Llama 3.1

### "Maximum tool iterations exceeded"
- Model is calling tools in a loop
- Usually fixed by Crustly's loop detection
- If persists: restart Crustly with Ctrl+N

### Empty responses
- Double BOS token issue (DeepSeek)
- Switch to different model

## Benchmark Results

**Test**: "create a plan to print hello world"

| Model | Prompt Tokens | Processing Time | Token Generation | Tool Calls | Status |
|-------|---------------|-----------------|------------------|------------|--------|
| Qwen 2.5 Coder 14B | 6,136 | ~4s | 27 tok/s | ✅ Yes | Working |
| Hermes 3 Llama 3.1 8B | 6,752 | ~10-15s | 15-20 tok/s | ✅ Yes | Working |
| DeepSeek V2 Lite 16B | 6,752 | 150s | 0 tok/s | ❌ Empty | Broken |

*Note: Prompt tokens are consistent (~6,000-7,000) because they include system prompt + 14 tool definitions*

## Recommended Setup for i7-7700K + RTX 3060

### Option 1: Qwen 2.5 Coder 14B (Best for Coding)

**Why**: Fastest tool calling, best coding knowledge, excellent cache reuse

**Settings**:
```
Model: Qwen 2.5 Coder 14B Q4_K_M or Q5_K_M
Context: 8192 (supports large context efficiently)
Max Tokens: 2048
Temperature: 0.8
GPU Layers: 40 (all layers on GPU)
```

**Expected Performance**:
- Prompt processing: 4-6 seconds
- Token generation: 25-30 tokens/second
- Plan creation: ~10-15 seconds total
- Memory usage: ~8-9GB VRAM
- Cache reuse: 98%+ on follow-ups

### Option 2: Hermes 3 Llama 3.1 8B (More Versatile)

**Why**: Excellent general-purpose model, slightly smaller

**Settings**:
```
Model: Hermes 3 Llama 3.1 8B Q4_K_M
Context: 4096 (balance of history and speed)
Max Tokens: 2048
Temperature: 0.7
GPU Layers: 40 (all layers on GPU)
```

**Expected Performance**:
- Prompt processing: 10-15 seconds
- Token generation: 15-20 tokens/second
- Plan creation: ~20-30 seconds total
- Memory usage: ~6-7GB VRAM

## References

- [LM Studio Model Catalog](https://lmstudio.ai/models)
- [Hermes 3 Model Card](https://huggingface.co/NousResearch/Hermes-3-Llama-3.1-8B)
- [OpenAI Tool Calling Spec](https://platform.openai.com/docs/guides/function-calling)

## See Also

- `DEEPSEEK_EMPTY_RESPONSE_ANALYSIS.md` - Detailed DeepSeek bug analysis
- `PLAN_MODE_EXECUTION_REVIEW.md` - Plan Mode implementation review
- `docs/PLAN_MODE_USER_GUIDE.md` - User guide for Plan Mode
