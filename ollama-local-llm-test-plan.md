# Ollama Local LLM Test Plan

Scope: verification of the two most recently merged pull requests —

- **PR #14 — "Add ollama-rs integration plan"** (merged 2026-07-02, branch
  `claude/ollama-rs-integration-8an4bc`): native Ollama provider
  (`src/llm/provider/ollama.rs`), model management
  (`src/llm/provider/ollama_models.rs`), CLI `ollama` subcommand, config
  (`OllamaProviderConfig`), factory wiring. Design doc:
  [`ollama-rs-integration-plan.md`](./ollama-rs-integration-plan.md).
- **PR #15 — "Add TUI ergonomics improvement plan"** (merged 2026-07-03,
  branch `claude/tui-ergonomics-ollama-4nym3m`): Enter/Shift+Enter
  keybinding swap, clipboard copy/paste, Model Info panel (`Ctrl+O`),
  Provider Switch dialog (`Ctrl+W`), Auto Mode (`Shift+Tab`), `/skills` and
  `/mcp` slash commands. Design doc:
  [`ergonomy-improvment.md`](./ergonomy-improvment.md).

The distinguishing requirement of this plan (vs. the design docs' own
"Plan de test" / acceptance-criteria sections) is **Section 3**: every
functional scenario is exercised against a real local model pulled through
Crustly's own tooling — not a mocked/stubbed Ollama server — so the two
features are validated the way an actual user experiences them together:
download a model from inside Crustly, then drive both the provider and the
ergonomics surface against it in one continuous session.

## 1. Objective

Confirm that:

1. The native Ollama provider works end-to-end (chat, streaming, tool
   calls, vision, embeddings, perf metrics) against a locally-downloaded
   model, without regressing the existing OpenAI-compatible route to
   Ollama/LM Studio.
2. All five TUI ergonomics phases work correctly and interoperate cleanly
   with a live Ollama session (e.g., Auto Mode + native Ollama tool calls;
   Model Info panel showing real perf metrics; Provider Switch dialog
   actually switching to a downloaded model).
3. Both features are safe by default (Auto Mode guardrails, sandbox policy
   layer) and degrade gracefully when Ollama or the `ollama` build feature
   is unavailable.

## 2. Test Environment Setup

### 2.1 Build matrix

| Build | Command | Purpose |
|---|---|---|
| Default (no LLM extras) | `cargo build` | Confirm `ollama` feature is truly optional; app must still build/run. |
| Ollama only | `cargo build --features ollama` | Primary build under test. |
| All providers | `cargo build --features all-llm` | Non-regression: Ollama coexists with OpenAI/Anthropic/Bedrock/Qwen. |

### 2.2 Automated test suite (regression gate — run first, before any manual steps)

```bash
cargo fmt --check
cargo clippy --all-targets --features all-llm -- -D warnings
cargo test --no-default-features
cargo test --features ollama
cargo test --features all-llm
```

Expected: all pass. These cover the unit tests already present in
`src/llm/provider/ollama.rs` (provider construction, request/response
mapping, error mapping, `keep_alive` parsing, perf-metric conversion, tool
schema conversion), `src/llm/provider/ollama_models.rs` (list/pull/delete/
show/embeddings against an in-process mock HTTP server), `src/tui/
ollama_download.rs` (suggestion filtering, pull-progress fraction), and the
ergonomics-side tests in `src/tui/events.rs`/`src/tui/render.rs`/`tests/
cli_test.rs` (submit/newline key matching, auto-approve CLI flag parsing,
status-bar mode labels).

### 2.3 Local LLM host

- A real, running Ollama daemon (`ollama serve`), default host
  `http://127.0.0.1:11434` (matches `DEFAULT_OLLAMA_HOST` in
  `src/llm/provider/ollama.rs:45`).
- **Do not pre-pull the test model with the `ollama` CLI.** Section 3
  specifically validates Crustly's own download path (`Ctrl+D` in the TUI,
  and `crustly ollama pull` on the CLI) — starting from zero models
  installed is part of the test.
- Machine capable of running at least one small coding model
  (`qwen2.5-coder:7b` or `llama3.2:3b`) at interactive speed; see the
  README's "Recommended Local Models for Coding" section for hardware
  guidance.
- A second, tiny model pulled for the Provider Switch scenario (Section
  5.3) so switching is observably different from the first (e.g.
  `llama3.2:3b` if the primary test model is `qwen2.5-coder:7b`).

### 2.4 Config

`config.toml` should **not** pre-configure `[providers.ollama]` at the
start of Section 3 — the goal is to prove the download-then-use flow works
from a clean slate. Configure it only for Section 6 (config-file-driven
non-regression checks). Reference snippet:

```toml
[providers.ollama]
host = "http://127.0.0.1:11434"
model = "qwen2.5-coder:7b"
# keep_alive = "30m"
# num_ctx = 8192
```

## 3. Local Model Acquisition Through Crustly (prerequisite for Sections 4–5)

| ID | Objective | Steps | Expected | Priority |
|---|---|---|---|---|
| TC-DL-01 | CLI pull with no models installed | `ollama list` (native) confirms zero models; run `crustly ollama pull qwen2.5-coder:7b` | Progress prints (manifest → layers → verifying → success); command exits 0; `crustly ollama list` now shows the model with a size. | P0 |
| TC-DL-02 | TUI download dialog, empty query | Launch `cargo run --features ollama` (no config yet), open chat, press `Ctrl+D` | Dialog opens; suggestion list shows curated models (`qwen2.5-coder:7b`, `gemma3:12b`, `llama3.1:8b`, `llama3.2:3b`, `mistral:latest`, `deepseek-r1:14b`) plus any already-installed models deduplicated at the top. | P0 |
| TC-DL-03 | TUI download dialog, filtered query | In the dialog, type `llama` | List narrows to `llama3.1:8b`/`llama3.2:3b` only, case-insensitively; `mistral:latest` disappears. | P1 |
| TC-DL-04 | TUI download with live progress | Select/confirm a model not yet installed and start the pull | A live progress bar/percentage updates from the streamed `PullProgress` (status text changes: "pulling manifest" → "pulling `<digest>`" with a growing fraction → "success"); dialog reports completion. | P0 |
| TC-DL-05 | Cancel an in-flight pull | Start a pull, press `Esc` before it finishes | The background pull task is aborted (no zombie download); dialog closes cleanly; re-opening `Ctrl+D` does not show a stuck "downloading" state. | P1 |
| TC-DL-06 | Pull a name that does not exist | `crustly ollama pull definitely-not-a-real-model:latest` | Clear error surfaced (not a panic or silent hang); non-zero exit code. | P1 |
| TC-DL-07 | `crustly ollama show` after pull | `crustly ollama show qwen2.5-coder:7b` | Prints license, parameters, template, capabilities without error. | P2 |
| TC-DL-08 | `crustly ollama embed` | `crustly ollama embed nomic-embed-text "hello world"` (pull the embed model first if needed) | Returns an embedding vector; no crash if the model lacks embedding capability (clear error instead). | P2 |
| TC-DL-09 | Non-ollama build attempts a pull | `cargo run` (default features, no `ollama`), open `Ctrl+D` dialog, attempt a pull | Dialog still opens (suggestions list still works — it's feature-independent), but starting the pull reports: *"This build of crustly was compiled without the 'ollama' feature. Rebuild with `--features ollama` (or `all-llm`)."* — no silent no-op. | P0 |

## 4. Native Ollama Provider — Functional Tests (PR #14)

Preconditions: `[providers.ollama]` configured (Section 2.4), model from
Section 3 available, `--features ollama` build.

| ID | Objective | Steps | Expected | Priority |
|---|---|---|---|---|
| TC-OL-01 | Basic chat completion | Send a simple prompt ("What is 2+2?") in Chat mode | Correct-shaped response; provider badge reads "ollama"; token usage populated from `prompt_eval_count`/`eval_count`. | P0 |
| TC-OL-02 | Streaming | Send a longer prompt and observe token-by-token rendering | Text streams incrementally (not all-at-once); stream terminates cleanly with a final `MessageStop`. | P0 |
| TC-OL-03 | Tool calling | Ask the agent to read a file in the repo ("show me the contents of Cargo.toml") | Model emits a tool call; `read_file` executes (subject to approval mode); result is fed back and a final answer is produced. | P0 |
| TC-OL-04 | Vision (if a vision model, e.g. `llava:13b`, is pulled) | Switch to the vision model (Section 5.3), attach/paste a base64 image, ask "what is this?" | Image is embedded via `Image::from_base64`; model responds referencing image content; `supports_vision()` correctly gates this (no vision UI offered on a non-vision model). | P2 |
| TC-OL-05 | Reasoning model, explicit `thinking` field | Pull and switch to a reasoning-capable model (e.g. `deepseek-r1:14b`) and ask a multi-step question | Response shows a distinct "thinking" segment separate from the final answer (from `message.thinking`, not `<think>` tag scraping). | P2 |
| TC-OL-06 | Reasoning model, `<think>` tag fallback | If a model emits `<think>...</think>` inline instead of the `thinking` field | Text before/after tags is correctly split into a Thinking block and a Text block; no literal `<think>` tags leak into the visible answer. | P2 |
| TC-OL-07 | `keep_alive` control | Set `keep_alive = "0"` in config, send a message, then immediately check `ollama ps` (native CLI) | Model unloads immediately after the response (vs. staying resident with `"-1"` or a duration like `"5m"`). | P2 |
| TC-OL-08 | `num_ctx` override | Set `num_ctx = 32768`, open Model Info panel (`Ctrl+O`, Section 5.2) | Context window displayed matches the override, not the 8192 default. | P2 |
| TC-OL-09 | Perf metrics surfaced | Send a message, then open Model Info panel | Load duration, prompt-eval duration, eval duration, total duration, and derived tokens/sec are all populated and plausible (not zero/garbage) for a completed response. | P0 |
| TC-OL-10 | Cold vs. warm start | Send a message right after a fresh model load, then a second message immediately after | First response's `model_was_loaded` reflects a nonzero load duration; second response's load duration is ~0 (model already warm). | P2 |
| TC-OL-11 | Model-not-found error | Point `providers.ollama.model` at a model that was never pulled, send a message | Clear `ModelNotFound`-style error surfaced to the user, not a generic crash/hang. | P1 |
| TC-OL-12 | Host unreachable | Stop the Ollama daemon (`systemctl stop ollama` / kill the process), send a message | Clear network-error message (mapped through `map_ollama_error`'s `ReqwestError`/network_error path); app remains usable (no panic), and recovers once Ollama is restarted. | P1 |
| TC-OL-13 | Invalid host string in config | Set `providers.ollama.host = "not a url"` | Falls back to `http://127.0.0.1:11434` with a logged warning, not a startup panic. | P2 |
| TC-OL-14 | JSON/structured output mode | Request a JSON-formatted answer (`response_format` json_object or a schema) | Response is valid JSON matching the requested shape/schema. | P2 |
| TC-OL-15 | Cost reporting | Check any cost/usage display after an Ollama exchange | Cost always reads $0.00 (`calculate_cost` is hardcoded 0 for local inference) — confirms no accidental cloud-pricing math applied to local usage. | P2 |

## 5. TUI Ergonomics — Functional Tests (PR #15)

All keybindings below are global (work regardless of input focus) unless
noted. Status-bar hint (bottom line) should read: `Shift+Tab: Auto Mode │
Ctrl+H: Help │ Ctrl+D: Download Model │ Ctrl+O: Model Info │ Ctrl+W: Switch
Model │ Ctrl+K: Clear │ Ctrl+L: Sessions │ Ctrl+N: New │ Ctrl+C: Quit` —
verify it matches exactly as a baseline sanity check before the detailed
cases.

### 5.1 Phase 1 — Send / Newline keybinding swap

| ID | Objective | Steps | Expected | Priority |
|---|---|---|---|---|
| TC-ERG-01 | Plain Enter sends | Type a message, press `Enter` | Message sends immediately (no newline inserted first). | P0 |
| TC-ERG-02 | Shift+Enter inserts newline (Kitty-capable terminal, e.g. Kitty/WezTerm/recent iTerm2) | Type text, press `Shift+Enter`, type more text, press `Enter` | A newline is inserted at cursor; multi-line message sends as one message on the following plain `Enter`. | P0 |
| TC-ERG-03 | Alt+Enter fallback (legacy terminal, e.g. plain xterm/tmux without passthrough) | In a non-Kitty terminal, press `Alt+Enter` | Newline inserted; UI indicates fallback mode is active (status bar/help screen). | P0 |
| TC-ERG-04 | Ctrl+Enter legacy alias | Press `Ctrl+Enter` | Still sends the message (no regression for old muscle memory). | P1 |
| TC-ERG-05 | Ctrl+Shift+Enter regression guard | Press `Ctrl+Shift+Enter` | Sends the message — must **not** insert a newline (this was a real bug found and fixed during PR #15's review; confirm the fix holds). | P0 |
| TC-ERG-06 | Help screen / README accuracy | Open `Ctrl+H`, compare against `README.md` shortcut tables | Both reflect `Enter`=send, `Shift+Enter`/`Alt+Enter`=newline; no stale reference to the old `Ctrl+Enter`-only scheme. | P2 |
| TC-ERG-07 | Startup/shutdown terminal-state integrity | Force-kill Crustly mid-session (`Ctrl+\` or `kill -9`) after Kitty protocol flags were pushed, then restart the terminal | Terminal is not left stuck in raw/alternate-screen mode (regression guard for the startup-cleanup-leak bug fixed in PR #15's review pass). | P1 |

### 5.2 Phase 2 — Copy/paste ergonomics

| ID | Objective | Steps | Expected | Priority |
|---|---|---|---|---|
| TC-ERG-08 | Mid-buffer editing | Type text, move cursor with Left/Right/Home/End into the middle, insert/delete characters | Edits apply at the cursor position, not just at the end (confirms `tui-textarea` migration, not the old append-only `String` buffer). | P0 |
| TC-ERG-09 | Word-jump / word-delete | Use Ctrl+Left/Right to jump words, Ctrl+Backspace/Delete to delete a word | Cursor/deletion operate on whole words. | P2 |
| TC-ERG-10 | Bracketed paste, mid-buffer | Position cursor mid-line, paste a multi-line block from the OS clipboard | Block inserts at the cursor, not appended to the end. | P0 |
| TC-ERG-11 | `Ctrl+V` explicit clipboard paste | Copy text externally, in Crustly press `Ctrl+V` | Text is pasted at the cursor (fallback path for terminals where bracketed paste is unreliable). | P1 |
| TC-ERG-12 | `Ctrl+Y` copy last response | Get an assistant reply with plain prose (no code block), press `Ctrl+Y`, paste into an external app | Full last response text is on the OS clipboard. | P0 |
| TC-ERG-13 | `Ctrl+Y` copy code block preference | Get an assistant reply containing a fenced code block plus surrounding prose, press `Ctrl+Y` | The nearest/last code block is copied (not the whole message) — verify against `docs/…` or README wording on this behavior. | P1 |
| TC-ERG-14 | No keybinding collisions | Cross-check `Ctrl+Y`/`Ctrl+V` against all other bound shortcuts (`Ctrl+C/N/L/H/K/P/D/O/W/A/R/I`) | No two actions fire from the same chord. | P2 |

### 5.3 Phase 3 — Ollama-in-TUI polish

| ID | Objective | Steps | Expected | Priority |
|---|---|---|---|---|
| TC-ERG-15 | Model Info panel opens | During/after an Ollama exchange, press `Ctrl+O` | Panel shows active provider name ("ollama"), model name, context window, and the last message's perf metrics (load/prefill/generation/total duration, tokens/sec, warm/cold-start flag). | P0 |
| TC-ERG-16 | Model Info panel — live update during streaming | Send a message, open `Ctrl+O` while the response is still streaming | Tokens/sec and duration figures update live rather than only appearing after the stream ends. | P1 |
| TC-ERG-17 | Model Info panel on a non-Ollama provider | Configure/switch to Anthropic or OpenAI, open `Ctrl+O` | Panel still opens without crashing; perf-metrics section is absent/blank (`perf_metrics: None`) rather than showing stale Ollama numbers — confirms non-regression for other providers. | P1 |
| TC-ERG-18 | Provider Switch dialog — list | With at least two models pulled (Section 3), press `Ctrl+W` | Dialog lists locally-installed Ollama models (from `/api/tags`), distinct from the curated-suggestions list used by `Ctrl+D`. | P0 |
| TC-ERG-19 | Provider Switch dialog — empty state | On a fresh install with zero models pulled, press `Ctrl+W` | Message reads "No Ollama models installed. Use Ctrl+D to download one first." — no crash, no empty silent list. | P1 |
| TC-ERG-20 | Provider Switch — actually switches | Select the second pulled model in the dialog, confirm, send a new message | The **next** message is answered by the newly selected model (verify via Model Info panel model name, or by asking the model to identify itself if it differs enough to notice); no restart required. | P0 |
| TC-ERG-21 | Provider Switch on non-`ollama` build | `cargo run` (default features), press `Ctrl+W` | Clear error: build lacks the `ollama` feature — no silent failure. | P2 |

### 5.4 Phase 4 — Auto Mode

| ID | Objective | Steps | Expected | Priority |
|---|---|---|---|---|
| TC-ERG-22 | Default is Interactive | Fresh session, no config override | Status bar shows `⚙ Interactive` (green); every tool call still prompts for approval as before this PR. | P0 |
| TC-ERG-23 | Shift+Tab cycles modes | Press `Shift+Tab` three times | Mode cycles `⚙ Interactive → ⚡ AutoPlan (yellow) → ⚡⚡ FullAuto (red) → ⚙ Interactive`, status bar label updates each time, unmissable. | P0 |
| TC-ERG-24 | AutoPlan suppresses low-risk approvals | In `AutoPlan`, ask the agent to do something using only read-only tools (`read_file`, `ls`, `grep`) | No approval dialog appears; tools execute directly. | P0 |
| TC-ERG-25 | AutoPlan still prompts for high-risk tools | In `AutoPlan`, ask the agent to run a shell command or write/edit a file | Approval dialog **still appears** for `bash`/`write_file`/`edit_file`/`code_exec` (per `is_high_risk_tool()`); confirms AutoPlan ≠ full bypass. | P0 |
| TC-ERG-26 | FullAuto bypasses even high-risk tools | Switch to `FullAuto`, repeat TC-ERG-25's request | No approval dialog for any tool listed above; action executes directly, and still appears in session history/logs identically to a manually-approved action. | P0 |
| TC-ERG-27 | SecurityConfig hard floor still enforced | With `deny_tools`/`deny_paths` configured in `config.toml`, switch to `FullAuto`, ask the agent to touch a denied path/tool | Action is blocked regardless of Auto Mode; `render_policy_denial` (or equivalent) surfaces the denial, it is not silent. | P0 |
| TC-ERG-28 | Esc/Ctrl+C interrupts | Start a long-running FullAuto task, press `Esc` mid-execution | Execution stops immediately; Auto Mode itself is not silently disabled by the interrupt (only the in-flight action is). | P1 |
| TC-ERG-29 | Plan Mode integration | Submit a multi-step plan while in `AutoPlan`/`FullAuto` | Plan skips the `Ctrl+A` manual approval gate and proceeds straight into execution via `PlanModeState::AutoExecuting`/`FullAuto`, using the resurrected progress UI (`render_auto_exec_progress`) instead of the blocking modal. | P1 |
| TC-ERG-30 | CLI `--auto-approve`/`--yolo` actually works | `crustly run --yolo "run: touch /tmp/yolo-test && echo done"` (adjust to a benign command) | Tool executes without any prompt (this was the specific bug fixed in PR #15 — previously `--yolo` parsed but had no runtime effect); confirm the file/output is actually produced. | P0 |
| TC-ERG-31 | `autoplan` CLI inherits the fix | `crustly autoplan "<simple benign goal>"` | Executes without hanging on an approval prompt that can never be answered non-interactively. | P1 |

### 5.5 Phase 5 — `/skills` and `/mcp` discovery commands

| ID | Objective | Steps | Expected | Priority |
|---|---|---|---|---|
| TC-ERG-32 | `/skills` lists discoverable skills | Place a `SKILL.md` under `.crustly/skills/<name>/`, type `/skills`, press Enter | List view opens showing the skill's name/description alongside any others (project-local and user-global, `.crustly`/`.claude` roots), even if never invoked this session. | P0 |
| TC-ERG-33 | `/mcp` lists configured servers | Configure one `[[mcp.servers]]` entry in `config.toml`, type `/mcp`, press Enter | List view shows the server's name/command, connection status, and discovered tool count. | P0 |
| TC-ERG-34 | MCP servers actually connect at startup | With an MCP server configured, start Crustly, open `/mcp` | Status shows "connected" and a nonzero tool count — confirms the startup-wiring bug fix (previously `config.mcp.servers` was parsed but never consumed). | P0 |
| TC-ERG-35 | Unrecognized `/word` falls through to chat | Type a message that starts with `/` but isn't `/skills`/`/mcp`/etc. (e.g. `/home/user/notes.txt exists?`) | Message is sent to the LLM normally, not swallowed or erroring as an unknown command. | P1 |
| TC-ERG-36 | Dialog conventions consistency | In both `/skills` and `/mcp` views | `Esc` closes, `Up`/`Down` navigate, styling matches Sessions (`Ctrl+L`) / Help (`Ctrl+H`). | P2 |

## 6. Cross-Cutting Regression Tests

| ID | Objective | Steps | Expected | Priority |
|---|---|---|---|---|
| TC-REG-01 | OpenAI-compatible Ollama route (LM Studio style) unaffected | Configure `providers.openai.base_url = "http://localhost:11434/v1"` (no `[providers.ollama]`), chat as usual | Works exactly as before; no tok/s segment in the header (that shim doesn't return perf data) — confirms the two routes stay independent. | P0 |
| TC-REG-02 | Both Ollama routes configured side by side | Configure both `[providers.ollama]` and `providers.openai.base_url` pointing at the same daemon | Each behaves per its own code path; switching between them (config or `Ctrl+W`) doesn't cross-contaminate state. | P1 |
| TC-REG-03 | Cloud providers unaffected by ergonomics changes | With Anthropic/OpenAI configured (no Ollama), exercise Enter/Shift+Enter, clipboard, Auto Mode, `/skills`, `/mcp` | All ergonomics features work identically; no Ollama-specific code path is required for them to function. | P0 |
| TC-REG-04 | `--no-default-features` build has no dead-feature crashes | `cargo run --no-default-features` (no `ollama`), exercise `Ctrl+D`, `Ctrl+W`, Model Info panel | All still render without panicking; Ollama-specific actions report the "rebuild with `--features ollama`" message instead of crashing or silently no-op'ing. | P0 |
| TC-REG-05 | DB migration for perf-metrics columns | Open a pre-existing session DB (created before this PR) with the new build | New nullable columns backfill as `NULL` without migration errors; old sessions still load and render. | P1 |
| TC-REG-06 | Plan Mode approval (pre-existing feature) unaffected in Interactive mode | With Auto Mode left at default `Interactive`, run a Plan Mode workflow (see `docs/test-manually/test-plan-mod.md`) | Behaves identically to pre-PR-15 behavior — `Ctrl+A`/`Ctrl+R`/`Ctrl+I` gate execution as before. | P1 |

## 7. Combined End-to-End Scenario (the "real" test)

This is the scenario that most closely matches the original ask — download
a local model from inside Crustly and drive both feature sets together in
one session:

1. Start Crustly fresh (no models, no `[providers.ollama]` config), build
   with `--features ollama`.
2. `Ctrl+D` → download `qwen2.5-coder:7b` (TC-DL-02/04).
3. `Ctrl+W` → switch active session to the freshly-downloaded model
   (TC-ERG-18/20).
4. Send a coding request that requires reading a file and proposing an
   edit, in default `Interactive` mode — confirm the approval dialog
   appears for `edit_file` (baseline, pre-Auto-Mode behavior).
5. `Ctrl+O` → confirm Model Info panel shows live perf metrics for the
   exchange that just happened (TC-ERG-15).
6. `Ctrl+Y` → copy the assistant's explanation to the clipboard, paste
   externally to confirm (TC-ERG-12).
7. `Shift+Tab` twice → enter `FullAuto` (TC-ERG-23).
8. Repeat a similar edit request — confirm it now executes without a
   prompt, and still shows up in history (TC-ERG-26).
9. Type `/skills` then `/mcp` — confirm both list views open correctly
   mid-session without disrupting the active chat (TC-ERG-32/33).
10. `Ctrl+D` again → download a second, smaller model
    (`llama3.2:3b`) while the FullAuto session is still live, then
    `Ctrl+W` → switch to it, and send one more message to confirm the
    switch took effect (TC-ERG-20 repeated under Auto Mode).
11. `Shift+Tab` back to `Interactive`, confirm the very next tool call
    prompts again (TC-ERG-23, disable path).

Pass criterion: all 11 steps complete without a crash, a stuck terminal, a
silently-ignored keypress, or a mismatch between status-bar state and
actual approval behavior.

## 8. Traceability

| Design-doc acceptance criterion | Covered by |
|---|---|
| ollama-rs-integration-plan.md §7 unit/rendering/regression tests | Section 2.2 (automated suite) |
| ollama-rs-integration-plan.md §7 manual local test (real Ollama, chat/stream/tool-call/pull) | Sections 3, 4, 7 |
| ollama-rs-integration-plan.md §7 OpenAI-compat non-regression | TC-REG-01 |
| ergonomy-improvment.md Phase 1 acceptance criteria | TC-ERG-01–07 |
| ergonomy-improvment.md Phase 2 acceptance criteria | TC-ERG-08–14 |
| ergonomy-improvment.md Phase 3 acceptance criteria | TC-ERG-15–21 |
| ergonomy-improvment.md Phase 4 acceptance criteria | TC-ERG-22–31 |
| ergonomy-improvment.md Phase 5 acceptance criteria | TC-ERG-32–36 |

## 9. Out of Scope

- Load/performance benchmarking of the local model itself (tokens/sec is
  observed for correctness of the metric, not benchmarked for speed).
- `keep_alive`/provider-config plumbing into the Model Info panel labeled
  "deferred" in the Implementation Tracking checklist (explicitly called
  out as not landed in PR #15).
- Multi-iteration `PlanModeState::FullAuto{goal, iteration, max_iterations}`
  autoplan loop beyond the single-flag CLI fix (deferred to a later phase
  per `ergonomy-improvment.md`'s Phase 4a.2 note).

## 10. Sign-off Checklist

- [ ] Section 2.2 automated suite green on all three feature combinations.
- [ ] Section 3 (local model acquisition via Crustly) fully passed — no
      model was pulled with the bare `ollama` CLI as a substitute.
- [ ] Section 4 (native provider) — all P0/P1 cases passed.
- [ ] Section 5 (ergonomics, all 5 phases) — all P0/P1 cases passed.
- [ ] Section 6 (regression) — all P0 cases passed.
- [ ] Section 7 (combined E2E scenario) — completed without failure.
- [ ] Any deviations logged with reproduction steps and linked to a
      follow-up issue before sign-off.
