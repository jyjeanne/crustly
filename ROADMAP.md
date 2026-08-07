# Crustly Roadmap

**Current Version:** 0.5.2 — July 2026
**Author:** Jeremy JEANNE

**Related strategy docs:** [`differentiation-strategy-vs-opencode.md`](./differentiation-strategy-vs-opencode.md) (competitive positioning — depth over breadth), [`docs/guides/SECURITY_MODEL.md`](./docs/guides/SECURITY_MODEL.md) (permission engine, compared to OpenCode's), [`llm-file-gguf-support.md`](./llm-file-gguf-support.md) (direct GGUF loading evaluation, conditional Go/No-Go), [`ccguf-managment-imrpoment-plan.md`](./ccguf-managment-imrpoment-plan.md) (GGUF model-management improvement plan, informed by a `llamastash` feature analysis — full design/rationale for the v0.5.3 milestone below)

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

### v0.5 — Gemini Provider & Claude Code / Qwen Compatibility
- Native Google Gemini provider (streaming, function calling, vision, extended thinking, structured output) — also serves Gemma 3/4 through the same API
- `apply_patch` tool: Codex-compatible multi-file patch support (22nd built-in tool)
- Tool name alias layer plus `file_path`/`directory`/`shell` argument compatibility for Claude Code and Qwen-trained models
- `grep` regex-by-default, `.gitignore`-aware `grep`/`glob`, hardened Qwen Hermes tool-call parsing
- Prior-read enforcement for `edit_file`/`write_file`/`apply_patch`
- Bug fixes: macOS symlinked-root path boundary check, `pulldown-cmark` 0.13 markdown renderer compatibility
- Completed `ratatui` 0.28 → 0.30 upgrade, migrating off unmaintained `tui-textarea`

### v0.5.2 — Local-Model Reliability & Per-Model Tuning
Hardening pass from end-to-end agentic testing against local Ollama models (qwen2.5-coder, gemma4, ornith):

- **Per-model Ollama settings** — `[providers.ollama.models."<name>"]` blocks for sampling (`temperature`/`top_p`/`top_k`), `num_ctx`, and `keep_alive`, with field-by-field fallback to the provider-level values. The per-model `num_ctx` is coupled to `context_window`, so the window compaction budgets against and the window Ollama allocates can never drift. One shared construction path is used at startup and by the `Ctrl+W` model switch
- **Approval semantics overhaul** — `security.allow_bash` is a no-prompt shortcut, not a wall: non-allowlisted commands and shell-operator chains (`mkdir x && cd x && cargo init`) reach the approval prompt (full command shown verbatim) and the user's approval runs them; operator commands are never auto-trusted, and Plan/read-only mode still rejects them
- **Reasoning-only answer fallback** — models that answer entirely in the thinking channel no longer produce blank messages: the reasoning is shown with a clear notice (display-only, never fed back into model context)
- **Fenced-JSON tool-call recovery** — tool calls printed as ```json blocks inside prose (qwen retry pattern) are recovered and executed; strict offered-tool/explicit-arguments matching, bare inline JSON is never executed
- **`--model` CLI flag** — one-shot default-model override for any command
- **Fixed:** the "Message not found" crash that broke every `crustly run` (sqlx-sqlite doesn't auto-commit `INSERT ... RETURNING` on a WAL pool — message creation now commits an explicit transaction); the tool-loop detector falsely aborting distinct path-based calls (wrong input key in the signature); `Ctrl+W` rebuilding a bare provider that dropped the entire `[providers.ollama]` config; TUI timestamps rendered in UTC instead of local time; `Ctrl+K` deleting messages out from under an in-flight response; raw-JSON model-not-found errors replaced with actionable guidance; chat input text rendering underlined (textarea cursor-line default style)

### Unreleased — In-Process llama.cpp Provider
The GGUF-loading backlog item below shipped: `providers.llama_cpp` loads a
`.gguf` file directly into the Crustly process via `llama-cpp-2`, no Ollama
daemon or server of any kind. Delivered across 10 phases plus a follow-up
grammar-constrained tool-calling phase — see
[`llama-cpp-2-integration-plan.md`](./llama-cpp-2-integration-plan.md) §0.0
for the phase-by-phase record.

- Worker-thread architecture (one dedicated OS thread per provider instance,
  per [ADR 0005](docs/architecture/decisions/0005-llama-cpp-in-process-worker-thread.md)),
  non-streaming and streaming completion, sampling/context/chat-template
  parity with the other providers
- Tool calling via the shared `tool_call_recovery` module (same mechanism
  native Ollama falls back to), optionally upgraded to a syntax guarantee
  via grammar-constrained decoding (`--features llama-cpp-llguidance`,
  `llguidance`/`toktrie`) — a mid-stream sampler swap wired directly into
  the decode loop, hardened through two rounds of code review (false-positive
  hijack trigger, sampler-state loss on swap, an O(n²) trigger-scan
  regression)
- Six optional GPU backend features (`llama-cpp-cuda`/`-metal`/`-vulkan`/`-rocm`/`-opencl`/`-mkl`)
- Local model management: `Ctrl+G` TUI dialog and `crustly llama-cpp list|pull|rm` CLI,
  `hf:org/repo/file.gguf` shorthand downloads with SHA-256 verification
- Idle-unload (`providers.llama_cpp.idle_unload_secs`)
- Full guide (`docs/guides/LLAMA_CPP_GUIDE.md`) and README coverage

**Still open:** end-to-end verification against a real `.gguf` model and
live terminal session — this sandbox has never had one available. The
design/safety reasoning (documented in the plan's §0.0 Phase 4b entry) is
judged sound enough to ship unverified, but a real run is the next step for
whoever has a model and terminal to try it against.

---

## Upcoming Milestones

### v0.5.3 — GGUF Model Management
**Target:** Q3 2026

**Full design/rationale:** [`ccguf-managment-imrpoment-plan.md`](./ccguf-managment-imrpoment-plan.md)
— informed by a feature analysis of [`llamastash`](https://github.com/llamastash/llamastash),
scoped to what's portable without adopting its daemon/launcher/proxy architecture (Crustly
already runs inference in-process, see the "Unreleased" entry above). Follows the same
point-release pattern as v0.4.2 (Native Ollama Integration) and v0.5.2 (Local-Model
Reliability): focused follow-up hardening on a subsystem that just shipped, ahead of v0.6's
unrelated stability/DX work. Items below are ordered by the plan's own priority (§4 gap
analysis), not by phase number — highest-value/most-blocking first. Six execution milestones
(DP1–DP6, ~29 dev-days total) map onto the groupings below; see the plan's §8 for file-level
tasks and acceptance criteria per item.

**Foundation — unblocks everything else below (DP1):**
- [x] **Decouple GGUF file management from the `llama-cpp` build feature** — `list`/`pull`/`rm`
  no longer require the cmake/C++ toolchain; new `gguf-management` feature carries just the
  management code (plus `sha2`), joins both `default` and `all-llm`, and `llama-cpp` depends
  on it so nothing regressed. Also decoupled the TUI Ctrl+G dialog's list/download/delete and
  the Model Info panel's quantization hint, which the source plan hadn't originally scoped in
  but turned out to need the same fix. Verified: zero `llama-cpp-2`/`-sys-2` in the dependency
  tree under `--features gguf-management`; 834/834 tests pass there, 851/851 under
  `--features llama-cpp`, 883/883 under `all-llm` — no regressions
- [x] **Pure-Rust GGUF header metadata parser** (`src/llm/provider/gguf_metadata.rs`, zero new
  dependencies) — real architecture/parameter-count/quantization (layered: precise
  `general.file_type` → coarser tensor-type mode → filename guess as last resort)/context-length/
  chat-template-presence, wired into `LocalGgufModel` and its TUI mirror. Hardened: every declared
  string/array length validated against a cap before any allocation, a 256 MiB cumulative budget
  backstops the per-value caps, any structural problem returns `None` for the whole file rather
  than a panic or partial/corrupt result. 13 new tests, 847/847 total under `gguf-management`

**Core management (DP2–DP4):**
- [ ] **Memory/VRAM footprint estimate** — "~4.9 GB at this context length" in `list`/TUI, advisory only
- [ ] **Multi-source discovery** — Ollama (via its manifests, not raw blob scanning), HuggingFace/LM
  Studio caches, extra configured paths — beyond today's single-directory scan
- [ ] **Deduplication** — symlink collapsing, split-GGUF (`-00001-of-00005.gguf`) unification
- [ ] **`mmproj` pairing** — vision/audio projector files shown attached to their base model
- [ ] **Download hardening** — disk-space precheck, `--revision` pinning, resumable downloads
- [ ] **Agent-facing `--json` CLI contract** — stable schema + documented exit codes on `crustly
  llama-cpp list/pull/rm`, extending Crustly's existing agent-facing positioning (`AGENTS.md`)
- [ ] **TUI enhancements** — Ctrl+G/Ctrl+O dialogs show real header-parsed metadata instead of the
  filename guess

**Hardware-aware local model selection (DP6 — new since the plan's Update 3):**
- [ ] **Hardware detection & display** — best-effort GPU vendor/VRAM + system RAM probe
  (`nvidia-smi`/`rocm-smi`/DXGI/`system_profiler`/`vulkaninfo`, subprocess-based, no SDK linkage),
  surfaced in `doctor` and the TUI host-info panel; display/advisory only, never auto-mutates
  `providers.llama_cpp` config
- [ ] **Local hardware-fit filtering** — `Fits`/`Tight`/`Won't fit` labels and a `crustly llama-cpp
  list --best-fit` sort over models already on disk, using the memory estimate above against
  detected hardware; explicitly not a HuggingFace catalog search

**Diagnostics (DP5, lowest priority — cheap once the above exists):**
- [ ] **`crustly llama-cpp doctor`** — structured diagnostics (build feature, `models_dir`
  writability, disk space, detected hardware), always exits 0

**Deferred — tracked in Backlog below, not this milestone:** catalog-based, benchmark-ranked
`recommend` for models not yet downloaded (needs an external CI-maintained benchmark dataset;
disproportionate maintenance burden for now — see the plan's §3 item 5 and Phase M10).

### v0.6 — Stability & Developer Experience
**Target:** Q4 2026

**Competitive positioning (from `differentiation-strategy-vs-opencode.md`)** — the study
concluded Crustly should compete on depth (resource efficiency, permission-model rigor,
auditability) rather than chasing OpenCode's breadth (75+ providers, plugin ecosystem,
multi-frontend client/server). The two cheapest, highest-confidence items from its roadmap
are done; the rest are tracked here as they get picked up:

- [x] **Security model documentation** — `docs/guides/SECURITY_MODEL.md` documents the
  `PermissionPolicy` engine (`Allow`/`Trusted`/`Deny`, composable `AndPolicy`/`OrPolicy`, path
  boundary enforcement with symlink/Windows-verbatim-prefix handling, bash allowlist hardened
  against shell-operator-chaining bypass), every claim tied to file/line and verified against
  the test suite (`cargo test --lib llm::tools::sandbox`, 22/22 passing). Compares directly to
  OpenCode's documented "no sandbox, no rule engine, no hooks" permission model
- [x] **Resource-footprint benchmark tooling** — `scripts/benchmark-vs-opencode.sh` measures
  cold-start time, peak RSS, and binary size reproducibly against OpenCode on the same machine
  (`benchmarks/README.md` for methodology and ground rules: real numbers only, publish
  unfavorable results too)
- [ ] **Publish first real crustly-vs-opencode benchmark report** — run the script above on a
  machine with both tools installed and commit the resulting report under `benchmarks/results/`
  (not yet done from this environment — OpenCode isn't installed here, so no comparative numbers
  have been fabricated)
- [ ] **Plan Mode audit export** — `crustly plan export --session <id>`, turning the existing
  persisted `PlanDocument`/`plan_tasks` model (ADR 0004) into a reviewable audit report; see
  `differentiation-strategy-vs-opencode.md` §3.4
- [ ] **"Zero-daemon" positioning in user-facing docs** — README/onboarding language stating
  plainly that Crustly never starts a background server or opens a local port, unlike
  OpenCode's Hono HTTP/SSE server (see `differentiation-strategy-vs-opencode.md` §3.2)
- [ ] **Integration test suite** — full chat/tool/approval flows with a mock provider (no live API calls required)
- [x] **CI/CD pipeline** — GitHub Actions (`.github/workflows/ci.yml`): `cargo test` (Linux/Windows/macOS × stable/beta),
  `cargo clippy -D warnings`, `cargo fmt --check`, release `cargo build`, and `cargo tarpaulin` coverage on every PR.
  `cargo audit` is not yet wired in
- [x] **Fixed the CI feature matrix** — `--all-features` was unconditionally enabling the six GPU backend features
  added for llama.cpp (`llama-cpp-cuda`/`-metal`/`-vulkan`/`-rocm`/`-opencl`/`-mkl`), each needing a GPU SDK the
  GitHub-hosted runners don't have (`cmake` failed with "CUDA Toolkit not found"), breaking Clippy/Build/Coverage
  and one Test matrix leg. Replaced with an explicit `CI_FEATURES` list covering only what a plain C/C++ toolchain
  builds. `.github/workflows/release.yml` has the same `--all-features` issue in its cross-compiled Unix release
  step (not yet fixed — needs verification of cross-compiled cmake/C++ toolchain behavior for `llama-cpp`)
- [ ] **Interactive settings TUI** — configure provider, API keys (masked), model, tool toggles, approval timeout from inside the TUI
- [ ] **Session search & export** — fuzzy search sessions by content; export to Markdown or JSON
- [ ] **Git status bar** — current branch, dirty/clean state, uncommitted changes count shown in footer
- [ ] **Approval memory** — "always allow this tool for this session" option; configurable per-tool whitelist

### v0.7 — Performance & Robustness
**Target:** Q1 2027

- [ ] **Benchmark suite** — `cargo bench` targets for DB operations, syntax highlighting, and TUI rendering
- [ ] **Message pagination** — virtual list rendering so large sessions (1000+ messages) don't degrade TUI performance
- [ ] **Lazy syntax highlighting** — cache rendered blocks; skip re-highlighting unchanged content
- [ ] **Crash recovery** — write a recovery manifest on panic; offer to restore the interrupted session on next launch
- [ ] **Structured error dialog** — TUI overlay showing full error with copy-to-clipboard; all errors written to `.crustly/logs/`
- [ ] **Rate-limit countdown** — show remaining backoff time in the status bar when a provider returns 429

### v0.8 — LSP & Code Intelligence
**Target:** Q2 2027

- [ ] **LSP client** — wire up `tower-lsp` for symbol definitions, references, and diagnostics
- [ ] **Diagnostics injection** — include active compiler errors in the agent context automatically
- [ ] **Jump-to-definition** — open file at symbol location from TUI (launches `$EDITOR`)
- [ ] **Hover information** — surface type signatures and docs in a side panel
- [ ] **Multi-language support** — Rust, TypeScript, Python, Go language servers

### v0.9 — Plugin System
**Target:** Q3 2027

- [ ] **Plugin API design** — stable ABI for custom tools and providers
- [ ] **WASM runtime** — sandboxed plugin execution via `wasmtime`
- [ ] **Plugin discovery** — scan `.crustly/plugins/` and `~/.config/crustly/plugins/`
- [ ] **Plugin SDK** — separate `crustly-plugin-sdk` crate with examples
- [ ] **Example plugins** — `git-tool`, `docker-tool`, `jira-tool`

### v1.0 — Production Release
**Target:** Q4 2027

- [ ] All v0.6–v0.9 milestones complete and stable
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
| Azure OpenAI provider | Azure-specific auth and endpoint routing |
| `crustly run` pipelines | Chain multiple prompts with conditional logic in a YAML file |
| Catalog-based, benchmark-ranked GGUF recommend | Rank models *not yet downloaded* against a benchmark dataset (llamastash's `recommend`, minus the VRAM-fit-only subset already covered by v0.5.3's hardware-aware local selection). Needs an external, CI-refreshed benchmark pipeline — deferred until a concrete need shows up; see `ccguf-managment-imrpoment-plan.md` Phase M10 |

---

## Guiding Principles

1. **Terminal-first** — never require a browser or GUI; every feature must work over SSH.
2. **Privacy by default** — local LLM support is a first-class citizen, not an afterthought.
3. **Explicit approval** — dangerous operations always pause for user confirmation; no silent side effects.
4. **Measurable before optimised** — add benchmarks before claiming performance improvements.
5. **Security before features** — path traversal, injection, and API key hygiene are non-negotiable pre-conditions for 1.0.
6. **Depth over breadth** — compete on resource efficiency, permission-model rigor, and
   auditability rather than chasing provider count, plugin ecosystems, or multi-frontend
   surface area; see `differentiation-strategy-vs-opencode.md` for the reasoning and the
   explicit list of what this project deliberately does not chase.
