---
type: Rust Function
title: layers
resource: src/config/mod.rs#L2017-L2019
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/config/test_from_layers_keeps_keys_the_later_file_does_not_set
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/test_from_layers_later_file_wins_on_the_same_key
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/test_from_layers_merges_nested_tables
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/test_from_layers_replaces_arrays_instead_of_concatenating
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/test_from_layers_reports_the_files_on_an_invalid_value
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/test_from_layers_reports_the_file_with_a_syntax_error
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn layers(files: &[&NamedTempFile]) -> Vec<PathBuf>`

# Called by

- [test_from_layers_keeps_keys_the_later_file_does_not_set](../../../functions/src/config/test_from_layers_keeps_keys_the_later_file_does_not_set.md)
- [test_from_layers_later_file_wins_on_the_same_key](../../../functions/src/config/test_from_layers_later_file_wins_on_the_same_key.md)
- [test_from_layers_merges_nested_tables](../../../functions/src/config/test_from_layers_merges_nested_tables.md)
- [test_from_layers_replaces_arrays_instead_of_concatenating](../../../functions/src/config/test_from_layers_replaces_arrays_instead_of_concatenating.md)
- [test_from_layers_reports_the_files_on_an_invalid_value](../../../functions/src/config/test_from_layers_reports_the_files_on_an_invalid_value.md)
- [test_from_layers_reports_the_file_with_a_syntax_error](../../../functions/src/config/test_from_layers_reports_the_file_with_a_syntax_error.md)