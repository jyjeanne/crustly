---
type: Rust Function
title: download_model
resource: src/llm/provider/llama_cpp_models.rs#L995-L1143
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/tui/error/ErrorInfo/with_context
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/events/EventHandler/next
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/ServiceManager/context
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/code_knowledge/CodeKnowledge/status
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp_models/check_disk_space
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp_models/available_space_at
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_acceptance_test/open
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp_models/to_hex
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/skill/SkillSource/display
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/cli/cmd_llama_cpp
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp_models/download_model_writes_the_file_and_reports_progress
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp_models/download_model_rejects_a_checksum_mismatch_and_cleans_up
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp_models/download_model_resumes_from_an_existing_partial_file
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp_models/download_model_verifies_checksum_across_a_resumed_download
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp_models/download_model_falls_back_to_a_fresh_start_when_the_server_ignores_range
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/llama_cpp_download/spawn_download
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn download_model( url: &str, models_dir: &Path, expected_sha256: Option<&str>, progress_tx: UnboundedSender<DownloadProgress>, ) -> Result<PathBuf>`

# Calls

- [with_context](../../../../../functions/src/tui/error/ErrorInfo/with_context.md)
- [next](../../../../../functions/src/tui/events/EventHandler/next.md)
- [context](../../../../../functions/src/services/ServiceManager/context.md)
- [status](../../../../../functions/src/memory/code_knowledge/CodeKnowledge/status.md)
- [check_disk_space](../../../../../functions/src/llm/provider/llama_cpp_models/check_disk_space.md)
- [available_space_at](../../../../../functions/src/llm/provider/llama_cpp_models/available_space_at.md)
- [open](../../../../../functions/tests/memory_acceptance_test/open.md)
- [to_hex](../../../../../functions/src/llm/provider/llama_cpp_models/to_hex.md)
- [display](../../../../../functions/src/llm/tools/skill/SkillSource/display.md)

# Called by

- [cmd_llama_cpp](../../../../../functions/src/cli/cmd_llama_cpp.md)
- [download_model_writes_the_file_and_reports_progress](../../../../../functions/src/llm/provider/llama_cpp_models/download_model_writes_the_file_and_reports_progress.md)
- [download_model_rejects_a_checksum_mismatch_and_cleans_up](../../../../../functions/src/llm/provider/llama_cpp_models/download_model_rejects_a_checksum_mismatch_and_cleans_up.md)
- [download_model_resumes_from_an_existing_partial_file](../../../../../functions/src/llm/provider/llama_cpp_models/download_model_resumes_from_an_existing_partial_file.md)
- [download_model_verifies_checksum_across_a_resumed_download](../../../../../functions/src/llm/provider/llama_cpp_models/download_model_verifies_checksum_across_a_resumed_download.md)
- [download_model_falls_back_to_a_fresh_start_when_the_server_ignores_range](../../../../../functions/src/llm/provider/llama_cpp_models/download_model_falls_back_to_a_fresh_start_when_the_server_ignores_range.md)
- [spawn_download](../../../../../functions/src/tui/llama_cpp_download/spawn_download.md)