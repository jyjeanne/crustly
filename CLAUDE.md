# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Crustly is a high-performance terminal AI assistant for software development written in Rust. It's a Rust reimplementation of Crush with focus on performance, memory efficiency, and reduced resource consumption.

**Key Technologies:**
- **Language:** Rust 1.75+
- **Async Runtime:** Tokio
- **TUI Framework:** Ratatui with Crossterm
- **Database:** SQLite with sqlx (WAL mode for concurrent reads)
- **LLM Providers:** Anthropic, OpenAI, AWS Bedrock, Ollama (via crabrace registry)
- **Tools Framework:** 21+ built-in tools for file operations, shell execution, web access, and agent orchestration

## Common Development Commands

### Building and Running
```bash
# Development build
cargo build

# Release build with full optimizations
cargo build --release

# Run in interactive TUI mode
cargo run

# Run with debug logging (creates log files in .crustly/logs/)
cargo run -- -d
cargo run -- --debug

# Non-interactive mode (single command)
cargo run -- run "What is Rust?"
```

### Testing
```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run tests with output
cargo test -- --nocapture

# Run tests for specific module
cargo test --package crustly --lib -- llm::tools::bash
```

### Code Quality
```bash
# Format code
cargo fmt

# Check code without building
cargo check

# Run linter
cargo clippy

# Run clippy with all warnings
cargo clippy -- -W clippy::all
```

### Database Operations
```bash
# Initialize database
cargo run -- db init

# Show database statistics
cargo run -- db stats
```

### Feature Flags
```bash
# Build with all LLM providers
cargo build --features all-llm

# Build with specific providers
cargo build --features openai
cargo build --features aws-bedrock
cargo build --features ollama

# Profiling (Unix only - no-op on Windows)
cargo build --features profiling
```

## Architecture Overview

### Core Module Structure

The codebase is organized around several key architectural layers:

**Domain Layer (`src/plan/`):**
- `Plan` - Shared domain model for structured task decomposition
- Lives at crate root to avoid circular dependencies (see ADR 0004)
- Used by TUI, database, plan tool, and plan service

**Application Layer (`src/app/`):**
- Application state management and orchestration
- Coordinates between TUI, LLM services, and database

**Service Layer (`src/services/`):**
- `message.rs` - Message handling and streaming
- `session.rs` - Session lifecycle management
- `plan.rs` - Plan execution and task management
- `file.rs` - File operations and workspace management

**LLM Layer (`src/llm/`):**
- `provider/` - LLM provider implementations (Anthropic, OpenAI, Bedrock, Ollama)
- `agent/` - Agent service orchestrating LLM interactions
- `tools/` - Tool execution framework (21+ tools)
- `prompt/` - Prompt templates and management

**Tools Framework (`src/llm/tools/`):**
Organized in 4 phases:
- **Phase 1:** Essential file operations (read, write, edit, glob, grep, ls, bash)
- **Phase 2:** Advanced features (code_exec, web_search, notebook, doc_parser)
- **Phase 3:** Workflow integration (http, task, context, plan_tool)
- **Phase 4:** Claw Code parity (agent, ask_user, skill, todo_write, web_fetch, powershell)

Key tool components:
- `trait.rs` - `Tool` trait and `ToolExecutionContext`
- `registry.rs` - Tool registration and discovery
- `cache.rs` - Tool result caching (read-only tools only, configurable TTL)
- `sandbox.rs` - Permission policies and security enforcement

**Database Layer (`src/db/`):**
- `models.rs` - Database entity models
- `repository/` - Data access layer with retry logic
  - `session.rs`, `message.rs`, `plan.rs`, `file.rs`, `memory.rs`, `compaction.rs`
- SQLite with WAL mode for concurrent reads during writes

**TUI Layer (`src/tui/`):**
- `app.rs` - Main TUI application loop
- `pages/` - Different UI screens (chat, help, sessions, etc.)
- `components/` - Reusable UI widgets
- `markdown.rs` - Markdown rendering with syntax highlighting
- `highlight.rs` - Code syntax highlighting (100+ languages via syntect)
- `ollama_download.rs` - Model download dialog for Ollama
- `prompt_analyzer.rs` - Smart model routing (fast/balanced/powerful)

**Configuration (`src/config/`):**
- `mod.rs` - Main config structure and loading
- `secrets.rs` - Secure API key management (OS keyring + env vars)
- `crabrace.rs` - Provider registry integration
- `update.rs` - Runtime provider configuration updates

**MCP Integration (`src/mcp/`):**
- Model Context Protocol server integration
- External tool server registration and management

### Key Architectural Patterns

**Tool Execution Flow:**
1. LLM requests tool use via structured output
2. `ToolRegistry` dispatches to appropriate tool implementation
3. `PermissionPolicy` checks security constraints (sandbox, path boundaries, bash allowlist)
4. Tool executes with `ToolExecutionContext` (may prompt user for approval)
5. Result cached (if read-only) and returned to LLM
6. Parallel dispatch via `join_all` for independent tools (≥40% faster)

**Provider Failover:**
- Primary provider fails → automatic retry on secondary provider
- Logged with `[FAILOVER]` tag
- Configured via `crabrace` registry

**Context Management:**
- At 80% capacity → automatic context compaction
- Last 10 turns preserved verbatim, older turns summarized
- `CompactionRecord` written to SQLite before modification (atomic operation)
- Episodic memory injected into new sessions within token budget

**Streaming Architecture:**
- Token-by-token rendering in TUI with live `[streaming]` indicator
- Thinking blocks filtered from live view (DeepSeek-R1, QwQ-32B)
- Three reasoning sources: Anthropic extended thinking, DeepSeek API, Ollama tag extraction
- Press `t` to expand/collapse thinking panels

**Security Model:**
- Path boundary enforcement (symlinks and `../../` escapes blocked)
- Bash command allowlist (`security.allow_bash` in config)
- Composable `AndPolicy` for chaining permission rules
- Three approval modes: Interactive (default), AutoPlan (low-risk auto), FullAuto (all auto)

## Database Schema

The application uses SQLite with the following key tables:
- `sessions` - Chat session metadata
- `messages` - Individual messages with token/cost tracking
- `plans` - Structured task plans with approval workflow
- `plan_tasks` - Individual tasks within plans
- `files` - Workspace file tracking
- `compaction_records` - Context compaction audit trail
- `memory_summaries` - Episodic memory across sessions

## Testing Strategy

**Unit Tests:**
- Located alongside implementation files
- Use `rstest` for parameterized tests
- Use `proptest` for property-based testing
- Use `mockall` for mocking dependencies

**Integration Tests:**
- Security tests: `src/llm/tools/plan_tool_security_tests.rs`
- Database tests with `tempfile` for isolated test databases

**Benchmarks:**
- `benches/database.rs` - Database performance
- `benches/parallel_tool_dispatch.rs` - Tool execution concurrency
- Use `criterion` harness (HTML reports generated)

## Knowledge Graph

This project has a knowledge graph at `docs/graph/` (graph.json, GRAPH_REPORT.md).

**Usage:**
- For codebase questions, first run `GRAPHIFY_OUT=docs/graph graphify query "<question>"`
- Use `graphify path "<A>" "<B>" --graph docs/graph/graph.json` for relationships
- Use `graphify explain "<concept>" --graph docs/graph/graph.json` for focused concepts
- Read `docs/graph/GRAPH_REPORT.md` for architecture-level questions (god nodes, cross-community connections)

**Maintenance:**
- Post-commit hook auto-refreshes graph for Rust code changes (AST-only, no LLM cost)
- Manual refresh for docs/images: `/graphify --update .` (requires LLM)
- See `docs/graph/README.md` and `scripts/setup-graphify-hooks.sh` for setup

## Architecture Validation Scripts

Several scripts in `scripts/` enforce architectural integrity:

```bash
# Generate architecture documentation
./scripts/generate-architecture-docs.sh

# Validate module dependencies
./scripts/validate-architecture.sh

# Check for architecture drift
./scripts/check-architecture-drift.sh

# Analyze module coupling
./scripts/analyze-module-coupling.sh

# Generate ctags symbol index
./scripts/generate-ctags.sh
```

## Important Patterns to Follow

**Error Handling:**
- Use `anyhow::Result` for application errors
- Use `thiserror` for custom error types
- Database operations use retry logic via `retry_db_*` functions

**Logging:**
- Debug mode (`-d` flag) creates log files in `.crustly/logs/`
- Production mode is silent (no log files)
- Use `tracing` macros: `tracing::info!`, `tracing::debug!`, `tracing::error!`
- Log cleanup keeps last 7 days automatically

**Configuration:**
- Config loaded from `~/.config/crustly/config.toml` (Linux/Mac) or `%APPDATA%\crustly\config.toml` (Windows)
- Secrets prefer OS keyring over environment variables
- Use `cargo run -- init` to generate default config

**Tool Development:**
- Implement `Tool` trait from `src/llm/tools/trait.rs`
- Register in `ToolRegistry` (`src/llm/tools/registry.rs`)
- Add to appropriate phase comment block in `src/llm/tools/mod.rs`
- Read-only tools should support caching
- Dangerous operations must integrate with permission system

**Plan Mode:**
- Plan domain model lives at `src/plan/mod.rs` (crate root, not under `tui/`)
- This prevents circular dependencies between TUI, database, and tools
- See ADR 0004 in `docs/architecture/decisions/` for rationale
