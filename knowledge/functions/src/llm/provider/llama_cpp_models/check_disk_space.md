---
type: Rust Function
title: check_disk_space
resource: src/llm/provider/llama_cpp_models.rs#L960-L982
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/llm/provider/llama_cpp_models/download_model
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp_models/check_disk_space_rejects_when_available_is_below_needed_plus_margin
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp_models/check_disk_space_allows_when_comfortably_available
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp_models/check_disk_space_skips_when_available_is_unknown
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn check_disk_space( available: Option<u64>, still_needed: u64, filename: &str, models_dir: &Path, ) -> Result<()>`

# Called by

- [download_model](../../../../../functions/src/llm/provider/llama_cpp_models/download_model.md)
- [check_disk_space_rejects_when_available_is_below_needed_plus_margin](../../../../../functions/src/llm/provider/llama_cpp_models/check_disk_space_rejects_when_available_is_below_needed_plus_margin.md)
- [check_disk_space_allows_when_comfortably_available](../../../../../functions/src/llm/provider/llama_cpp_models/check_disk_space_allows_when_comfortably_available.md)
- [check_disk_space_skips_when_available_is_unknown](../../../../../functions/src/llm/provider/llama_cpp_models/check_disk_space_skips_when_available_is_unknown.md)