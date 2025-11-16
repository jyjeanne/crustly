# Debug and Logging System

## Overview

Crustly's improved logging system provides conditional file logging that only creates log files when debug mode is explicitly enabled. This keeps your workspace clean while providing detailed debugging information when needed.

## Key Features

- **Conditional File Logging**: Log files only created in debug mode
- **Local `.crustly/` Folder**: Logs stored in current working directory
- **Automatic Cleanup**: Old logs automatically removed (7-day retention)
- **Log Management CLI**: Commands to view, clean, and manage logs
- **TUI-Safe**: Silent logging by default to avoid interfering with TUI

## Usage

### Enabling Debug Mode

Add the `-d` or `--debug` flag to enable debug logging:

```bash
# Start with debug mode enabled
crustly -d

# Or with full flag name
crustly --debug

# Debug mode with any command
crustly -d run "analyze this code"
```

When debug mode is enabled:
- Log files created in `.crustly/logs/` folder
- Daily rolling log rotation
- DEBUG level logging (more verbose)
- Thread IDs, file names, and line numbers included

### Log Management Commands

```bash
# Check logging status
crustly logs status

# View recent log entries (last 50 lines by default)
crustly logs view

# View more lines
crustly logs view -l 100

# Clean up old logs (default: older than 7 days)
crustly logs clean

# Clean logs older than 3 days
crustly logs clean -d 3

# Open log directory in file manager
crustly logs open
```

## Log File Location

Logs are stored in your current working directory:

```
./
├── .crustly/
│   ├── .gitignore      # Auto-generated to ignore log files
│   └── logs/
│       ├── crustly.2024-01-15     # Daily log files
│       ├── crustly.2024-01-16
│       └── crustly.2024-01-17
└── ... your project files
```

The `.crustly/` folder contains a `.gitignore` file that automatically ignores all runtime files, so logs won't be committed to your repository.

## Log Levels

| Mode | Log Level | File Output | Console Output | Purpose |
|------|-----------|-------------|----------------|---------|
| Normal | WARN | ❌ None | Silent | Production use, no clutter |
| Debug (`-d`) | DEBUG | ✅ `.crustly/logs/` | Silent | Development, troubleshooting |

### Normal Mode (Default)

- **No log files created**
- Minimal logging (warnings and errors only)
- Silent output to avoid TUI interference
- Clean workspace with no runtime artifacts

### Debug Mode

- **Log files created** in `.crustly/logs/`
- Verbose DEBUG level logging
- Detailed information for troubleshooting:
  - Tool execution traces
  - Provider API interactions
  - Database queries
  - UI event handling
  - Error stack traces

## Log File Contents

When debug mode is enabled, log files contain:

```log
2024-01-15T10:30:45.123Z INFO crustly::logging src/logging.rs:150 🚀 Crustly debug mode enabled
2024-01-15T10:30:45.124Z INFO crustly::logging src/logging.rs:151 📁 Log directory: /home/user/project/.crustly/logs
2024-01-15T10:30:45.125Z INFO crustly::cli src/cli/mod.rs:465 Connecting to database
2024-01-15T10:30:45.200Z DEBUG crustly::llm::agent::service src/llm/agent/service.rs:230 Starting tool execution loop
2024-01-15T10:30:45.250Z INFO crustly::tui::prompt_analyzer src/tui/prompt_analyzer.rs:125 🔍 Detected PLAN intent in prompt
```

Each log entry includes:
- **Timestamp** (ISO 8601 format with milliseconds)
- **Log level** (DEBUG, INFO, WARN, ERROR)
- **Target** (module path)
- **Source location** (file:line)
- **Thread ID** (for async debugging)
- **Message**

## Automatic Maintenance

### Log Rotation

- Daily rolling log files
- Each day creates a new log file
- Named: `crustly.YYYY-MM-DD`

### Automatic Cleanup

When running in debug mode:
- Old logs (>7 days) are automatically cleaned
- Notification shown if logs were removed:
  ```
  🧹 Cleaned up 3 old log file(s)
  ```

### Manual Cleanup

```bash
# Remove logs older than 7 days (default)
crustly logs clean

# Remove logs older than 3 days
crustly logs clean -d 3

# Remove logs older than 1 day (aggressive)
crustly logs clean -d 1
```

## Environment Variables

Additional logging control via environment variables:

```bash
# Override log level (combine with -d flag)
RUST_LOG=trace crustly -d

# Enable specific module logging
RUST_LOG=crustly::llm=debug crustly -d

# Disable noisy modules
RUST_LOG=crustly=debug,sqlx=warn,hyper=warn crustly -d
```

## Troubleshooting

### Common Issues

**Q: I don't see any log files after running Crustly**

A: Make sure you're using the `-d` flag:
```bash
crustly -d
```

**Q: Logs are too verbose**

A: Adjust the log level:
```bash
RUST_LOG=crustly=info crustly -d
```

**Q: Logs are cluttering my repository**

A: The `.crustly/` folder includes a `.gitignore` that ignores all files. If it's still showing up:
```bash
# Add to your .gitignore
.crustly/
```

**Q: Log files are getting too large**

A: Clean up old logs more aggressively:
```bash
crustly logs clean -d 1
```

### Viewing Logs

```bash
# Quick view of recent logs
crustly logs view

# View more history
crustly logs view -l 200

# Search for specific errors
crustly logs view -l 1000 | grep "ERROR"

# Watch logs in real-time (if you have another terminal)
tail -f .crustly/logs/crustly.$(date +%Y-%m-%d)
```

## API Usage (for developers)

```rust
use crustly::logging::{LogConfig, init_logging};

// Custom logging configuration
let config = LogConfig::new()
    .with_debug_mode(true)
    .with_log_level(tracing::Level::TRACE)
    .with_log_prefix("myapp".to_string());

let _guard = init_logging(config)?;

// Or use the convenience function
let _guard = crustly::logging::setup_from_cli(debug_flag)?;
```

## Benefits

### Before (Always logging)
- ❌ Log files created on every run
- ❌ Clutter in system directories
- ❌ Disk space consumed unnecessarily
- ❌ Privacy concerns (logs contain user data)

### After (Conditional logging)
- ✅ Clean workspace by default
- ✅ Logs only when explicitly needed
- ✅ Local to project (`.crustly/` folder)
- ✅ Auto-cleanup prevents bloat
- ✅ Git-ignored automatically
- ✅ Easy to manage and view

## Best Practices

1. **Normal Development**: Don't use `-d` flag unless debugging
2. **Issue Investigation**: Enable `-d` to capture detailed logs
3. **Before Reporting Issues**: Capture logs with `-d` and include relevant sections
4. **Regular Cleanup**: Run `crustly logs clean` periodically
5. **Security**: Don't share full log files without reviewing for sensitive data

## Implementation Details

- **Module**: `src/logging.rs`
- **Integration**: `src/main.rs`, `src/cli/mod.rs`
- **Dependencies**: `tracing`, `tracing-subscriber`, `tracing-appender`
- **Tests**: `cargo test --lib logging` (5 tests)

The logging system uses Rust's `tracing` ecosystem for high-performance, structured logging with minimal overhead when disabled.
