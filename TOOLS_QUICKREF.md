# Crustly Tools - Quick Reference

## 🔧 13 Tools Available

### 📁 File Operations
| Tool | Purpose | Approval |
|------|---------|----------|
| `read_file` | Read file contents | No |
| `write_file` | Create/overwrite files | **Yes** |
| `edit_file` | Modify existing files | **Yes** |
| `ls` | List directories | No |
| `glob` | Find files by pattern | No |
| `grep` | Search file contents | No |

### ⚡ Execution
| Tool | Purpose | Approval |
|------|---------|----------|
| `bash` | Run shell commands | **Yes** |
| `execute_code` | Run Python/JS/Rust/Bash code | **Yes** |

### 🌐 External Data
| Tool | Purpose | Approval |
|------|---------|----------|
| `web_search` | Search the internet | **Yes** |
| `http_request` | Call HTTP APIs | **Yes** |

### 🎯 Workflow
| Tool | Purpose | Approval |
|------|---------|----------|
| `task_manager` | Track tasks & dependencies | No |
| `session_context` | Store variables & facts | No |

### 📓 Specialized
| Tool | Purpose | Approval |
|------|---------|----------|
| `notebook_edit` | Edit Jupyter notebooks | **Yes** |

---

## 🚀 Common Patterns

### 1. Explore Codebase
```
1. ls (path: "src", recursive: true)
2. glob (pattern: "**/*.rs")
3. grep (pattern: "TODO", path: "src")
4. read_file (path: "src/main.rs")
```

### 2. Implement Feature
```
1. task_manager (operation: "create", description: "Add login")
2. web_search (query: "rust authentication JWT")
3. bash (command: "cargo add jsonwebtoken")
4. write_file (path: "src/auth.rs", content: "...")
5. bash (command: "cargo test")
6. task_manager (operation: "update", status: "completed")
```

### 3. Debug Issue
```
1. grep (pattern: "ERROR", path: "logs")
2. session_context (operation: "add_fact", fact: "Error occurs on startup")
3. read_file (path: "src/problematic_file.rs")
4. edit_file (operation: "replace", ...)
5. bash (command: "cargo run")
```

### 4. API Integration
```
1. session_context (operation: "set", key: "api_url", value: "...")
2. http_request (method: "GET", url: "https://api.example.com")
3. execute_code (language: "python3", code: "# test response parsing")
4. write_file (path: "src/api_client.rs")
```

---

## 💡 Pro Tips

- **Use glob > bash** for file searches (faster, safer)
- **Use grep > bash** for content searches (structured output)
- **Set backups: true** when editing (safety first)
- **Store common values** in session_context
- **Track complex work** with task_manager
- **Test snippets** with execute_code before writing files

---

## 🔒 Security

**Approval Required For:**
- File modifications (write_file, edit_file)
- Command execution (bash, execute_code)
- Network access (web_search, http_request)
- Notebook editing (notebook_edit)

**Always review before approving!**

---

See **TOOLS.md** for complete documentation.
