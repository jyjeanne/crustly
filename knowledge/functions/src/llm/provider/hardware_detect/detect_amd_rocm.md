---
type: Rust Function
title: detect_amd_rocm
resource: src/llm/provider/hardware_detect.rs#L243-L247
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/provider/hardware_detect/run_and_capture_stdout
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/hardware_detect/parse_rocm_smi_json
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn detect_amd_rocm() -> Option<GpuInfo>`

# Calls

- [run_and_capture_stdout](../../../../../functions/src/llm/provider/hardware_detect/run_and_capture_stdout.md)
- [parse_rocm_smi_json](../../../../../functions/src/llm/provider/hardware_detect/parse_rocm_smi_json.md)