---
type: Rust Function
title: cmd_llama_cpp
resource: src/cli/mod.rs#L2585-L2817
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/config/ProviderConfigs/llama_cpp_models_dir
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/ProviderConfigs/llama_cpp_extra_model_paths
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/ProviderConfigs/llama_cpp_ollama_models_dir
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp_models/list_all_local_models
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/hardware_detect/detect_hardware
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/hardware_detect/HardwareInfo/budget_bytes
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/llama_cpp_models/sort_by_fit
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/hardware_fit_label
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp_models/hardware_fit
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp_models/resolve_download_source
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp_models/download_model
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/ServiceManager/context
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/error/ErrorInfo/with_context
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/resolve_llama_cpp_model_path
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/llama_cpp_doctor_findings
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/skill/SkillSource/display
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn cmd_llama_cpp(config: &crate::config::Config, operation: LlamaCppCommands) -> Result<()>`

# Calls

- [llama_cpp_models_dir](../../../functions/src/config/ProviderConfigs/llama_cpp_models_dir.md)
- [llama_cpp_extra_model_paths](../../../functions/src/config/ProviderConfigs/llama_cpp_extra_model_paths.md)
- [llama_cpp_ollama_models_dir](../../../functions/src/config/ProviderConfigs/llama_cpp_ollama_models_dir.md)
- [list_all_local_models](../../../functions/src/llm/provider/llama_cpp_models/list_all_local_models.md)
- [detect_hardware](../../../functions/src/llm/provider/hardware_detect/detect_hardware.md)
- [budget_bytes](../../../functions/src/llm/provider/hardware_detect/HardwareInfo/budget_bytes.md)
- [sort_by_fit](../../../functions/src/llm/provider/llama_cpp_models/sort_by_fit.md)
- [hardware_fit_label](../../../functions/src/cli/hardware_fit_label.md)
- [hardware_fit](../../../functions/src/llm/provider/llama_cpp_models/hardware_fit.md)
- [resolve_download_source](../../../functions/src/llm/provider/llama_cpp_models/resolve_download_source.md)
- [download_model](../../../functions/src/llm/provider/llama_cpp_models/download_model.md)
- [context](../../../functions/src/services/ServiceManager/context.md)
- [with_context](../../../functions/src/tui/error/ErrorInfo/with_context.md)
- [resolve_llama_cpp_model_path](../../../functions/src/cli/resolve_llama_cpp_model_path.md)
- [llama_cpp_doctor_findings](../../../functions/src/cli/llama_cpp_doctor_findings.md)
- [display](../../../functions/src/llm/tools/skill/SkillSource/display.md)