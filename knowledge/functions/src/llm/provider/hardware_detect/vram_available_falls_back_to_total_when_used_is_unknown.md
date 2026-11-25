---
type: Rust Function
title: vram_available_falls_back_to_total_when_used_is_unknown
resource: src/llm/provider/hardware_detect.rs#L615-L623
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/provider/hardware_detect/GpuInfo/vram_available_bytes
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn vram_available_falls_back_to_total_when_used_is_unknown()`

# Calls

- [vram_available_bytes](../../../../../functions/src/llm/provider/hardware_detect/GpuInfo/vram_available_bytes.md)