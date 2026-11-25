---
type: Rust Function
title: detect_hardware_uncached
resource: src/llm/provider/hardware_detect.rs#L139-L150
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/provider/hardware_detect/detect_nvidia
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/hardware_detect/system_ram_total_bytes
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn detect_hardware_uncached() -> HardwareInfo`

# Calls

- [detect_nvidia](../../../../../functions/src/llm/provider/hardware_detect/detect_nvidia.md)
- [system_ram_total_bytes](../../../../../functions/src/llm/provider/hardware_detect/system_ram_total_bytes.md)