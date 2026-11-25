---
type: Rust Function
title: nvidia_smi_only_reads_the_first_gpu_line
resource: src/llm/provider/hardware_detect.rs#L500-L506
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

`fn nvidia_smi_only_reads_the_first_gpu_line()`

# Calls

- [parse_nvidia_smi_csv](../../../../../functions/src/llm/provider/hardware_detect/parse_nvidia_smi_csv.md)