# Crustly Tools Documentation

This document provides a comprehensive guide to all 13 tools available in Crustly for AI-assisted development.

## Table of Contents

- [Phase 1: Essential File Operations (7 tools)](#phase-1-essential-file-operations)
- [Phase 2: Advanced Features (3 tools)](#phase-2-advanced-features)
- [Phase 3: Workflow & Integration (3 tools)](#phase-3-workflow--integration)

---

## Phase 1: Essential File Operations

### 1. Read File (`read_file`)

**Purpose:** Read file contents from the filesystem.

**Capabilities:** ReadFiles

**Requires Approval:** No

**Input Schema:**
```json
{
  "path": "path/to/file.txt",
  "start_line": 0,        // Optional: Line to start reading from (0-indexed)
  "end_line": 100         // Optional: Line to end reading at
}
```

**Example Use Cases:**
- Read configuration files
- Inspect source code
- Read log files
- View documentation

**Example:**
```json
{
  "path": "src/main.rs"
}
```

---

### 2. Write File (`write_file`)

**Purpose:** Create new files or overwrite existing files.

**Capabilities:** WriteFiles, SystemModification

**Requires Approval:** Yes

**Input Schema:**
```json
{
  "path": "path/to/file.txt",
  "content": "File content here",
  "create_backup": true    // Optional: Create .backup file (default: true)
}
```

**Example Use Cases:**
- Create new source files
- Generate configuration files
- Write documentation
- Create scripts

**Example:**
```json
{
  "path": "config.json",
  "content": "{\n  \"version\": \"1.0\"\n}",
  "create_backup": true
}
```

---

### 3. Edit File (`edit_file`)

**Purpose:** Modify existing files with various edit operations.

**Capabilities:** ReadFiles, WriteFiles, SystemModification

**Requires Approval:** Yes

**Operations:**
- `replace` - Find and replace text
- `replace_lines` - Replace specific line ranges
- `insert_line` - Insert text at a specific line
- `delete_lines` - Delete line ranges
- `regex_replace` - Regex-based find and replace

**Input Schema:**
```json
{
  "path": "file.txt",
  "operation": "replace",
  "old_text": "old value",
  "new_text": "new value",
  "create_backup": true
}
```

**Example Use Cases:**
- Refactor code
- Update configuration values
- Fix bugs
- Modify documentation

**Example - Replace text:**
```json
{
  "path": "src/config.rs",
  "operation": "replace",
  "old_text": "const VERSION: &str = \"1.0\";",
  "new_text": "const VERSION: &str = \"2.0\";",
  "create_backup": true
}
```

**Example - Replace lines:**
```json
{
  "path": "README.md",
  "operation": "replace_lines",
  "start_line": 5,
  "end_line": 7,
  "new_text": "## New Section\nUpdated content here",
  "create_backup": true
}
```

---

### 4. Bash (`bash`)

**Purpose:** Execute shell commands in the working directory.

**Capabilities:** ExecuteShell, SystemModification, WriteFiles

**Requires Approval:** Yes

**Input Schema:**
```json
{
  "command": "ls -la",
  "timeout_secs": 30       // Optional: Command timeout (default: 30, max: 300)
}
```

**Example Use Cases:**
- Run build commands
- Execute tests
- Git operations
- Package management (npm, cargo, pip)
- System administration tasks

**Example:**
```json
{
  "command": "cargo test --lib",
  "timeout_secs": 60
}
```

---

### 5. List Directory (`ls`)

**Purpose:** List files and directories with optional details.

**Capabilities:** ReadFiles

**Requires Approval:** No

**Input Schema:**
```json
{
  "path": ".",             // Optional: Directory path (default: current)
  "detailed": false,       // Optional: Show size, permissions, modified time
  "recursive": false,      // Optional: List subdirectories recursively
  "show_hidden": false     // Optional: Include hidden files (default: false)
}
```

**Example Use Cases:**
- Explore directory structure
- Find files
- Verify file existence
- Check directory contents

**Example:**
```json
{
  "path": "src",
  "detailed": true,
  "recursive": false
}
```

---

### 6. Glob Pattern Matching (`glob`)

**Purpose:** Find files matching glob patterns (wildcards).

**Capabilities:** ReadFiles

**Requires Approval:** No

**Input Schema:**
```json
{
  "pattern": "**/*.rs",     // Glob pattern
  "base_path": ".",         // Optional: Base directory (default: current)
  "include_hidden": false,  // Optional: Include hidden files
  "limit": 100              // Optional: Max results (default: 100)
}
```

**Pattern Examples:**
- `*.rs` - All Rust files in current directory
- `**/*.js` - All JavaScript files recursively
- `src/**/*.test.ts` - All test TypeScript files in src
- `*.{md,txt}` - All markdown and text files

**Example Use Cases:**
- Find all files of a specific type
- Locate test files
- Search for configuration files
- Build file lists for processing

**Example:**
```json
{
  "pattern": "**/*.toml",
  "base_path": ".",
  "limit": 50
}
```

---

### 7. Grep (Content Search) (`grep`)

**Purpose:** Search for patterns in file contents.

**Capabilities:** ReadFiles

**Requires Approval:** No

**Input Schema:**
```json
{
  "pattern": "TODO",        // Search pattern (regex supported)
  "path": ".",              // Optional: File or directory path
  "file_pattern": "*.rs",   // Optional: Filter by file pattern
  "case_insensitive": false, // Optional: Case-insensitive search
  "show_line_numbers": true, // Optional: Show line numbers (default: true)
  "context_lines": 2,       // Optional: Show N lines around match
  "limit": 100              // Optional: Max matches (default: 100)
}
```

**Example Use Cases:**
- Find TODOs and FIXMEs
- Search for function definitions
- Locate error messages
- Find API usage
- Search documentation

**Example:**
```json
{
  "pattern": "fn main\\(",
  "path": "src",
  "file_pattern": "*.rs",
  "show_line_numbers": true,
  "context_lines": 3
}
```

---

## Phase 2: Advanced Features

### 8. Web Search (`web_search`)

**Purpose:** Search the internet for real-time information using DuckDuckGo.

**Capabilities:** Network

**Requires Approval:** Yes

**Input Schema:**
```json
{
  "query": "rust async programming",
  "max_results": 10         // Optional: Max results (default: 10, max: 20)
}
```

**Example Use Cases:**
- Find documentation
- Research APIs and libraries
- Get current information
- Find code examples
- Check latest versions

**Example:**
```json
{
  "query": "tokio runtime configuration",
  "max_results": 5
}
```

**Note:** Uses DuckDuckGo API, no API key required.

---

### 9. Code Execution (`execute_code`)

**Purpose:** Execute code in various languages within a sandboxed environment.

**Capabilities:** ExecuteShell, SystemModification, WriteFiles

**Requires Approval:** Yes

**Supported Languages:**
- Python (`python`, `python3`)
- JavaScript (`javascript`, `js`, `node`)
- Rust (`rust`)
- Bash/Shell (`sh`, `bash`)

**Input Schema:**
```json
{
  "language": "python3",
  "code": "print('Hello World')",
  "args": [],               // Optional: Additional interpreter args
  "timeout_secs": 30        // Optional: Execution timeout (default: 30, max: 60)
}
```

**Example Use Cases:**
- Test code snippets
- Validate algorithms
- Run quick calculations
- Test regular expressions
- Prototype solutions

**Example - Python:**
```json
{
  "language": "python3",
  "code": "import json\ndata = {'key': 'value'}\nprint(json.dumps(data, indent=2))",
  "timeout_secs": 10
}
```

**Example - Rust:**
```json
{
  "language": "rust",
  "code": "fn main() {\n    println!(\"{}\", 2 + 2);\n}",
  "timeout_secs": 30
}
```

---

### 10. Notebook Edit (`notebook_edit`)

**Purpose:** Edit Jupyter notebook files (.ipynb) cell by cell.

**Capabilities:** ReadFiles, WriteFiles, SystemModification

**Requires Approval:** Yes

**Operations:**
- `add_cell` - Add a new cell
- `edit_cell` - Modify existing cell
- `delete_cell` - Remove a cell
- `clear_outputs` - Clear all cell outputs

**Input Schema:**
```json
{
  "path": "notebook.ipynb",
  "operation": "add_cell",
  "cell_type": "code",       // code, markdown, or raw
  "source": ["print('hello')"],
  "position": 0,             // Optional: Insert position
  "index": 0,                // For edit_cell, delete_cell operations
  "create_backup": true      // Optional: Backup before editing (default: true)
}
```

**Example Use Cases:**
- Modify data analysis notebooks
- Add documentation cells
- Update code cells
- Clean notebook outputs

**Example - Add code cell:**
```json
{
  "path": "analysis.ipynb",
  "operation": "add_cell",
  "cell_type": "code",
  "source": ["import pandas as pd", "df = pd.read_csv('data.csv')"],
  "position": 1
}
```

**Example - Clear outputs:**
```json
{
  "path": "analysis.ipynb",
  "operation": "clear_outputs",
  "create_backup": true
}
```

---

## Phase 3: Workflow & Integration

### 11. Task Manager (`task_manager`)

**Purpose:** Organize and track multi-step workflows with priorities and dependencies.

**Capabilities:** ReadFiles, WriteFiles

**Requires Approval:** No

**Operations:**
- `create` - Create a new task
- `update` - Update task status, description, priority
- `list` - List/filter tasks
- `get` - Get detailed task info
- `delete` - Delete a task
- `clear_completed` - Remove completed tasks

**Task Properties:**
- **Status:** pending, in_progress, completed, blocked, cancelled
- **Priority:** low, medium, high, critical
- **Dependencies:** List of task IDs that must be completed first
- **Tags:** Custom categorization

**Storage:** `.crustly/tasks.json` in working directory

**Input Schema:**

**Create task:**
```json
{
  "operation": "create",
  "description": "Implement user authentication",
  "priority": "high",
  "tags": ["backend", "security"],
  "dependencies": []
}
```

**Update task:**
```json
{
  "operation": "update",
  "task_id": "550e8400-e29b-41d4-a716-446655440000",
  "status": "in_progress",
  "priority": "critical",
  "blocked_reason": "Waiting for API keys"
}
```

**List tasks:**
```json
{
  "operation": "list",
  "status": "in_progress",    // Optional: Filter by status
  "priority": "high",          // Optional: Filter by priority
  "show_completed": false      // Optional: Include completed (default: false)
}
```

**Example Use Cases:**
- Break down large projects
- Track implementation progress
- Manage dependencies between tasks
- Prioritize work
- Record blockers

**Workflow Example:**
1. Create tasks for a feature
2. Set dependencies (e.g., "implement API" depends on "design schema")
3. Update status as work progresses
4. Mark tasks completed
5. Clear completed tasks when done

---

### 12. Session Context (`session_context`)

**Purpose:** Store session variables, track facts and decisions, maintain conversation state.

**Capabilities:** ReadFiles, WriteFiles

**Requires Approval:** No

**Operations:**
- `set` - Store a variable (any JSON type)
- `get` - Retrieve a variable
- `delete` - Remove a variable
- `list` - List all variables (with optional tag filter)
- `add_fact` - Record an important fact
- `add_decision` - Record a key decision
- `summary` - Generate session summary
- `clear` - Clear all context (requires confirm=true)

**Storage:** `.crustly/context_{session_id}.json` in working directory

**Input Schema:**

**Set variable:**
```json
{
  "operation": "set",
  "key": "api_endpoint",
  "value": "https://api.example.com/v1",
  "description": "Production API endpoint",
  "tags": ["config", "api"]
}
```

**Get variable:**
```json
{
  "operation": "get",
  "key": "api_endpoint"
}
```

**Add fact:**
```json
{
  "operation": "add_fact",
  "fact": "User prefers TypeScript over JavaScript for this project"
}
```

**Add decision:**
```json
{
  "operation": "add_decision",
  "decision": "Using PostgreSQL instead of MongoDB for better transaction support"
}
```

**List variables:**
```json
{
  "operation": "list",
  "tag": "api"              // Optional: Filter by tag
}
```

**Get summary:**
```json
{
  "operation": "summary"
}
```

**Example Use Cases:**
- Store configuration values
- Remember user preferences
- Track important decisions made during development
- Record facts discovered during code analysis
- Maintain state across long conversations

**Example Workflow:**
1. Store API keys and endpoints as variables
2. Record architectural decisions
3. Track discovered bugs or issues as facts
4. Generate summary at end of session

---

### 13. HTTP Request (`http_request`)

**Purpose:** Make HTTP requests to external APIs and web services.

**Capabilities:** Network

**Requires Approval:** Yes

**Supported Methods:** GET, POST, PUT, PATCH, DELETE, HEAD, OPTIONS

**Input Schema:**
```json
{
  "method": "GET",
  "url": "https://api.github.com/repos/user/repo",
  "headers": {              // Optional: Custom headers
    "Authorization": "Bearer token",
    "Accept": "application/json"
  },
  "query": {                // Optional: Query parameters
    "per_page": "10",
    "page": "1"
  },
  "body": {                 // Optional: Request body (POST/PUT/PATCH only)
    "name": "value"
  },
  "timeout_secs": 30,       // Optional: Request timeout (default: 30, max: 120)
  "follow_redirects": true  // Optional: Follow redirects (default: true)
}
```

**Example Use Cases:**
- Interact with GitHub API
- Send Slack notifications
- Query Jira issues
- Access database HTTP APIs
- Trigger webhooks
- Fetch external data

**Example - GitHub API:**
```json
{
  "method": "GET",
  "url": "https://api.github.com/repos/rust-lang/rust/pulls",
  "headers": {
    "Accept": "application/vnd.github.v3+json",
    "User-Agent": "Crustly"
  },
  "query": {
    "state": "open",
    "per_page": "5"
  },
  "timeout_secs": 30
}
```

**Example - POST request:**
```json
{
  "method": "POST",
  "url": "https://api.example.com/data",
  "headers": {
    "Content-Type": "application/json",
    "Authorization": "Bearer YOUR_TOKEN"
  },
  "body": {
    "name": "test",
    "value": 42
  },
  "timeout_secs": 30
}
```

**Example - Slack Webhook:**
```json
{
  "method": "POST",
  "url": "https://hooks.slack.com/services/YOUR/WEBHOOK/URL",
  "body": {
    "text": "Deployment completed successfully!"
  }
}
```

---

## Tool Categories Summary

### File Operations (7 tools)
- **read_file** - Read file contents
- **write_file** - Create/overwrite files
- **edit_file** - Modify existing files
- **ls** - List directories
- **glob** - Pattern-based file search
- **grep** - Content search

### Execution & Commands (2 tools)
- **bash** - Execute shell commands
- **execute_code** - Run code in multiple languages

### External Data (2 tools)
- **web_search** - Internet search
- **http_request** - HTTP API calls

### Specialized (2 tools)
- **notebook_edit** - Jupyter notebook editing
- **task_manager** - Workflow orchestration

### Context Management (2 tools)
- **session_context** - Store variables and track decisions
- **task_manager** - Task tracking and dependencies

---

## Best Practices

### Security
- **Approval Required:** Some tools require explicit approval before execution:
  - write_file, edit_file (modify filesystem)
  - bash, execute_code (execute commands)
  - web_search, http_request (network access)
  - notebook_edit (modify files)

- **Review changes:** Always review what will be executed/modified before approving

### Workflow
1. **Exploration:** Use ls, glob, grep, read_file to understand the codebase
2. **Planning:** Use task_manager to break down work
3. **Implementation:** Use write_file, edit_file, bash to make changes
4. **Testing:** Use execute_code to test snippets
5. **Context:** Use session_context to track decisions and state
6. **Integration:** Use http_request to interact with external services

### File Management
- **Backups:** Most file modification tools create .backup files by default
- **Use edit_file:** Prefer edit_file over write_file for existing files
- **Verify paths:** Use ls or glob to verify file locations before operations

### Task Management
- **Dependencies:** Set task dependencies to ensure correct order
- **Priorities:** Use priorities to focus on critical work
- **Tags:** Use tags to categorize related tasks
- **Status updates:** Keep task status current

### Session Context
- **Variables:** Store frequently used values (API keys, endpoints, paths)
- **Facts:** Record important discoveries
- **Decisions:** Document architectural choices
- **Summary:** Generate summaries before major changes

---

## Advanced Patterns

### Multi-step Workflows

**Pattern 1: Code Refactoring**
1. `grep` - Find all occurrences of pattern
2. `task_manager` (create) - Create tasks for each file to modify
3. `read_file` - Read each file
4. `edit_file` - Apply changes
5. `bash` - Run tests
6. `task_manager` (update) - Mark tasks completed

**Pattern 2: API Integration**
1. `web_search` - Research API documentation
2. `session_context` (set) - Store API endpoint and keys
3. `http_request` - Test API calls
4. `write_file` - Create integration module
5. `execute_code` - Test the integration

**Pattern 3: Bug Investigation**
1. `grep` - Find error messages in logs
2. `session_context` (add_fact) - Record findings
3. `read_file` - Read relevant source files
4. `bash` - Run debugging commands
5. `session_context` (add_decision) - Document fix approach
6. `edit_file` - Apply fix
7. `bash` - Verify fix with tests

### Combining Tools

**Example: Feature Implementation**
```
1. task_manager (create) - "Implement user login"
2. session_context (add_fact) - "Using JWT for authentication"
3. web_search - Research JWT libraries
4. session_context (set) - Store chosen library name
5. bash - "cargo add jsonwebtoken"
6. write_file - Create auth module
7. execute_code - Test JWT generation
8. grep - Find all files needing auth
9. edit_file - Add auth to endpoints
10. bash - "cargo test"
11. task_manager (update) - Mark completed
```

---

## Troubleshooting

### Common Issues

**Permission Denied:**
- Check file permissions with `ls` (detailed=true)
- Use `bash` with appropriate permissions

**File Not Found:**
- Use `ls` or `glob` to verify file existence
- Check working directory with `bash` "pwd"

**Timeout Errors:**
- Increase timeout_secs parameter
- Break large operations into smaller steps

**Network Failures:**
- Check internet connectivity
- Verify URLs and API endpoints
- Review authentication headers

### Performance Tips

- **Use glob instead of bash ls** for file listing
- **Use grep instead of bash grep** for content search
- **Set limits** on grep and glob to avoid overwhelming results
- **Batch operations** when possible
- **Use task_manager** to track long-running workflows

---

## Tool Coverage Matrix

| Capability | Tools |
|------------|-------|
| Read Files | read_file, ls, glob, grep, edit_file |
| Write Files | write_file, edit_file, bash, execute_code |
| Execute Shell | bash, execute_code |
| Network | web_search, http_request |
| System Modification | write_file, edit_file, bash, execute_code, notebook_edit |

---

## Future Tools (Proposed)

Based on the original requirements, potential future additions:

- **Agent/Sub-agent** - Spawn specialized helper agents
- **MCP Connector** - Advanced external system integration
- **Files API** - Batch file operations
- **Text Editor** - Interactive editing capabilities
- **Slash Commands** - Custom command definitions
- **Plugin System** - Extensible tool framework
- **Prompt Caching** - Optimize repeated operations

---

## Getting Help

For more information:
- Run Crustly with `-h` or `--help` for CLI options
- Check the main README.md for installation and setup
- Review source code in `src/llm/tools/` for implementation details
- Use `session_context` (summary) to review your session state

---

*Last Updated: Phase 3 Implementation*
*Total Tools: 13*
