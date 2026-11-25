---
type: Rust Function
title: parse_size_string
resource: src/llm/provider/hardware_detect.rs#L419-L429
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/db/models/PlanTaskStatus/parse
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/provider/hardware_detect/parse_size_string_handles_mb_and_gb
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/hardware_detect/parse_size_string_rejects_unknown_units
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn parse_size_string(s: &str) -> Option<u64>`

# Calls

- [parse](../../../../../functions/src/db/models/PlanTaskStatus/parse.md)

# Called by

- [parse_size_string_handles_mb_and_gb](../../../../../functions/src/llm/provider/hardware_detect/parse_size_string_handles_mb_and_gb.md)
- [parse_size_string_rejects_unknown_units](../../../../../functions/src/llm/provider/hardware_detect/parse_size_string_rejects_unknown_units.md)