# DeepSeek Coder V2 Lite - Empty Response Analysis

## Date: 2025-11-12

## Problem
DeepSeek Coder V2 Lite generates empty responses when called from Crustly:
- `tool_calls: []`
- `content: ""`
- `completion_tokens: 0`

## Log Analysis

### Performance Stats
```
Prompt tokens: 6754
Prompt processing time: 55 seconds (47.7% progress)
Total time: 148 seconds for 2 tokens
Context length: 8192
Max tokens: 4096
```

### GPU Memory Usage (Working Correctly)
```
CUDA0 (RTX 3060): 12287 MiB total
- Model: 8975 MiB ✓ (fully loaded on GPU)
- Context: 2160 MiB
- Compute: 208 MiB
- Unaccounted: 943 MiB
```

**Conclusion**: GPU offload is working. The model IS on GPU.

### Root Cause: Double BOS Token

**Warning (appears 3x in logs)**:
```
Added a BOS token to the prompt as specified by the model but the prompt
also starts with a BOS token. So now the final prompt starts with 2 BOS tokens.
```

This tokenizer misconfiguration might cause DeepSeek to:
1. Misinterpret the prompt structure
2. Generate empty output
3. Fail to recognize tool calling format

### Contributing Factor: Massive Context

**Prompt size**: 6,754 tokens (from accumulated conversation history)
- Normal prompt: ~200-300 tokens
- Current prompt: 20x larger than normal
- Processing: 148 seconds vs expected 5-10 seconds

Large context can cause:
1. Extreme slowness
2. Model confusion with too much history
3. Context window saturation

## Solutions

### Immediate: Clear Conversation History
```bash
# In Crustly, press: Ctrl+N
```

This will:
- Reduce prompt from 6,754 to ~300 tokens (20x reduction)
- Speed up from 148s to 5-10s (15-30x faster)
- Might resolve empty response issue

### Short-term: Configure LM Studio BOS Handling

**Option 1 - Try Different Model**:
- DeepSeek Coder V2 Lite might have tokenizer issues
- Try: **Qwen 2.5 Coder 7B** (better tool calling support)
- Or: **Hermes 3 Llama 3.1 8B** (confirmed working)

**Option 2 - LM Studio Settings**:
1. In LM Studio, go to model settings
2. Look for "Add BOS Token" option
3. Try disabling it (if available)

### Long-term: Reduce Context Window

Configure in LM Studio:
```
Context Length: 2048 (down from 8192)
Max Tokens: 1024 (down from 4096)
```

Benefits:
- Less GPU memory for context (2160 MiB → ~540 MiB)
- Faster processing
- Forces model to stay focused

## Test Plan

1. **Start fresh Crustly session** (Ctrl+N)
2. Try simple plan: "create a plan to print hello world"
3. Check logs for:
   - Prompt token count (should be ~300)
   - Processing time (should be <10s)
   - Tool calls (should not be empty)

4. If still empty response:
   - Switch to Hermes 3 Llama 3.1 8B
   - Or try Qwen 2.5 Coder 7B

## Expected Results

### After Fresh Session
```
Prompt tokens: ~300 (was 6754)
Processing time: 5-10s (was 148s)
Tool calls: [plan(...)] (was [])
Content: "Plan created..." (was "")
```

## Compatibility Matrix

| Model | Tool Calling | Performance | Status |
|-------|-------------|-------------|--------|
| DeepSeek V2 Lite | Empty response | Slow (148s) | ❌ Not working |
| Hermes 3 Llama 3.1 8B | Working | Medium | ✅ Confirmed |
| Qwen 2.5 Coder 7B | Text only | Fast | ⚠️ No tool calls |
| Qwen 2.5 Coder 14B | Text only | Medium | ⚠️ No tool calls |

## Next Steps

1. ✅ Press Ctrl+N to start fresh session
2. ⏳ Test with small prompt
3. ⏳ Switch model if still broken
4. ⏳ Reduce context window in LM Studio
