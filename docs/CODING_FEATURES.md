# 👨‍💻 Why Crustly for Coding?

Crustly is specifically designed to be a **developer's best friend** in the terminal. Here's what makes it perfect for coders:

### 🚀 **Core Coding Features**

#### 1. **Built-in Tool Execution System**
Execute commands and manipulate files directly from chat:

```
You: "Read the contents of src/main.rs"
Crustly: [executes read tool] Here's your code: ...

You: "Create a new test file with basic structure"
Crustly: [executes write tool] Created tests/integration_test.rs with: ...

You: "Run cargo test"
Crustly: [executes bash tool] Running tests... ✅ 145 tests passed
```

**Available Tools (15 total):**
- 📖 **`read_file`** - Read file contents with syntax awareness
- ✏️ **`write_file`** - Create or modify files
- ✂️ **`edit_file`** - Precise text replacements in files
- 💻 **`bash`** - Execute shell commands safely
- 📂 **`ls`** - List directory contents
- 🔍 **`glob`** - Find files matching patterns
- 🔎 **`grep`** - Search file contents with regex
- 🌐 **`web_search`** - Search the web for information
- 🐍 **`execute_code`** - Run code in various languages
- 📓 **`notebook_edit`** - Edit Jupyter notebooks
- 📄 **`parse_document`** - Extract text from PDF, DOCX, HTML files
- 📋 **`task_manager`** - Manage agent tasks
- 🔗 **`http_request`** - Make HTTP requests
- 📊 **`session_context`** - Access session information
- 📝 **`plan`** - Create execution plans

#### 2. **Syntax Highlighting for 100+ Languages**
Code appears with proper highlighting in the terminal:
- Rust, Python, JavaScript, TypeScript, Go, Java, C++, and 100+ more
- Uses `syntect` with professional color schemes
- Automatic language detection
- Line numbers for easy reference

#### 3. **Markdown Code Blocks**
Code snippets are beautifully rendered:
```rust
╭─ rust ─────────────────╮
│  1 │ fn fibonacci(n: u32) -> u32 {
│  2 │     match n {
│  3 │         0 => 0,
│  4 │         1 => 1,
│  5 │         _ => fibonacci(n-1) + fibonacci(n-2)
│  6 │     }
│  7 │ }
╰────────────────────────╯
```

#### 4. **Multi-line Input with Real Cursor Editing**
Write or paste long code snippets naturally:
- Press `Shift+Enter` (or `Alt+Enter` on terminals without Kitty keyboard
  protocol support) for new lines
- `Enter` to send (`Ctrl+Enter` still works too)
- Real cursor movement (arrow keys, `Ctrl+Left`/`Right` to jump by word,
  `Home`/`End`) and mid-buffer editing - fix a typo in the middle of a long
  message without deleting everything after it
- `Ctrl+Backspace`/`Ctrl+Delete` to delete a whole word at a time
- Pasted text (including multi-line) is inserted at the cursor, not always
  appended at the end
- `Ctrl+Y` copies the last response to the system clipboard - just its
  code block if it has one, otherwise the full text
- `Ctrl+V` pastes from the system clipboard at the cursor, as a fallback
  alongside automatic bracketed paste
- Perfect for pasting entire functions or classes

#### 5. **Session-Based Context**
Crustly remembers your entire conversation:
```
You: "I'm working on a REST API in Rust"
Crustly: Great! I'll help you...

[Later in same session]
You: "Add error handling to the API"
Crustly: [Remembers you're working on Rust REST API]
```

#### 6. **Terminal-Native Workflow**
Stay in your terminal, no context switching:
- Launch with `crustly` or `cargo run`
- Split screen with your editor
- No browser tabs needed
- Fast keyboard shortcuts (`Ctrl+H` for help)

#### 7. **Local LLM Support (Privacy)**
Run completely offline with LM Studio:
- **100% Private** - Your proprietary code never leaves your machine
- **Zero API Costs** - Use local models like DeepSeek-Coder
- **Offline Development** - Work on sensitive projects securely
- See detailed guide above ⬆️

#### 8. **Streaming Responses**
See code generation in real-time:
- Character-by-character streaming
- Animated spinner shows processing
- No waiting for complete response
- Stop mid-generation if needed

#### 9. **Cost & Token Tracking**
Monitor your API usage:
```
💬 Tokens: 1,248  💰 Cost: $0.0037
```
- Per-message tracking
- Session totals
- Database persistence
- Budget control

---

### 🎯 **Common Coding Tasks**

#### **Code Generation**
```
You: "Write a binary search function in Rust with tests"
Crustly: [Generates implementation + tests with proper syntax highlighting]
```

#### **Code Review**
```
You: "Review this code for potential bugs"
[Paste your code]
Crustly: [Analyzes and provides feedback with specific line references]
```

#### **Debugging Help**
```
You: "I'm getting 'borrow checker error' in this code"
[Paste code]
Crustly: [Explains the issue and shows the fix with highlighting]
```

#### **Refactoring**
```
You: "Refactor this function to be more idiomatic Rust"
Crustly: [Shows before/after with explanations]
```

#### **Documentation**
```
You: "Generate doc comments for this module"
Crustly: [Creates comprehensive rustdoc comments]
```

#### **Testing**
```
You: "Write unit tests for this struct"
Crustly: [Generates test cases with proper assertions]
```

#### **Command Execution**
```
You: "Show me all TODO comments in the project"
Crustly: [Executes] grep -r "TODO" src/
```

---

### 🔄 **Typical Developer Workflow**

**Morning:**
```bash
$ crustly
> "Show me what we worked on yesterday"
[Crustly loads previous session and summarizes]

> "Let's continue with the authentication module"
[Crustly maintains context from yesterday]
```

**Implementing Feature:**
```
> "Create a new user authentication service"
[Crustly generates code with write tool]

> "Add password hashing with bcrypt"
[Crustly adds the feature]

> "Write integration tests"
[Crustly creates test file]

> "Run the tests"
[Executes: cargo test]
```

**Debugging:**
```
> "The login endpoint returns 500, here's the error:"
[Paste error]

> "Read the auth service file"
[Crustly reads it with read tool]

> "Fix the issue"
[Crustly modifies file with write tool]

> "Run tests again"
[Executes: cargo test] ✅ All passing!
```

**Documentation:**
```
> "Generate API documentation for the endpoints"
[Crustly creates comprehensive docs]

> "Add examples to the README"
[Crustly updates README with code examples]
```

---

### 💡 **Pro Tips for Coders**

1. **Keep Context in Sessions:**
   - Start new session per feature/bug
   - Use `Ctrl+L` to switch between projects
   - Session history persists indefinitely

2. **Leverage Tool System:**
   - Let Crustly read files instead of pasting
   - Use bash tool for git commands
   - Write tool for quick file generation

3. **Use Local LLMs for Sensitive Code:**
   - Company proprietary code
   - Pre-release features
   - Security-sensitive implementations

4. **Keyboard Shortcuts:**
   ```
   Enter        - Send message (Ctrl+Enter still works too)
   Shift+Enter  - New line (Alt+Enter on non-Kitty terminals)
   Ctrl+H       - Help (full command list)
   Ctrl+N       - New session (new feature)
   Ctrl+L       - Switch sessions (different projects)
   Page Up/Down - Scroll through long code outputs
   ```

5. **Multi-line for Code:**
   - Paste entire functions
   - Press Shift+Enter (or Alt+Enter) for newlines
   - `Enter` when ready to send

6. **Markdown for Formatting:**
   - Use triple backticks for code blocks
   - Specify language for syntax highlighting
   - Makes responses easier to read

---

### 🆚 **Comparison with Other Coding Assistants**

| Feature | Crustly | GitHub Copilot | ChatGPT | Cursor |
|---------|---------|----------------|---------|--------|
| **Terminal Native** | ✅ | ❌ | ❌ | ❌ |
| **File Operations** | ✅ Built-in | ❌ | ❌ | ✅ |
| **Command Execution** | ✅ | ❌ | ❌ | ❌ |
| **Local LLM Support** | ✅ | ❌ | ❌ | ❌ |
| **Session History** | ✅ Persistent | ❌ | ✅ Limited | ✅ |
| **Syntax Highlighting** | ✅ 100+ langs | ✅ | ❌ | ✅ |
| **Cost Tracking** | ✅ | ❌ | ❌ | ❌ |
| **Offline Mode** | ✅ | ❌ | ❌ | ❌ |
| **Open Source** | ✅ | ❌ | ❌ | ❌ |
| **Privacy First** | ✅ | ⚠️ | ⚠️ | ⚠️ |

---

### 🎓 **Perfect For:**

- ✅ **Backend Developers** - Rust, Go, Python, Node.js
- ✅ **Systems Programmers** - C, C++, Rust
- ✅ **DevOps Engineers** - Shell scripting, automation
- ✅ **Full-Stack Developers** - Multiple languages
- ✅ **Open Source Contributors** - Code review, documentation
- ✅ **Students** - Learning programming concepts
- ✅ **Security-Conscious Devs** - Local inference for proprietary code
- ✅ **CLI Enthusiasts** - Terminal workflow lovers
- ✅ **Budget-Conscious** - Cost tracking + local LLMs

---

### 🚀 **Future Coding Features (Planned)**

- 🔜 **LSP Integration** - Semantic code understanding
- 🔜 **Git Integration** - Commit message generation, PR reviews
- 🔜 **Project Context** - Auto-load `.cursorrules`, codebase awareness
- 🔜 **Code Search** - Grep across entire projects
- 🔜 **Refactoring Tools** - Automated code transformations
- 🔜 **Test Generation** - Intelligent test case creation
- 🔜 **Performance Analysis** - Profiling suggestions
- 🔜 **Security Scanning** - Vulnerability detection

---

**Ready to supercharge your coding workflow?** 🚀

```bash
cargo run
# Start coding with Crustly!
```

---

