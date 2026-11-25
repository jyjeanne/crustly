---
type: Rust Method
title: anchors
resource: src/memory/anchors.rs#L150-L152
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/db/repository/memory/MemoryRepository/anchors_of
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/cli/cmd_memory
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_code_knowledge_test/a_memory_naming_a_function_is_anchored_to_it_and_an_ambiguous_name_is_not
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_code_knowledge_test/a_concept_gone_from_a_newer_bundle_is_flagged_and_not_from_an_older_one
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_code_knowledge_test/a_slow_okf_mcp_is_skipped_and_recall_still_works
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn anchors(&self, id: Uuid) -> Result<Vec<(String, DateTime<Utc>)>>`

# Calls

- [anchors_of](../../../../../functions/src/db/repository/memory/MemoryRepository/anchors_of.md)

# Called by

- [cmd_memory](../../../../../functions/src/cli/cmd_memory.md)
- [a_memory_naming_a_function_is_anchored_to_it_and_an_ambiguous_name_is_not](../../../../../functions/tests/memory_code_knowledge_test/a_memory_naming_a_function_is_anchored_to_it_and_an_ambiguous_name_is_not.md)
- [a_concept_gone_from_a_newer_bundle_is_flagged_and_not_from_an_older_one](../../../../../functions/tests/memory_code_knowledge_test/a_concept_gone_from_a_newer_bundle_is_flagged_and_not_from_an_older_one.md)
- [a_slow_okf_mcp_is_skipped_and_recall_still_works](../../../../../functions/tests/memory_code_knowledge_test/a_slow_okf_mcp_is_skipped_and_recall_still_works.md)