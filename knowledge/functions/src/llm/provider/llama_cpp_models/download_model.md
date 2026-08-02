---
type: Rust Function
title: download_model
resource: src/llm/provider/llama_cpp_models.rs#L182-L254
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/tui/error/ErrorInfo/with_context
  - functions/src/tui/events/EventHandler/next
  - functions/src/config/secrets/SecretString/is_empty
  - functions/src/config/secrets/SecretString/len
  - functions/src/llm/tools/task/FileLock/drop/drop
  - functions/src/llm/provider/llama_cpp_models/to_hex
  called_by:
  - functions/src/cli/cmd_llama_cpp
  - functions/src/llm/provider/llama_cpp_models/download_model_writes_the_file_and_reports_progress
  - functions/src/llm/provider/llama_cpp_models/download_model_rejects_a_checksum_mismatch_and_cleans_up
  - functions/src/tui/llama_cpp_download/spawn_download
---

# Signature

`pub async fn download_model( url: &str, models_dir: &Path, expected_sha256: Option<&str>, progress_tx: UnboundedSender<DownloadProgress>, ) -> Result<PathBuf>`

# Calls

- [with_context](../../../../../functions/src/tui/error/ErrorInfo/with_context.md)
- [next](../../../../../functions/src/tui/events/EventHandler/next.md)
- [is_empty](../../../../../functions/src/config/secrets/SecretString/is_empty.md)
- [len](../../../../../functions/src/config/secrets/SecretString/len.md)
- [drop](../../../../../functions/src/llm/tools/task/FileLock/drop/drop.md)
- [to_hex](../../../../../functions/src/llm/provider/llama_cpp_models/to_hex.md)

# Called by

- [cmd_llama_cpp](../../../../../functions/src/cli/cmd_llama_cpp.md)
- [download_model_writes_the_file_and_reports_progress](../../../../../functions/src/llm/provider/llama_cpp_models/download_model_writes_the_file_and_reports_progress.md)
- [download_model_rejects_a_checksum_mismatch_and_cleans_up](../../../../../functions/src/llm/provider/llama_cpp_models/download_model_rejects_a_checksum_mismatch_and_cleans_up.md)
- [spawn_download](../../../../../functions/src/tui/llama_cpp_download/spawn_download.md)