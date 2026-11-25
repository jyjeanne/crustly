---
type: Rust Function
title: run_with_timeout
resource: src/llm/provider/hardware_detect.rs#L171-L194
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/llm/provider/hardware_detect/run_and_capture_stdout
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/hardware_detect/run_with_timeout_returns_none_for_a_nonexistent_binary
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/hardware_detect/run_with_timeout_returns_output_for_a_fast_real_command
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn run_with_timeout(cmd: &mut Command) -> Option<Output>`

# Called by

- [run_and_capture_stdout](../../../../../functions/src/llm/provider/hardware_detect/run_and_capture_stdout.md)
- [run_with_timeout_returns_none_for_a_nonexistent_binary](../../../../../functions/src/llm/provider/hardware_detect/run_with_timeout_returns_none_for_a_nonexistent_binary.md)
- [run_with_timeout_returns_output_for_a_fast_real_command](../../../../../functions/src/llm/provider/hardware_detect/run_with_timeout_returns_output_for_a_fast_real_command.md)