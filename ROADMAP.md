# Crustly Roadmap

**Current Version:** 0.4.1 — May 2026
**Author:** Jeremy JEANNE

---

## Completed Milestones

### v0.1 — Foundation
- Ratatui TUI with streaming chat, plan mode, tool approval dialogs
- Anthropic provider with streaming, tool use, and cost tracking
- SQLite persistence (sessions, messages, plans)
- Phase 1 tools: `read_file`, `write_file`, `edit_file`, `bash`, `ls`, `glob`, `grep`

### v0.2 — Multi-Provider & Advanced Tools
- OpenAI provider (GPT-4, GPT-3.5, OpenAI-compatible endpoints)
- Local LLM support via LM Studio and Ollama
- AWS Bedrock provider (`--features aws-bedrock`)
- Phase 2 tools: `web_search`, `execute_code`, `notebook_edit`, `parse_document`
- OS keyring integration for secure API key storage

### v0.3 — Workflow & Intelligence
- Phase 3 tools: `plan`, `task`, `context`, `http_request`
- Smart model routing (keyword-based tier selection, no API call)
- Parallel tool dispatch via `join_all` (≥40% faster on concurrent reads)
- Tool result caching with TTL for read-only tools
- Provider failover (auto-retry on secondary provider on rate-limit / timeout)
- Sandbox & permission policy (path boundary enforcement, bash allowlist, `AndPolicy`)
- Episodic memory (past session summaries injected into new context)
- Context compaction at 80% window capacity

### v0.4 — Claw Code Parity & Reasoning
- Reasoning / thinking display (Anthropic extended thinking, DeepSeek-R1, QwQ-32B via `<think>` tags)
- Codebase index with file watcher (`notify` crate, symbol index, FTS search)
- MCP tool server integration (`[[mcp.servers]]` config, auto-registered at startup)
- Phase 4 tools: `web_fetch`, `todo_write`, `ask_user`, `skill`, `agent`, `powershell`
- `SubAgentLauncher` trait — decoupled sub-agent spawning with recursion guard

### v0.4.2 — Native Ollama Integration
- Native Ollama provider (`--features ollama`, built on [`ollama-rs`](https://github.com/pepperoni21/ollama-rs)) talking directly to `/api/chat`, alongside the existing OpenAI-compatible route
- Runtime performance metrics — tokens/sec, model load time, warm/cold-start — surfaced in the TUI header and under each reply; provider identity badge in the header for every configured provider
- `Ctrl+D` **Model Download dialog** — pick or type a model, pull with a live progress bar, cancel with `Esc`, without leaving the TUI
- Model management CLI: `crustly ollama list|pull|rm|show|embed`
- `keep_alive` / `num_ctx` control; `OLLAMA_HOST`/`OLLAMA_MODEL` env vars
- See [`ollama-rs-integration-plan.md`](./ollama-rs-integration-plan.md) for the full design and implementation status

---

## Upcoming Milestones

### v0.5 — Stability & Developer Experience
**Target:** Q3 2026

- [ ] **Integration test suite** — full chat/tool/approval flows with a mock provider (no live API calls required)
- [ ] **CI/CD pipeline** — GitHub Actions: `cargo test`, `cargo clippy`, `cargo fmt --check`, `cargo audit` on every PR
- [ ] **Interactive settings TUI** — configure provider, API keys (masked), model, tool toggles, approval timeout from inside the TUI
- [ ] **Session search & export** — fuzzy search sessions by content; export to Markdown or JSON
- [ ] **Git status bar** — current branch, dirty/clean state, uncommitted changes count shown in footer
- [ ] **Approval memory** — "always allow this tool for this session" option; configurable per-tool whitelist

### v0.6 — Performance & Robustness
**Target:** Q4 2026

- [ ] **Benchmark suite** — `cargo bench` targets for DB operations, syntax highlighting, and TUI rendering
- [ ] **Message pagination** — virtual list rendering so large sessions (1000+ messages) don't degrade TUI performance
- [ ] **Lazy syntax highlighting** — cache rendered blocks; skip re-highlighting unchanged content
- [ ] **Crash recovery** — write a recovery manifest on panic; offer to restore the interrupted session on next launch
- [ ] **Structured error dialog** — TUI overlay showing full error with copy-to-clipboard; all errors written to `.crustly/logs/`
- [ ] **Rate-limit countdown** — show remaining backoff time in the status bar when a provider returns 429

### v0.7 — LSP & Code Intelligence
**Target:** Q1 2027

- [ ] **LSP client** — wire up `tower-lsp` for symbol definitions, references, and diagnostics
- [ ] **Diagnostics injection** — include active compiler errors in the agent context automatically
- [ ] **Jump-to-definition** — open file at symbol location from TUI (launches `$EDITOR`)
- [ ] **Hover information** — surface type signatures and docs in a side panel
- [ ] **Multi-language support** — Rust, TypeScript, Python, Go language servers

### v0.8 — Plugin System
**Target:** Q2 2027

- [ ] **Plugin API design** — stable ABI for custom tools and providers
- [ ] **WASM runtime** — sandboxed plugin execution via `wasmtime`
- [ ] **Plugin discovery** — scan `.crustly/plugins/` and `~/.config/crustly/plugins/`
- [ ] **Plugin SDK** — separate `crustly-plugin-sdk` crate with examples
- [ ] **Example plugins** — `git-tool`, `docker-tool`, `jira-tool`

### v1.0 — Production Release
**Target:** Q3 2027

- [ ] All v0.5–v0.8 milestones complete and stable
- [ ] External security audit (API key handling, tool sandbox, path traversal)
- [ ] Cross-platform validation (Linux Ubuntu/Arch, macOS Intel/Apple Silicon, Windows 11)
- [ ] Package manager distribution (Homebrew, AUR, Scoop, Chocolatey)
- [ ] Full user documentation (mdBook with tutorials and API reference)
- [ ] Semantic versioning with automated releases via `cargo-release`

---

## Backlog (Post-1.0)

| Item | Notes |
|------|-------|
| RAG / vector store | Semantic search over the codebase; integrate with the existing codebase index. Raw embedding generation is already available via the native Ollama provider (`crustly ollama embed`) — no retrieval layer wired up yet |
| Multi-pane TUI | Chat + file preview split; tabs for multiple conversations |
| Web interface | Optional `crustly serve` command exposing a browser UI |
| Multi-user / team | Shared sessions, role-based approval, audit log export |
| Telemetry (opt-in) | Anonymous usage stats; cost analytics charts in TUI |
| Google Gemini provider | HTTP client implementation |
| Azure OpenAI provider | Azure-specific auth and endpoint routing |
| `crustly run` pipelines | Chain multiple prompts with conditional logic in a YAML file |

---

## Guiding Principles

1. **Terminal-first** — never require a browser or GUI; every feature must work over SSH.
2. **Privacy by default** — local LLM support is a first-class citizen, not an afterthought.
3. **Explicit approval** — dangerous operations always pause for user confirmation; no silent side effects.
4. **Measurable before optimised** — add benchmarks before claiming performance improvements.
5. **Security before features** — path traversal, injection, and API key hygiene are non-negotiable pre-conditions for 1.0.
