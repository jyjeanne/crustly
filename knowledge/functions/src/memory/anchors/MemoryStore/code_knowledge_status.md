---
type: Rust Method
title: code_knowledge_status
resource: src/memory/anchors.rs#L63-L69
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/memory/code_knowledge/CodeKnowledge/status_line
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/cli/cmd_memory
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_code_knowledge_test/an_older_okf_mcp_turns_the_feature_off_and_recall_still_works
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_code_knowledge_test/without_code_knowledge_nothing_changes
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn code_knowledge_status(&self) -> String`

# Calls

- [status_line](../../../../../functions/src/memory/code_knowledge/CodeKnowledge/status_line.md)

# Called by

- [cmd_memory](../../../../../functions/src/cli/cmd_memory.md)
- [an_older_okf_mcp_turns_the_feature_off_and_recall_still_works](../../../../../functions/tests/memory_code_knowledge_test/an_older_okf_mcp_turns_the_feature_off_and_recall_still_works.md)
- [without_code_knowledge_nothing_changes](../../../../../functions/tests/memory_code_knowledge_test/without_code_knowledge_nothing_changes.md)