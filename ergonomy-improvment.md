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

## Suggested Execution Order

1. Phase 1 (keybinding swap) — small, self-contained, immediately visible.
2. Phase 3 (Model Info panel + provider switcher) — closes the known Ollama
   gap, moderate size, independent of input-handling changes.
3. Phase 2 (tui-textarea migration + clipboard) — largest change, touches
   input handling broadly; sequenced last so it only needs to account for
   the final Phase 1 keybinding scheme once.

## Open Decisions

- [ ] Exact fallback key for newline insertion on non-Kitty terminals:
      `Alt+Enter` (proposed) vs. alternative.
- [ ] Exact keybinding for "copy last response" and "copy code block"
      (`Ctrl+Y` proposed for copy).
- [ ] Exact keybinding for Model Info panel (`Ctrl+M` proposed).
- [ ] Whether `Ctrl+Enter` is kept as a legacy send alias after the swap.
