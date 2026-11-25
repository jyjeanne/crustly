---
type: Rust Function
title: memory_status_lines
resource: src/cli/mod.rs#L1073-L1142
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/agent/service/tool_round/ToolRound/push
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/display_path
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/cli/cmd_memory
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/memory_status_reports_semantic_recall_and_extraction_when_on
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn memory_status_lines( config: &crate::config::Config, project: &crate::memory::project::ProjectIdentity, memories: &[crate::memory::model::Memory], embedding: Option<(String, i64, i64)>, ) -> Vec<String>`

# Calls

- [push](../../../functions/src/llm/agent/service/tool_round/ToolRound/push.md)
- [display_path](../../../functions/src/cli/display_path.md)

# Called by

- [cmd_memory](../../../functions/src/cli/cmd_memory.md)
- [memory_status_reports_semantic_recall_and_extraction_when_on](../../../functions/src/cli/memory_status_reports_semantic_recall_and_extraction_when_on.md)