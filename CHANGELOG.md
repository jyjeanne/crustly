# Changelog

All notable user-facing changes to Crustly are documented here, newest first.
For the forward-looking plan, see [ROADMAP.md](ROADMAP.md).

---

### v0.5.2 — Local-Model Reliability & Per-Model Tuning

A hardening pass driven by end-to-end testing of the full agentic tool loop against local Ollama models (qwen2.5-coder, gemma4, ornith).

#### Per-Model Ollama Settings
Sampling (`temperature`/`top_p`/`top_k`), `num_ctx`, and `keep_alive` can now be set per model via `[providers.ollama.models."<name>"]` blocks, with field-by-field fallback to the provider-level values — tuning one model no longer silently degrades the others. The per-model `num_ctx` is coupled to `context_window`, so the window compaction budgets against and the window Ollama allocates can never drift. One shared construction path is used at startup and by the `Ctrl+W` model switch.

#### Approval That Means Yes
`security.allow_bash` is now strictly a no-prompt shortcut, not a wall: allowlisted operator-free commands run silently as before, while everything else — including shell-operator chains like `mkdir x && cd x && cargo init` — prompts with the full command shown verbatim, and the user's approval runs exactly that. Operator commands are never auto-trusted (the allowlist checks only the first token, so `ls && rm -rf /` always prompts), and Plan/read-only mode still rejects them outright. Previously, both non-allowlisted and operator commands were silently refused *after* the user approved them.

#### Reasoning-Only Answers Surfaced, Not Blank
Reasoning models (ornith, DeepSeek-R1, QwQ — and gemma under context pressure) that put their entire answer in the thinking channel no longer produce blank messages: the reasoning is promoted to the visible answer with a clear "this model returned only its reasoning" notice — display-only, never persisted into the history the model sees.

#### Tool Calls Recovered from Prose
Tool calls printed as ```json fenced blocks *inside* explanatory prose (qwen2.5-coder's retry pattern after a rejected command) are now recovered and executed, on top of the existing whole-message recovery. Recovery stays strict: only fenced blocks, only offered tools, only explicit arguments — bare JSON mentioned in prose is never executed.

#### `--model` CLI Flag
Override the configured default model for a single invocation of any command: `crustly --model "gemma4:12B" run "..."`.

#### Per-Model `think` Control
Ollama's native `think` parameter is now configurable per model (and provider-wide): `think = false` disables a reasoning model's thinking channel entirely, `"low"`/`"medium"`/`"high"` set an effort level. This exists because gemma4 was observed spending two minutes and ~10k tokens "thinking" mid-task and ending its turn without ever emitting the tool call it was reasoning about — with `think = false` it answers decisively in visible content (an observed 11-token tool call instead of a multi-minute spiral). An explicit extended-thinking request still wins over the configured default.

#### Resilient Plan Creation
Plan Mode now survives sparse tool calls from small local models: `plan create` and `add_task` require only a `title` (`description` defaults to empty, `task_type` to `"other"`), where the previous hard requirements sent models into "missing field" retry spirals that the loop breaker then killed — leaving one-task plans. Plan-building turns are also exempt from the silent-tool-call drift guard, since building a plan is legitimately `create` + one `add_task` per step + `finalize` with nothing to say in between.

#### Qwen3-Coder-Next & Qwen3.6-27B Support
Added `qwen3-coder-next` and `qwen3.6-27b` to the Qwen provider's known model list with their real 256K context window (previously fell back to a conservative 32K default, triggering premature context compaction). The provider also now auto-selects the OpenAI tool parser when `default_model` contains `coder-next`, matching Qwen3-Coder-Next's documented vLLM/SGLang serving recipe (`--tool-call-parser qwen3_coder`), which returns structured `tool_calls` rather than Hermes `<tool_call>` tags. See [QWEN_INTEGRATION.md](docs/guides/QWEN_INTEGRATION.md) for the updated serving instructions.

#### Bug Fixes
- **"Message not found" crash** that broke every `crustly run` invocation: sqlx-sqlite does not auto-commit `INSERT ... RETURNING`, so on the file-backed WAL pool a just-created message was invisible to the next pooled connection. Message creation now commits an explicit transaction
- **Tool-loop detector false positive** aborting consecutive `edit_file`/`write_file`/`read_file` calls to *different* paths (the signature read the `file_path` alias instead of the canonical `path` key, so distinct calls collided)
- **`Ctrl+W` model switch dropped the entire `[providers.ollama]` config** (per-model settings, sampling, `num_ctx`, `keep_alive`), so switched-to models ran unconfigured
- **TUI timestamps rendered in UTC** instead of the local timezone (chat header, session list, plan, error dialogs)
- **`Ctrl+K` (clear session) deleted messages out from under an in-flight response**, crashing its trailing DB update; it is now refused with a hint until the response finishes
- **Model-not-found errors were raw JSON**; they now say what to do (install with `ollama pull`, switch with `Ctrl+W`, or fix `default_model` in config.toml)
- **Everything typed into the chat input rendered underlined** (ratatui-textarea underlines the cursor line by default; the input now clears that style)

---

### v0.5.0 — Gemini Provider & Claude Code / Qwen Compatibility

#### Native Google Gemini Provider
Gemini joins Anthropic and OpenAI as a third fully-implemented cloud provider. It also serves Google's open-weight **Gemma 3/4** models through the same API — no local GPU or Ollama required, and Gemma usage through the Gemini API is free of charge. Supports streaming, function calling, vision, extended thinking (`thinkingConfig`/`includeThoughts`), and JSON/structured output (`responseSchema`). See the **Supported AI Providers** section in [README.md](README.md) for setup.

#### `apply_patch` Tool
A new 22nd built-in tool: real Codex-compatible multi-file patch support, so the model can describe a coordinated set of file edits in a single structured patch instead of issuing one `edit_file` call per file.

#### Claude Code / Qwen Compatibility Layer
A round of interoperability fixes so Crustly's tool set behaves the same way regardless of which model is driving it:

- **Tool name alias layer** — models trained on Claude Code's or Qwen's tool names resolve to Crustly's built-in tools without a prompt-side mapping
- **`file_path`/`directory` argument aliases and `shell` field compatibility** — accepts the parameter names Claude Code and Qwen actually send
- **`grep` defaults to regex** and both **`grep`/`glob` now respect `.gitignore`**, matching Claude Code/qwen-code semantics
- **Prior-read enforcement** — `edit_file`, `write_file`, and `apply_patch` now require the target file to have been read first in the session, catching a class of blind overwrite mistakes
- **Hardened Qwen Hermes tool-call parsing** — resilient to truncated or malformed JSON tool calls, and MCP tool naming/`edit_file` schema is now compatible with Qwen/Claude Code

#### Bug Fixes & Dependency Upgrades
- Fixed the macOS-only path boundary check incorrectly rejecting valid paths to not-yet-existing files under a symlinked root
- Updated the markdown renderer for the `pulldown-cmark` 0.13 API
- Completed the `ratatui` 0.28 → 0.30 upgrade, migrating off the now-unmaintained `tui-textarea`

### Phase 4 Tools — Claw Code Parity (v0.4.1)

Six new tools bring Crustly to full feature parity with Claw Code's tool set:

| Tool | Description |
|------|-------------|
| `web_fetch` | Fetch a URL and extract readable text (strips scripts/styles, converts HTML to plain text) |
| `todo_write` | Persistent todo list — read or write a structured task list stored in `.crustly/todos.json` |
| `ask_user` | Pause agent execution and ask the user a clarifying question before continuing |
| `skill` | Load a named slash command from `SKILL.md` files (project-local `.crustly/skills/` or user-global `~/.claude/skills/`) |
| `agent` | Spawn a background sub-agent for a focused task; writes output to `.crustly/agents/<id>.md` |
| `powershell` | Execute PowerShell (`pwsh` / `powershell.exe`) commands with background execution, read-only mode guards, and cmdlet allowlisting |

Sub-agents are recursion-safe: agents spawned by `AgentTool` cannot themselves spawn further sub-agents.

### Real-time Streaming TUI
LLM responses now render **token-by-token** as they arrive. A live `[streaming]` label appears while the model is generating; the final message — complete with thinking block and syntax highlighting — replaces it when the stream ends. For models that embed reasoning inside `<think>` tags (DeepSeek-R1, QwQ-32B via Ollama), thinking content is **filtered from the live view** so the user only ever sees clean visible text during generation.

### Reasoning Display
Extended thinking is now a first-class citizen in the TUI. Crustly surfaces reasoning content from three sources:

| Source | How |
|--------|-----|
| **Anthropic extended thinking** | `ThinkingDelta` events in the stream |
| **DeepSeek-R1 direct API** | `reasoning_content` field in the response |
| **Ollama reasoning models** (DeepSeek-R1, QwQ-32B) | `<think>…</think>` tag extraction |

Press **`t`** on any assistant message to expand or collapse the `[Thinking ▸]` panel. The panel is collapsed by default to keep the chat clean.

### Context Compaction
When the conversation window reaches **80% capacity**, Crustly automatically compacts the context: older turns are summarised, the last 10 turns are preserved verbatim, and a `CompactionRecord` is written to SQLite before any context is modified — so a failed compaction leaves the session untouched.

### Smart Model Routing
Crustly automatically picks the right model tier based on what you're asking:

| Tier | When | Examples |
|------|------|---------|
| **Fast** | Simple lookups | "what is", "list", "summarize" |
| **Balanced** | General work | Default for most requests |
| **Powerful** | Deep reasoning | "refactor", "debug", "architecture" |

No extra API call — keyword classification runs locally in microseconds.

### Parallel Tool Dispatch
Independent tool calls (multiple `read_file`, `glob`, `grep`) now execute **concurrently** via `join_all`. Benchmark result: **≥40% faster** than sequential dispatch on 10 concurrent reads (measured: 62% speedup on typical hardware).

### Tool Result Caching
Read-only tools (`read_file`, `glob`, `grep`) are cached with a configurable TTL. A cache hit skips filesystem I/O entirely. Write tools are never cached.

### Provider Failover
If the primary LLM provider returns a rate-limit or timeout, Crustly automatically retries on a secondary provider. Failover events are logged with a `[FAILOVER]` tag.

### Sandbox & Permission Policy
- **Path boundary enforcement**: symlinks and `../../` escapes outside the project root are denied at the tool layer — no prompt needed
- **Bash allowlist**: restrict shell commands to an explicit set (e.g. `["cargo", "git"]`)
- **Composable `AndPolicy`**: chain multiple rules; first `Deny` wins

### Episodic Memory
Summaries from past sessions are injected into new conversation contexts within a configurable token budget. Oversized summaries are **truncated** (not dropped) to fit the budget exactly.

### Codebase Index
A file watcher (powered by the `notify` crate) re-indexes Rust source files automatically on save. The symbol index supports `query_symbol` and `fts_search` for functions, structs, enums, traits, consts, and more.

### MCP Tool Server Integration
Configure external MCP tool servers in `.crustly/config.toml` under `[[mcp.servers]]`. They are auto-registered as tools at startup. Built-in tool names take precedence. Crashed or unreachable MCP servers return graceful errors — no panics, no hangs. Type `/mcp` in the chat input to see each configured server's connection status and discovered tool count without leaving the TUI.

### `/skills` and `/mcp` Discovery Commands
Type `/skills` or `/mcp` and press Enter in the chat input to open a list view — these are intercepted before being sent to the LLM, so a message that happens to start with `/` for any other reason (e.g. a file path) is still sent normally.

- **`/skills`** — lists every discoverable skill (project-local `.crustly/skills/`/`.claude/skills/` and user-global `~/.config/crustly/skills/`/`~/.claude/skills/`) with its name and description, parsed from each `SKILL.md`'s frontmatter.
- **`/mcp`** — lists every configured `[[mcp.servers]]` entry with its connection status and discovered tool count, as of startup.

Both views follow the existing dialog conventions: `Esc` closes, `Up`/`Down` navigate.

### AWS Bedrock Support
AWS Bedrock is now a supported provider. Enable it with `--features aws-bedrock` and configure your AWS credentials as usual.

### Native Ollama Provider & Model Download Dialog
Crustly now speaks Ollama's native `/api/chat` protocol directly (via [`ollama-rs`](https://github.com/pepperoni21/ollama-rs), `--features ollama`), alongside the existing OpenAI-compatible route. This unlocks:

- **Runtime performance metrics** — generation throughput (tokens/sec), model load time, and warm/cold-start status shown live in the TUI header and under each reply
- **`Ctrl+D` Model Download dialog** — type or pick an Ollama model from suggested/installed names, pull it with a live progress bar, and cancel mid-download with `Esc`, all without leaving the TUI
- **`Ctrl+O` Model Info panel** — active provider/model, context window, and the last response's performance metrics (load/prefill/generation/total time, tokens/sec), all without leaving the TUI
- **`Ctrl+W` Provider Switch dialog** — switch to a different locally-installed Ollama model at runtime, without editing config.toml or restarting the app
- **Model management CLI** — `crustly ollama list|pull|rm|show|embed`
- **`keep_alive` / `num_ctx` control** and provider identity (badge + icon) shown in the header for every configured provider, not just Ollama

Both the native (`providers.ollama`) and OpenAI-compatible (`providers.openai.base_url`) routes to Ollama can be configured side by side — see the "Native Ollama" section under Providers in [README.md](README.md), and [`ollama-rs-integration-plan.md`](ollama-rs-integration-plan.md) for the full design.
