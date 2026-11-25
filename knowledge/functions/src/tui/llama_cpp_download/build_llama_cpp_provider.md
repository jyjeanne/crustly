---
type: Rust Function
title: build_llama_cpp_provider
resource: src/tui/llama_cpp_download.rs#L246-L255
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/provider/llama_cpp/LlamaCppProvider/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub fn build_llama_cpp_provider( model_path: PathBuf, config: Option<&crate::config::LlamaCppProviderConfig>, ) -> Result<Arc<dyn crate::llm::provider::Provider>, String>`

# Calls

- [new](../../../../functions/src/llm/provider/llama_cpp/LlamaCppProvider/new.md)