---
type: Rust Function
title: llama_cpp_exit_code_checks_the_whole_chain_not_just_the_outermost_context
resource: src/cli/mod.rs#L3512-L3521
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/services/ServiceManager/context
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/llama_cpp_exit_code
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn llama_cpp_exit_code_checks_the_whole_chain_not_just_the_outermost_context()`

# Calls

- [context](../../../functions/src/services/ServiceManager/context.md)
- [llama_cpp_exit_code](../../../functions/src/cli/llama_cpp_exit_code.md)