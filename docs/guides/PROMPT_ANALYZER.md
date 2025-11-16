# Prompt Analyzer - Automatic Tool Call Detection

## Overview

The Prompt Analyzer automatically detects keywords in user prompts and transforms them to include explicit tool call hints for the LLM. This ensures that the AI uses the correct tools when processing user requests.

## How It Works

When you press **Ctrl+Enter** to submit a message, the system:

1. **Analyzes** your prompt for specific keywords
2. **Detects** the intended tool usage (plan, read_file, grep, etc.)
3. **Transforms** the prompt by adding explicit tool hints
4. **Sends** the enhanced prompt to the LLM

The original prompt is shown in the UI, while the transformed prompt with tool hints is sent to the LLM behind the scenes.

## Supported Tools and Keywords

### 1. Plan Tool
**Keywords:** `make a plan`, `create a plan`, `plan for`, `plan to implement`, `planning`

**Example:**
```
User: make a plan for implementing JWT authentication
→ Transformed: [original prompt] + TOOL HINT: Use the `plan` tool
```

### 2. Read File Tool
**Keywords:** `read file`, `read the file`, `show me file`, `what's in`, `view file`, `check file`

**Example:**
```
User: read the file src/main.rs and explain it
→ Transformed: [original prompt] + TOOL HINT: Use the `read_file` tool
```

### 3. Grep/Search Tool
**Keywords:** `search for`, `find`, `look for`, `grep`, `search code`, `where is`, `locate`

**Example:**
```
User: search for the function getUserData
→ Transformed: [original prompt] + TOOL HINT: Use the `grep` tool
```

### 4. Write File Tool
**Keywords:** `create file`, `write file`, `write to file`, `make a file`, `new file`

**Example:**
```
User: create file README.md with project documentation
→ Transformed: [original prompt] + TOOL HINT: Use the `write_file` tool
```

### 5. Edit File Tool
**Keywords:** `edit file`, `modify file`, `update file`, `change file`, `fix in file`

**Example:**
```
User: edit file config.toml and update the database URL
→ Transformed: [original prompt] + TOOL HINT: Use the `edit_file` tool
```

### 6. Bash Tool
**Keywords:** `run command`, `execute command`, `run shell`, `shell command`, `bash command`

**Example:**
```
User: run command cargo test
→ Transformed: [original prompt] + TOOL HINT: Use the `bash` tool
```

### 7. Web Search Tool
**Keywords:** `search online`, `search the web`, `google`, `search internet`, `look up online`

**Example:**
```
User: search the web for rust async best practices
→ Transformed: [original prompt] + TOOL HINT: Use the `web_search` tool
```

## Multiple Tool Detection

The analyzer can detect multiple tools in a single prompt:

**Example:**
```
User: read file config.toml and make a plan to update it
→ Transformed: [original prompt]
              + TOOL HINT: Use the `read_file` tool
              + TOOL HINT: Use the `plan` tool
```

## Benefits

✅ **Ensures Correct Tool Usage** - The LLM receives explicit hints about which tools to use
✅ **Improves Accuracy** - Reduces ambiguity in user requests
✅ **Transparent** - User sees original prompt, LLM gets enhanced version
✅ **Non-Intrusive** - Only activates when keywords are detected
✅ **Case-Insensitive** - Works with any capitalization

## Implementation Details

- **Location:** `src/tui/prompt_analyzer.rs`
- **Integration:** `src/tui/app.rs` (send_message function)
- **Trigger:** Ctrl+Enter keyboard shortcut
- **Detection:** Regex-based keyword matching with word boundaries
- **Logging:** Transformation events are logged at INFO level

## Testing

Run the prompt analyzer tests:
```bash
cargo test --lib prompt_analyzer
```

All tests verify:
- Individual tool detection
- Multiple tool detection
- Case-insensitive matching
- No false positives
- Transformation accuracy

## Configuration

The keyword lists can be extended by editing:
```rust
// src/tui/prompt_analyzer.rs
const PLAN_KEYWORDS: &[&str] = &[
    "make a plan",
    "create a plan",
    // Add more keywords here
];
```

## Future Enhancements

Potential improvements:
- [ ] User-configurable keyword lists
- [ ] Machine learning-based intent detection
- [ ] Natural language understanding for complex prompts
- [ ] Context-aware tool suggestions
- [ ] Tool usage analytics and recommendations
