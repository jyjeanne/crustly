---
type: Rust Function
title: parse_vulkaninfo_summary
resource: src/llm/provider/hardware_detect.rs#L447-L461
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/tui/events/EventHandler/next
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/provider/hardware_detect/detect_vulkan_fallback
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/hardware_detect/parses_a_real_shaped_vulkaninfo_summary
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/hardware_detect/vulkaninfo_no_device_name_line_is_none
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn parse_vulkaninfo_summary(stdout: &str) -> Option<GpuInfo>`

# Calls

- [next](../../../../../functions/src/tui/events/EventHandler/next.md)

# Called by

- [detect_vulkan_fallback](../../../../../functions/src/llm/provider/hardware_detect/detect_vulkan_fallback.md)
- [parses_a_real_shaped_vulkaninfo_summary](../../../../../functions/src/llm/provider/hardware_detect/parses_a_real_shaped_vulkaninfo_summary.md)
- [vulkaninfo_no_device_name_line_is_none](../../../../../functions/src/llm/provider/hardware_detect/vulkaninfo_no_device_name_line_is_none.md)