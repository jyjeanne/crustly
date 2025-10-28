# Crustly - User Guide

## Getting Started

### Installation

```bash
# Build from source
git clone https://github.com/your-repo/crustly
cd crustly
cargo build --release

# The binary will be at target/release/crustly
```

### First Run

1. **Set your API key:**
```bash
export ANTHROPIC_API_KEY="sk-ant-..."
```

2. **Initialize configuration (optional):**
```bash
crustly init
```

3. **Start chatting:**
```bash
crustly
```

## Commands

### Interactive Mode (Default)

Launch the TUI for interactive chatting:

```bash
# Default command
crustly

# Or explicitly
crustly chat

# With debug logging
crustly --debug chat
```

**Keyboard Shortcuts in TUI:**
- `Ctrl+Enter` - Send message
- `Escape` - Clear input/Cancel
- `Ctrl+N` - New session
- `Ctrl+L` - List sessions
- `Ctrl+H` - Show help
- `Ctrl+C` - Quit
- `Page Up/Down` - Scroll chat history
- `↑/↓` - Navigate sessions list

### Non-Interactive Mode

Run a single command and exit:

```bash
# Simple query
crustly run "What is Rust?"

# With JSON output
crustly run --format json "Explain async/await"

# With markdown output
crustly run --format markdown "Write a README for my project"
```

**Output Formats:**
- `text` (default) - Plain text with statistics
- `json` - Structured JSON output
- `markdown` - Formatted markdown

### Configuration Management

```bash
# Initialize config file
crustly init

# Show current configuration
crustly config

# Show config including API keys
crustly config --show-secrets

# Force reinitialize
crustly init --force
```

**Config File Locations:**
- System: `~/.config/crustly/config.toml` (Linux/Mac)
- System: `C:\Users\<user>\AppData\Roaming\crustly\config.toml` (Windows)
- Local: `./crustly.toml` (project-specific)

### Database Management

```bash
# Initialize database (automatic on first run)
crustly db init

# Show statistics
crustly db stats
```

## Configuration

### Environment Variables

```bash
# Required
export ANTHROPIC_API_KEY="sk-ant-..."

# Optional
export RUST_LOG="crustly=debug"  # Logging level
```

### Config File (config.toml)

```toml
[database]
# Database file location
path = "~/.local/share/crustly/crustly.db"

[logging]
# Log level: trace, debug, info, warn, error
level = "info"

[providers.anthropic]
enabled = true
default_model = "claude-3-5-sonnet-20240620"
# API key can be set here or via environment variable
# api_key = "sk-ant-..."
```

## Features

### Tool Execution

Crustly can execute tools to interact with your system:

- **read** - Read files
- **write** - Write/edit files
- **bash** - Execute shell commands

Example conversation:
```
You: Can you read the README file?
Assistant: [Uses read tool to read README.md]
          Here's the content of README.md...

You: Add a new section about installation
Assistant: [Uses write tool to update README.md]
          I've added the installation section!
```

### Session Management

- All conversations are saved in sessions
- Resume previous conversations
- View session history
- Automatic context management

### Cost Tracking

- Track token usage for each message
- Calculate cost per session
- View total costs

## Tips & Tricks

### For Developers

```bash
# Quick code explanation
crustly run "Explain this file" < src/main.rs

# Code review
crustly run "Review this for issues" < mycode.rs

# Generate tests
crustly run "Write unit tests for this" < mycode.rs
```

### For Writing

```bash
# Blog post
crustly run --format markdown "Write a blog post about Rust async"

# Documentation
crustly run "Generate API documentation for this code" < api.rs
```

### Debugging

```bash
# Enable debug logs
RUST_LOG=debug crustly

# Or use debug flag
crustly --debug chat
```

## Troubleshooting

### API Key Not Set

**Error:** `Anthropic API key not set`

**Solution:**
```bash
# Set environment variable
export ANTHROPIC_API_KEY="sk-ant-..."

# Or add to config file
crustly init
# Edit ~/.config/crustly/config.toml
```

### Database Errors

**Error:** `unable to open database file`

**Solution:**
```bash
# Initialize database
crustly db init

# Check permissions
ls -la ~/.local/share/crustly/
```

### Connection Errors

**Error:** `Failed to connect to API`

**Solution:**
- Check internet connection
- Verify API key is valid
- Check Anthropic service status

## Advanced Usage

### Custom Config Path

```bash
crustly --config /path/to/custom/config.toml chat
```

### Batch Processing

```bash
# Process multiple files
for file in src/*.rs; do
    crustly run "Explain this file" < "$file" > "$file.explanation.md"
done
```

### Integration with Other Tools

```bash
# Use with git
git diff | crustly run "Summarize these changes"

# Use with jq
crustly run --format json "List 5 programming languages" | jq '.content'
```

## FAQ

**Q: Where is my data stored?**
A: All data is stored locally in SQLite database at `~/.local/share/crustly/crustly.db`

**Q: Can I use multiple AI providers?**
A: Currently only Anthropic/Claude is supported. More providers coming soon.

**Q: How do I delete old sessions?**
A: Use the TUI (`Ctrl+L` for sessions) or directly manipulate the SQLite database.

**Q: What models are supported?**
A: All Claude 3 models:
- `claude-3-opus-20240229`
- `claude-3-sonnet-20240229`
- `claude-3-5-sonnet-20240620` (default)
- `claude-3-haiku-20240307`

**Q: Is my API key secure?**
A: API keys are stored in plaintext in the config file. Keep your config file permissions restricted (chmod 600).

## Support

- GitHub Issues: https://github.com/your-repo/crustly/issues
- Documentation: https://docs.crustly.dev
- Discord: https://discord.gg/crustly

## License

MIT License - See LICENSE file for details
