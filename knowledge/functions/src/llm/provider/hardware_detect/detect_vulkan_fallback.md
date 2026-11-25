---
type: Rust Function
title: detect_vulkan_fallback
resource: src/llm/provider/hardware_detect.rs#L438-L441
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/provider/hardware_detect/run_and_capture_stdout
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/hardware_detect/parse_vulkaninfo_summary
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn detect_vulkan_fallback() -> Option<GpuInfo>`

# Calls

- [run_and_capture_stdout](../../../../../functions/src/llm/provider/hardware_detect/run_and_capture_stdout.md)
- [parse_vulkaninfo_summary](../../../../../functions/src/llm/provider/hardware_detect/parse_vulkaninfo_summary.md)