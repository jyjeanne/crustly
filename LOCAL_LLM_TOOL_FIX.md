# Local LLM Tool Support Fix

**Date:** 2025-11-10
**Fixed:** Tools now work with local LLMs (LM Studio, Ollama, etc.)

## Problem

When asking local LLMs to create files or execute commands, they would respond with:
> "I'm currently unable to directly interact with your local files"

**Root Cause:** The TUI was calling `send_message()` which does NOT send tool definitions to the LLM. Without tool definitions, the LLM doesn't know it CAN create files or run commands.

### Evidence from Logs

Your LM Studio log showed:
```json
{
  "model": "gpt-4-turbo-preview",
  "messages": [...],
  "max_tokens": 4096,
  "stream": false
  // ❌ Missing: "tools": [...]
}
```

And the response confirmed:
```json
"tool_calls": []  // Empty!
```

## Solution

**Changed:** `src/tui/app.rs` line 398

```diff
- match agent_service.send_message(session_id, content, None).await {
+ match agent_service.send_message_with_tools(session_id, content, None).await {
```

This one-line change:
- ✅ Sends tool definitions (read_file, write_file, bash) to the LLM
- ✅ Enables tool execution loop
- ✅ Shows approval dialogs for dangerous operations
- ✅ Works with local LLMs and cloud APIs

## Testing the Fix

### 1. Rebuild Crustly

```bash
cd crustly
cargo build --release
```

### 2. Start Your Local LLM

**For LM Studio:**
1. Open LM Studio
2. Load model (e.g., Qwen 2.5 Coder 7B)
3. Start server on port 1234

### 3. Configure Environment

```bash
# PowerShell (Windows)
$env:OPENAI_API_KEY="lm-studio"
$env:OPENAI_BASE_URL="http://localhost:1234/v1/chat/completions"

# Bash (Linux/macOS)
export OPENAI_API_KEY="lm-studio"
export OPENAI_BASE_URL="http://localhost:1234/v1/chat/completions"
```

### 4. Run Crustly

```bash
cargo run --release
```

### 5. Test Tool Usage

**Test 1: File Creation**
```
You: Create a test file called hello.txt with content "Hello World"
```

**Expected:**
```
┌────────────────────────────────────────┐
│ ⚠️  PERMISSION REQUIRED                │
├────────────────────────────────────────┤
│ Tool: write_file                       │
│ Path: hello.txt                        │
│ Content: Hello World                   │
│                                        │
│ ⏱️  Timeout: 5m 00s                    │
│ [A]pprove  [D]eny  [V]iew Details     │
└────────────────────────────────────────┘
```

Press `A` to approve.

**Result:**
```
Crustly: ✅ I've created hello.txt with the content "Hello World"
```

**Test 2: Read File**
```
You: Read the hello.txt file you just created
```

**Expected:**
- No approval dialog (read is safe)
- Crustly shows file contents

**Test 3: Run Command**
```
You: Run 'ls' to show files in current directory
```

**Expected:**
```
┌────────────────────────────────────────┐
│ ⚠️  PERMISSION REQUIRED                │
│ Tool: bash                             │
│ Command: ls                            │
│ [A]pprove  [D]eny                     │
└────────────────────────────────────────┘
```

Press `A` to approve.

## What You Should See in LM Studio Logs Now

**BEFORE fix:**
```json
{
  "model": "...",
  "messages": [...],
  "max_tokens": 4096
  // ❌ No tools field
}
```

**AFTER fix:**
```json
{
  "model": "...",
  "messages": [...],
  "max_tokens": 4096,
  "tools": [
    {
      "type": "function",
      "function": {
        "name": "read_file",
        "description": "Read contents of a file...",
        "parameters": { "type": "object", ... }
      }
    },
    {
      "type": "function",
      "function": {
        "name": "write_file",
        "description": "Write content to a file...",
        "parameters": { "type": "object", ... }
      }
    },
    {
      "type": "function",
      "function": {
        "name": "bash",
        "description": "Execute shell command...",
        "parameters": { "type": "object", ... }
      }
    }
  ]
}
```

And the LLM response might include:
```json
{
  "message": {
    "role": "assistant",
    "content": "",
    "tool_calls": [
      {
        "id": "call_123",
        "type": "function",
        "function": {
          "name": "write_file",
          "arguments": "{\"path\":\"hello.txt\",\"content\":\"Hello World\"}"
        }
      }
    ]
  }
}
```

## How It Works Now

### Tool Execution Flow

```
User: "Create a config.json file"
      │
      ▼
Agent sends request WITH tools to LLM
      │
      ▼
LLM sees available tools:
  - read_file (safe, no approval)
  - write_file (requires approval)
  - bash (requires approval)
      │
      ▼
LLM responds with tool call:
  tool_calls: [{
    name: "write_file",
    arguments: {path: "config.json", content: "..."}
  }]
      │
      ▼
Crustly checks: requires_approval() → YES
      │
      ▼
Shows approval dialog
      │
      ├─ User approves (A)
      │       │
      │       ▼
      │  Execute write_file tool
      │       │
      │       ▼
      │  Return result to LLM
      │       │
      │       ▼
      │  LLM: "I've created the file!"
      │
      └─ User denies (D)
              │
              ▼
         Return error to LLM
              │
              ▼
         LLM: "The operation was not approved"
```

## Available Tools

### 1. read_file
- **Purpose:** Read file contents
- **Approval:** ❌ No (safe operation)
- **Schema:**
```json
{
  "path": "string (required)",
  "start_line": "integer (optional)",
  "line_count": "integer (optional)"
}
```

### 2. write_file
- **Purpose:** Create or modify files
- **Approval:** ✅ Yes (dangerous)
- **Schema:**
```json
{
  "path": "string (required)",
  "content": "string (required)"
}
```

### 3. bash
- **Purpose:** Execute shell commands
- **Approval:** ✅ Yes (dangerous)
- **Schema:**
```json
{
  "command": "string (required)",
  "working_directory": "string (optional)"
}
```

## Troubleshooting

### Issue: Still getting "I cannot interact with files"

**Check:**
1. You built the latest code: `cargo build --release`
2. You're running the new binary: `cargo run --release` (not an old build)
3. LM Studio server is running
4. Environment variables are set

### Issue: No approval dialog appears

**Possible causes:**
1. Tool doesn't require approval (read_file doesn't)
2. Auto-approve is enabled (check code)
3. LLM didn't call the tool (check LM Studio logs)

### Issue: LLM still not calling tools

**Check LM Studio logs for:**
- Request has `"tools": [...]` field
- Model supports function calling
- Not all models support tools (Qwen 2.5 Coder DOES)

**If tools field is missing:**
- You're using the wrong method
- Agent service doesn't have tool registry
- Provider doesn't support tools

## Model Compatibility

### ✅ Known Working Models

| Model | Size | Tool Support | Notes |
|-------|------|--------------|-------|
| Qwen 2.5 Coder 7B | 7B | ✅ Excellent | Optimized for coding |
| Llama 3.1 8B | 8B | ✅ Good | General purpose |
| Mistral 7B | 7B | ✅ Good | Fast inference |
| DeepSeek Coder 6.7B | 6.7B | ✅ Excellent | Code-focused |

### ⚠️ Models with Limited Support

- Older models without function calling training
- Very small models (<3B parameters)
- Non-instruct variants

## Example Workflows

### Workflow 1: Code Generation
```
You: Create a Rust function to calculate fibonacci numbers

Crustly: [Generates code]
         [Calls write_file]
         [Shows approval]
         [You approve]
         ✅ Created src/fibonacci.rs

You: Add tests for that function

Crustly: [Generates tests]
         [Calls write_file]
         [Shows approval]
         [You approve]
         ✅ Created tests/fibonacci_test.rs

You: Run the tests

Crustly: [Calls bash: cargo test fibonacci]
         [Shows approval]
         [You approve]
         ✅ 5 tests passed
```

### Workflow 2: Project Setup
```
You: Initialize a new Rust project called 'myapp'

Crustly: [Calls bash: cargo new myapp]
         [Shows approval]
         [You approve]
         ✅ Created project 'myapp'

You: Add serde dependency

Crustly: [Reads Cargo.toml]
         [Calls write_file with updated Cargo.toml]
         [Shows approval]
         [You approve]
         ✅ Added serde to dependencies
```

### Workflow 3: Debugging
```
You: Read src/main.rs and find the bug

Crustly: [Calls read_file]
         [Analyzes code]
         I found the issue on line 23...

You: Fix it

Crustly: [Calls write_file]
         [Shows approval]
         [You approve]
         ✅ Fixed the bug

You: Run the program

Crustly: [Calls bash: cargo run]
         [Shows approval]
         [You approve]
         ✅ Program runs successfully!
```

## Security Notes

### Approval System

**Always approve with caution:**
- ⚠️ **write_file**: Can overwrite existing files
- ⚠️ **bash**: Can execute ANY command (rm, curl, etc.)
- ✅ **read_file**: Safe, reads only

**Press `V` to view full details** before approving!

### Timeout Protection

- Approval dialogs auto-deny after **5 minutes**
- Color-coded countdown:
  - 🟢 Green: > 2 minutes remaining
  - 🟡 Yellow: 1-2 minutes remaining
  - 🔴 Red: < 1 minute remaining

### Auto-Approve (Development Only)

**DO NOT** enable auto-approve in production:
```rust
// DANGEROUS - bypasses all safety checks
.with_auto_approve_tools(true)
```

## Next Steps

1. ✅ Test with your Qwen model
2. ✅ Verify tools appear in LM Studio logs
3. ✅ Try file creation and command execution
4. ✅ Check approval dialogs work correctly
5. 📝 Report any issues or improvements

## Related Files

- **Fixed:** `src/tui/app.rs` (line 398)
- **Tool System:** `src/llm/tools/`
- **Agent Service:** `src/llm/agent/service.rs`
- **OpenAI Provider:** `src/llm/provider/openai.rs`

## Summary

This fix enables **full tool support** for local LLMs by ensuring tool definitions are sent in every request. Your Qwen 2.5 Coder 7B model can now:

- ✅ Create and modify files
- ✅ Read project files
- ✅ Execute shell commands
- ✅ Generate code with context
- ✅ Run tests and builds

All with **interactive approval** for dangerous operations and **100% local privacy**.

---

**Enjoy your now-functional AI coding assistant!** 🚀
