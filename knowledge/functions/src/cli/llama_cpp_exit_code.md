---
type: Rust Function
title: llama_cpp_exit_code
resource: src/cli/mod.rs#L646-L668
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/cli/run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/llama_cpp_exit_code_maps_each_known_message_prefix
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/llama_cpp_exit_code_checks_the_whole_chain_not_just_the_outermost_context
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/llama_cpp_exit_code_falls_back_to_one_for_an_unrecognized_error
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn llama_cpp_exit_code(err: &anyhow::Error) -> i32`

# Called by

- [run](../../../functions/src/cli/run.md)
- [llama_cpp_exit_code_maps_each_known_message_prefix](../../../functions/src/cli/llama_cpp_exit_code_maps_each_known_message_prefix.md)
- [llama_cpp_exit_code_checks_the_whole_chain_not_just_the_outermost_context](../../../functions/src/cli/llama_cpp_exit_code_checks_the_whole_chain_not_just_the_outermost_context.md)
- [llama_cpp_exit_code_falls_back_to_one_for_an_unrecognized_error](../../../functions/src/cli/llama_cpp_exit_code_falls_back_to_one_for_an_unrecognized_error.md)