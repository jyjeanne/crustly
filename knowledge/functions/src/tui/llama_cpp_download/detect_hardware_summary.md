---
type: Rust Function
title: detect_hardware_summary
resource: src/tui/llama_cpp_download.rs#L129-L144
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/provider/hardware_detect/detect_hardware
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/hardware_detect/GpuInfo/vram_available_bytes
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn detect_hardware_summary() -> HardwareSummary`

# Calls

- [detect_hardware](../../../../functions/src/llm/provider/hardware_detect/detect_hardware.md)
- [vram_available_bytes](../../../../functions/src/llm/provider/hardware_detect/GpuInfo/vram_available_bytes.md)