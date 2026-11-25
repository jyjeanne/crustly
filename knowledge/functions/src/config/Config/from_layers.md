---
type: Rust Method
title: from_layers
resource: src/config/mod.rs#L1428-L1449
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/config/Config/default/default
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/tui/error/ErrorInfo/with_context
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/merge_toml_tables
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/MemoryConfig/sanitized
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/config/Config/load
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/Config/load_from_path
    resolved_by: tree-sitter
    confidence: exact
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
  - target: functions/src/config/test_from_layers_without_files_is_default
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

`fn from_layers(paths: &[PathBuf]) -> Result<Self>`

# Calls

- [default](../../../../functions/src/config/Config/default/default.md)
- [with_context](../../../../functions/src/tui/error/ErrorInfo/with_context.md)
- [merge_toml_tables](../../../../functions/src/config/merge_toml_tables.md)
- [sanitized](../../../../functions/src/config/MemoryConfig/sanitized.md)

# Called by

- [load](../../../../functions/src/config/Config/load.md)
- [load_from_path](../../../../functions/src/config/Config/load_from_path.md)
- [test_from_layers_keeps_keys_the_later_file_does_not_set](../../../../functions/src/config/test_from_layers_keeps_keys_the_later_file_does_not_set.md)
- [test_from_layers_later_file_wins_on_the_same_key](../../../../functions/src/config/test_from_layers_later_file_wins_on_the_same_key.md)
- [test_from_layers_merges_nested_tables](../../../../functions/src/config/test_from_layers_merges_nested_tables.md)
- [test_from_layers_replaces_arrays_instead_of_concatenating](../../../../functions/src/config/test_from_layers_replaces_arrays_instead_of_concatenating.md)
- [test_from_layers_without_files_is_default](../../../../functions/src/config/test_from_layers_without_files_is_default.md)
- [test_from_layers_reports_the_files_on_an_invalid_value](../../../../functions/src/config/test_from_layers_reports_the_files_on_an_invalid_value.md)
- [test_from_layers_reports_the_file_with_a_syntax_error](../../../../functions/src/config/test_from_layers_reports_the_file_with_a_syntax_error.md)