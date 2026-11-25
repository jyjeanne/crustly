---
type: Rust Method
title: llama_cpp_ollama_models_dir
resource: src/config/mod.rs#L863-L878
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/cli/cmd_chat
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/cmd_llama_cpp
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/llama_cpp_doctor_findings
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn llama_cpp_ollama_models_dir(&self) -> Option<std::path::PathBuf>`

# Called by

- [cmd_chat](../../../../functions/src/cli/cmd_chat.md)
- [cmd_llama_cpp](../../../../functions/src/cli/cmd_llama_cpp.md)
- [llama_cpp_doctor_findings](../../../../functions/src/cli/llama_cpp_doctor_findings.md)