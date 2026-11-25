---
type: Rust Function
title: embedder_from_config
resource: src/memory/mod.rs#L856-L891
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/memory/semantic/EmbedRuntime/default/default
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/memory/embed/Prefixes/for_model
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/embed/OllamaEmbedder/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/memory/embed/OpenAiCompatibleEmbedder/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  called_by:
  - target: functions/src/cli/build_memory_store
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_semantic_test/the_embeddings_settings_build_the_right_provider_or_none
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn embedder_from_config( config: &crate::config::MemoryEmbeddingsConfig, ) -> Result<Option<EmbedderParts>, embed::EmbedError>`

# Calls

- [default](../../../functions/src/memory/semantic/EmbedRuntime/default/default.md)
- [for_model](../../../functions/src/memory/embed/Prefixes/for_model.md)
- [new](../../../functions/src/memory/embed/OllamaEmbedder/new.md)
- [new](../../../functions/src/memory/embed/OpenAiCompatibleEmbedder/new.md)

# Called by

- [build_memory_store](../../../functions/src/cli/build_memory_store.md)
- [the_embeddings_settings_build_the_right_provider_or_none](../../../functions/tests/memory_semantic_test/the_embeddings_settings_build_the_right_provider_or_none.md)