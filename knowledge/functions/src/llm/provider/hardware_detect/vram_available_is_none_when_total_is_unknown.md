---
type: Rust Function
title: vram_available_is_none_when_total_is_unknown
resource: src/llm/provider/hardware_detect.rs#L626-L634
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

`fn vram_available_is_none_when_total_is_unknown()`

# Calls

- [vram_available_bytes](../../../../../functions/src/llm/provider/hardware_detect/GpuInfo/vram_available_bytes.md)