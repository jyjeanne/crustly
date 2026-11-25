---
type: Rust Function
title: detect_hardware
resource: src/llm/provider/hardware_detect.rs#L135-L137
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/cli/cmd_llama_cpp
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/llama_cpp_doctor_findings
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/hardware_detect/detect_hardware_is_cached_across_calls
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/llama_cpp_download/detect_hardware_summary
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn detect_hardware() -> &'static HardwareInfo`

# Called by

- [cmd_llama_cpp](../../../../../functions/src/cli/cmd_llama_cpp.md)
- [llama_cpp_doctor_findings](../../../../../functions/src/cli/llama_cpp_doctor_findings.md)
- [detect_hardware_is_cached_across_calls](../../../../../functions/src/llm/provider/hardware_detect/detect_hardware_is_cached_across_calls.md)
- [detect_hardware_summary](../../../../../functions/src/tui/llama_cpp_download/detect_hardware_summary.md)