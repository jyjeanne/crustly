---
type: Rust Method
title: vram_available_bytes
resource: src/llm/provider/hardware_detect.rs#L90-L96
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/cli/llama_cpp_doctor_findings
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/hardware_detect/parses_a_real_shaped_nvidia_smi_line
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/hardware_detect/vram_available_falls_back_to_total_when_used_is_unknown
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/hardware_detect/vram_available_is_none_when_total_is_unknown
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/llama_cpp_download/detect_hardware_summary
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn vram_available_bytes(&self) -> Option<u64>`

# Called by

- [llama_cpp_doctor_findings](../../../../../../functions/src/cli/llama_cpp_doctor_findings.md)
- [parses_a_real_shaped_nvidia_smi_line](../../../../../../functions/src/llm/provider/hardware_detect/parses_a_real_shaped_nvidia_smi_line.md)
- [vram_available_falls_back_to_total_when_used_is_unknown](../../../../../../functions/src/llm/provider/hardware_detect/vram_available_falls_back_to_total_when_used_is_unknown.md)
- [vram_available_is_none_when_total_is_unknown](../../../../../../functions/src/llm/provider/hardware_detect/vram_available_is_none_when_total_is_unknown.md)
- [detect_hardware_summary](../../../../../../functions/src/tui/llama_cpp_download/detect_hardware_summary.md)