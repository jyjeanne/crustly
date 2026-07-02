# TUI Ergonomics Improvement Plan

Status: Draft / Specification
Owner: TUI team
Related: `ollama-rs-integration-plan.md` (native Ollama provider, already implemented)

## Goal

Improve day-to-day usability of the Crustly TUI:

1. Make `Enter` send the message and `Shift+Enter` insert a newline (swap of
   today's `Ctrl+Enter` send / `Enter` newline scheme).
2. Make copy/paste into and out of the TUI reliable and ergonomic.
3. Expose the existing native Ollama integration (`ollama-rs`) more directly
   inside the TUI (not just via config file + `Ctrl+D` download dialog).
4. Add an opt-in "Auto Mode" that bypasses manual tool-call / plan approval
   prompts for users who want the agent to run unattended.

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
  time). Candidate keybinding: `Ctrl+M` (currently free).
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

- `Ctrl+M` opens a panel showing the active model's stats, live-updating
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

## Open Decisions

- [ ] Exact fallback key for newline insertion on non-Kitty terminals:
      `Alt+Enter` (proposed) vs. alternative.
- [ ] Exact keybinding for "copy last response" and "copy code block"
      (`Ctrl+Y` proposed for copy).
- [ ] Exact keybinding for Model Info panel (`Ctrl+M` proposed).
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
