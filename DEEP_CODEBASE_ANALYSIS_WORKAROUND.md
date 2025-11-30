# Deep Codebase Analysis Workaround - Qwen 2.5 Coder 14B

## Problem

Qwen 2.5 Coder 14B has weak tool calling - it often calls a tool once and then stops, writing a text summary instead of continuing to explore deeply.

**Your Log Evidence:**
```
Converting 1 tool calls from OpenAI response  ← Called ls once
Converted tool call: ls with id 295549473
...
Converting 0 tool calls from OpenAI response  ← Then stopped calling tools
Found 0 tool uses to execute
No tool uses found, completing with final response
```

The model generated 747 tokens of text analysis instead of calling more tools.

---

## Workarounds

### Option 1: Force Tool Usage with Explicit Instructions

Instead of:
```
❌ "deeply analyze a codebase into folder with a limit of maximum 10 sublevels"
```

Use explicit tool commands:
```
✅ "Execute these steps:
1. glob(pattern='src/**/*.rs') to find all Rust files
2. bash(command='tree src -L 3') to show directory structure
3. read_file(file_path='src/lib.rs') to read main module
4. grep(pattern='pub mod', path='src') to find module declarations
5. Create a summary of the architecture"
```

### Option 2: Use Glob Instead of ls

The `glob` tool is more powerful for deep analysis:

**Instead of:**
```
❌ "list all files in src with 10 sublevels"
```

**Use:**
```
✅ "Use glob to find all files:
- glob(pattern='src/**/*.rs') for all Rust files
- glob(pattern='src/**/mod.rs') for all module files
- glob(pattern='src/**/*test*.rs') for all test files"
```

### Option 3: Create a Plan First

Force the model to think through steps:

```
Create a detailed plan to analyze the src directory with these requirements:
- Explore up to 10 subdirectory levels
- Find all Rust source files
- Identify the module structure
- Read key entry point files

After creating the plan, ask me to approve it, then execute each step.
```

This uses the `plan` tool which forces structured thinking.

### Option 4: Use Bash Commands Directly

Ask for bash/find commands:

```
Run these bash commands to analyze the codebase:
1. find src -type f -name "*.rs" | wc -l  (count Rust files)
2. find src -type d | head -50  (list directories)
3. tree src -L 4  (show tree structure)
4. Then read the main files: src/lib.rs, src/main.rs
```

---

## Example: Proper Deep Analysis Query

### ❌ What NOT to Do (Too Vague)
```
User: "deeply analyze a codebase into folder with a limit of maximum 10 sublevels"
```
**Result:** Model calls ls once, then writes text.

### ✅ What TO Do (Explicit)
```
User: "Analyze the Rust codebase in ./src by:

1. First, use glob(pattern='src/**/*.rs') to find ALL Rust files
2. Then use bash(command='find src -type d -maxdepth 10') to list all directories up to 10 levels
3. Read the main entry point: read_file(file_path='src/lib.rs')
4. Search for module declarations: grep(pattern='pub mod', path='src')
5. Search for main structures: grep(pattern='pub struct', path='src')
6. Summarize the architecture based on the tool results

Execute each step and show me the results."
```

**Result:** Model is forced to call tools in sequence.

---

## Better Queries for Different Use Cases

### 1. Count Files and Directories
```
Use these tools to analyze the codebase:
- glob(pattern='src/**/*.rs') to find all Rust files
- bash(command='find src -type d | wc -l') to count directories
- bash(command='wc -l src/**/*.rs | tail -1') to count total lines of code
```

### 2. Find Module Structure
```
Analyze the module structure:
1. glob(pattern='src/**/mod.rs') to find all module files
2. read_file(file_path='src/lib.rs') to see root module
3. grep(pattern='pub mod', path='src') to find all module declarations
4. Create a module dependency tree
```

### 3. Find Key Components
```
Find the main components:
1. grep(pattern='pub struct', path='src', glob='**/*.rs') to find all public structs
2. grep(pattern='pub trait', path='src', glob='**/*.rs') to find all traits
3. grep(pattern='pub fn', path='src/lib.rs') to find public functions in lib
4. Categorize components by purpose
```

### 4. Analyze Test Coverage
```
Check test coverage:
1. glob(pattern='src/**/*test*.rs') to find test files
2. glob(pattern='src/**/tests/**/*.rs') to find test modules
3. bash(command='grep -r "#\[test\]" src | wc -l') to count test functions
4. Compare with total functions
```

---

## Understanding the Model's Behavior

### Why Qwen 2.5 Coder 14B Stops Early

**Server log shows the problem:**
```
[INFO] [qwen/qwen2.5-coder-14b] Model generated tool calls: []
               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ Empty array!
```

**What happens:**
1. Model sees your vague request: "deeply analyze"
2. Model thinks: "I should list the directory first"
3. Model calls: `ls(path="./src")`
4. Model sees result with directory names
5. Model thinks: "I have enough info now, I'll write a summary"
6. Model generates: Long text response (747 tokens)
7. Model signals: **STOP** (no more tools)

**Why this happens:**
- Qwen 2.5 14B is trained primarily for **code generation**, not **tool orchestration**
- It sees tool calling as "optional" rather than "required"
- It prefers generating text over calling tools
- It doesn't understand "deep analysis" means "call tools multiple times"

### How Claude/GPT-4 Would Handle It

**Same query with Claude Sonnet 4:**
```
1. ls(./src) ✅
2. ls(./src/llm) ✅
3. ls(./src/llm/agent) ✅
4. ls(./src/llm/provider) ✅
5. ls(./src/llm/tools) ✅
6. glob(src/**/*.rs) ✅
7. read_file(src/lib.rs) ✅
8. grep(pub mod, src) ✅
9. Generates comprehensive summary ✅
```

Claude doesn't stop after one tool call - it keeps calling tools until it has complete information.

---

## Recommended Approach

### Best Practice: Multi-Step Explicit Instructions

```
Step 1: Find all Rust files
Use: glob(pattern='src/**/*.rs')

Step 2: List directory structure
Use: bash(command='find src -type d -maxdepth 10 | sort')

Step 3: Count files per directory
Use: bash(command='find src -type f -name "*.rs" | sed "s|/[^/]*$||" | uniq -c | sort -rn | head -20')

Step 4: Read main entry points
Use: read_file(file_path='src/lib.rs')
Use: read_file(file_path='src/main.rs')

Step 5: Find module structure
Use: grep(pattern='pub mod', path='src', output_mode='content')

Step 6: Analyze and summarize
Based on the above results, create a comprehensive architecture summary.
```

This forces the model to execute tools in order.

---

## Alternative: Use a Bash Script

Create a custom bash command:

```bash
# Ask Crustly to run this:
bash(command='
echo "=== Directory Structure ==="
find src -type d | head -50

echo -e "\n=== Rust Files Count ==="
find src -type f -name "*.rs" | wc -l

echo -e "\n=== Lines of Code ==="
find src -name "*.rs" -exec wc -l {} + | tail -1

echo -e "\n=== Module Files ==="
find src -name "mod.rs"

echo -e "\n=== Test Files ==="
find src -name "*test*.rs"
')
```

Then ask Crustly to analyze the bash output.

---

## Why Your Increased Loop Threshold Didn't Help

**Your fix was correct** - you increased the loop threshold from 3 to 10 for exploration tools.

**But the model never hit the limit** because:
- It only called `ls` **once** (iteration 1 of 20)
- Then it **stopped calling tools entirely** (0 tool calls)
- Loop detection never triggered (requires 10 **identical** calls)

**The problem isn't the loop threshold** - it's that the model **stops calling tools too early**.

---

## Long-Term Solution

### Option A: Use Better Model (Recommended)

Switch to a model with better tool calling:

```toml
# crustly.toml
[providers.anthropic]
api_key = "sk-ant-..."
default_model = "claude-sonnet-4"
```

Claude Sonnet 4 will:
- Call tools multiple times automatically
- Explore deeply without prompting
- Understand "analyze codebase" = "use multiple tools"

### Option B: Fine-tune Prompting

Add a system prompt that forces tool usage:

```rust
// In src/llm/agent/service.rs or CLI args
let system_prompt = "
You are a codebase analysis assistant. When analyzing code:
1. ALWAYS use multiple tools (glob, ls, read_file, grep)
2. Don't stop after one tool call - explore deeply
3. Call tools until you have complete information
4. Only write a summary AFTER you've collected all data
5. For 'deep analysis', explore at least 5-10 levels of directories

CRITICAL: Use tools extensively before responding with text.
";
```

But this only helps marginally with Qwen 2.5 14B.

---

## Summary

**Your Situation:**
- ✅ Loop threshold fix is correct (allows 10 ls calls)
- ❌ Model only calls ls once, then stops
- ❌ Model writes text instead of exploring

**Root Cause:**
- Qwen 2.5 Coder 14B weak tool calling (2/5 stars)
- Model prefers text generation over tool usage
- Vague queries like "deeply analyze" don't trigger multi-tool usage

**Solutions (Pick One):**

1. **Immediate Fix:** Use explicit multi-step tool instructions
2. **Better Queries:** Request specific tools (glob, bash commands)
3. **Plan First:** Use plan tool to force structured approach
4. **Long-term:** Upgrade to Claude Sonnet 4 or GPT-4 Turbo

**Recommended Query Template:**
```
Analyze the ./src codebase by executing these tools in order:

1. glob(pattern='src/**/*.rs') - find all Rust files
2. bash(command='find src -type d | head -50') - list directories
3. read_file(file_path='src/lib.rs') - read main module
4. grep(pattern='pub mod', path='src') - find module declarations
5. grep(pattern='pub struct', path='src') - find main structures
6. bash(command='wc -l src/**/*.rs | tail -1') - count total lines

Then create a comprehensive summary of:
- Directory structure
- Module organization
- Key components
- Architecture patterns
```

This forces the model to call tools sequentially.

---

**Status:** Documented
**Next Step:** Either use explicit tool instructions OR upgrade to better model
**Related Docs:** PROVIDER_COMPARISON_OPENAI_VS_QWEN.md, LLM_TOOL_CALLING_ISSUE.md
