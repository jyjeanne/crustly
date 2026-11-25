---
type: Rust Function
title: check_settings
resource: src/memory/embed.rs#L120-L131
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/memory/embed/OllamaEmbedder/new
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/embed/OpenAiCompatibleEmbedder/new
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn check_settings(settings: &EmbedSettings) -> Result<(), EmbedError>`

# Called by

- [new](../../../../functions/src/memory/embed/OllamaEmbedder/new.md)
- [new](../../../../functions/src/memory/embed/OpenAiCompatibleEmbedder/new.md)