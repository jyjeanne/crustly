---
type: Rust Function
title: trim_dxgi_description
resource: src/llm/provider/hardware_detect.rs#L311-L314
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/llm/provider/hardware_detect/detect_windows_dxgi
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/hardware_detect/trim_dxgi_description_stops_at_the_first_nul
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/hardware_detect/trim_dxgi_description_handles_a_buffer_with_no_nul_terminator
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn trim_dxgi_description(raw: &[u16]) -> String`

# Called by

- [detect_windows_dxgi](../../../../../functions/src/llm/provider/hardware_detect/detect_windows_dxgi.md)
- [trim_dxgi_description_stops_at_the_first_nul](../../../../../functions/src/llm/provider/hardware_detect/trim_dxgi_description_stops_at_the_first_nul.md)
- [trim_dxgi_description_handles_a_buffer_with_no_nul_terminator](../../../../../functions/src/llm/provider/hardware_detect/trim_dxgi_description_handles_a_buffer_with_no_nul_terminator.md)