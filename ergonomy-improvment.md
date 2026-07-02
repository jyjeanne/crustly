# TUI Ergonomics Improvement Plan

Status: In Progress — see Implementation Tracking
Owner: TUI team
Related: `ollama-rs-integration-plan.md` (native Ollama provider, already implemented)

## Implementation Tracking

Each phase is split into small, independently-committable steps so progress
can land incrementally instead of one large change per phase. Check items
off as they land; leave a one-line note (commit hash) next to completed
items.

**Post-implementation review**: after landing Phase 1, 4a, and 3.1, a
high-effort code review (7 parallel finder angles + direct verification)
found and fixed 10 issues, most notably a real regression where
`Ctrl+Shift+Enter` silently inserted a newline instead of submitting (fixed
by excluding `CONTROL` from `is_newline`), and a terminal-state leak where a
failure during startup (after the Kitty protocol flags were pushed) would
skip cleanup entirely, leaving the user's terminal stuck in raw/alternate-
screen mode. Also fixed: a duplicated linear scan over `app.messages` per
render, several formatting inconsistencies in the new Model Info panel
(duration/tokens-per-second precision, close-hint styling) versus existing
UI conventions, a misaligned help-screen row, and a synchronous/unbounded
terminal query on the startup path (now timeout-bounded via
`spawn_blocking`). See commit history for details.

- [x] **Phase 1 — Send / Newline Keybinding Swap**
  - [x] 1.1 Kitty keyboard protocol detection at startup (`runner.rs`), with
        graceful push/pop and no-op on unsupported terminals.
  - [x] 1.2 Swap `is_submit`/newline logic in `events.rs`/`app.rs`
        (`Enter`=send, `Shift+Enter`=newline, `Ctrl+Enter` kept as legacy
        alias, `Alt+Enter` as non-Kitty newline fallback).
  - [x] 1.3 Update UI copy (input hint, help screen) and README shortcut
        tables to reflect the new binding + active fallback mode.
- [x] **Phase 4a — Auto Mode CLI bug fix** *(pulled forward: small,
      isolated, independent of all other phases)*
  - [x] 4a.1 Wire `--auto-approve`/`--yolo` in `cmd_run` to actually call
        `.with_auto_approve_tools()`.
  - [x] 4a.2 `cmd_autoplan` already delegates to `cmd_run(..., true, ...)`,
        so it inherits the 4a.1 fix automatically - no separate change
        needed for the CLI flag to have real effect. The richer
        `PlanModeState::FullAuto` multi-iteration loop (the `let _ = state`
        in `cmd_autoplan`) remains a separate, larger feature, deferred to
        Phase 4b alongside the TUI `Shift+Tab` mode cycle.
- [ ] **Phase 3 — Ollama-in-TUI Polish**
  - [x] 3.1 Model Info panel (`Ctrl+O`) — provider, model, context window
        (via new `AgentService::provider_context_window()` /
        `Provider::context_window()`), and the last assistant message's
        `PerfMetrics` (load/prefill/generation/total duration, warm/cold
        start, tokens/sec). `keep_alive` deliberately **deferred**: it's a
        private field on `OllamaProvider` behind `Arc<dyn Provider>`, not
        reachable without either a new `Provider` trait method or plumbing
        `Config` into `App` just for this label — not worth it for 3.1's
        scope; revisit if/when Phase 3.3's provider switcher needs
        provider-specific config access anyway.
  - [x] 3.2 Investigation found this was actually a **bug**, not just a
        missing feature: since the TUI only ever uses the streaming path
        (never `Provider::complete()`), and `StreamEvent` had no slot for
        `PerfMetrics`, the Model Info panel built in 3.1 was reading fields
        that were *always* `None` in real usage - Ollama's `stream()`
        computed `final_perf` correctly but explicitly discarded it
        (`let _ = final_perf;`), and `drain_stream_to_response` hardcoded
        `perf_metrics: None`. Fixed by adding `perf_metrics: Option<PerfMetrics>`
        to `StreamEvent::MessageDelta` (types.rs) and threading it through
        Ollama's `stream()` → `drain_stream_to_response` (service.rs) →
        `LLMResponse`/`AgentResponse` → `DisplayMessage`, closing the loop
        so the panel now reflects real data after each response. **True
        mid-stream (before `done`) throughput remains structurally
        unavailable for every wired provider** (Ollama/OpenAI/Anthropic/
        Qwen all only report token counts/timing on the final chunk) - a
        genuinely "live" indicator would have to be a client-side estimate
        from chunk arrival timing, clearly labeled as approximate. Deferred
        as a separate, smaller enhancement rather than bundled into this
        bug fix.
  - [x] 3.3 In-TUI provider/model quick-switch dialog, scoped to **switching
        between locally-installed Ollama models** (`Ctrl+W`), not full
        cross-provider switching. Reasoning: `App` has no `Config` access
        today (only used transiently at startup to build the initial
        `AgentService`, then dropped) and `Config` is where cloud-provider
        API keys live - enumerating/switching to e.g. a configured
        Anthropic or OpenAI provider would need `Config` plumbed into
        `App` as a new field, a bigger and separately-reviewable change.
        Ollama needs no secrets and already has `App.ollama_host` +
        `ollama_models::list_models()` wired for the Model Download
        dialog, reused here directly. Also found and fixed a correctness
        trap during design: `AgentService` isn't `Clone` and has no
        in-place provider setter, so naively rebuilding it via
        `AgentService::new(new_provider, context)` would silently drop the
        tool registry (back to empty), the approval callback (silently
        disabling interactive tool approval), the compaction pool, and the
        session's tool-result cache. Fixed by adding a genuine
        `AgentService::set_provider(&mut self, ...)` mutator and swapping
        it in place via `Arc::get_mut(&mut self.agent_service)`, which
        fails safely (visible error, no swap) rather than corrupting state
        if a background task is holding a clone during an in-flight
        request. Full cross-provider switching (with secrets) is a
        separate follow-up, not scoped here.
  - [ ] 3.4 (new, optional) Client-side "~N tok/s (live)" estimate during
        active streaming, derived from chunk arrival timing, replaced by
        the authoritative number once the response completes.
- [ ] **Phase 2 — Copy / Paste Ergonomics**
  - [x] 2.1 Migrate chat input from `String` to `tui-textarea::TextArea`.
        **Found and fixed a blocking dependency issue first**: `Cargo.lock`
        was gitignored despite this being a binary crate, and a fresh
        resolve put `tui-textarea` (and the already-unused `ratatui-image`)
        on `ratatui 0.30.2` while crustly itself pins `0.26` - two
        incompatible copies of the `Widget` trait in the same build, which
        would have made `f.render_widget(textarea.widget(), area)` fail to
        compile. Fixed via `cargo update -p ratatui@0.30.2 --precise
        0.26.3` (both dependencies declare wide-open ranges like `>=0.23`
        that 0.26.3 already satisfies) and committing the resulting
        lockfile so the fix is durable, not a one-off local state.
        Migration itself: `App.textarea: TextArea<'static>` replaces
        `input_buffer: String`; `handle_chat_key` now wires explicit
        cursor movement (arrows, Ctrl+Left/Right word-jump, Home/End) and
        word-delete (Ctrl+Backspace/Delete) rather than using
        `TextArea::input()`'s built-in Emacs-style keymap, since that
        keymap's `Ctrl+<letter>` bindings collide with crustly's own
        global shortcuts (its `Ctrl+W` is "delete word", crustly's is
        Provider Switch, etc.) - `input_without_shortcuts()` handles plain
        character/Backspace/Delete/Enter insertion instead, with an
        explicit `KeyCode::Enter => {}` no-op arm to stop it from
        resurrecting the Phase-1 "blank-buffer Enter inserts a newline"
        bug through its own unconditional Enter handling. Paste and the
        Plan Mode revision pre-fill now insert at the cursor instead of
        always appending at the end. `render_input` clones the `TextArea`
        per frame (keeping `render_*` functions read-only like every other
        one) to apply block/style before calling `.widget()`.
  - [x] 2.2/2.3 Added `arboard`. `Ctrl+Y` copies the last assistant
        response to the system clipboard - combined the two planned
        actions into one smart keybinding rather than two separate ones:
        if the response has a fenced code block, only the *last* code
        block's raw content is copied (new `markdown::last_code_block()`,
        reusing `pulldown_cmark` rather than hand-rolling fence matching);
        otherwise the full response text is copied. `Ctrl+V` is the
        clipboard-paste fallback, inserting at the cursor alongside the
        existing bracketed-paste path. Both fail gracefully into
        `error_message` instead of panicking or hanging - verified
        empirically in this headless sandbox (no X11/Wayland) that
        `arboard::Clipboard::new()` fails in ~3ms, not a multi-second
        hang, so there's no need for a timeout wrapper around it.
- [ ] **Phase 4b — Auto Mode TUI toggle** *(depends on 1.2 for the
      finalized keybinding scheme)*
  - [x] 4b.1 `Shift+Tab` mode-cycle wired to a new `App` field +
        `ApprovalCallback`. **Correction to the original design**: reused
        `PlanModeState::is_high_risk_tool(tool_name)` (a plain, stateless
        classifier) rather than `tool_needs_approval(&self, ..)` as
        originally named - the latter is an instance method keyed off a
        *specific* `PlanModeState` variant tied to an in-progress plan
        (`AutoExecuting { task_index, total, .. }` etc.), which isn't
        generally available for an arbitrary tool call happening outside
        active plan-task execution (e.g. a normal chat message that
        triggers a tool call with no plan involved at all). The
        stateless classifier is the right fit for a global toggle that
        must work regardless of whether a plan is active.
      - New `App.auto_mode: Arc<Mutex<PlanExecMode>>`, seeded from
        `config.plan_mode.mode` and shared (not copied) with the approval
        callback built in `cli::cmd_chat` - toggling it in the TUI takes
        effect on the very next tool call, from any `AppMode`.
      - The core decision (`Interactive` never bypasses; `AutoPlan`
        bypasses everything except `bash`/`write_file`/`edit_file`/
        `code_exec`; `FullAuto` bypasses everything) is a pure, directly
        unit-tested function (`cli::auto_mode_bypasses_approval`), kept
        separate from the channel/TUI plumbing specifically so this
        security-relevant logic has isolated test coverage.
      - Documented, not silently expanded: `is_high_risk_tool()` doesn't
        include `powershell` (an equally-capable command-execution tool on
        Windows) - `AutoPlan` will bypass it the same as a read-only tool.
        Left as the open decision it already was rather than unilaterally
        widening a classifier also used by pre-existing plan auto-run code
        this phase didn't otherwise touch.
      - `SecurityConfig`/`sandbox.rs`'s policy chain is untouched - it's a
        separate, earlier check in `ToolRegistry::execute()` and stays
        enforced under every Auto Mode level.
      - Status bar always shows the current level (`⚙ Interactive` /
        `⚡ AutoPlan` / `⚡⚡ FullAuto`) with a distinct background color,
        survives error/processing states, and is documented in the Help
        screen - satisfies the "persistent, unmissable indicator"
        requirement from 4b.3 without needing the dead dialog components.
  - [ ] 4b.2 **Not implemented**: Plan Mode's own approval step
        (`Ctrl+A`/`R`/`I` in `handle_plan_key`) is untouched - Auto Mode
        only affects *individual tool-call* approval via the callback, not
        *plan* approval. A plan the agent creates still requires an
        explicit `Ctrl+A` before `execute_plan_tasks()` runs, even with
        `FullAuto` active. Wiring plan-level auto-approval through
        `PlanModeState::AutoExecuting`/`FullAuto` is separate follow-up
        work, not bundled into this pass.
  - [ ] 4b.3 **Partially implemented**: the persistent status-bar
        indicator is done (see 4b.1 notes above). Resurrecting
        `render_auto_exec_progress`/`render_policy_denial`
        (`tui/components/dialogs/mod.rs`, still dead code) as a visible
        "here's what Auto Mode just did" progress trail is **not** done -
        auto-approved tool calls currently proceed with no more visual
        feedback than a normal streamed response gets. Left as a UX
        follow-up rather than blocking the core safety mechanism on it.
- [ ] **Phase 5 — `/skills` and `/mcp`**
  - [ ] 5.1 Slash-command interception layer in `handle_chat_key`.
  - [ ] 5.2 Fix MCP config-wiring gap (`register_mcp_server` never called).
  - [ ] 5.3 `/mcp` list view with connection status + tool count.
  - [ ] 5.4 Skill enumeration function + `/skills` list view.
  - [ ] 5.5 Optional CLI symmetry (`crustly mcp list`, `crustly skill list`).

## Goal

Improve day-to-day usability of the Crustly TUI:

1. Make `Enter` send the message and `Shift+Enter` insert a newline (swap of
   today's `Ctrl+Enter` send / `Enter` newline scheme).
2. Make copy/paste into and out of the TUI reliable and ergonomic.
3. Expose the existing native Ollama integration (`ollama-rs`) more directly
   inside the TUI (not just via config file + `Ctrl+D` download dialog).
4. Add an opt-in "Auto Mode" that bypasses manual tool-call / plan approval
   prompts for users who want the agent to run unattended.
5. Add `/skills` and `/mcp` slash commands to list installed skills and
   configured MCP servers from inside the TUI.

## Current State (as of this plan)

- TUI framework: `ratatui` 0.26 + `crossterm` 0.27 (`event-stream` feature).
  `tui-textarea` 0.4 is a declared dependency but **unused** — the chat input
  is a raw `String` (`App::input_buffer`) mutated by manual push/pop.
- Keybindings are hardcoded, not user-configurable:
  - Send message: `Ctrl+Enter` — `keys::is_submit()` in `src/tui/events.rs:268-271`.
  - Newline: plain `Enter` — `src/tui/app.rs:501-503`.
  - Global bindings already taken: `Ctrl+C` quit, `Ctrl+N` new session,
    `Ctrl+L` list sessions, `Ctrl+H` help, `Ctrl+K` clear session,
    `Ctrl+P` plan mode, `Ctrl+D` model download, `Ctrl+A`/`Ctrl+R`/`Ctrl+I`
    plan approve/reject/revise (`src/tui/events.rs:234-266`, `src/tui/app.rs:544,563,577`).
  - No `KeyModifiers::SHIFT` handling exists anywhere in the input path.
- Paste: `EnableBracketedPaste` is on (`src/tui/runner.rs:25`), and pasted
  text is appended to `input_buffer` (`src/tui/app.rs:241-246`). There is
  **no system-clipboard copy-out** anywhere in the codebase (`arboard` is not
  a dependency); it only appears as a TODO in `ROADMAP.md` and
  `docs/development/SPRINT_5_PLAN.md` / `SPRINT_9_PLAN.md`.
- Ollama: fully implemented native provider (`src/llm/provider/ollama.rs`),
  model management (`src/llm/provider/ollama_models.rs`), TUI download dialog
  (`src/tui/ollama_download.rs`, `Ctrl+D`), config (`OllamaProviderConfig` in
  `src/config/mod.rs:351-393`), factory wiring
  (`src/llm/provider/factory.rs:130-191`). Documented remaining gaps in
  `ollama-rs-integration-plan.md`: no "Model Info" panel in the TUI, no live
  streaming perf metrics attached to `StreamEvent`, switching providers
  requires editing config and restarting (no in-TUI switcher).

## Phase 1 — Send / Newline Keybinding Swap

### Changes

- `keys::is_submit()` (`src/tui/events.rs:268-271`): trigger on plain
  `KeyCode::Enter` with no modifiers (or only bracketed-paste-safe checks).
- `src/tui/app.rs:501-503`: insert `\n` only when `KeyCode::Enter` is
  combined with `KeyModifiers::SHIFT`.
- Terminal capability handling (**hard constraint**): most terminals
  (xterm, Terminal.app, tmux without passthrough) cannot distinguish
  `Shift+Enter` from plain `Enter` over legacy terminal protocols. Detecting
  it requires the **Kitty keyboard protocol**:
  - At startup in `src/tui/runner.rs`, call
    `crossterm::terminal::supports_keyboard_enhancement()`.
  - If supported, `execute!(stdout, PushKeyboardEnhancementFlags(KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES))`
    and pop it on exit alongside `LeaveAlternateScreen`.
  - If unsupported, fall back to `Alt+Enter` for newline (widely supported
    across terminals without special protocols) and surface which mode is
    active in the status bar / help screen.
- **This same ambiguity likely already affects today's `Ctrl+Enter` send
  binding, and is a probable root cause of the original complaint**:
  without the Kitty protocol / xterm `modifyOtherKeys`, most terminals emit
  the same raw byte (`0x0D`) for plain Enter and Ctrl+Enter, so
  `is_submit()`'s `modifiers.contains(CONTROL)` check may simply never be
  satisfied — `Ctrl+Enter` can silently do nothing in affected terminals
  today. Worth keeping in mind when validating this phase: the fix isn't
  just ergonomic preference, it may also fix a binding that doesn't
  reliably work at all in some environments.
- **Errata found during review, relevant to keybinding choices elsewhere in
  this plan**: any `Ctrl+<letter>` binding whose letter has a legacy
  single-byte control meaning collides with that meaning outside the Kitty
  protocol — `Ctrl+H` = Backspace (`0x08`), `Ctrl+I` = Tab (`0x09`), `Ctrl+M`
  = Enter/CR (`0x0D`), `Ctrl+[` = Esc (`0x1B`). This codebase already has a
  live instance: the existing plan-revision binding
  (`event.code == KeyCode::Char('i') && modifiers.contains(CONTROL)`,
  `src/tui/app.rs:577`) is almost certainly non-functional in legacy
  terminals, since crossterm's non-Kitty parser reports raw `0x09` as
  `KeyCode::Tab`, never as `Char('i')` + `CONTROL`. This is out of this
  plan's original scope (pre-existing bug, not introduced here), but is
  flagged as a candidate fix-while-we're-in-there once the Kitty-protocol
  detection added by this phase is in place — see Open Decisions. It's also
  why the Phase 3 candidate keybinding was changed from `Ctrl+M` to `Ctrl+O`
  (`Ctrl+M` is Enter's own control code — see Phase 3).
- Update all UI copy referencing the old binding:
  - Input box hint: `src/tui/render.rs:430`.
  - Status bar hint: `src/tui/render.rs:1649`.
  - Help screen: `src/tui/render.rs:506-880`.
  - `README.md` shortcut tables: lines ~495-499, ~2410-2422, ~2953-2959.

### Acceptance criteria

- In a Kitty-protocol-capable terminal: `Enter` sends, `Shift+Enter` inserts
  a newline, `Ctrl+Enter` still works as a legacy alias for send (no
  regression for muscle memory).
- In a legacy terminal: `Enter` sends, `Alt+Enter` inserts a newline, and the
  UI clearly indicates the fallback is active.
- Help screen and README reflect the actual active binding.

## Phase 2 — Copy / Paste Ergonomics

### Changes

- Replace the raw `String` input buffer with `tui-textarea::TextArea`
  (already a dependency, currently unused) in `src/tui/app.rs`:
  - Cursor movement: Left/Right/Up/Down, Home/End, word-jump
    (Ctrl+Left/Right).
  - Word delete (Ctrl+Backspace/Delete).
  - In-place paste insertion at cursor instead of blind append.
  - Undo/redo if exposed by the widget.
- Add `arboard` as a dependency for real OS clipboard access:
  - **Copy out**: keybinding to copy the last assistant response to the
    system clipboard (exact key TBD — `Ctrl+C` is taken by Quit; candidate:
    `Ctrl+Y`, currently free).
  - **Copy code block**: when the last response contains fenced code
    blocks, a secondary action to copy just the nearest/last code block.
  - **Paste in**: keep bracketed paste as the primary path (already works),
    add an explicit clipboard-paste action as a fallback for terminals/
    multiplexers where bracketed paste is unreliable.
- Resolve keybinding conflicts before implementation — do not reuse
  `Ctrl+C`/`Ctrl+N`/`Ctrl+L`/`Ctrl+H`/`Ctrl+K`/`Ctrl+P`/`Ctrl+D`/`Ctrl+A`/
  `Ctrl+R`/`Ctrl+I`, all already bound.

### Acceptance criteria

- Cursor can be moved and edited mid-buffer without deleting to the end.
- Pasting a multi-line block inserts it at the cursor position, not just at
  the end of the buffer.
- A single keybinding copies the last assistant reply to the OS clipboard,
  verified by pasting into an external application.

## Phase 3 — Ollama-in-TUI Polish

Ollama support itself is done; this phase closes the TUI-surface gaps noted
in `ollama-rs-integration-plan.md`.

### Changes

- **Model Info panel** (new dialog, modeled on the existing Model Download
  dialog pattern in `src/tui/ollama_download.rs`): shows current model name,
  context window, `keep_alive`, and live `PerfMetrics` (tokens/sec, load
  time). Candidate keybinding: `Ctrl+O` (currently free — **not** `Ctrl+M`,
  which is Enter's own raw control code, `0x0D`, and would be
  indistinguishable from pressing Enter in legacy terminals; see the Phase 1
  errata note).
- **Live streaming perf metrics**: attach perf data to `StreamEvent`
  (`src/llm/provider/types.rs`) during generation so the Model Info panel and
  status bar can show tok/s in real time, not just after completion.
- **In-TUI provider/model switcher**: a quick-switch dialog to change the
  active provider/model (e.g., Ollama vs. cloud provider, or between local
  Ollama models) without editing `config.toml` and restarting the app.
  Reuses `OllamaProviderConfig` (`src/config/mod.rs:351-393`) and the
  provider factory (`src/llm/provider/factory.rs:130-191`) as the source of
  truth for available providers.

### Acceptance criteria

- `Ctrl+O` opens a panel showing the active model's stats, live-updating
  tok/s while a response streams.
- User can switch from a cloud provider to a local Ollama model (and back)
  from within a running TUI session, with the change taking effect on the
  next message.

## Phase 4 — Auto Mode (bypass manual approval)

### Current state — more plumbing exists than expected, and it's partly broken

Tool-call approval today: `AppMode::ToolApproval` (`src/tui/events.rs:142`)
blocks on a modal (`render_approval`, `src/tui/render.rs:1142+`) with keys
`A`/`Y` approve, `D`/`N` deny, `V` view details, `Esc` cancel
(`src/tui/events.rs:304-322`), backed by a `ToolApprovalRequest`/`Response`
pair sent over an `mpsc` channel (`src/tui/events.rs:54-124`) with a
hardcoded 5-minute auto-deny timeout. Plan Mode approval
(`Ctrl+A`/`Ctrl+R`/`Ctrl+I`, `src/tui/app.rs:543-576`) is a separate,
higher-level concept — approving a *plan* still routes each individual tool
call inside it through the same `ToolApproval` dialog.

The bypass mechanism itself **already exists in the agent layer** — it's
just not exposed anywhere in the TUI, and it's actively broken on the CLI
path:

- `AgentService` already has `auto_approve_tools: bool` and
  `approval_callback: Option<ApprovalCallback>` fields with builders
  `.with_auto_approve_tools()` / `.with_approval_callback()`
  (`src/llm/agent/service.rs:78,81,324-330`). The approval check is
  `tool.requires_approval() && !self.auto_approve_tools && !tool_context.auto_approve`
  (`service.rs:964-970`) — flipping `auto_approve_tools` to `true` already
  skips the dialog end-to-end. **Nothing in the TUI currently sets this.**
- `config/mod.rs:18-49` already defines `PlanExecMode::{Interactive, AutoPlan, FullAuto}`
  and `PlanModeConfig { mode, auto_approval_threshold: u8, max_auto_iterations }`,
  but no runtime code reads `config.plan_mode.mode` — declared and unused.
- `src/tui/plan.rs:793-932` already defines a richer state machine
  (`PlanModeState::{Idle, Planning, AwaitingApproval, Executing, AutoExecuting{mode}, FullAuto{goal, iteration, max_iterations}, Paused{reason}, Done, Failed}`)
  with `tool_needs_approval(tool_name, threshold)` (858-866) and
  `is_high_risk_tool()` (929-931, hardcoded: `bash`, `write_file`,
  `edit_file`, `code_exec`) — exactly the risk-tiering logic Auto Mode
  needs. It is **not wired** into `app.rs` key handling or `AgentService`.
- Three dialog renderers already exist for this exact feature —
  `render_auto_exec_progress`, `render_crash_recovery_dialog`,
  `render_policy_denial` (`src/tui/components/dialogs/mod.rs`) — but are
  dead code, never called from anywhere in the app.
- Per-tool risk is a flat boolean via `Tool::requires_approval()`
  (`src/llm/tools/trait.rs:174-186`), hardcoded `true` on `bash.rs`,
  `powershell.rs`, `write.rs`, `edit.rs`, `code_exec.rs`, `notebook.rs`,
  `http.rs`, and `false` on read-only tools (`read.rs`, `ls.rs`, `glob.rs`,
  `grep.rs`, `web_search.rs`, `web_fetch.rs`, `context.rs`, `task.rs`,
  `todo_write.rs`, `plan_tool.rs`, `agent.rs`, `ask_user.rs`, `skill.rs`).
- A separate, deterministic safety layer already exists and runs
  independently of approval: `SecurityConfig` (`config/mod.rs:51-94`,
  `allow_bash`/`deny_paths`/`deny_tools`) compiles to a `PermissionPolicy`
  chain (`src/llm/tools/sandbox.rs`) evaluated in `ToolRegistry::execute()`
  (`src/llm/tools/registry.rs:80-96`) *before* the approval check. This
  should stay active under Auto Mode as the hard floor — deny-listed
  tools/paths/bash patterns stay blocked no matter what.

**Known bug to fix as part of this phase**: `crustly run --auto-approve`
(alias `--yolo`, `src/cli/mod.rs:137-139`) parses correctly (tested in
`tests/cli_test.rs:108-119`) but **is never actually applied** — `cmd_run`
builds `AgentService::new(...)` (`cli/mod.rs:910`) without calling
`.with_auto_approve_tools()` or supplying an `approval_callback`, so any
dangerous tool call is silently auto-denied regardless of the flag
(`cli/mod.rs:961-963` only prints a warning after the fact, no actual
effect). `crustly autoplan <goal>` (`cli/mod.rs:1180-1204`) has the same
bug: it constructs a `PlanModeState::FullAuto{...}` value and immediately
discards it (`let _ = state;`) before delegating to the broken `cmd_run`
path. Prior art (`docs/development/APPROVAL_SYSTEM_COMPLETE.md`) already
flags `with_auto_approve_tools(true)` as dangerous and lists "session
memory" (always-allow for this session) and "tool whitelist" as planned but
unbuilt follow-ups, consistent with this phase's scope.

### Design

Mostly **wiring up what already exists** plus the missing TUI surface,
rather than building new plumbing from scratch:

1. **Fix the CLI bug first**: make `cmd_run`/`cmd_autoplan`
   (`src/cli/mod.rs`) actually call `.with_auto_approve_tools(auto_approve)`
   so `--yolo`/`--auto-approve` and `autoplan` behave as documented. Cheap,
   high-value, and unblocks testing the rest of this phase from the CLI
   before the TUI toggle exists.
2. **TUI runtime toggle**: `Shift+Tab` (crossterm `KeyCode::BackTab`,
   confirmed free — only plain `Tab` is bound, and only inside the Model
   Download dialog, `src/tui/app.rs:1633`) cycles the mode, mirroring the
   same convention used by Claude Code itself for switching permission
   modes. Proposed cycle: `Interactive → AutoPlan → FullAuto → Interactive`,
   matching the existing `PlanExecMode` variants (`config/mod.rs:18-29`)
   one-to-one rather than a plain on/off toggle — so `Shift+Tab` steps
   through increasing autonomy instead of a binary switch. Each step flips
   a new `App` field (mirroring `AgentService::auto_approve_tools`) and
   swaps in an `ApprovalCallback` that consults
   `PlanModeState::tool_needs_approval()` (`tui/plan.rs:858-866`) instead of
   showing the interactive dialog — reusing the existing risk-tiering
   function rather than inventing a new one. Startup default comes from
   `PlanModeConfig.mode` (`config/mod.rs:31-49`); `Shift+Tab` overrides it
   per-session. Bound globally (like the other `Ctrl+*` shortcuts) so it
   works whether focus is in the chat input or elsewhere — needs a check
   that it doesn't collide with any future widget-level tab-focus-cycling.
3. **Plan Mode integration**: when Auto Mode is active, submitted plans
   skip `Ctrl+A` and go straight to `execute_plan_tasks()`
   (`app.rs:1277+`), using the already-defined `PlanModeState::AutoExecuting`/
   `FullAuto` variants instead of `Executing`.
4. **Resurrect the dead progress UI**: wire `render_auto_exec_progress`
   (`tui/components/dialogs/mod.rs`) in place of the blocking approval
   modal while Auto Mode runs, so the user sees what's happening without
   being asked to confirm each step; wire `render_policy_denial` for when
   the `SecurityConfig` policy layer blocks something Auto Mode would
   otherwise allow (so a deny stays visible, never silent).
5. **Safety guardrails** (non-negotiable — this removes a safety net):
   - Off by default; must be explicitly enabled (config or keybinding).
   - Persistent, unmissable UI indicator while active (e.g. `⚡ AUTO` badge
     in the status bar).
   - `SecurityConfig`/`sandbox.rs` policy chain (`deny_tools`, `deny_paths`,
     `allow_bash`) stays enforced under Auto Mode — the hard floor Auto
     Mode cannot override; only the interactive prompt is skipped.
   - `is_high_risk_tool()` (`tui/plan.rs:929-931`: `bash`, `write_file`,
     `edit_file`, `code_exec`) still prompts by default even in Auto Mode
     unless the user raises `auto_approval_threshold` (`config/mod.rs:31-49`)
     or picks `FullAuto` over `AutoPlan`.
   - Every auto-approved action still produces the same audit trail as a
     manually-approved one — bypassing the dialog doesn't bypass logging.
   - `Esc`/`Ctrl+C` still interrupt in-flight execution immediately; one
     keypress disables Auto Mode.

### Acceptance criteria

- `crustly run --yolo "..."` actually skips approval prompts end-to-end
  (fixes the current silent-deny bug).
- `Shift+Tab` cycles `Interactive → AutoPlan → FullAuto → Interactive` at
  any time during a session; the status bar always shows the current mode
  name, not just an on/off badge.
- In `AutoPlan`/`FullAuto`, the TUI suppresses the tool-approval dialog and
  the Plan Mode approval step for tools below the risk threshold.
- High-risk tools (`bash`, `write_file`, `edit_file`, `code_exec`) still
  prompt by default under `AutoPlan`; only `FullAuto` (or an explicit
  threshold override) bypasses them too.
- `SecurityConfig` deny-lists still block matching tools/paths/bash
  commands even with Auto Mode fully enabled.
- Status bar/help screen shows Auto Mode is active at all times while
  enabled, with no way to miss it.
- Disabling Auto Mode mid-session immediately restores manual prompts for
  the next tool call.
- Every auto-approved action still appears in logs/history identically to a
  manually-approved one.

## Phase 5 — `/skills` and `/mcp` Discovery Commands

### Current state

- **No slash-command layer exists at all** — confirmed by reading
  `handle_chat_key`/`send_message` in full
  (`src/tui/app.rs:459-509,706-783`): every character typed is pushed
  verbatim into `input_buffer`, and on submit the *entire* buffer is passed
  straight to `AgentService::send_message_with_tools_and_mode_streaming`
  after only `PromptAnalyzer::analyze_and_transform`
  (`src/tui/prompt_analyzer.rs:127`, which adds tool-usage hints, not
  command parsing). There's no `starts_with('/')` check anywhere, no
  `AppMode::Command` variant, no command registry. `/skills` and `/mcp`
  need a brand-new interception point.
- **MCP is half-wired**: `McpServerConfig`/`McpConfig`
  (`src/config/mod.rs:130-137,193-198`, `Config.mcp: McpConfig`) is a real,
  parsed config section (`[[mcp.servers]]` in TOML), and
  `ToolRegistry::register_mcp_server()` (`src/llm/tools/registry.rs:119-146`)
  fully implements connect + `tools/list` discovery + tool registration.
  **But nothing ever calls it** — grepping the whole codebase for
  `register_mcp_server`/`config.mcp` shows `Config.mcp.servers` is parsed
  and then never consumed. Configured MCP servers currently have zero
  runtime effect. There's also no status/enumeration API (connected?, tool
  count?) — only `MCPClient::is_healthy()`/`server_name()`
  (`src/mcp/client.rs:142-148`) exist, and only for a client already
  connected and held somewhere, which nothing does today.
- **Skills have no list function**: `src/llm/tools/skill.rs` defines a
  well-specified discovery order for a single named skill
  (`resolve_skill_path`, line 137-174, walking
  `.crustly/skills/`, `.claude/skills/`,
  `~/.config/crustly/skills/`, `~/.claude/skills/`, then legacy flat
  `<name>.md`), but only resolves one name at a time. The private
  `skill_lookup_roots` (line 177-203) builds the ordered root-directory
  list but nothing enumerates all `SKILL.md` files within those roots.
  Frontmatter parsing (`parse_skill_frontmatter_value`, line 218, handles
  `name`/`description`) is already reusable for a list view.
- **Dialog pattern is well-established and lightweight** — every existing
  dialog (Sessions/`Ctrl+L`, Help/`Ctrl+H`, Model Download/`Ctrl+D`) is just
  three pieces: an `AppMode` variant (`src/tui/events.rs:127-148`), an
  `open_*`/`handle_*_key` pair in `src/tui/app.rs` (e.g.
  `handle_sessions_key`, lines 512-530), and a `render_*` function in
  `src/tui/render.rs` (e.g. `render_sessions`, lines 457-503) wired into
  the `match app.mode` dispatch. No separate "dialog" trait/abstraction
  exists — `/skills` and `/mcp` views should follow this exact shape.
- No `crustly mcp list` / `crustly skill list` CLI subcommands exist either
  (`Commands` enum, `src/cli/mod.rs:123-196`) — nothing to reuse there, but
  worth adding symmetrically (see Design) following the existing
  `Ollama { operation: OllamaCommands }` subcommand shape
  (`cli/mod.rs:192-225`) as precedent for a shared service module backing
  both CLI and TUI.

### Design

1. **Slash-command interception**: in `handle_chat_key`
   (`src/tui/app.rs:459-509`), before falling through to normal character
   input, or in `send_message` (`app.rs:706-783`) before the buffer is
   forwarded to the LLM — detect `input_buffer.starts_with('/')` on submit
   and dispatch to a small command table (`/skills`, `/mcp`, room for more
   later, e.g. `/help` as an alias for `Ctrl+H`) instead of sending it as a
   chat message. Unrecognized `/word` falls through to the LLM as before
   (don't break messages that legitimately start with `/`, e.g. paths).
2. **Fix the MCP wiring gap**: at startup (wherever `ToolRegistry` is
   built, e.g. `cli/mod.rs` around the existing tool-registration calls),
   iterate `config.mcp.servers` and call `register_mcp_server()` for each
   — this is a real bug fix (config silently doing nothing today),
   independent of and smaller than the `/mcp` view itself.
3. **`/mcp` view**: new `AppMode::Mcp`, `handle_mcp_key`, `render_mcp`
   modeled on `render_sessions`. Add a small status-tracking layer (e.g.
   `ToolRegistry` keeps `Vec<(McpServerConfig, connected: bool, tool_count: usize)>`
   after Step 2's wiring) so the view can show each configured server's
   name, command, connection status, and discovered tool count — not just
   the static config list.
4. **`/skills` view**: new `AppMode::Skills`, `handle_skills_key`,
   `render_skills`. Requires adding a new `pub(crate)` enumeration function
   in `skill.rs` (expose `skill_lookup_roots`, scan each root directory for
   `*/SKILL.md` and legacy `*.md`, parse frontmatter via the existing
   helper) to list every discoverable skill with its name, source root, and
   description. Selecting a skill in the list can show its description /
   prompt preview, reusing the existing `execute()` read path.
5. **Optional CLI symmetry**: add `crustly mcp list` / `crustly skill list`
   subcommands backed by the same new enumeration functions, following the
   `Ollama { operation }` precedent — not required for the TUI feature to
   work, but cheap once the underlying functions exist and useful for
   scripting/debugging outside the TUI.

Implementation note: `AppMode` (`src/tui/events.rs:127-148`) has no
wildcard/catch-all match arms at its call sites, so adding `AppMode::Mcp`
and `AppMode::Skills` will force the compiler to flag every existing
`match app.mode`/`match self.mode` site that needs a new arm — at minimum
the render dispatch in `render()` (`src/tui/render.rs:38-69`) and the
status-bar mode-label match (`render.rs` around line 1630-1640, not yet
precisely pinned down). Treat the compiler errors as the checklist rather
than trying to enumerate every site by hand up front.

### Acceptance criteria

- Typing `/skills` and pressing `Enter`/submit opens a list view of every
  discoverable skill (project-local and user-global, both `.crustly` and
  `.claude` roots), not just ones already invoked this session.
- Typing `/mcp` opens a list view of every configured MCP server with live
  connection status and discovered tool count.
- Configuring an MCP server in `config.toml` actually connects it at
  startup (fixes the current silent no-op).
- A message that happens to start with `/` but isn't a recognized command
  (e.g. a file path) is still sent to the LLM normally, not swallowed.
- Both views follow the existing dialog conventions: `Esc` closes,
  `Up`/`Down` navigate, consistent styling with Sessions/Help.

## Suggested Execution Order

1. Phase 1 (keybinding swap) — small, self-contained, immediately visible.
2. Phase 3 (Model Info panel + provider switcher) — closes the known Ollama
   gap, moderate size, independent of input-handling changes.
3. Phase 2 (tui-textarea migration + clipboard) — largest change, touches
   input handling broadly; sequenced last so it only needs to account for
   the final Phase 1 keybinding scheme once.
4. Phase 4 (Auto Mode) — the CLI bug fix (`--yolo`/`autoplan` not actually
   bypassing approval) can land anytime, independently, as a small isolated
   fix. The TUI toggle is sequenced last since it should build on the
   finalized keybinding scheme (Phase 1) and ideally the Model Info/
   status-bar work (Phase 3) so the "Auto Mode active" indicator has an
   established place to live in the UI.
5. Phase 5 (`/skills`, `/mcp`) — the MCP wiring bug fix (Step 2) can land
   anytime, independently, like the Auto Mode CLI fix. The slash-command
   interception layer it introduces is also the natural foundation for
   exposing other future commands (e.g. `/help`, `/clear`), so doing this
   phase before further keybinding work is worth considering if more
   commands are anticipated — otherwise it's independent of Phases 1-4 and
   can be done in any order.

## Open Decisions

- [ ] Exact fallback key for newline insertion on non-Kitty terminals:
      `Alt+Enter` (proposed) vs. alternative.
- [ ] Exact keybinding for "copy last response" and "copy code block"
      (`Ctrl+Y` proposed for copy).
- [ ] Exact keybinding for Model Info panel (`Ctrl+O` proposed — `Ctrl+M`
      ruled out, see Phase 1 errata).
- [ ] Whether `Ctrl+Enter` is kept as a legacy send alias after the swap.
- [ ] Whether `Shift+Tab` cycles through all three `PlanExecMode` variants
      (`Interactive → AutoPlan → FullAuto`) or is a plain two-state toggle
      (`Interactive ↔ last-used-auto-mode`) — cycling is proposed since the
      config already models three distinct levels.
- [ ] Whether the default high-risk list stays exactly
      `is_high_risk_tool()`'s current set (`bash`, `write_file`,
      `edit_file`, `code_exec`, `tui/plan.rs:929-931`) or should also cover
      `powershell`/`http`/`notebook` (currently `requires_approval() == true`
      but not in the high-risk list).
- [ ] CLI flag name for Auto Mode (keep existing `--auto-approve`/`--yolo`
      vs. introduce `--dangerously-skip-permissions`).
- [ ] Whether Auto Mode persists across sessions via `PlanModeConfig.mode`
      in `config.toml` or defaults to session-only (must be re-enabled each
      launch).
- [ ] Whether `/mcp` should trigger a fresh connect-and-discover for
      not-yet-connected servers when opened (live check), or only show
      last-known status from startup (cheaper, possibly stale).
- [ ] Whether `/skills` should also show skills that fail to parse
      (malformed frontmatter) as an error row, or silently skip them.
- [ ] Whether to add the optional `crustly mcp list` / `crustly skill list`
      CLI subcommands in this phase or defer them.
- [ ] Whether to fix the pre-existing, out-of-scope `Ctrl+I` plan-revision
      bug (`src/tui/app.rs:577`, likely non-functional in legacy terminals
      per the Phase 1 errata) opportunistically while Phase 1 adds Kitty-
      protocol detection, or file it separately.
- [ ] Whether `Ctrl+H` (Help) should be re-evaluated too — it collides with
      Backspace's legacy control code (`0x08`) on terminals/configs that
      still send `0x08` for the Backspace key (most modern terminals send
      `0x7F`/DEL instead, so this is narrower than the `Ctrl+M`/`Ctrl+I`
      cases, but worth a quick check rather than assuming it's fine
      everywhere).
