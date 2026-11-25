---
type: Rust Function
title: llama_cpp_doctor_findings
resource: src/cli/mod.rs#L2857-L3071
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/agent/service/tool_round/ToolRound/push
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/project/write
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp_models/available_space_at
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/ProviderConfigs/llama_cpp_extra_model_paths
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/code_knowledge/CodeKnowledge/exists
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/ProviderConfigs/llama_cpp_ollama_models_dir
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/hardware_detect/detect_hardware
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/hardware_detect/GpuInfo/vram_available_bytes
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp_models/list_all_local_models
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/hardware_detect/HardwareInfo/budget_bytes
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/llama_cpp_models/best_fitting_model
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/skill/SkillSource/display
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/cli/cmd_llama_cpp
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/llama_cpp_doctor_findings_reports_a_writable_models_dir_as_ok
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/llama_cpp_doctor_findings_warns_on_a_missing_models_dir
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/llama_cpp_doctor_findings_reports_extra_model_paths_existence
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/llama_cpp_doctor_findings_says_nothing_about_ollama_when_not_opted_in
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/llama_cpp_doctor_findings_reports_build_features
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/llama_cpp_doctor_findings_always_reports_detected_hardware
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/llama_cpp_doctor_findings_skips_the_best_fit_finding_when_no_models_present
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn llama_cpp_doctor_findings( config: &crate::config::Config, models_dir: &std::path::Path, ) -> Vec<DoctorFinding>`

# Calls

- [push](../../../functions/src/llm/agent/service/tool_round/ToolRound/push.md)
- [write](../../../functions/src/memory/project/write.md)
- [available_space_at](../../../functions/src/llm/provider/llama_cpp_models/available_space_at.md)
- [llama_cpp_extra_model_paths](../../../functions/src/config/ProviderConfigs/llama_cpp_extra_model_paths.md)
- [exists](../../../functions/src/memory/code_knowledge/CodeKnowledge/exists.md)
- [llama_cpp_ollama_models_dir](../../../functions/src/config/ProviderConfigs/llama_cpp_ollama_models_dir.md)
- [detect_hardware](../../../functions/src/llm/provider/hardware_detect/detect_hardware.md)
- [vram_available_bytes](../../../functions/src/llm/provider/hardware_detect/GpuInfo/vram_available_bytes.md)
- [list_all_local_models](../../../functions/src/llm/provider/llama_cpp_models/list_all_local_models.md)
- [budget_bytes](../../../functions/src/llm/provider/hardware_detect/HardwareInfo/budget_bytes.md)
- [best_fitting_model](../../../functions/src/llm/provider/llama_cpp_models/best_fitting_model.md)
- [display](../../../functions/src/llm/tools/skill/SkillSource/display.md)

# Called by

- [cmd_llama_cpp](../../../functions/src/cli/cmd_llama_cpp.md)
- [llama_cpp_doctor_findings_reports_a_writable_models_dir_as_ok](../../../functions/src/cli/llama_cpp_doctor_findings_reports_a_writable_models_dir_as_ok.md)
- [llama_cpp_doctor_findings_warns_on_a_missing_models_dir](../../../functions/src/cli/llama_cpp_doctor_findings_warns_on_a_missing_models_dir.md)
- [llama_cpp_doctor_findings_reports_extra_model_paths_existence](../../../functions/src/cli/llama_cpp_doctor_findings_reports_extra_model_paths_existence.md)
- [llama_cpp_doctor_findings_says_nothing_about_ollama_when_not_opted_in](../../../functions/src/cli/llama_cpp_doctor_findings_says_nothing_about_ollama_when_not_opted_in.md)
- [llama_cpp_doctor_findings_reports_build_features](../../../functions/src/cli/llama_cpp_doctor_findings_reports_build_features.md)
- [llama_cpp_doctor_findings_always_reports_detected_hardware](../../../functions/src/cli/llama_cpp_doctor_findings_always_reports_detected_hardware.md)
- [llama_cpp_doctor_findings_skips_the_best_fit_finding_when_no_models_present](../../../functions/src/cli/llama_cpp_doctor_findings_skips_the_best_fit_finding_when_no_models_present.md)