---
type: Rust Function
title: quitting_writes_the_session_episode_before_exit
resource: src/tui/app.rs#L6441-L6447
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/tui/app/app_with_extraction
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/finish_memory
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/episodes
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn quitting_writes_the_session_episode_before_exit()`

# Calls

- [app_with_extraction](../../../../functions/src/tui/app/app_with_extraction.md)
- [finish_memory](../../../../functions/src/tui/app/App/finish_memory.md)
- [episodes](../../../../functions/src/tui/app/episodes.md)