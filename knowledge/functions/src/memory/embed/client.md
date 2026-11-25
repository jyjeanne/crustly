---
type: Rust Function
title: client
resource: src/memory/embed.rs#L133-L138
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/agent/service/subagents/ChildTemplate/build
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/memory/embed/OllamaEmbedder/new
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/embed/OpenAiCompatibleEmbedder/new
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn client(timeout: Duration) -> Result<reqwest::Client, EmbedError>`

# Calls

- [build](../../../../functions/src/llm/agent/service/subagents/ChildTemplate/build.md)

# Called by

- [new](../../../../functions/src/memory/embed/OllamaEmbedder/new.md)
- [new](../../../../functions/src/memory/embed/OpenAiCompatibleEmbedder/new.md)