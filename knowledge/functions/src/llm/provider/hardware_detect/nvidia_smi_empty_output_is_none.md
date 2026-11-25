---
type: Rust Function
title: nvidia_smi_empty_output_is_none
resource: src/llm/provider/hardware_detect.rs#L509-L512
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

`fn nvidia_smi_empty_output_is_none()`

# Calls

- [parse_nvidia_smi_csv](../../../../../functions/src/llm/provider/hardware_detect/parse_nvidia_smi_csv.md)