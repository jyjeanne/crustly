---
type: Rust Function
title: build_llama_cpp_provider
resource: src/tui/llama_cpp_download.rs#L102-L111
generated:
  by: okf-rs/0.3.0
---

# Signature

`pub fn build_llama_cpp_provider( model_path: PathBuf, config: Option<&crate::config::LlamaCppProviderConfig>, ) -> Result<Arc<dyn crate::llm::provider::Provider>, String>`