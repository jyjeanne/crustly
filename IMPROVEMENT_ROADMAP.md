# Crustly Improvement Roadmap

**Generated:** 2025-10-28
**Current Version:** 0.1.0
**Test Status:** ✅ 145 tests passing

This document outlines prioritized improvements for the Crustly project, organized by impact and urgency.

---

## 🔴 Critical Priority (Sprint 10)

### 1. **Multi-Provider Support**
**Status:** ⚠️ Only Anthropic implemented
**Impact:** HIGH - Core feature blocking production use
**Effort:** 3-5 days

**Tasks:**
- [ ] Implement `OpenAIProvider` using `async-openai` crate
- [ ] Implement `BedrockProvider` using `aws-sdk-bedrockruntime`
- [ ] Implement `GeminiProvider` via HTTP client
- [ ] Implement `AzureOpenAIProvider`
- [ ] Add provider selection in TUI (Settings mode)
- [ ] Add `--provider` flag to CLI run command
- [ ] Add provider switching in active sessions
- [ ] Test all providers with approval system
- [ ] Document provider-specific features and limitations

**Why Critical:** Users expect multi-provider support as advertised. Currently only Anthropic works despite config supporting 6+ providers.

**Files to modify:**
- `src/llm/provider/openai.rs` (create)
- `src/llm/provider/bedrock.rs` (create)
- `src/llm/provider/gemini.rs` (create)
- `src/llm/provider/azure.rs` (create)
- `src/cli/mod.rs` (add provider selection)
- `src/tui/app.rs` (add provider switching UI)

---

### 2. **Error Recovery & Resilience**
**Status:** ⚠️ Limited error handling
**Impact:** HIGH - Affects stability
**Effort:** 2-3 days

**Tasks:**
- [ ] Implement automatic retry with exponential backoff for API calls
- [ ] Add network timeout handling (configurable, default 60s)
- [ ] Handle rate limiting gracefully (show countdown in TUI)
- [ ] Recover from temporary database locks
- [ ] Add graceful degradation when syntax highlighting fails
- [ ] Implement crash recovery (save session state)
- [ ] Add error reporting dialog in TUI with copy-to-clipboard
- [ ] Log all errors to file for debugging

**Why Critical:** Current implementation can crash or hang on network issues. Production apps need resilience.

**Files to modify:**
- `src/llm/provider/trait.rs` (add retry logic)
- `src/llm/agent/service.rs` (handle provider failures)
- `src/tui/app.rs` (add error dialog)
- `src/db/mod.rs` (handle lock contention)

---

### 3. **Security Hardening**
**Status:** ⚠️ Basic security only
**Impact:** HIGH - Security concerns
**Effort:** 2-3 days

**Tasks:**
- [ ] Implement secure API key storage using OS keyring (keyring crate)
- [ ] Never log API keys (audit all debug/trace statements)
- [ ] Add approval timeout (auto-deny after 5 minutes)
- [ ] Implement approval history audit log (read-only SQLite table)
- [ ] Add file path validation (prevent directory traversal attacks)
- [ ] Sanitize bash command inputs (prevent injection)
- [ ] Add configurable tool blacklist (block dangerous commands: `rm -rf /`, `format C:`, etc.)
- [ ] Implement read-only mode flag (disable write/bash tools)

**Why Critical:** Handling user code and API keys requires robust security. Current implementation stores keys in plaintext.

**Files to create/modify:**
- `src/config/secrets.rs` (enhance with keyring)
- `src/llm/tools/bash.rs` (add command validation)
- `src/llm/tools/write.rs` (add path validation)
- `src/llm/tools/registry.rs` (add tool blacklist)
- `src/db/repository/audit.rs` (create audit log)

---

## 🟡 High Priority (Sprint 11-12)

### 4. **Enhanced Tool System**
**Status:** ⚠️ Only 3 basic tools
**Impact:** HIGH - Feature completeness
**Effort:** 4-5 days

**Tasks:**
- [ ] **ListTool** - List directory contents with filtering
- [ ] **SearchTool** - Search files using ripgrep
- [ ] **GitTool** - Git operations (status, diff, log, branch)
- [ ] **WebFetchTool** - Fetch URLs for research (with approval)
- [ ] **EditorTool** - Open file in user's $EDITOR
- [ ] **DiffTool** - Show git-style diffs before file writes
- [ ] **TestTool** - Run project-specific test commands
- [ ] **LintTool** - Run linters (cargo clippy, eslint, etc.)
- [ ] Tool categories and organization in registry
- [ ] Per-tool enable/disable config
- [ ] Tool usage statistics

**Why High:** Users need more tools for real coding workflows. Current 3 tools are too limited.

**Files to create:**
- `src/llm/tools/list.rs`
- `src/llm/tools/search.rs`
- `src/llm/tools/git.rs`
- `src/llm/tools/web.rs`
- `src/llm/tools/editor.rs`
- `src/llm/tools/diff.rs`
- `src/llm/tools/test.rs`
- `src/llm/tools/lint.rs`

---

### 5. **Approval System Enhancements**
**Status:** ⚠️ Basic v1 complete, v2 features needed
**Impact:** MEDIUM-HIGH - User experience
**Effort:** 3-4 days

**Tasks:**
- [ ] Session memory: "Always allow write_file for this session"
- [ ] Tool whitelist in config file
- [ ] File diff preview before approving writes
- [ ] Approval history view in TUI (browse past decisions)
- [ ] Bulk approve: "Approve next 3 write_file requests"
- [ ] Pattern matching: "Approve all read_file in src/"
- [ ] Dry run mode: Show effects without executing
- [ ] Undo last operation (where possible)
- [ ] Export audit log to JSON/CSV

**Why High:** Current approval system is functional but limited. These features improve workflow.

**Files to modify:**
- `src/tui/app.rs` (add approval memory state)
- `src/tui/render.rs` (add diff preview, history view)
- `src/llm/agent/service.rs` (implement whitelist/patterns)
- `src/config/mod.rs` (add approval config)

---

### 6. **Local LLM Integration**
**Status:** ⚠️ Documented but untested
**Impact:** MEDIUM-HIGH - Key differentiator
**Effort:** 2-3 days

**Tasks:**
- [ ] Test LM Studio integration thoroughly
- [ ] Test Ollama integration
- [ ] Add automatic provider detection (check localhost:1234, :11434)
- [ ] Implement model selection UI for local providers
- [ ] Add streaming support verification
- [ ] Document performance benchmarks per model
- [ ] Add local model pull/download UI
- [ ] Handle no-auth local providers
- [ ] Add model context length detection

**Why High:** Local LLM support is a major feature. Need to verify it works flawlessly.

**Files to modify:**
- `src/llm/provider/openai.rs` (use for OpenAI-compatible local)
- `src/config/mod.rs` (add auto-detection)
- `src/tui/app.rs` (add model selection UI)

---

### 7. **Performance Optimization**
**Status:** ⚠️ No benchmarks, unoptimized
**Impact:** MEDIUM - User experience
**Effort:** 3-4 days

**Tasks:**
- [ ] Create benchmark suite (database, rendering, highlighting)
- [ ] Profile memory usage with large sessions (>1000 messages)
- [ ] Optimize syntax highlighting (lazy load syntaxes, cache results)
- [ ] Implement message pagination in TUI (don't render all messages)
- [ ] Add database query optimization and indexes
- [ ] Reduce binary size (currently ~10MB stripped)
- [ ] Implement incremental rendering for streaming responses
- [ ] Add connection pooling for HTTP clients
- [ ] Profile and optimize hot paths

**Why High:** Performance is key for terminal apps. No benchmarks means we don't know bottlenecks.

**Files to create:**
- `benches/database.rs`
- `benches/llm_processing.rs`
- `benches/tui_rendering.rs`
- `benches/syntax_highlighting.rs`

**Files to modify:**
- `src/tui/render.rs` (pagination, incremental rendering)
- `src/tui/highlight.rs` (caching)
- `src/db/mod.rs` (query optimization)

---

## 🟢 Medium Priority (Sprint 13-15)

### 8. **MCP Protocol Support**
**Status:** ❌ TODO stub only
**Impact:** MEDIUM - Advanced feature
**Effort:** 5-7 days

**Tasks:**
- [ ] Implement MCP server/client protocol
- [ ] Add transport layer (stdio, HTTP, WebSocket)
- [ ] Create MCP tool adapter (wrap MCP servers as tools)
- [ ] Add MCP server discovery and registration
- [ ] Implement context synchronization
- [ ] Add MCP prompt management
- [ ] Test with Claude Desktop MCP servers
- [ ] Document MCP server creation

**Why Medium:** MCP enables ecosystem extensions but not critical for core functionality.

**Files to implement:**
- `src/mcp/mod.rs`
- `src/mcp/transport/mod.rs`
- `src/mcp/protocol.rs`
- `src/mcp/client.rs`
- `src/mcp/server.rs`

---

### 9. **LSP Integration**
**Status:** ❌ TODO stub only
**Impact:** MEDIUM - Advanced feature
**Effort:** 4-5 days

**Tasks:**
- [ ] Implement LSP client using tower-lsp
- [ ] Add code intelligence in tool context (definitions, references)
- [ ] Provide symbol information to LLM
- [ ] Enable jump-to-definition from TUI
- [ ] Add diagnostics integration (show compiler errors)
- [ ] Implement hover information
- [ ] Add completion suggestions
- [ ] Support multiple language servers

**Why Medium:** LSP adds code intelligence but requires significant effort. Nice-to-have.

**Files to implement:**
- `src/lsp/mod.rs`
- `src/lsp/client.rs`
- `src/lsp/manager.rs`
- `src/lsp/diagnostics.rs`

---

### 10. **Enhanced Session Management**
**Status:** ⚠️ Basic implementation
**Impact:** MEDIUM - Usability
**Effort:** 2-3 days

**Tasks:**
- [ ] Add session search and filtering in TUI
- [ ] Implement session tags/labels
- [ ] Add session export (Markdown, JSON)
- [ ] Session import from files
- [ ] Session merging (combine multiple sessions)
- [ ] Session archiving (compress old sessions)
- [ ] Add session templates (predefined contexts)
- [ ] Implement session sharing (export with anonymized content)
- [ ] Add session statistics (token usage per session, cost analysis)

**Why Medium:** Improves session organization for power users.

**Files to modify:**
- `src/tui/app.rs` (session UI enhancements)
- `src/services/session.rs` (new operations)
- `src/db/repository/session.rs` (tags, search)

---

### 11. **Git Integration**
**Status:** ⚠️ Basic git2 dependency, no UI
**Impact:** MEDIUM - Developer workflow
**Effort:** 3-4 days

**Tasks:**
- [ ] Show current git branch in status bar
- [ ] Display dirty/clean state
- [ ] Show uncommitted changes count
- [ ] Add git commit helper (stage, commit with AI-generated message)
- [ ] Implement smart context (include uncommitted changes in context)
- [ ] Add .gitignore awareness (don't read ignored files)
- [ ] Show git blame information
- [ ] Add branch switching with confirmation

**Why Medium:** Git awareness improves context for coding assistants.

**Files to create:**
- `src/services/git.rs`
- `src/tui/components/git_status.rs`

---

### 12. **Configuration UI**
**Status:** ⚠️ Config command shows text, no interactive UI
**Impact:** MEDIUM - User experience
**Effort:** 2-3 days

**Tasks:**
- [ ] Create interactive settings TUI page
- [ ] Add provider configuration editor
- [ ] API key input with masking
- [ ] Model selection dropdown
- [ ] Tool enable/disable toggles
- [ ] Approval settings (timeout, whitelist)
- [ ] Theme customization
- [ ] Keybinding customization
- [ ] Save settings to file from UI

**Why Medium:** CLI config is functional but UI improves discoverability.

**Files to modify:**
- `src/tui/app.rs` (Settings mode implementation)
- `src/tui/render.rs` (render settings UI)
- `src/config/mod.rs` (runtime updates)

---

### 13. **Testing Infrastructure**
**Status:** ⚠️ 145 unit tests, no integration/E2E tests
**Impact:** MEDIUM - Quality assurance
**Effort:** 3-4 days

**Tasks:**
- [ ] Add integration tests for full flows (chat, tools, approval)
- [ ] Create mock provider for testing without API calls
- [ ] Add E2E tests with recorded provider responses (VCR-style)
- [ ] Snapshot testing for TUI rendering
- [ ] Property-based tests for critical functions
- [ ] Add code coverage reporting
- [ ] Set up mutation testing
- [ ] Create smoke tests for all commands

**Why Medium:** Current tests are good but incomplete. Need broader coverage.

**Files to create:**
- `tests/integration/chat_flow.rs`
- `tests/integration/tool_execution.rs`
- `tests/integration/approval_system.rs`
- `tests/e2e/full_session.rs`
- `tests/mocks/provider.rs`

---

## 🔵 Low Priority (Sprint 16+)

### 14. **Advanced TUI Features**
**Status:** ⚠️ Basic TUI functional
**Impact:** LOW - Polish
**Effort:** 4-5 days

**Tasks:**
- [ ] Multi-pane layout (chat + file preview)
- [ ] Split view (compare two files)
- [ ] Tabs for multiple conversations
- [ ] Command palette (fuzzy search commands)
- [ ] File tree browser
- [ ] Integrated terminal emulator pane
- [ ] Screenshot/recording support
- [ ] Custom themes (dracula, gruvbox, etc.)
- [ ] Mouse support
- [ ] Vim keybindings mode

**Why Low:** Nice-to-have features that improve UX but aren't essential.

**Files to create:**
- `src/tui/layouts/mod.rs`
- `src/tui/components/file_tree.rs`
- `src/tui/components/terminal.rs`
- `src/tui/themes/mod.rs`

---

### 15. **Telemetry & Analytics**
**Status:** ❌ Not implemented
**Impact:** LOW - Operational insight
**Effort:** 2-3 days

**Tasks:**
- [ ] Add opt-in anonymous usage telemetry
- [ ] Track command usage statistics
- [ ] Monitor tool execution frequency
- [ ] Token usage analytics (charts in TUI)
- [ ] Cost tracking per project/session
- [ ] Error rate monitoring
- [ ] Performance metrics
- [ ] Export metrics to Prometheus
- [ ] Privacy-respecting implementation (no content sent)

**Why Low:** Helpful for understanding usage patterns but not essential.

**Files to create:**
- `src/telemetry/mod.rs`
- `src/telemetry/collector.rs`
- `src/telemetry/exporter.rs`

---

### 16. **Plugin System**
**Status:** ❌ Not planned
**Impact:** LOW - Extensibility
**Effort:** 7-10 days

**Tasks:**
- [ ] Design plugin API
- [ ] Implement WASM-based plugin runtime
- [ ] Add plugin discovery and loading
- [ ] Create plugin SDK in Rust
- [ ] Support custom tools via plugins
- [ ] Add plugin marketplace/registry
- [ ] Implement sandboxing for security
- [ ] Create example plugins
- [ ] Document plugin development

**Why Low:** Major undertaking, benefits mostly power users and developers.

**Files to create:**
- `src/plugins/mod.rs`
- `src/plugins/runtime.rs`
- `src/plugins/api.rs`
- `crustly-plugin-sdk/` (new crate)

---

### 17. **Documentation Expansion**
**Status:** ⚠️ Good README, limited docs
**Impact:** LOW-MEDIUM - Onboarding
**Effort:** 3-4 days

**Tasks:**
- [ ] Create comprehensive user guide (book format)
- [ ] Add architecture documentation
- [ ] API documentation for all public types
- [ ] Tutorial series (beginner to advanced)
- [ ] Video walkthrough
- [ ] Troubleshooting guide
- [ ] FAQ section
- [ ] Contributing guide
- [ ] Code of conduct
- [ ] Release notes template

**Why Low:** Current docs are functional. Enhancement improves adoption.

**Files to create:**
- `docs/book/` (mdBook)
- `docs/architecture/` (diagrams)
- `docs/tutorials/`
- `CONTRIBUTING.md`
- `CODE_OF_CONDUCT.md`

---

### 18. **CI/CD Pipeline**
**Status:** ❌ No automation
**Impact:** LOW-MEDIUM - Development velocity
**Effort:** 2-3 days

**Tasks:**
- [ ] Set up GitHub Actions workflow
- [ ] Automated testing on PR
- [ ] Code coverage reporting
- [ ] Clippy linting
- [ ] Format checking (rustfmt)
- [ ] Security audit (cargo-audit)
- [ ] Automated releases (cargo-release)
- [ ] Binary artifact publishing
- [ ] Docker image builds
- [ ] Documentation deployment

**Why Low:** Manual releases work but automation improves quality.

**Files to create:**
- `.github/workflows/ci.yml`
- `.github/workflows/release.yml`
- `.github/workflows/docs.yml`

---

### 19. **Examples & Templates**
**Status:** ❌ Empty directories
**Impact:** LOW - Developer experience
**Effort:** 2-3 days

**Tasks:**
- [ ] Create example: Simple chat bot
- [ ] Create example: Code review automation
- [ ] Create example: Documentation generator
- [ ] Create example: Custom tool implementation
- [ ] Create example: MCP server
- [ ] Create example: Provider implementation
- [ ] Add project templates (cargo generate)
- [ ] Create starter kits for common use cases

**Why Low:** Helps new developers but not blocking.

**Files to create:**
- `examples/simple_bot.rs`
- `examples/code_review.rs`
- `examples/doc_generator.rs`
- `examples/custom_tool.rs`
- `examples/mcp_server/`
- `templates/` (cargo-generate)

---

### 20. **Cross-Platform Polish**
**Status:** ⚠️ Developed on Windows, Linux/Mac untested
**Impact:** LOW - Platform support
**Effort:** 2-3 days

**Tasks:**
- [ ] Test thoroughly on Linux (Ubuntu, Arch, Fedora)
- [ ] Test thoroughly on macOS (Intel, Apple Silicon)
- [ ] Fix platform-specific path handling issues
- [ ] Test terminal compatibility (iTerm2, Alacritty, Windows Terminal, etc.)
- [ ] Add platform-specific installation guides
- [ ] Create native installers (MSI for Windows, DMG for macOS, .deb/.rpm for Linux)
- [ ] Add to package managers (Homebrew, AUR, Scoop, Chocolatey)
- [ ] Test SSH connections and tmux compatibility

**Why Low:** Core functionality likely works but polish needed for wide adoption.

**Files to modify:**
- `src/config/mod.rs` (path handling)
- Build scripts and packaging

---

## 📊 Summary Statistics

| Priority | Items | Estimated Days | Status |
|----------|-------|----------------|--------|
| 🔴 Critical | 3 | 7-11 days | Sprint 10 |
| 🟡 High | 4 | 12-17 days | Sprint 11-12 |
| 🟢 Medium | 6 | 21-29 days | Sprint 13-15 |
| 🔵 Low | 7 | 22-36 days | Sprint 16+ |
| **Total** | **20** | **62-93 days** | **~3-4 months** |

---

## 🎯 Recommended Sprint Sequence

### Sprint 10 (2 weeks) - Critical Foundations
1. Multi-Provider Support
2. Error Recovery & Resilience
3. Security Hardening

### Sprint 11 (2 weeks) - Core Features
4. Enhanced Tool System
5. Approval System v2
6. Local LLM Integration

### Sprint 12 (1 week) - Performance
7. Performance Optimization & Benchmarks

### Sprint 13-14 (3 weeks) - Advanced Features
8. MCP Protocol Support
9. LSP Integration
10. Enhanced Session Management

### Sprint 15 (2 weeks) - Usability
11. Git Integration
12. Configuration UI
13. Testing Infrastructure

### Sprint 16+ (Ongoing) - Polish & Extensions
14-20. Advanced TUI, Telemetry, Plugins, Documentation, CI/CD, Examples, Cross-Platform

---

## 🚨 Blocking Issues

**Must Fix Before 1.0:**
- Only Anthropic provider works (advertised as multi-provider)
- No error recovery (crashes on network issues)
- API keys stored in plaintext
- No retry logic for API failures
- Hard-coded model in TUI render

**Should Fix Before 1.0:**
- Limited tool selection (only 3 tools)
- No local LLM testing/validation
- No benchmarks or performance data
- Approval system v1 limitations

---

## 💡 Quick Wins (High Value, Low Effort)

1. **Fix Hard-Coded Model** (30 minutes)
   - Replace `let model = "claude-3-5-sonnet"` with dynamic model from state
   - File: `src/tui/render.rs:line 413`

2. **Add Custom Config Path Support** (1 hour)
   - Implement TODO in `src/cli/mod.rs:line 125`

3. **Create First Benchmark** (2 hours)
   - Simple database operation benchmark
   - Baseline for future optimization

4. **Add Connection Timeout** (1 hour)
   - Prevent hanging on network issues
   - Configure in settings (default: 60s)

5. **Implement Approval Timeout** (2 hours)
   - Auto-deny after 5 minutes
   - Prevent stale approval requests

---

## 🔗 Dependencies Between Items

```
Multi-Provider (1) → Local LLM (6) → Performance (7)
Error Recovery (2) → All features require resilience
Security (3) → Must be early foundation
Enhanced Tools (4) → Approval v2 (5) → MCP (8)
Git Integration (11) → Enhanced Tools (4)
Config UI (12) → All config-related features
Testing (13) → Should grow with each feature
```

---

## 📝 Notes

- **Version Target:** Most critical items should be in v0.5.0 (beta), all high priority in v1.0.0
- **Breaking Changes:** Multi-provider implementation may require config format changes
- **Community Input:** Survey users on tool priorities before implementing all tools
- **Performance:** Benchmark before optimizing (measure twice, cut once)
- **Security:** All security items must pass external audit before 1.0

**Last Updated:** 2025-10-28
**Next Review:** After Sprint 10 completion
