---
type: Rust Function
title: list_local_models
resource: src/llm/provider/llama_cpp_models.rs#L77-L113
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/tui/error/ErrorInfo/with_context
  - functions/src/config/secrets/SecretString/len
  called_by:
  - functions/src/cli/cmd_llama_cpp
  - functions/src/llm/provider/llama_cpp_models/list_local_models_on_nonexistent_dir_returns_empty_not_error
  - functions/src/llm/provider/llama_cpp_models/list_local_models_only_lists_gguf_files
  - functions/src/llm/provider/ollama_models/list_models
  - functions/src/tui/llama_cpp_download/list_local
---

# Signature

`pub fn list_local_models(models_dir: &Path) -> Result<Vec<LocalGgufModel>>`

# Calls

- [with_context](../../../../../functions/src/tui/error/ErrorInfo/with_context.md)
- [len](../../../../../functions/src/config/secrets/SecretString/len.md)

# Called by

- [cmd_llama_cpp](../../../../../functions/src/cli/cmd_llama_cpp.md)
- [list_local_models_on_nonexistent_dir_returns_empty_not_error](../../../../../functions/src/llm/provider/llama_cpp_models/list_local_models_on_nonexistent_dir_returns_empty_not_error.md)
- [list_local_models_only_lists_gguf_files](../../../../../functions/src/llm/provider/llama_cpp_models/list_local_models_only_lists_gguf_files.md)
- [list_models](../../../../../functions/src/llm/provider/ollama_models/list_models.md)
- [list_local](../../../../../functions/src/tui/llama_cpp_download/list_local.md)