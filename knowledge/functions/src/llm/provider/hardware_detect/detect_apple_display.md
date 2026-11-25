---
type: Rust Function
title: detect_apple_display
resource: src/llm/provider/hardware_detect.rs#L368-L373
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/provider/hardware_detect/run_and_capture_stdout
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/hardware_detect/parse_system_profiler_json
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn detect_apple_display() -> Option<GpuInfo>`

# Calls

- [run_and_capture_stdout](../../../../../functions/src/llm/provider/hardware_detect/run_and_capture_stdout.md)
- [parse_system_profiler_json](../../../../../functions/src/llm/provider/hardware_detect/parse_system_profiler_json.md)