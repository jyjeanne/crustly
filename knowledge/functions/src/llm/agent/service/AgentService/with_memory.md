---
type: Rust Method
title: with_memory
resource: src/llm/agent/service.rs#L638-L641
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/cli/cmd_chat
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/cmd_run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/recall_block_reaches_the_model_but_not_the_database
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn with_memory(mut self, store: Option<Arc<MemoryStore>>) -> Self`

# Called by

- [cmd_chat](../../../../../../functions/src/cli/cmd_chat.md)
- [cmd_run](../../../../../../functions/src/cli/cmd_run.md)
- [recall_block_reaches_the_model_but_not_the_database](../../../../../../functions/src/llm/agent/service/recall_block_reaches_the_model_but_not_the_database.md)