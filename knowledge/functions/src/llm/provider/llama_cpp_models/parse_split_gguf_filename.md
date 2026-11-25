---
type: Rust Function
title: parse_split_gguf_filename
resource: src/llm/provider/llama_cpp_models.rs#L500-L519
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/memory/MemoryStore/all
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/models/PlanTaskStatus/parse
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/provider/llama_cpp_models/parse_split_gguf_filename_matches_the_convention
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp_models/parse_split_gguf_filename_rejects_mismatched_widths
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp_models/parse_split_gguf_filename_rejects_non_split_names_and_out_of_range_parts
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn parse_split_gguf_filename(filename: &str) -> Option<(&str, u32, u32)>`

# Calls

- [all](../../../../../functions/src/memory/MemoryStore/all.md)
- [parse](../../../../../functions/src/db/models/PlanTaskStatus/parse.md)

# Called by

- [parse_split_gguf_filename_matches_the_convention](../../../../../functions/src/llm/provider/llama_cpp_models/parse_split_gguf_filename_matches_the_convention.md)
- [parse_split_gguf_filename_rejects_mismatched_widths](../../../../../functions/src/llm/provider/llama_cpp_models/parse_split_gguf_filename_rejects_mismatched_widths.md)
- [parse_split_gguf_filename_rejects_non_split_names_and_out_of_range_parts](../../../../../functions/src/llm/provider/llama_cpp_models/parse_split_gguf_filename_rejects_non_split_names_and_out_of_range_parts.md)