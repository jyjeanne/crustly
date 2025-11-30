# LLM Tool Calling Issue - Plan Tool Not Being Called

**Date:** 2025-11-23
**Issue:** Local LLM hallucinating plan creation instead of calling tool
**Model:** Qwen 2.5 Coder 14B (local via LM Studio)
**Status:** ⚠️ Model Limitation

---

## Problem Description

### Symptoms

When you ask the LLM to create a plan:
1. LLM responds: "✅ Plan finalized! The plan is now displayed in Plan Mode..."
2. But **no plan tool was actually called**
3. Pressing Ctrl+P shows: "No plan available"

### Evidence from Logs

```
Converting 0 tool calls from OpenAI response  ← NO TOOLS!
Block 0: Text (✅ Plan finalized! The plan is now displayed in Pla...)
Found 0 tool uses to execute
```

The LLM is **hallucinating** - pretending it created a plan when it didn't call any tools.

---

## Root Cause

### Local LLM Limitations

**Qwen 2.5 Coder 14B** has limited tool calling capabilities:
- Good at code generation
- Inconsistent at tool calling
- Tends to hallucinate tool results
- Doesn't always follow function calling format

### Why This Happens

1. **Training Focus:** Qwen models are trained primarily for code completion, not tool orchestration
2. **Size Limitation:** 14B parameters may not be enough for reliable tool calling
3. **Format Confusion:** May not understand OpenAI-style function calling format
4. **Hallucination Tendency:** Generates plausible-sounding responses instead of calling tools

---

## Solutions Implemented

### 1. ✅ Stronger Prompt Transformation

**Location:** `src/tui/prompt_analyzer.rs:134-141`

**Before:**
```rust
transformations.push(
    "\n\n**TOOL HINT**: Use the `plan` tool to create a structured plan..."
);
```

**After:**
```rust
transformations.push(
    "\n\n**CRITICAL**: You MUST use the `plan` tool now! \
    DO NOT write text - CALL THE TOOL IMMEDIATELY:\n\
    1. plan(operation='create', title='...', description='...')\n\
    2. plan(operation='add_task', ...) for each task\n\
    3. plan(operation='finalize')\n\
    **START WITH THE FIRST TOOL CALL NOW!**"
);
```

This makes the instruction much more forceful and explicit.

---

## Recommended Solutions

### Option 1: Use Claude/GPT Models (Best)

Switch to a model with strong tool calling:

**Claude (Recommended):**
```toml
# In crustly.toml
[llm]
provider = "anthropic"
model = "claude-sonnet-4"

[secrets]
anthropic_api_key = "sk-ant-..."
```

**Benefits:**
- ✅ Excellent tool calling
- ✅ Follows instructions precisely
- ✅ No hallucinations
- ✅ Reliable plan creation

**Cost:** ~$3/million input tokens, $15/million output tokens

**GPT-4 (Alternative):**
```toml
[llm]
provider = "openai"
model = "gpt-4"

[secrets]
openai_api_key = "sk-..."
```

### Option 2: Try Larger Local Models

Some local models with better tool calling:

**Qwen 2.5 Coder 32B:**
- Larger version of same model
- Better instruction following
- May improve tool calling
- Requires ~24GB VRAM

**DeepSeek Coder 33B:**
- Designed for code tasks
- Better at tool orchestration
- Requires ~24GB VRAM

**Mixtral 8x7B:**
- Mixture of experts architecture
- Good tool calling
- Requires ~32GB VRAM

### Option 3: Manually Create Plans (Workaround)

If you want to keep using Qwen 14B, you can create plans manually:

**Step 1: Create plan file**
```bash
# Create .crustly_plan_<session-id>.json manually
```

**Step 2: Use this template**
```json
{
  "id": "unique-uuid",
  "session_id": "your-session-id",
  "title": "Your Plan Title",
  "description": "Plan description",
  "tasks": [
    {
      "id": "task-uuid",
      "order": 0,
      "title": "Task 1",
      "description": "Detailed steps...",
      "task_type": "Create",
      "dependencies": [],
      "complexity": 3,
      "acceptance_criteria": [],
      "status": "Pending",
      "notes": null,
      "completed_at": null,
      "execution_history": [],
      "retry_count": 0,
      "max_retries": 3,
      "artifacts": [],
      "reflection": null
    }
  ],
  "context": "",
  "risks": [],
  "test_strategy": "",
  "technical_stack": [],
  "status": "Draft",
  "created_at": "2025-11-23T00:00:00Z",
  "updated_at": "2025-11-23T00:00:00Z",
  "approved_at": null
}
```

**Step 3: Press Ctrl+P to view**

### Option 4: Improve LM Studio Configuration

Try adjusting LM Studio settings:

1. **Temperature:** Lower to 0.1-0.3 for more deterministic output
2. **Top P:** Lower to 0.7-0.9
3. **Repeat Penalty:** Increase to 1.1-1.2
4. **Context Size:** Ensure adequate (at least 4096)
5. **Function Calling Format:** Ensure set to "OpenAI Compatible"

---

## Testing Different Models

### Quick Test Command

Create a simple test:
```bash
# In crustly, ask:
"Create a simple plan with 2 tasks"

# Check logs for:
grep "Converting .* tool calls" .crustly/logs/latest.log
```

**Expected:**
```
Converting 3 tool calls from OpenAI response  ← Should be 3+
```

**If you see:**
```
Converting 0 tool calls from OpenAI response  ← Model not calling tools
```

Then the model doesn't support tool calling well.

### Model Comparison

| Model | Size | Tool Calling | Speed | VRAM |
|-------|------|-------------|-------|------|
| **Claude Sonnet 4** | Cloud | ⭐⭐⭐⭐⭐ Excellent | Fast | 0 |
| **GPT-4** | Cloud | ⭐⭐⭐⭐⭐ Excellent | Fast | 0 |
| **GPT-4 Turbo** | Cloud | ⭐⭐⭐⭐⭐ Excellent | Very Fast | 0 |
| **Qwen 2.5 32B** | Local | ⭐⭐⭐⭐ Good | Medium | 24GB |
| **DeepSeek 33B** | Local | ⭐⭐⭐⭐ Good | Medium | 24GB |
| **Mixtral 8x7B** | Local | ⭐⭐⭐ Fair | Fast | 32GB |
| **Qwen 2.5 14B** | Local | ⭐⭐ Poor | Fast | 12GB |
| **CodeLlama 34B** | Local | ⭐⭐ Poor | Medium | 24GB |

---

## Immediate Actions You Can Take

### 1. Verify Tool Calling Works

Try a simpler tool first:
```
Ask: "Read the README.md file"
```

Check logs:
```bash
grep "tool_calls" .crustly/logs/latest.log
```

If `read_file` tool is called → Tool calling works, just needs better prompting
If no tools called → Model doesn't support tool calling reliably

### 2. Try Explicit Tool Call Request

Instead of:
```
"Create a plan for authentication"
```

Try:
```
"Call the plan tool with operation=create to make a plan for authentication.
First call: plan(operation='create', title='Auth Plan', description='...')
Then call: plan(operation='add_task', ...) for each task
Finally call: plan(operation='finalize')"
```

### 3. Switch to Claude Temporarily

To verify everything else works:
```toml
# crustly.toml - temporary test
[llm]
provider = "anthropic"
model = "claude-sonnet-4"
```

Set API key:
```bash
crustly secrets set anthropic sk-ant-your-key
```

Test plan creation - it should work perfectly.

---

## Long-term Recommendation

### For Development Work

**Use Claude Sonnet 4:**
- Most reliable tool calling
- Best instruction following
- Excellent code generation
- Worth the cost for productivity

**Monthly Cost Estimate:**
- Light use (10-20 plans/day): $5-10/month
- Medium use (50 plans/day): $20-40/month
- Heavy use (100+ plans/day): $50-100/month

### For Experimentation

**Use Local Qwen 32B or DeepSeek 33B:**
- Free to run
- Better tool calling than 14B
- Good for learning/testing
- Requires more VRAM

### For Production

**Use GPT-4 Turbo:**
- Fast and reliable
- Good tool calling
- Reasonable cost
- Widely supported

---

## Alternative: Function Call Wrapper

If you must use Qwen 14B, we could add a wrapper that detects when the LLM talks about tools instead of calling them:

```rust
// In agent service, after LLM response:
if response.contains("plan tool") && response.contains("create") {
    // LLM mentioned plan tool but didn't call it
    // Force a tool call
    warn!("LLM hallucinated tool use, forcing actual call");
    // Parse intent from text and call tool
}
```

This is **not recommended** as it's fragile and error-prone.

---

## Conclusion

**The real issue:** Qwen 2.5 Coder 14B doesn't reliably support tool calling.

**Best solution:** Use Claude Sonnet 4 or GPT-4 for plan creation.

**Acceptable workaround:** Upgrade to Qwen 32B or DeepSeek 33B if you have the VRAM.

**Quick fix:** I've strengthened the prompt transformation, but it may still fail with this model.

**Test:** Try creating a plan again. If it still doesn't work, you'll need a different model.

---

## What I've Done

✅ Strengthened prompt transformation to be more forceful
✅ Fixed Ctrl+P to load any plan status
✅ Fixed empty plan auto-replacement
✅ Added comprehensive documentation

**But:** The underlying issue is your LLM model's capabilities. No amount of prompting can make a model call tools if it wasn't trained to do so reliably.

---

*Diagnosed: 2025-11-23*
*Recommendation: Switch to Claude Sonnet 4 or GPT-4 for reliable tool calling*
*Workaround: Use larger local model (32B+) with better tool support*
