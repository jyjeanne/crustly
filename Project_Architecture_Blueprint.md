# Crustly - Project Architecture Blueprint

> **Generated**: 2025-01-15  
> **Project**: crustly (jyjeanne/crustly)  
> **Version**: 0.4.1  
> **Language**: Rust 2021 Edition  
> **License**: FSL-1.1-MIT  

---

## Table of Contents

1. [Architecture Detection and Analysis](#1-architecture-detection-and-analysis)
2. [Architectural Overview](#2-architectural-overview)
3. [Architecture Visualization](#3-architecture-visualization)
4. [Core Architectural Components](#4-core-architectural-components)
5. [Architectural Layers and Dependencies](#5-architectural-layers-and-dependencies)
6. [Data Architecture](#6-data-architecture)
7. [Cross-Cutting Concerns](#7-cross-cutting-concerns)
8. [Extension Patterns](#8-extension-patterns)
9. [Blueprint for New Development](#9-blueprint-for-new-development)

---

## 1. Architecture Detection and Analysis

### 1.1 Technology Stack

**Core Stack:**
- **Language**: Rust 2021 Edition (MSRV: 1.75)
- **Build**: Cargo
- **Async Runtime**: Tokio 1.35 (full features)
- **CLI Framework**: Clap v4.5 (derive, env, cargo)
- **TUI Framework**: Ratatui 0.26 (all-widgets) + Crossterm 0.27
- **Database**: SQLite via SQLx 0.7 (runtime-tokio-native-tls)
- **Serialization**: Serde + serde_json
- **HTTP**: Reqwest 0.11 (json, native-tls, stream)
- **LSP**: tower-lsp 0.20 + lsp-types 0.95
- **Provider Registry**: Crabrace 0.1.0

**LLM Provider Support:**
- Anthropic, OpenAI (async-openai), AWS Bedrock, Azure, VertexAI, Qwen/DashScope, Google Gemini

**Features:**
- Optional: `profiling` (pprof on Unix), `openai`, `aws-bedrock`, `all-llm`

### 1.2 Architectural Patterns

- **Layered Architecture**: Clear separation between CLI, TUI, Services, DB, LLM layers
- **Dependency Injection**: ServiceContext pattern for shared resources
- **Provider Pattern**: Unified abstraction over multiple LLM providers
- **Repository Pattern**: Database access abstraction
- **Event-Driven**: TUI events, LLM streaming
- **Plugin/Tool System**: Extensible tool registry
- **MCP Support**: Model Context Protocol integration
- **LSP Integration**: Language Server Protocol for code intelligence

---

## 2. Architectural Overview

### 2.1 Approach

Crustly employs a **clean layered architecture** with **14 modules** organized in a single Rust crate:

1. **CLI Layer** (`cli/`): Argument parsing, command dispatch
2. **TUI Layer** (`tui/`): Ratatui-based terminal interface
3. **Services Layer** (`services/`): Business logic orchestration
4. **LLM Layer** (`llm/`): Provider abstraction, agent services, tools
5. **Database Layer** (`db/`): SQLite persistence, models, repositories
6. **Config Layer** (`config/`): Settings, secrets, provider management
7. **Support Modules**: `app/`, `error/`, `events/`, `logging/`, `macros/`, `mcp/`, `message/`, `sync/`, `utils/`

### 2.2 Guiding Principles

- **Performance**: Native Rust, async I/O, optimized binaries
- **Privacy**: Local-first (SQLite), local LLM support (LM Studio, Ollama)
- **Extensibility**: Provider abstraction, tool system, MCP integration
- **Terminal-Native**: Keyboard shortcuts, no context switching
- **Safety**: Memory-safe, proper error handling
- **Observability**: Tracing, logging, cost tracking

### 2.3 Hybrid Patterns

| Pattern | Implementation | Purpose |
|---------|---------------|---------|
| Layered | CLI → TUI → Services → LLM → DB | Separation of concerns |
| Provider | `llm/provider/` trait + implementations | Multi-LLM support |
| Repository | `db/repository/` for data access | DB abstraction |
| Service | `services/` for business logic | Orchestration |
| Event-Driven | `tui/events.rs`, `events/` | Decoupled UI updates |
| Registry | `llm/tools/` ToolRegistry | Extensible tools |

---

## 3. Architecture Visualization

### 3.1 High-Level Component Diagram

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                              CRUSTLY SYSTEM                                      │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  ┌─────────────────┐                                                         │
│  │   User (CLI)    │ ──┐                                                      │
│  └─────────────────┘  │                                                      │
│                      ▼                                                      │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │                         CLI Layer (clap)                              │   │
│  │   - Argument parsing      - Command dispatch          - Debug mode   │   │
│  └───────────────────────┬───────────────────────────────────────────────┘   │
│                          │                                                   │
│                          ▼                                                   │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │                       TUI Layer (ratatui/crossterm)                      │   │
│  │  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐  ┌────────────┐  │   │
│  │  │    app      │  │   runner     │  │    pages    │  │  styles    │  │   │
│  │  │  (App, App)  │  │  (run loop)  │  │  (screens)   │  │ (theming)  │  │   │
│  │  └─────────────┘  └─────────────┘  └─────────────┘  └────────────┘  │   │
│  │  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────────────┐  │   │
│  │  │  components  │  │  highlight   │  │     markdown/render           │  │   │
│  │  │  (widgets)   │  │ (syntax)     │  │    (content rendering)        │  │   │
│  │  └─────────────┘  └─────────────┘  └─────────────────────────────┘  │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                          │                                                   │
│                          ▼                                                   │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │                      Services Layer (Business Logic)                     │   │
│  │  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐  │   │
│  │  │  session     │  │  message     │  │   file      │  │   plan      │  │   │
│  │  │  (chats)     │  │  (history)    │  │ (I/O ops)    │  │ (workflows) │  │   │
│  │  └─────────────┘  └─────────────┘  └─────────────┘  └─────────────┘  │   │
│  │                          │                                                  │   │
│  │  ┌─────────────────────────────┐                                      │   │
│  │  │         ServiceManager        │                                      │   │
│  │  │    (orchestrates services)    │                                      │   │
│  │  └─────────────────────────────┘                                      │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                          │                                                   │
│          ┌───────────────────────────┼───────────────────────────┐          │
│          │                           │                           │          │
│          ▼                           ▼                           ▼          │
│  ┌─────────────┐            ┌─────────────┐            ┌─────────────┐    │
│  │   LLM       │            │    Config    │            │     DB      │    │
│  │  Layer      │            │   Layer      │            │   Layer      │    │
│  │             │            │             │            │             │    │
│  │  ┌───────┐  │            │  ┌───────┐  │            │  ┌───────┐  │    │
│  │  │agent  │  │            │  │crabrace│  │            │  │models│  │    │
│  │  └───────┘  │            │  └───────┘  │            │  └───────┘  │    │
│  │  ┌───────┐  │            │  ┌───────┐  │            │  ┌───────┐  │    │
│  │  │provider│  │            │  │secrets│  │            │  │repo  │  │    │
│  │  │  ├─ anth│  │            │  └───────┘  │            │  └───────┘  │    │
│  │  │  ├─ open │  │            │  ┌───────┐  │            │             │    │
│  │  │  ├─ qwen │  │            │  │update │  │            │  SQLite    │    │
│  │  │  └─ ...  │  │            │  └───────┘  │            │  (SQLx)    │    │
│  │  └───────┘  │            └─────────────┘            └─────────────┘    │
│  └─────────────┘            ┌─────────────┐                                  │
│                              │   utils/    │                                  │
│                              │   macros/   │                                  │
│                              │   error/    │                                  │
│                              └─────────────┘                                  │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

### 3.2 Data Flow Diagram

```
User Input (Keyboard)
       │
       ▼
┌─────────────────────────────┐
│   CLI Layer                  │
│  - clap argument parsing     │
│  - Command dispatch          │
└───────────────┬─────────────┘
                │
                ▼
┌─────────────────────────────┐
│   TUI Layer                   │
│  - Event handling            │
│  - App state management      │
│  - Rendering loop            │
└───────────────┬─────────────┘
                │
                ▼
┌─────────────────────────────┐
│   Services Layer             │
│  - Session management        │
│  - Message persistence        │
│  - File operations           │
│  - Plan workflows            │
└───────────────┬─────────────┘
                │
        ┌───────┼───────┐
        │       │       │
        ▼       ▼       ▼
┌──────────┐ ┌─────────┐ ┌──────────┐
│  LLM     │ │ Config  │ │   DB     │
│  - Agent │ │ - Crabrace│ │  - SQLite │
│  - Provider││ - Secrets│ │  - Repos │
│  - Tools │ │ - Update │ │          │
└──────────┘ └─────────┘ └──────────┘
       │               │              │
       ▼               ▼              ▼
┌─────────────────────────────────────────┐
│   External Systems                           │
│  - LLM APIs (Anthropic, OpenAI, etc.)       │
│  - MCP Servers                              │
│  - File System                              │
└─────────────────────────────────────────┘
```

### 3.3 Module Dependency Graph

```
main.rs
  └── cli/
      └── lib.rs (uses: app, tui, services, config, llm)

lib.rs
  ├── app/
  ├── cli/
  ├── config/
  │   ├── crabrace.rs
  │   ├── secrets.rs
  │   └── update.rs
  ├── db/
  │   ├── models.rs
  │   ├── repository/
  │   └── retry.rs
  ├── error.rs
  ├── events/
  ├── llm/
  │   ├── agent/
  │   │   ├── context.rs
  │   │   ├── error.rs
  │   │   └── service.rs
  │   ├── provider/
  │   │   ├── mod.rs (trait + types)
  │   │   ├── anthropic.rs
  │   │   ├── azure.rs
  │   │   ├── openai.rs
  │   │   ├── qwen.rs
  │   │   ├── factory.rs
  │   │   ├── error.rs
  │   │   └── retry.rs
  │   └── tools/
  │       └── mod.rs (ToolRegistry)
  ├── logging.rs
  ├── macros/
  ├── mcp/
  ├── message/
  ├── services/
  │   ├── mod.rs (ServiceManager, ServiceContext)
  │   ├── file.rs
  │   ├── message.rs
  │   ├── plan.rs
  │   └── session.rs
  ├── sync/
  ├── tui/
  │   ├── mod.rs
  │   ├── app.rs
  │   ├── error.rs
  │   ├── events.rs
  │   ├── plan.rs
  │   ├── prompt_analyzer.rs
  │   ├── runner.rs
  │   ├── splash.rs
  │   ├── render.rs
  │   ├── highlight.rs
  │   ├── markdown.rs
  │   ├── components/
  │   ├── pages/
  │   ├── styles/
  │   └── utils/
  └── utils/
```

---

## 4. Core Architectural Components

### 4.1 CLI Module (`cli/`)

**Purpose:** Command-line interface entry point and argument parsing

**Responsibility:**
- Parse CLI arguments (clap v4)
- Configure debug/logging mode
- Dispatch to appropriate handlers
- System prompt management

**Key Files:**
- `mod.rs`: CLI struct definition, subcommands
- `run.rs` (likely): Main execution logic

**Dependencies:** app, tui, services, config, llm, anyhow, clap, clap_complete

**Notable Features:**
- Debug mode with log files in `.crustly/logs/`
- Auto-completion support (clap_complete)
- Rich system prompt for tool usage guidance

### 4.2 TUI Module (`tui/`)

**Purpose:** Terminal user interface built with Ratatui

**Responsibility:**
- Interactive terminal rendering
- Event handling (keyboard, system)
- App state management
- Multi-page navigation
- Syntax highlighting (syntect)
- Markdown rendering (pulldown-cmark)

**Key Submodules:**
- `app.rs`: Main TUI application state (`App` struct)
- `runner.rs`: TUI event loop and runner
- `events.rs`: Event types and handling
- `plan.rs`: Plan mode implementation
- `prompt_analyzer.rs`: Prompt analysis for plan creation
- `render.rs`: Rendering utilities
- `highlight.rs`: Syntax highlighting
- `markdown.rs`: Markdown to text rendering
- `splash.rs`: Splash screen
- `components/`: UI components/widgets
- `pages/`: Page/screen definitions
- `styles/`: Styling and theming
- `utils/`: TUI utilities

**Key Types:**
- `App`: Main application state
- `AppMode`: Current UI mode
- `TuiEvent`: Event enum (Key, Mouse, Paste, Resize, Custom)
- `DisplayMessage`: Messages to display in UI
- `PlanDocument`, `PlanTask`, `TaskStatus`, `TaskType`: Plan mode types

### 4.3 Services Module (`services/`)

**Purpose:** Business logic layer orchestrating database operations

**Responsibility:**
- Session management (create, list, delete chats)
- Message history and persistence
- File operations (read, write, edit)
- Plan workflow management
- Business rule enforcement

**Key Types:**
- `ServiceContext`: Shared resources (database pool)
- `ServiceManager`: Aggregates all services
- `SessionService`: Chat session management
- `MessageService`: Message history and retrieval
- `FileService`: File I/O operations
- `PlanService`: Plan creation and execution

**Design Pattern:**
- ServiceManager acts as a facade
- Each service receives a `ServiceContext` with shared DB pool
- Services are independent but can be composed

### 4.4 LLM Module (`llm/`)

**Purpose:** LLM provider abstraction and agent services

**Submodules:**

#### 4.4.1 Provider (`llm/provider/`)
- **Purpose:** Unified interface for LLM providers
- **Key Files:**
  - `mod.rs`: Trait definition and re-exports
  - `trait.rs`: `Provider` trait and `ProviderCapabilities`
  - `types.rs`: Request/response types
  - `factory.rs`: Provider factory for creation
  - `error.rs`: Provider-specific errors
  - `retry.rs`: Retry logic for provider calls
  - Provider implementations: `anthropic.rs`, `openai.rs`, `azure.rs`, `qwen.rs`

- **Key Types:**
  - `Provider`: Async trait for LLM operations
  - `ProviderStream`: Streaming response handle
  - `LLMRequest`, `LLMResponse`: Request/response types
  - `Message`, `ContentBlock`, `Role`: Message types
  - `Tool`, `ToolCallParser`: Tool calling support
  - `StopReason`, `TokenUsage`: Response metadata

#### 4.4.2 Agent (`llm/agent/`)
- **Purpose:** High-level agent functionality
- **Key Files:**
  - `mod.rs`: Re-exports
  - `context.rs`: `AgentContext` for conversation state
  - `service.rs`: `AgentService` for orchestration
  - `error.rs`: Agent-specific errors

- **Key Types:**
  - `AgentService`: Main agent orchestrating LLM calls
  - `AgentContext`: Conversation context, tool registry
  - `AgentResponse`, `AgentStreamResponse`: Response types
  - `ApprovalCallback`, `ToolApprovalInfo`: Tool approval handling

#### 4.4.3 Tools (`llm/tools/`)
- **Purpose:** Tool registry and execution
- **Key Files:** `mod.rs`
- **Key Types:**
  - `ToolRegistry`: Registry of available tools
  - `ToolResult`: Result of tool execution
  - `ToolError`: Tool-specific errors

### 4.5 Database Module (`db/`)

**Purpose:** SQLite database layer with SQLx

**Responsibility:**
- Database connection management
- Model definitions
- Repository implementations
- Retry logic for database operations

**Key Files:**
- `mod.rs`: Database connection manager, Pool type
- `models.rs`: Database entity definitions
- `repository/`: Repository implementations
- `retry.rs`: Retry configuration and utilities

**Key Types:**
- `Database`: Connection manager
- `Pool`: Type alias for SqlitePool
- `DbRetryConfig`: Retry configuration
- Various model structs and repository traits

**Connection Features:**
- SQLite with WAL mode (`?mode=rwc`)
- Busy timeout: 5 seconds
- Max connections: 5
- Connection acquire timeout: 10 seconds

### 4.6 Config Module (`config/`)

**Purpose:** Application configuration management

**Responsibility:**
- Main configuration loading
- Provider configurations
- Secrets management
- Crabrace integration (provider registry)
- Update checking

**Key Files:**
- `mod.rs`: Main Config struct and submodules
- `crabrace.rs`: Crabrace provider registry integration
- `secrets.rs`: Secure secret storage
- `update.rs`: Provider configuration updates

**Key Types:**
- `Config`: Main configuration
- `DatabaseConfig`: Database settings
- `LoggingConfig`: Logging settings
- `DebugConfig`: Debug options
- `ProviderConfigs`: All provider configurations
- `ProviderConfig`: Individual provider settings
- `CrabraceConfig`, `CrabraceIntegration`: Provider registry
- `ProviderSecrets`, `SecretString`: Secure storage

### 4.7 Support Modules

| Module | Purpose |
|--------|---------|
| `app/` | Application lifecycle and state |
| `error.rs` | Error types and handling |
| `events/` | Event types for TUI |
| `logging.rs` | Logging configuration and cleanup |
| `macros/` | Custom derive macros |
| `mcp/` | Model Context Protocol integration |
| `message/` | Message handling utilities |
| `sync/` | Synchronization primitives |
| `utils/` | General utilities |

---

## 5. Architectural Layers and Dependencies

### 5.1 Layer Structure

```
┌─────────────────────────────────────────────────────────────┐
│                        LAYER 4: CLI                           │
│  ┌─────────────────────────────────────────────────────────┐│
│  │                    cli/                                   ││
│  │  - Argument parsing (clap)                              ││
│  │  - Command dispatch                                     ││
│  │  - Debug mode configuration                             ││
│  └─────────────────────────────────────────────────────────┘│
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                      LAYER 3: TUI                              │
│  ┌─────────────────────────────────────────────────────────┐│
│  │                    tui/                                   ││
│  │  - App state management                                 ││
│  │  - Event loop (runner)                                  ││
│  │  - Rendering (highlight, markdown)                        ││
│  │  - Pages/screens                                        ││
│  │  - Components/widgets                                   ││
│  │  - Plan mode                                           ││
│  └─────────────────────────────────────────────────────────┘│
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                     LAYER 2: Services                           │
│  ┌─────────────────────────────────────────────────────────┐│
│  │                  services/                                 ││
│  │  - ServiceManager (orchestrator)                        ││
│  │  - SessionService (chats)                               ││
│  │  - MessageService (history)                             ││
│  │  - FileService (I/O)                                    ││
│  │  - PlanService (workflows)                              ││
│  └─────────────────────────────────────────────────────────┘│
└─────────────────────────────────────────────────────────────┘
              ┌────────────────────────────┼────────────────────────────┐
              │                            │                            │
              ▼                            ▼                            ▼
┌─────────────────────┐   ┌─────────────────┐   ┌─────────────────┐
│     LAYER 1: LLM     │   │    LAYER 1:      │   │    LAYER 1:      │
│      (llm/)          │   │    Config        │   │      DB          │
│                     │   │   (config/)       │   │    (db/)          │
│  - agent/           │   │  - crabrace.rs    │   │  - models.rs     │
│  - provider/        │   │  - secrets.rs    │   │  - repository/   │
│  - tools/           │   │  - update.rs      │   │  - retry.rs      │
└─────────────────────┘   └─────────────────┘   └─────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                      LAYER 0: External                          │
│  - LLM APIs (Anthropic, OpenAI, AWS Bedrock, Azure, etc.)     │
│  - SQLite database                                          │
│  - File System                                               │
│  - MCP Servers                                               │
└─────────────────────────────────────────────────────────────┘
```

### 5.2 Dependency Rules

**Within Layers:**
- CLI can depend on: TUI, Services, Config, LLM
- TUI can depend on: Services, LLM, Config, DB
- Services can depend on: LLM, Config, DB
- LLM, Config, DB are independent of each other

**Cross-Layer Communication:**
- **Top-Down**: Commands flow from CLI → TUI → Services → LLM/DB
- **Bottom-Up**: Events flow from LLM/DB → Services → TUI
- **Service Pattern**: Services orchestrate between DB and LLM
- **Trait Objects**: Provider trait allows dynamic dispatch

### 5.3 Module Dependency Matrix

| Module | cli | tui | services | llm | db | config | app | error | events | logging | utils |
|--------|-----|-----|----------|-----|----|--------|-----|-------|--------|---------|-------|
| cli | - | ✓ | ✓ | ✓ | | ✓ | | | | | | |
| tui | | - | ✓ | ✓ | | | | ✓ | ✓ | ✓ | ✓ | | ✓ |
| services | | | - | ✓ | ✓ | ✓ | | | | | | | |
| llm | | | | - | | | | | | | | | |
| db | | | | | - | | | | | | | | |
| config | | | | | | - | | | | | | | |
| app | | ✓ | | | | | - | | | | | | |
| error | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | - | | | | |
| events | | ✓ | | | | | | | | - | | | |
| logging | | | | | | | | | | | | - | |
| utils | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | - |

**Key:** ✓ = depends on

### 5.4 Circular Dependency Prevention

- **Single Crate Design**: All code in one crate prevents circular dependencies between crates
- **Module Hierarchy**: Clear parent-child relationships via `mod.rs` files
- **Trait-Based Abstraction**: Provider trait allows breaking direct dependencies
- **Service Pattern**: Services act as intermediaries between layers

---

## 6. Data Architecture

### 6.1 Domain Model

**Core Entities:**

```
┌─────────────────────────────────────────────────────────────┐
│                      DOMAIN MODEL                              │
├─────────────────────────────────────────────────────────────┤
│                                                                 │
│  ┌─────────────────┐    ┌─────────────────┐                 │
│  │    Session      │    │    Message      │                 │
│  │                 │    │                 │                 │
│  │ - id: UUID      │    │ - id: UUID      │                 │
│  │ - user_id: i64  │    │ - session_id    │                 │
│  │ - title: String │    │ - role: Role    │                 │
│  │ - created_at    │    │ - content:      │                 │
│  │ - updated_at    │    │   String        │                 │
│  │ - metadata      │    │ - token_count   │                 │
│  └─────────────────┘    │ - created_at    │                 │
│                        └─────────────────┘                 │
│                                    │                              │
│  ┌─────────────────┐    ┌───────┴───────┐                 │
│  │   Plan          │    │   PlanTask     │                 │
│  │                 │    │                 │                 │
│  │ - id: UUID      │    │ - id: UUID      │                 │
│  │ - session_id    │    │ - plan_id       │                 │
│  │ - title        │    │ - title         │                 │
│  │ - description   │    │ - description   │                 │
│  │ - status        │    │ - task_type     │                 │
│  │ - created_at    │    │ - status        │                 │
│  │ - updated_at    │    │ - complexity     │                 │
│  └─────────────────┘    │ - dependencies   │                 │
│                        │ - order          │                 │
│                        └─────────────────┘                 │
│                                                                 │
│  ┌─────────────────┐    ┌─────────────────┐                 │
│  │   FileRecord    │    │   ToolCall       │                 │
│  │                 │    │                 │                 │
│  │ - path          │    │ - tool_name     │                 │
│  │ - operation      │    │ - arguments     │                 │
│  │ - content       │    │ - result        │                 │
│  │ - timestamp      │    │ - status        │                 │
│  └─────────────────┘    └─────────────────┘                 │
│                                                                 │
└─────────────────────────────────────────────────────────────┘
```

### 6.2 Entity Relationships

```
Session ┌─────────────────────────────┐ Message
       │ 1..* (has)                      │ 1..*
       ▼                                ▼
     Plan ─────────────────────────── PlanTask
       │ 1..* (contains)               │ 1..*
       ▼                                │
   ToolCall (0..*) ──────────────────── FileRecord (0..*)
```

### 6.3 Database Schema (SQLite)

**Tables (inferred from models):**
- `sessions`: Chat sessions
- `messages`: Conversation messages
- `plans`: Workflow plans
- `plan_tasks`: Individual tasks in plans
- `file_records`: File operation history
- `tool_calls`: Tool execution history

**Connection Pool:**
- SQLite with WAL mode
- Max 5 connections
- 5-second busy timeout
- 10-second acquire timeout

### 6.4 Data Access Patterns

**Repository Pattern:**
```rust
// db/repository/mod.rs (inferred structure)
pub trait SessionRepository {
    async fn create(&self, session: &Session) -> Result<Session>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Session>>;
    async fn list_all(&self, user_id: i64) -> Result<Vec<Session>>;
    async fn update(&self, session: &Session) -> Result<Session>;
    async fn delete(&self, id: Uuid) -> Result<()>;
}
```

**Service Usage:**
```rust
// services/session.rs
pub struct SessionService {
    context: ServiceContext,
}

impl SessionService {
    pub async fn create_session(&self, user_id: i64, title: String) -> Result<Session> {
        // Uses repository via ServiceContext
    }
}
```

### 6.5 Caching & Retry

**Retry Logic:**
- `db/retry.rs`: Database operation retry utilities
- `llm/provider/retry.rs`: Provider call retry with exponential backoff
- Configurable retry counts and delays

---

## 7. Cross-Cutting Concerns

### 7.1 Authentication & Authorization

**Security Model:**
- **Provider Authentication**: API keys via environment variables or config
- **Secret Storage**: Keyring integration for secure storage
- **Tool Approval**: Interactive approval for dangerous operations

**Implementation:**
- `config/secrets.rs`: `ProviderSecrets`, `SecretString` for secure storage
- `llm/agent/service.rs`: `ApprovalCallback`, `ToolApprovalInfo` for tool permissions
- Environment variable support for all providers

**Supported Providers:**
- Anthropic: `ANTHROPIC_API_KEY`
- OpenAI: `OPENAI_API_KEY`
- AWS Bedrock: AWS credentials
- Azure: `AZURE_OPENAI_API_KEY`
- VertexAI: GCP credentials
- Qwen/DashScope: `DASHSCOPE_API_KEY`

### 7.2 Error Handling & Resilience

**Error Pattern:**
- `thiserror` for typed errors in each module
- `anyhow` for ad-hoc errors
- Module-specific error types
- Error propagation via `?` operator

**Key Error Types:**
- `CrustlyError` (root error type)
- `ProviderError` (LLM provider errors)
- `AgentError` (agent service errors)
- `ToolError` (tool execution errors)
- Database errors (SQLx)
- IO errors

**Retry Implementation:**
- Database: `retry_db_anyhow`, `retry_db_sqlx` in `db/retry.rs`
- Provider: Retry logic in `llm/provider/retry.rs`
- Configurable retry policies

**Resilience Features:**
- Graceful degradation for unavailable features
- Streaming with error recovery
- Connection timeout handling

### 7.3 Logging & Monitoring

**Logging Stack:**
- `tracing`: Primary logging framework
- `tracing-subscriber`: Log collection and filtering
- `tracing-appender`: File appender for logs
- `color-eyre`: Colored error reporting

**Configuration:**
- Debug mode: Creates log files in `.crustly/logs/`
- JSON output format available
- Environment filter support
- Automatic cleanup of old logs (7 days)

**Setup:**
```rust
// logging.rs
pub fn setup_from_cli(debug: bool) -> Result<tracing_appender::non_blocking::WorkerGuard> {
    // Creates file appender when debug=true
    // Returns guard that flushes on drop
}

pub fn cleanup_old_logs(days: u32) -> Result<usize> {
    // Removes log files older than specified days
}
```

### 7.4 Validation

**Validation Layers:**
1. **CLI**: Clap validates arguments at parse time
2. **Config**: Serde deserialization with validation
3. **Provider**: API key and endpoint validation
4. **Database**: Query validation, foreign key constraints
5. **Tool**: Input validation before execution

### 7.5 Configuration Management

**Configuration Hierarchy:**
```
┌─────────────────────────────────────────┐
│          Configuration Sources             │
├─────────────────────────────────────────┤
│                                           │
│  1. CLI Arguments (highest priority)      │
│     --debug, etc.                          │
│                                           │
│  2. Environment Variables                 │
│     ANTHROPIC_API_KEY, etc.                │
│                                           │
│  3. Configuration File                    │
│     ~/.config/crustly/config.toml          │
│                                           │
│  4. Default Values (lowest priority)      │
│     Built-in defaults                      │
│                                           │
└─────────────────────────────────────────┘
```

**Configuration Structure:**
```toml
[database]
# SQLite path
path = "~/.local/share/crustly/crustly.db"

[logging]
level = "info"
file_enabled = true

[debug]
debug_lsp = false
profiling = false

[providers.anthropic]
enabled = true
# api_key loaded from environment
base_url = "https://api.anthropic.com"

[providers.openai]
enabled = false
# api_key loaded from environment
base_url = "https://api.openai.com"
```

---

## 8. Extension Patterns

### 8.1 Provider Extension

**Adding New LLM Providers:**

1. **Implement Provider Trait:**
```rust
// llm/provider/my_provider.rs
use super::{Provider, ProviderStream, LLMRequest, LLMResponse, ProviderError};

pub struct MyProvider {
    api_key: String,
    base_url: String,
    // ...
}

#[async_trait::async_trait]
impl Provider for MyProvider {
    async fn send_message(&self, request: LLMRequest) -> Result<LLMResponse, ProviderError> {
        // Implementation
    }
    
    async fn stream_message(&self, request: LLMRequest) -> Result<ProviderStream, ProviderError> {
        // Streaming implementation
    }
    
    fn name(&self) -> &str {
        "my_provider"
    }
    
    fn capabilities(&self) -> &ProviderCapabilities {
        &ProviderCapabilities {
            supports_streaming: true,
            supports_tools: true,
            // ...
        }
    }
}
```

2. **Register in Factory:**
```rust
// llm/provider/factory.rs
pub fn create_provider(config: &ProviderConfig) -> Box<dyn Provider> {
    match config.provider_type {
        "my_provider" => Box::new(MyProvider::new(config)),
        // ...
    }
}
```

3. **Add to Config:**
```rust
// config/mod.rs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderConfigs {
    // ...
    pub my_provider: Option<ProviderConfig>,
}
```

### 8.2 Tool Extension

**Adding New Tools:**

```rust
// llm/tools/mod.rs
pub struct ToolRegistry {
    tools: HashMap<String, Arc<dyn ToolExecutor>>,
}

impl ToolRegistry {
    pub fn register(&mut self, name: &str, executor: Arc<dyn ToolExecutor>) {
        self.tools.insert(name.to_string(), executor);
    }
}

pub trait ToolExecutor: Send + Sync {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn parameters(&self) -> serde_json::Value;
    async fn execute(&self, args: serde_json::Value) -> Result<ToolResult, ToolError>;
}
```

### 8.3 MCP Integration

**Model Context Protocol:**
- `mcp/` module for MCP server integration
- Dynamic tool and resource discovery
- Stdio-based communication
- Background process management

### 8.4 LSP Integration

**Language Server Protocol:**
- `lsp/` module for LSP client implementation
- Code intelligence and analysis
- Semantic understanding of code
- tower-lsp + lsp-types for implementation

### 8.5 Plan Mode

**Structured Workflow:**
- `tui/plan.rs`: Plan document and task management
- `tui/prompt_analyzer.rs`: Automatic plan generation
- Keyboard shortcuts: Ctrl+A (approve), Ctrl+R (reject), Esc (cancel)
- Automatic export to PLAN.md on approval

---

## 9. Blueprint for New Development

### 9.1 Development Workflow

**Starting Points by Feature Type:**

| Feature Type | Location | Key Files |
|--------------|----------|-----------|
| New CLI command | `cli/mod.rs` | Add subcommand |
| New TUI page | `tui/pages/` | Create new page module |
| New TUI component | `tui/components/` | Create new component |
| New service | `services/` | Create new service + register |
| New LLM provider | `llm/provider/` | Implement Provider trait |
| New tool | `llm/tools/` | Register in ToolRegistry |
| New DB model | `db/models.rs` + `db/repository/` | Add model + repo |
| New config option | `config/mod.rs` | Add to Config struct |

**Component Creation Sequence:**

1. **Design Phase:**
   - Identify the appropriate module
   - Determine dependencies
   - Design public interface

2. **Implementation Phase:**
   - Create the module file
   - Implement functionality
   - Add tests in `tests/`

3. **Integration Phase:**
   - Export from parent `mod.rs`
   - Update dependencies
   - Register if needed (tools, providers)

4. **Documentation Phase:**
   - Add module doc comments
   - Update relevant documentation
   - Add examples if applicable

### 9.2 Implementation Templates

**New Service:**
```rust
// services/my_service.rs
use crate::db::{Pool, ServiceContext};
use anyhow::Result;

pub struct MyService {
    context: ServiceContext,
}

impl MyService {
    pub fn new(context: ServiceContext) -> Self {
        Self { context }
    }
    
    pub async fn do_something(&self, input: &str) -> Result<String> {
        // Implementation using self.context.pool()
        Ok("result".to_string())
    }
}

// Register in services/mod.rs ServiceManager
```

**New Provider:**
```rust
// llm/provider/my_provider.rs
use super::*;

pub struct MyProvider {
    api_key: String,
    client: reqwest::Client,
}

#[async_trait::async_trait]
impl Provider for MyProvider {
    async fn send_message(&self, request: LLMRequest) -> Result<LLMResponse, ProviderError> {
        let response = self.client
            .post(&format!("{}/messages", self.base_url))
            .json(&request)
            .send()
            .await?;
        // Parse response
    }
    
    // ... other trait methods
}

// Export and register in factory
```

**New TUI Page:**
```rust
// tui/pages/my_page.rs
use ratatui::prelude::*;
use crate::tui::{App, AppMode, EventHandler};

pub struct MyPage {
    // State
}

impl MyPage {
    pub fn new() -> Self {
        Self { /* init */ }
    }
    
    pub fn render(&self, frame: &mut Frame) {
        // Render page content
    }
    
    pub fn handle_event(&mut self, event: TuiEvent) -> EventHandler {
        // Handle events
        EventHandler::Continue
    }
}
```

### 9.3 Common Pitfalls

**Architecture Violations to Avoid:**

1. **Circular Dependencies:**
   - Don't create circular module imports
   - Use trait objects or move code to parent module

2. **Layer Violations:**
   - TUI shouldn't directly access DB
   - Use Services layer for orchestration

3. **Blocking in Async:**
   - Don't use blocking operations in async functions
   - Use async versions of all I/O

4. **Global State:**
   - Minimize global mutable state
   - Use Arc<Mutex<T>> or Arc<RwLock<T>> when necessary

5. **Error Handling:**
   - Don't use unwrap() in library code
   - Use ? operator for propagation
   - Convert errors appropriately

**Performance Considerations:**

- Clone Arc, not the underlying data
- Use references where possible
- Batch database operations
- Use streaming for large responses
- Minimize allocations in hot paths

**Testing Blind Spots:**

- Test error cases, not just success
- Test with concurrent access
- Test cross-module interactions
- Use mockall for mocking traits
- Test TUI with mock event sources

### 9.4 Recommendations

**Keeping the Blueprint Updated:**
- Update when adding/removing modules
- Update when changing architectural patterns
- Update when adding new extension points
- Review quarterly for accuracy

**Project-Specific Recommendations:**

1. **Complete LSP Integration:**
   - Currently stubbed in `lsp/` module
   - Add full LSP client implementation
   - Integrate with code analysis features

2. **Enhance MCP Support:**
   - Expand `mcp/` module
   - Add more MCP server types
   - Improve tool discovery

3. **Improve Testing:**
   - Add more unit tests for services
   - Add integration tests for TUI
   - Use mockall for provider mocking

4. **Optimize Performance:**
   - Profile with pprof (Unix)
   - Optimize token streaming
   - Reduce allocations in rendering

5. **Document Architecture:**
   - Add module-level documentation
   - Document public APIs
   - Create architecture decision records (ADRs)

---

## Appendix: File Structure

```
crustly/
├── Cargo.toml                    # Package manifest with features
├── README.md                     # Project documentation
├── LICENSE.md                    # FSL-1.1-MIT license
├── config.toml.example           # Example configuration
├── .rustfmt.toml                 # Code formatting
├── benches/                      # Benchmark suites
│   └── database.rs
├── docs/                         # Documentation
│   └── screenshots/               # Screenshots
├── migrations/                   # Database migrations (future)
├── src/
│   ├── main.rs                   # Binary entry point
│   ├── lib.rs                   # Library root
│   ├── app/
│   │   └── mod.rs               # Application state
│   ├── cli/
│   │   └── mod.rs               # CLI arguments and dispatch
│   ├── config/
│   │   ├── mod.rs               # Main config types
│   │   ├── crabrace.rs           # Crabrace provider registry
│   │   ├── secrets.rs            # Secret storage
│   │   └── update.rs             # Update checking
│   ├── db/
│   │   ├── mod.rs               # Database connection
│   │   ├── models.rs             # Database models
│   │   ├── repository/           # Repository implementations
│   │   └── retry.rs              # Retry utilities
│   ├── error.rs                 # Root error types
│   ├── events/
│   │   └── mod.rs               # Event types
│   ├── llm/
│   │   ├── mod.rs               # LLM module root
│   │   ├── agent/
│   │   │   ├── mod.rs           # Agent re-exports
│   │   │   ├── context.rs       # Agent context
│   │   │   ├── error.rs         # Agent errors
│   │   │   └── service.rs       # Agent service
│   │   ├── provider/
│   │   │   ├── mod.rs           # Provider trait + types
│   │   │   ├── trait.rs         # Provider trait
│   │   │   ├── types.rs         # Request/response types
│   │   │   ├── factory.rs       # Provider factory
│   │   │   ├── error.rs         # Provider errors
│   │   │   ├── retry.rs         # Retry logic
│   │   │   ├── anthropic.rs     # Anthropic implementation
│   │   │   ├── azure.rs         # Azure OpenAI implementation
│   │   │   ├── openai.rs        # OpenAI implementation
│   │   │   └── qwen.rs          # Qwen/DashScope implementation
│   │   └── tools/
│   │       └── mod.rs           # Tool registry
│   ├── logging.rs               # Logging setup
│   ├── macros/
│   │   └── mod.rs               # Custom macros
│   ├── mcp/
│   │   └── mod.rs               # MCP integration
│   ├── message/
│   │   └── mod.rs               # Message handling
│   ├── services/
│   │   ├── mod.rs               # Services root (ServiceManager)
│   │   ├── file.rs              # File service
│   │   ├── message.rs           # Message service
│   │   ├── plan.rs              # Plan service
│   │   └── session.rs           # Session service
│   ├── sync/
│   │   └── mod.rs               # Synchronization
│   ├── tui/
│   │   ├── mod.rs               # TUI root
│   │   ├── app.rs               # TUI app state
│   │   ├── error.rs             # TUI errors
│   │   ├── events.rs            # TUI events
│   │   ├── plan.rs              # Plan mode
│   │   ├── prompt_analyzer.rs   # Prompt analysis
│   │   ├── runner.rs            # TUI runner
│   │   ├── splash.rs            # Splash screen
│   │   ├── render.rs            # Rendering
│   │   ├── highlight.rs         # Syntax highlighting
│   │   ├── markdown.rs          # Markdown rendering
│   │   ├── components/          # UI components
│   │   ├── pages/               # UI pages
│   │   ├── styles/              # Styling
│   │   └── utils/               # TUI utilities
│   └── utils/
│       └── mod.rs               # General utilities
└── tests/
    └── (test files)
```

---

## Key Features Summary

| Category | Features |
|----------|----------|
| **LLM** | Multi-provider, streaming, tool calling, conversation context |
| **TUI** | Ratatui, syntax highlighting, markdown, keyboard shortcuts, plan mode |
| **Tools** | File ops, bash commands, custom tools, approval system |
| **Database** | SQLite, SQLx, retry logic, migrations (future) |
| **Config** | TOML, hierarchical, provider-specific, secrets management |
| **Extensibility** | Provider abstraction, tool registry, MCP, LSP (stubbed) |
| **Performance** | Tokio, async I/O, optimized binaries, profiling support |

---

*Generated using Architecture Blueprint Generator. Keep updated as architecture evolves.*
