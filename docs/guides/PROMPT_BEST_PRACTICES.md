# 💡 Best Practices for Using Crustly

### Writing Effective Prompts

Crustly is equipped with **powerful tools** (file operations, code execution, web search, etc.). To get the most out of it, **encourage tool usage** in your prompts.

---

### ✅ Sample Prompts (Recommended)

These prompts **encourage Crustly to explore and use tools**:

#### 1. **Codebase Exploration**
```
Analyze this codebase:
1. Explore the /src directory structure
2. Identify the main entry points
3. Find all dependencies in Cargo.toml
4. List the design patterns used
5. Summarize the architecture

Start by using glob to find all Rust files.
```

**Why it works:** Explicitly tells Crustly to use tools (glob, read_file)

---

#### 2. **Deep Code Analysis**
```
I need a comprehensive analysis of the authentication system:
1. Find all files related to authentication (grep for "auth", "login", "session")
2. Read the main authentication modules
3. Document the flow from login to session creation
4. Identify security best practices used
5. Suggest improvements

Use grep and read_file tools to explore the code.
```

**Why it works:** Mentions specific tools, gives clear steps

---

#### 3. **Bug Investigation**
```
I'm getting a "connection timeout" error in the API client.
1. Find all files containing "timeout" or "connect"
2. Read the network client implementation
3. Check the configuration for timeout settings
4. Explain what's causing the issue
5. Suggest a fix

Start by using grep to locate the relevant code.
```

**Why it works:** Asks Crustly to investigate systematically

---

#### 4. **Feature Implementation**
```
I need to add rate limiting to the API:
1. Explore the current request handling code (find files with "request", "handler")
2. Read the middleware implementation
3. Research rate limiting strategies (use web_search if available)
4. Create a rate limiting middleware
5. Write tests for the new feature

Begin by exploring the existing middleware architecture.
```

**Why it works:** Multi-step task encourages thorough exploration

---

#### 5. **Documentation Generation**
```
Generate comprehensive documentation for this project:
1. Read README.md to understand current docs
2. Explore all modules in /src (use glob for *.rs files)
3. For each module, read and document:
   - Purpose and functionality
   - Public API
   - Usage examples
4. Create a DEVELOPER_GUIDE.md

Start by listing all source files.
```

**Why it works:** Structured task with clear tool usage

---

#### 6. **Dependency Analysis**
```
I want to understand all external dependencies:
1. Read Cargo.toml
2. For each dependency, search the code for usage (grep)
3. Document what each dependency is used for
4. Identify any unused dependencies
5. Suggest lightweight alternatives

Begin by reading the Cargo.toml file.
```

**Why it works:** Specific files mentioned, clear methodology

---

### ❌ Ineffective Prompts (To Avoid)

These prompts **don't encourage tool usage**, leading to generic responses:

```
❌ "What does this codebase do?"
   Better: "Explore the /src directory and summarize what this codebase does"

❌ "Explain how authentication works"
   Better: "Find and read all authentication-related files, then explain the flow"

❌ "Is there a bug in the code?"
   Better: "Search for potential bugs by reading the error handling code in /src"

❌ "What design patterns are used?"
   Better: "Analyze the codebase structure (use ls -R) and identify design patterns"

❌ "Improve the README"
   Better: "Read README.md, analyze the project structure (glob *.rs), then suggest improvements"
```

---

### Key Principles for Effective Prompts

1. **Be Specific About Tools:**
   - ✅ "Use glob to find all TypeScript files"
   - ❌ "Find TypeScript files"

2. **Give Step-by-Step Instructions:**
   - ✅ "1. Read the file, 2. Analyze the code, 3. Suggest improvements"
   - ❌ "Improve this file"

3. **Mention Files/Directories Explicitly:**
   - ✅ "Explore the /src/llm directory"
   - ❌ "Look at the code"

4. **Encourage Exploration:**
   - ✅ "Start by listing all files, then read the main modules"
   - ❌ "Tell me about the codebase"

5. **Request Evidence:**
   - ✅ "Read the test file and show me the test cases"
   - ❌ "Are there tests?"

---

### Pro Tips for Local LLM Users

When using **local models** (LM Studio, Ollama), keep these tips in mind:

1. **Be Even More Explicit:**
   - Local models may need more guidance
   - Example: "Use the glob tool with pattern **/*.rs to find all Rust files, then use read_file to read src/main.rs"

2. **Break Down Complex Tasks:**
   - Instead of: "Analyze the entire codebase"
   - Use: "First, list all directories. Then, for each directory, show me the key files."

3. **Provide Context:**
   - Include relevant background in your prompt
   - Example: "This is a Rust project using Tokio for async. Analyze the async patterns used."

4. **Use Examples:**
   - Show the model what you want
   - Example: "Find TODO comments using: grep -r 'TODO' src/"

5. **Iterate:**
   - If the first response is generic, follow up with: "Please use the glob tool to actually explore the files"

---

### Sample Workflow Session

Here's a **real example** of an effective Crustly session:

```
User: I want to understand how this Rust project handles errors.
      Start by using glob to find all Rust files, then grep for
      error handling patterns like "Result<", "Error", "unwrap".

Crustly: [Uses glob tool to find 92 .rs files]
         [Uses grep to search for error patterns]
         [Reads key error handling modules]

         Based on my analysis of the codebase:
         1. Found 156 instances of Result<T, E>
         2. Error types defined in src/error.rs
         3. Custom error enum with 12 variants
         4. Comprehensive error handling in services layer

         [Provides detailed summary with file references]

User: Great! Now read src/error.rs and explain the error handling strategy.

Crustly: [Uses read_file tool]
         [Provides detailed explanation with code examples]

User: Are there any places where errors are unwrapped unsafely?

Crustly: [Uses grep to search for ".unwrap()" and ".expect()"]
         [Reports findings with line numbers and context]
```

**Why this works:**
- User explicitly requests tools (glob, grep, read_file)
- Crustly uses the tools proactively
- Follow-up questions build on previous exploration
- Results are concrete and evidence-based

---

