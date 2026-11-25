---
type: Rust Function
title: parses_a_real_shaped_nvidia_smi_line
resource: src/llm/provider/hardware_detect.rs#L487-L497
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/provider/hardware_detect/parse_nvidia_smi_csv
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/hardware_detect/GpuInfo/vram_available_bytes
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn parses_a_real_shaped_nvidia_smi_line()`

# Calls

- [parse_nvidia_smi_csv](../../../../../functions/src/llm/provider/hardware_detect/parse_nvidia_smi_csv.md)
- [vram_available_bytes](../../../../../functions/src/llm/provider/hardware_detect/GpuInfo/vram_available_bytes.md)