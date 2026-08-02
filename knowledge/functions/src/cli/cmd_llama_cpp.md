---
type: Rust Function
title: cmd_llama_cpp
resource: src/cli/mod.rs#L1251-L1354
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/config/ProviderConfigs/llama_cpp_models_dir
  - functions/src/llm/provider/llama_cpp_models/list_local_models
  - functions/src/config/secrets/SecretString/is_empty
  - functions/src/llm/provider/llama_cpp_models/resolve_download_source
  - functions/src/llm/provider/llama_cpp_models/download_model
  - functions/src/tui/error/ErrorInfo/with_context
  - functions/src/cli/resolve_llama_cpp_model_path
  - functions/src/config/secrets/SecretString/len
---

# Signature

`async fn cmd_llama_cpp(config: &crate::config::Config, operation: LlamaCppCommands) -> Result<()>`

# Calls

- [llama_cpp_models_dir](../../../functions/src/config/ProviderConfigs/llama_cpp_models_dir.md)
- [list_local_models](../../../functions/src/llm/provider/llama_cpp_models/list_local_models.md)
- [is_empty](../../../functions/src/config/secrets/SecretString/is_empty.md)
- [resolve_download_source](../../../functions/src/llm/provider/llama_cpp_models/resolve_download_source.md)
- [download_model](../../../functions/src/llm/provider/llama_cpp_models/download_model.md)
- [with_context](../../../functions/src/tui/error/ErrorInfo/with_context.md)
- [resolve_llama_cpp_model_path](../../../functions/src/cli/resolve_llama_cpp_model_path.md)
- [len](../../../functions/src/config/secrets/SecretString/len.md)