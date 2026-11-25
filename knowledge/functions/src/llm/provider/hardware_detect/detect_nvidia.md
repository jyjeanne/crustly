---
type: Rust Function
title: detect_nvidia
resource: src/llm/provider/hardware_detect.rs#L208-L214
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/provider/hardware_detect/run_and_capture_stdout
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/hardware_detect/parse_nvidia_smi_csv
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/provider/hardware_detect/detect_hardware_uncached
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn detect_nvidia() -> Option<GpuInfo>`

# Calls

- [run_and_capture_stdout](../../../../../functions/src/llm/provider/hardware_detect/run_and_capture_stdout.md)
- [parse_nvidia_smi_csv](../../../../../functions/src/llm/provider/hardware_detect/parse_nvidia_smi_csv.md)

# Called by

- [detect_hardware_uncached](../../../../../functions/src/llm/provider/hardware_detect/detect_hardware_uncached.md)