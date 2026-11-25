---
type: Rust Function
title: nvidia_smi_degrades_to_name_only_when_memory_fields_are_unparseable
resource: src/llm/provider/hardware_detect.rs#L520-L529
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/provider/hardware_detect/parse_nvidia_smi_csv
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn nvidia_smi_degrades_to_name_only_when_memory_fields_are_unparseable()`

# Calls

- [parse_nvidia_smi_csv](../../../../../functions/src/llm/provider/hardware_detect/parse_nvidia_smi_csv.md)