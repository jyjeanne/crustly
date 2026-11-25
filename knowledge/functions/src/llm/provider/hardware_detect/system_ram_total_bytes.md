---
type: Rust Function
title: system_ram_total_bytes
resource: src/llm/provider/hardware_detect.rs#L474-L478
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/llm/provider/hardware_detect/detect_hardware_uncached
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/hardware_detect/system_ram_total_bytes_returns_a_plausible_value_on_a_real_machine
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn system_ram_total_bytes() -> Option<u64>`

# Called by

- [detect_hardware_uncached](../../../../../functions/src/llm/provider/hardware_detect/detect_hardware_uncached.md)
- [system_ram_total_bytes_returns_a_plausible_value_on_a_real_machine](../../../../../functions/src/llm/provider/hardware_detect/system_ram_total_bytes_returns_a_plausible_value_on_a_real_machine.md)