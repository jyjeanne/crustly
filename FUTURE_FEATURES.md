# Future Features Roadmap

This document outlines placeholder modules and planned features for Crustly.

## Status: Placeholder Modules

The following modules exist as placeholders in the codebase but are not yet implemented:

### 1. LSP Integration (`src/lsp/`)

**Status:** 🔴 Planned
**Priority:** High
**Complexity:** High

**Purpose:**
Language Server Protocol integration for semantic code understanding.

**Features:**
- Go-to-definition
- Find references
- Code completion suggestions
- Semantic code analysis
- Symbol search across codebase

**Use Cases:**
- AI can understand code structure
- Better refactoring suggestions
- Smarter code generation
- Context-aware tool calling

**Dependencies:**
- `tower-lsp` crate
- Language-specific LSP servers

**Implementation Notes:**
- Start with Rust LSP (rust-analyzer)
- Expand to TypeScript, Python, Go
- Integrate with tool system

---

### 2. Model Context Protocol (`src/mcp/`)

**Status:** 🔴 Planned
**Priority:** Medium
**Complexity:** Medium

**Purpose:**
Support for Anthropic's Model Context Protocol for enhanced AI capabilities.

**Features:**
- MCP server connections
- Resource providers
- Prompt templates
- Tool extensions

**Use Cases:**
- Connect to external data sources
- Custom tool integrations
- Enterprise system integration

**Dependencies:**
- MCP protocol specification
- Transport layer (stdio, HTTP, WebSocket)

**Implementation Notes:**
- Implement client-side MCP
- Support stdio and HTTP transports
- Add MCP server discovery

---

### 3. Event System (`src/events/`)

**Status:** 🔴 Planned
**Priority:** Low
**Complexity:** Low

**Purpose:**
Centralized event bus for cross-module communication.

**Features:**
- Publish-subscribe pattern
- Event routing
- Async event handlers
- Event logging/replay

**Use Cases:**
- TUI reactivity
- Plugin system
- Audit logging
- State synchronization

**Dependencies:**
- `tokio::sync::broadcast`
- Custom event types

**Implementation Notes:**
- Simple event bus with typed events
- Non-blocking handlers
- Optional persistence

---

### 4. Message Module (`src/message/`)

**Status:** 🔴 Planned
**Priority:** Low
**Complexity:** Low

**Purpose:**
Enhanced message handling and formatting.

**Features:**
- Message templates
- Rich formatting
- Message transformations
- Export to various formats

**Use Cases:**
- Export conversations
- Message search
- Format conversion (JSON, Markdown, HTML)

**Dependencies:**
- Template engine (handlebars?)
- Format converters

**Implementation Notes:**
- Build on existing Message type
- Add formatting layer
- Template system

---

### 5. Sync Module (`src/sync/`)

**Status:** 🟡 Future
**Priority:** Very Low
**Complexity:** High

**Purpose:**
Synchronization across devices/sessions.

**Features:**
- Cloud sync (optional)
- Conflict resolution
- Incremental sync
- Encryption

**Use Cases:**
- Multi-device usage
- Team collaboration
- Backup/restore

**Dependencies:**
- Cloud storage backend
- Encryption libraries
- Sync protocol

**Implementation Notes:**
- Privacy-first (opt-in only)
- End-to-end encryption
- Consider self-hosted option

---

### 6. Macros Module (`src/macros/`)

**Status:** 🟢 Can be removed
**Priority:** None
**Complexity:** N/A

**Purpose:**
Proc macros if needed.

**Current Status:**
No proc macros are currently needed. This module can be removed.

**Recommendation:** Remove in next cleanup.

---

## TUI Placeholder Components

Several TUI components are stubbed out:

### `src/tui/components/chat/`
- **Status:** Partially implemented (chat exists in main TUI)
- **Action:** Remove stub, feature is implemented elsewhere

### `src/tui/components/dialogs/`
- **Status:** Implemented (approval dialogs exist)
- **Action:** Remove stub, feature is implemented

### `src/tui/pages/`
- **Status:** May not be needed
- **Action:** Evaluate if multi-page TUI is desired

### `src/tui/styles/`
- **Status:** Styles currently inline
- **Action:** Could centralize styles here

### `src/tui/utils/`
- **Status:** Utils currently inline
- **Action:** Could centralize here

---

## Implementation Priority

Based on value and complexity:

### Phase 1 (High Value, Lower Complexity)
1. **Clean up TUI stubs** - Immediate
2. **Event system** - 1-2 weeks
3. **Message formatting** - 1 week

### Phase 2 (High Value, High Complexity)
4. **LSP Integration** - 3-4 weeks
5. **MCP Support** - 2-3 weeks

### Phase 3 (Future/Optional)
6. **Sync module** - 4-6 weeks (if needed)

### No Implementation Needed
- **Macros** - Remove

---

## Decision Matrix

| Module | Implement? | When? | Priority |
|--------|-----------|-------|----------|
| LSP | ✅ Yes | Q1 2025 | High |
| MCP | ✅ Yes | Q1 2025 | Medium |
| Events | ✅ Yes | Soon | Low |
| Message | ✅ Yes | Soon | Low |
| Sync | ❓ Maybe | Future | Very Low |
| Macros | ❌ No | Never | None |
| TUI stubs | ⚠️  Clean up | Immediate | N/A |

---

## How to Contribute

If you want to implement one of these features:

1. Review this document
2. Check if dependencies are available
3. Create an issue proposing the implementation
4. Submit a detailed design doc
5. Implement with tests
6. Submit PR

---

## Additional Provider Implementations

See [PROVIDER_IMPLEMENTATION_GUIDE.md](PROVIDER_IMPLEMENTATION_GUIDE.md) for:
- Google Gemini
- AWS Bedrock
- Google VertexAI
- Other LLM providers

---

**Last Updated:** 2025-11-22
