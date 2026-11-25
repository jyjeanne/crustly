---
type: Rust Function
title: settings_are_checked_before_any_request
resource: src/memory/embed.rs#L374-L380
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/memory/embed/settings
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/embed/OllamaEmbedder/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`fn settings_are_checked_before_any_request()`

# Calls

- [settings](../../../../functions/src/memory/embed/settings.md)
- [new](../../../../functions/src/memory/embed/OllamaEmbedder/new.md)