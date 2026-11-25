---
type: Rust Method
title: budget_bytes
resource: src/llm/provider/hardware_detect.rs#L119-L125
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/cli/cmd_llama_cpp
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/cli/llama_cpp_doctor_findings
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/hardware_detect/budget_bytes_sums_vram_and_ram_when_both_known
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/hardware_detect/budget_bytes_is_ram_only_when_no_gpu
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/hardware_detect/budget_bytes_is_none_when_nothing_was_determined
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub fn budget_bytes(&self) -> Option<u64>`

# Called by

- [cmd_llama_cpp](../../../../../../functions/src/cli/cmd_llama_cpp.md)
- [llama_cpp_doctor_findings](../../../../../../functions/src/cli/llama_cpp_doctor_findings.md)
- [budget_bytes_sums_vram_and_ram_when_both_known](../../../../../../functions/src/llm/provider/hardware_detect/budget_bytes_sums_vram_and_ram_when_both_known.md)
- [budget_bytes_is_ram_only_when_no_gpu](../../../../../../functions/src/llm/provider/hardware_detect/budget_bytes_is_ram_only_when_no_gpu.md)
- [budget_bytes_is_none_when_nothing_was_determined](../../../../../../functions/src/llm/provider/hardware_detect/budget_bytes_is_none_when_nothing_was_determined.md)