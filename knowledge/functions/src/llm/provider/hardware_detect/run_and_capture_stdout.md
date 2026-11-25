---
type: Rust Function
title: run_and_capture_stdout
resource: src/llm/provider/hardware_detect.rs#L196-L202
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/provider/hardware_detect/run_with_timeout
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/provider/hardware_detect/detect_nvidia
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/hardware_detect/detect_amd_rocm
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/hardware_detect/detect_apple_display
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/hardware_detect/detect_vulkan_fallback
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn run_and_capture_stdout(cmd: &mut Command) -> Option<String>`

# Calls

- [run_with_timeout](../../../../../functions/src/llm/provider/hardware_detect/run_with_timeout.md)

# Called by

- [detect_nvidia](../../../../../functions/src/llm/provider/hardware_detect/detect_nvidia.md)
- [detect_amd_rocm](../../../../../functions/src/llm/provider/hardware_detect/detect_amd_rocm.md)
- [detect_apple_display](../../../../../functions/src/llm/provider/hardware_detect/detect_apple_display.md)
- [detect_vulkan_fallback](../../../../../functions/src/llm/provider/hardware_detect/detect_vulkan_fallback.md)