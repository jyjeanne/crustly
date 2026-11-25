---
type: Rust Module
title: tui
resource: src/tui/mod.rs#L1-L28
generated:
  by: okf-rs/0.8.0
relationships:
  imports:
  - target: external/pub-use-app-app-displaymessage
    resolved_by: tree-sitter
    confidence: exact
  - target: external/pub-use-events-appmode-eventhandler-tuievent
    resolved_by: tree-sitter
    confidence: exact
  - target: external/pub-use-prompt-analyzer-promptanalyzer
    resolved_by: tree-sitter
    confidence: exact
  - target: external/pub-use-runner-run
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/crustly
    resolved_by: tree-sitter
    confidence: exact
---

# Imports

- `pub use app::{App, DisplayMessage}`
- `pub use events::{AppMode, EventHandler, TuiEvent}`
- `pub use prompt_analyzer::PromptAnalyzer`
- `pub use runner::run`

# Member of

- [crustly](../../packages/crustly.md)