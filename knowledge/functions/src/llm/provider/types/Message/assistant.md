---
type: Rust Method
title: assistant
resource: src/llm/provider/types.rs#L39-L44
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/llm/agent/compaction/compaction_refuses_when_nothing_would_be_dropped
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/types/test_message_creation
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn assistant(text: impl Into<String>) -> Self`

# Called by

- [compaction_refuses_when_nothing_would_be_dropped](../../../../../../functions/src/llm/agent/compaction/compaction_refuses_when_nothing_would_be_dropped.md)
- [test_message_creation](../../../../../../functions/src/llm/provider/types/test_message_creation.md)