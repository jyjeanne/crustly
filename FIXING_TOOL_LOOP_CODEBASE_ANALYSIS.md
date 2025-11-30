# Fixing Tool Loop Issue for Codebase Analysis

## Your Problem

When you ask: **"deeply analyse a codebase into local folder .\src"**

The LLM gets stuck calling `ls` repeatedly (3+ times) and tool loop detection blocks it:
```
WARN: Detected tool loop: 'ls' called 3 times in a row. Breaking loop.
```

---

## Root Causes

### 1. Model Confusion About Path Format
Your logs show the model switching between:
- `.\src` (Windows style)
- `./src` (Unix style)
- `.src` (typo!)

The model is confused and keeps calling `ls` to "figure out" the right path.

### 2. Tool Loop Detection is TOO Aggressive

Current logic in `src/llm/agent/service.rs:372-415`:
```rust
// For add_task, don't consider it a loop - it's expected
if operation == "add_task" {
    // Include task title to distinguish different tasks
    if let Some(title) = input.get("title").and_then(|v| v.as_str()) {
        format!("{}:{}:{}", name, operation, title)
    } else {
        format!("{}:{}", name, operation)
    }
} else {
    format!("{}:{}", name, operation)
}
```

**Problem:** For `ls`, it only checks the tool name, NOT the arguments!

Calling `ls` with different paths should be allowed:
- `ls(path="./src")` - Different from
- `ls(path="./src/cli")` - Different from
- `ls(path="./src/tui")` - All different!

But current code sees them all as `"ls"` → triggers loop detection after 3 calls.

---

## Solutions

### Solution 1: Include Arguments in Loop Detection Signature (RECOMMENDED)

**Modify:** `src/llm/agent/service.rs` lines 372-415

**Current Code:**
```rust
let current_call_signature = tool_uses
    .iter()
    .map(|(_, name, input)| {
        if name == "plan" {
            // Special handling for plan tool
            if let Some(operation) = input.get("operation").and_then(|v| v.as_str()) {
                if operation == "add_task" {
                    if let Some(title) = input.get("title").and_then(|v| v.as_str()) {
                        format!("{}:{}:{}", name, operation, title)
                    } else {
                        format!("{}:{}", name, operation)
                    }
                } else {
                    format!("{}:{}", name, operation)
                }
            } else {
                name.to_string()
            }
        } else {
            name.to_string()  // ← PROBLEM: Only tool name!
        }
    })
    .collect::<Vec<_>>()
    .join(",");
```

**Fixed Code:**
```rust
let current_call_signature = tool_uses
    .iter()
    .map(|(_, name, input)| {
        if name == "plan" {
            // Special handling for plan tool
            if let Some(operation) = input.get("operation").and_then(|v| v.as_str()) {
                if operation == "add_task" {
                    if let Some(title) = input.get("title").and_then(|v| v.as_str()) {
                        format!("{}:{}:{}", name, operation, title)
                    } else {
                        format!("{}:{}", name, operation)
                    }
                } else {
                    format!("{}:{}", name, operation)
                }
            } else {
                name.to_string()
            }
        } else {
            // Include primary argument in signature for tools with path/file arguments
            match name.as_str() {
                "ls" | "glob" | "grep" | "read" => {
                    // These tools take path-like arguments
                    if let Some(path) = input.get("path")
                        .or_else(|| input.get("file_path"))
                        .or_else(|| input.get("pattern"))
                        .and_then(|v| v.as_str())
                    {
                        format!("{}:{}", name, path)
                    } else {
                        name.to_string()
                    }
                }
                "write" | "edit" => {
                    // File modification tools - include file path
                    if let Some(path) = input.get("file_path").and_then(|v| v.as_str()) {
                        format!("{}:{}", name, path)
                    } else {
                        name.to_string()
                    }
                }
                "bash" => {
                    // Include command for bash tool
                    if let Some(cmd) = input.get("command").and_then(|v| v.as_str()) {
                        // Take first 50 chars of command to avoid huge signatures
                        let cmd_short: String = cmd.chars().take(50).collect();
                        format!("{}:{}", name, cmd_short)
                    } else {
                        name.to_string()
                    }
                }
                _ => name.to_string(),  // Other tools: just use name
            }
        }
    })
    .collect::<Vec<_>>()
    .join(",");
```

**Benefits:**
- ✅ Allows `ls` with different paths
- ✅ Blocks true loops: `ls(./src)` called 3 times = blocked
- ✅ Allows exploration: `ls(./src)` → `ls(./src/cli)` → `ls(./src/tui)` = allowed
- ✅ Works for other file tools (glob, grep, read, write, edit)

---

### Solution 2: Increase Loop Threshold for Exploration Tools

**Modify:** `src/llm/agent/service.rs` around line 417

**Current Code:**
```rust
if !current_call_signature.is_empty()
    && current_call_signature == last_call_signature
    && current_call_signature == second_last_call_signature
{
    tracing::warn!(
        "⚠️ Detected tool loop: '{}' called 3 times in a row. Breaking loop.",
        current_call_signature
    );
    break;
}
```

**Alternative: Different thresholds for different tools:**
```rust
// Different loop thresholds for different tool types
let loop_threshold = match tool_uses.first().map(|(_, name, _)| name.as_str()) {
    Some("ls") | Some("glob") | Some("grep") | Some("read") => 5,  // Higher for exploration
    Some("write") | Some("edit") | Some("bash") => 2,              // Lower for modifications
    _ => 3,                                                         // Default
};

// Check for loop based on threshold
let is_loop = if loop_threshold == 2 {
    !current_call_signature.is_empty()
        && current_call_signature == last_call_signature
} else if loop_threshold == 3 {
    !current_call_signature.is_empty()
        && current_call_signature == last_call_signature
        && current_call_signature == second_last_call_signature
} else {
    // For threshold 5: check last 3 signatures match
    !current_call_signature.is_empty()
        && current_call_signature == last_call_signature
        && current_call_signature == second_last_call_signature
        // Would need to track 4th and 5th signatures
};

if is_loop {
    tracing::warn!(
        "⚠️ Detected tool loop: '{}' called {} times in a row. Breaking loop.",
        current_call_signature,
        loop_threshold
    );
    break;
}
```

**Benefits:**
- ✅ Allows more exploration with read-only tools
- ✅ Stricter with destructive tools
- ⚠️ More complex logic

---

### Solution 3: Smart Loop Detection (Best for Production)

Combine both approaches with context awareness:

```rust
// Build signature with context awareness
let current_call_signature = tool_uses
    .iter()
    .map(|(_, name, input)| {
        // Always include primary argument that differentiates calls
        let signature = match name.as_str() {
            "plan" => {
                // Existing plan logic...
                if let Some(operation) = input.get("operation").and_then(|v| v.as_str()) {
                    if operation == "add_task" {
                        if let Some(title) = input.get("title").and_then(|v| v.as_str()) {
                            format!("{}:{}:{}", name, operation, title)
                        } else {
                            format!("{}:{}", name, operation)
                        }
                    } else {
                        format!("{}:{}", name, operation)
                    }
                } else {
                    name.to_string()
                }
            }

            // File system exploration tools
            "ls" => {
                if let Some(path) = input.get("path").and_then(|v| v.as_str()) {
                    format!("ls:{}", path)
                } else {
                    "ls:".to_string()
                }
            }

            "glob" => {
                if let Some(pattern) = input.get("pattern").and_then(|v| v.as_str()) {
                    format!("glob:{}", pattern)
                } else {
                    "glob:".to_string()
                }
            }

            "grep" => {
                // Include pattern AND path to distinguish searches
                let pattern = input.get("pattern").and_then(|v| v.as_str()).unwrap_or("");
                let path = input.get("path").and_then(|v| v.as_str()).unwrap_or("");
                format!("grep:{}:{}", pattern, path)
            }

            "read" => {
                if let Some(path) = input.get("file_path").and_then(|v| v.as_str()) {
                    format!("read:{}", path)
                } else {
                    "read:".to_string()
                }
            }

            // File modification tools
            "write" | "edit" => {
                if let Some(path) = input.get("file_path").and_then(|v| v.as_str()) {
                    format!("{}:{}", name, path)
                } else {
                    format!("{}:", name)
                }
            }

            // Command execution
            "bash" => {
                if let Some(cmd) = input.get("command").and_then(|v| v.as_str()) {
                    // Normalize path separators for comparison
                    let cmd_normalized = cmd.replace('\\', "/");
                    let cmd_short: String = cmd_normalized.chars().take(100).collect();
                    format!("bash:{}", cmd_short)
                } else {
                    "bash:".to_string()
                }
            }

            // Other tools: just use name
            _ => name.to_string(),
        };

        signature
    })
    .collect::<Vec<_>>()
    .join(",");

// Detect loops with tool-specific thresholds
if !current_call_signature.is_empty() {
    let is_exploration_tool = current_call_signature.starts_with("ls:")
        || current_call_signature.starts_with("glob:")
        || current_call_signature.starts_with("grep:")
        || current_call_signature.starts_with("read:");

    let is_loop = if is_exploration_tool {
        // For exploration tools, only trigger if exact same call repeated 3 times
        current_call_signature == last_call_signature
            && current_call_signature == second_last_call_signature
    } else {
        // For other tools, trigger on 2 consecutive identical calls
        current_call_signature == last_call_signature
    };

    if is_loop {
        tracing::warn!(
            "⚠️ Detected tool loop: '{}' called multiple times in a row. Breaking loop.",
            current_call_signature
        );

        // Add helpful hint about what went wrong
        if is_exploration_tool {
            tracing::info!(
                "💡 Hint: The model is stuck trying to access the same path. \
                 This often means the path doesn't exist or the model is confused about the directory structure."
            );
        }

        break;
    }
}
```

**Benefits:**
- ✅ Different paths treated as different calls
- ✅ Tool-specific logic (exploration vs modification)
- ✅ Helpful debug messages
- ✅ Path normalization (handles `.\src` vs `./src`)

---

## Implementation

### Step 1: Apply Solution 3 (Recommended)

Edit `src/llm/agent/service.rs` around lines 361-420.

### Step 2: Add Path Normalization Helper

At the top of the file:
```rust
/// Normalize path separators for consistent comparison
fn normalize_path(path: &str) -> String {
    // Convert Windows backslashes to forward slashes
    path.replace('\\', "/")
        .trim_start_matches("./")
        .trim_start_matches(".\\")
        .to_string()
}
```

Use it in signature building:
```rust
"ls" => {
    if let Some(path) = input.get("path").and_then(|v| v.as_str()) {
        format!("ls:{}", normalize_path(path))
    } else {
        "ls:".to_string()
    }
}
```

### Step 3: Test

After making changes:
```bash
cargo fmt
cargo clippy --all-targets -- -D warnings
cargo build --release
```

Try your query again:
```
User: deeply analyse a codebase into local folder .\src
```

Expected behavior:
- `ls(path="./src")` - OK
- `ls(path="./src/cli")` - OK (different path)
- `ls(path="./src/tui")` - OK (different path)
- `ls(path="./src")` - OK (first time)
- `ls(path="./src")` - OK (second time)
- `ls(path="./src")` - **BLOCKED** (third identical call)

---

## Alternative: Increase Max Iterations

**Quick fix if you don't want to modify loop detection:**

Edit `src/llm/agent/service.rs`:
```rust
impl AgentService {
    pub fn new(provider: Arc<dyn Provider>, context: ServiceContext) -> Self {
        Self {
            provider,
            context,
            tool_registry: Arc::new(ToolRegistry::new()),
            max_tool_iterations: 20,  // ← Increase from 10 to 20
            default_system_prompt: None,
            auto_approve_tools: false,
            approval_callback: None,
            working_directory: std::env::current_dir().unwrap_or_default(),
        }
    }
}
```

**Config option in `crustly.toml`:**
```toml
[agent]
max_tool_iterations = 30  # Allow more iterations for deep analysis
```

**But this doesn't fix the root cause!**

---

## Testing Your Fix

### Test Case 1: Exploring Multiple Directories
```
User: List all files in ./src, ./docs, and ./tests directories
```

Expected:
- `ls(./src)` - OK
- `ls(./docs)` - OK (different path)
- `ls(./tests)` - OK (different path)
- NO loop detection

### Test Case 2: True Loop (Should Block)
Model gets confused and calls:
- `ls(./src)` - OK
- `ls(./src)` - OK
- `ls(./src)` - **BLOCKED** (same path 3rd time)

### Test Case 3: Path Variations (Should Allow)
- `ls(.\src)` - OK (normalized to `src`)
- `ls(./src)` - **BLOCKED** (same as above after normalization)

---

## Long-Term Fix: Better Model Prompting

Your real issue is **Qwen 2.5 Coder 14B's weak tool calling**. Consider:

### 1. Add System Prompt Guidance

```rust
let system_prompt = "When exploring a codebase:
1. Use `glob` to find files by pattern (e.g., \"**/*.rs\" for all Rust files)
2. Use `read` to examine specific files
3. Use `grep` to search for specific code patterns
4. Don't call `ls` repeatedly on the same directory

For analysis tasks, plan your tool usage before executing.";
```

### 2. Use Better Model

Upgrade to:
- **Claude Sonnet 4** (best tool calling)
- **GPT-4 Turbo** (excellent tool calling)
- **Qwen 3 32B+** (better than 2.5)

### 3. Add Tool Usage Examples

Modify tool descriptions to include usage hints:
```rust
// In src/llm/tools/ls.rs
pub fn description() -> &'static str {
    "List directory contents. Use once per directory. \
     For finding files, prefer 'glob' with patterns like '**/*.rs'."
}
```

---

## Summary

**Your issue:** Tool loop detection blocks legitimate exploration because it only checks tool name, not arguments.

**Best fix:** Solution 3 - Include arguments in loop signature and normalize paths.

**Quick fix:** Increase `max_tool_iterations` to 30.

**Long-term:** Upgrade model or improve prompting.

---

**Status:** Ready to implement
**Files to modify:** `src/llm/agent/service.rs` (lines 361-420)
**Testing:** Run codebase analysis queries
**Verification:** Check logs for loop detection with different paths
