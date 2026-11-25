---
type: Rust Function
title: parse_system_profiler_json
resource: src/llm/provider/hardware_detect.rs#L384-L413
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/memory/MemoryStore/find
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/vector_index/VectorIndex/contains
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/provider/hardware_detect/detect_apple_display
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/hardware_detect/parses_a_real_shaped_system_profiler_json_blob
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/hardware_detect/system_profiler_name_only_when_no_vram_key_present
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/hardware_detect/system_profiler_malformed_or_empty_is_none
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn parse_system_profiler_json(stdout: &str) -> Option<GpuInfo>`

# Calls

- [find](../../../../../functions/src/memory/MemoryStore/find.md)
- [contains](../../../../../functions/src/memory/vector_index/VectorIndex/contains.md)

# Called by

- [detect_apple_display](../../../../../functions/src/llm/provider/hardware_detect/detect_apple_display.md)
- [parses_a_real_shaped_system_profiler_json_blob](../../../../../functions/src/llm/provider/hardware_detect/parses_a_real_shaped_system_profiler_json_blob.md)
- [system_profiler_name_only_when_no_vram_key_present](../../../../../functions/src/llm/provider/hardware_detect/system_profiler_name_only_when_no_vram_key_present.md)
- [system_profiler_malformed_or_empty_is_none](../../../../../functions/src/llm/provider/hardware_detect/system_profiler_malformed_or_empty_is_none.md)