---
type: Rust Function
title: parse_rocm_smi_json
resource: src/llm/provider/hardware_detect.rs#L258-L275
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/tui/events/EventHandler/next
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/provider/hardware_detect/detect_amd_rocm
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/hardware_detect/parses_a_real_shaped_rocm_smi_json_blob
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/hardware_detect/rocm_smi_malformed_json_is_none
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn parse_rocm_smi_json(stdout: &str) -> Option<GpuInfo>`

# Calls

- [next](../../../../../functions/src/tui/events/EventHandler/next.md)

# Called by

- [detect_amd_rocm](../../../../../functions/src/llm/provider/hardware_detect/detect_amd_rocm.md)
- [parses_a_real_shaped_rocm_smi_json_blob](../../../../../functions/src/llm/provider/hardware_detect/parses_a_real_shaped_rocm_smi_json_blob.md)
- [rocm_smi_malformed_json_is_none](../../../../../functions/src/llm/provider/hardware_detect/rocm_smi_malformed_json_is_none.md)