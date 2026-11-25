---
type: Rust Method
title: apply
resource: src/memory/embed.rs#L101-L106
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/memory/embed/OllamaEmbedder/embedder/embed
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/embed/OpenAiCompatibleEmbedder/embedder/embed
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn apply(&self, purpose: EmbedPurpose, text: &str) -> String`

# Called by

- [embed](../../../../../functions/src/memory/embed/OllamaEmbedder/embedder/embed.md)
- [embed](../../../../../functions/src/memory/embed/OpenAiCompatibleEmbedder/embedder/embed.md)