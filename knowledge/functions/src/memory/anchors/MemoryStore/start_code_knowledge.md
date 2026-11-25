---
type: Rust Method
title: start_code_knowledge
resource: src/memory/anchors.rs#L81-L86
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/cli/cmd_memory
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_code_knowledge_test/a_memory_naming_a_function_is_anchored_to_it_and_an_ambiguous_name_is_not
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_code_knowledge_test/naming_the_concept_or_a_caller_recalls_the_anchored_memory
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_code_knowledge_test/reading_the_file_that_defines_the_concept_brings_the_memory_once
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_code_knowledge_test/a_concept_gone_from_a_newer_bundle_is_flagged_and_not_from_an_older_one
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_code_knowledge_test/an_older_okf_mcp_turns_the_feature_off_and_recall_still_works
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_code_knowledge_test/a_slow_okf_mcp_is_skipped_and_recall_still_works
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_code_knowledge_test/without_code_knowledge_nothing_changes
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn start_code_knowledge(&self) -> bool`

# Called by

- [cmd_memory](../../../../../functions/src/cli/cmd_memory.md)
- [a_memory_naming_a_function_is_anchored_to_it_and_an_ambiguous_name_is_not](../../../../../functions/tests/memory_code_knowledge_test/a_memory_naming_a_function_is_anchored_to_it_and_an_ambiguous_name_is_not.md)
- [naming_the_concept_or_a_caller_recalls_the_anchored_memory](../../../../../functions/tests/memory_code_knowledge_test/naming_the_concept_or_a_caller_recalls_the_anchored_memory.md)
- [reading_the_file_that_defines_the_concept_brings_the_memory_once](../../../../../functions/tests/memory_code_knowledge_test/reading_the_file_that_defines_the_concept_brings_the_memory_once.md)
- [a_concept_gone_from_a_newer_bundle_is_flagged_and_not_from_an_older_one](../../../../../functions/tests/memory_code_knowledge_test/a_concept_gone_from_a_newer_bundle_is_flagged_and_not_from_an_older_one.md)
- [an_older_okf_mcp_turns_the_feature_off_and_recall_still_works](../../../../../functions/tests/memory_code_knowledge_test/an_older_okf_mcp_turns_the_feature_off_and_recall_still_works.md)
- [a_slow_okf_mcp_is_skipped_and_recall_still_works](../../../../../functions/tests/memory_code_knowledge_test/a_slow_okf_mcp_is_skipped_and_recall_still_works.md)
- [without_code_knowledge_nothing_changes](../../../../../functions/tests/memory_code_knowledge_test/without_code_knowledge_nothing_changes.md)