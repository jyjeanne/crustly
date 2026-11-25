---
type: Rust Function
title: parse_hf_shorthand
resource: src/llm/provider/llama_cpp_models.rs#L847-L863
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/tui/events/EventHandler/next
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/provider/llama_cpp_models/resolve_download_source
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp_models/parse_hf_shorthand_extracts_org_repo_file
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp_models/parse_hf_shorthand_extracts_a_pinned_revision
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp_models/parse_hf_shorthand_none_for_direct_url
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp_models/parse_hf_shorthand_none_for_malformed_shorthand
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn parse_hf_shorthand(source: &str) -> Option<(&str, &str, &str, Option<&str>)>`

# Calls

- [next](../../../../../functions/src/tui/events/EventHandler/next.md)

# Called by

- [resolve_download_source](../../../../../functions/src/llm/provider/llama_cpp_models/resolve_download_source.md)
- [parse_hf_shorthand_extracts_org_repo_file](../../../../../functions/src/llm/provider/llama_cpp_models/parse_hf_shorthand_extracts_org_repo_file.md)
- [parse_hf_shorthand_extracts_a_pinned_revision](../../../../../functions/src/llm/provider/llama_cpp_models/parse_hf_shorthand_extracts_a_pinned_revision.md)
- [parse_hf_shorthand_none_for_direct_url](../../../../../functions/src/llm/provider/llama_cpp_models/parse_hf_shorthand_none_for_direct_url.md)
- [parse_hf_shorthand_none_for_malformed_shorthand](../../../../../functions/src/llm/provider/llama_cpp_models/parse_hf_shorthand_none_for_malformed_shorthand.md)