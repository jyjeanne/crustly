---
type: Rust Function
title: parse_nvidia_smi_csv
resource: src/llm/provider/hardware_detect.rs#L222-L237
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/memory/MemoryStore/find
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/events/EventHandler/next
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/models/PlanTaskStatus/parse
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/provider/hardware_detect/detect_nvidia
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/hardware_detect/parses_a_real_shaped_nvidia_smi_line
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/hardware_detect/nvidia_smi_only_reads_the_first_gpu_line
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/hardware_detect/nvidia_smi_degrades_to_name_only_when_memory_fields_are_unparseable
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/hardware_detect/nvidia_smi_empty_output_is_none
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/hardware_detect/nvidia_smi_line_with_no_usable_fields_is_none
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn parse_nvidia_smi_csv(stdout: &str) -> Option<GpuInfo>`

# Calls

- [find](../../../../../functions/src/memory/MemoryStore/find.md)
- [next](../../../../../functions/src/tui/events/EventHandler/next.md)
- [parse](../../../../../functions/src/db/models/PlanTaskStatus/parse.md)

# Called by

- [detect_nvidia](../../../../../functions/src/llm/provider/hardware_detect/detect_nvidia.md)
- [parses_a_real_shaped_nvidia_smi_line](../../../../../functions/src/llm/provider/hardware_detect/parses_a_real_shaped_nvidia_smi_line.md)
- [nvidia_smi_only_reads_the_first_gpu_line](../../../../../functions/src/llm/provider/hardware_detect/nvidia_smi_only_reads_the_first_gpu_line.md)
- [nvidia_smi_degrades_to_name_only_when_memory_fields_are_unparseable](../../../../../functions/src/llm/provider/hardware_detect/nvidia_smi_degrades_to_name_only_when_memory_fields_are_unparseable.md)
- [nvidia_smi_empty_output_is_none](../../../../../functions/src/llm/provider/hardware_detect/nvidia_smi_empty_output_is_none.md)
- [nvidia_smi_line_with_no_usable_fields_is_none](../../../../../functions/src/llm/provider/hardware_detect/nvidia_smi_line_with_no_usable_fields_is_none.md)