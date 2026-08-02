---
type: Rust Function
title: minimal_notebook_json
resource: src/llm/tools/notebook.rs#L344-L346
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  called_by:
  - functions/src/llm/tools/notebook/test_add_cell_within_working_directory_succeeds
  - functions/src/llm/tools/notebook/test_path_outside_working_directory_is_denied
---

# Signature

`fn minimal_notebook_json() -> &'static str`

# Called by

- [test_add_cell_within_working_directory_succeeds](../../../../../functions/src/llm/tools/notebook/test_add_cell_within_working_directory_succeeds.md)
- [test_path_outside_working_directory_is_denied](../../../../../functions/src/llm/tools/notebook/test_path_outside_working_directory_is_denied.md)