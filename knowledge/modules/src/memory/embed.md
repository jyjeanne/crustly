---
type: Rust Module
title: embed
resource: src/memory/embed.rs#L1-L470
generated:
  by: okf-rs/0.8.0
relationships:
  imports:
  - target: external/super-vector-normalize
    resolved_by: tree-sitter
    confidence: exact
  - target: external/async-trait-async-trait
    resolved_by: tree-sitter
    confidence: exact
  - target: external/serde-deserialize
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-fmt
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-time-duration
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super
    resolved_by: tree-sitter
    confidence: exact
  - target: external/tokio-io-asyncreadext-asyncwriteext
    resolved_by: tree-sitter
    confidence: exact
  - target: external/tokio-net-tcplistener
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/crustly
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [EmbedPurpose](../../../classes/src/memory/embed/EmbedPurpose.md)
- [EmbedError](../../../classes/src/memory/embed/EmbedError.md)
- [fmt](../../../functions/src/memory/embed/EmbedError/fmt-display/fmt.md)
- [Embedder](../../../interfaces/src/memory/embed/Embedder.md)
- [Prefixes](../../../classes/src/memory/embed/Prefixes.md)
- [for_model](../../../functions/src/memory/embed/Prefixes/for_model.md)
- [apply](../../../functions/src/memory/embed/Prefixes/apply.md)
- [EmbedSettings](../../../classes/src/memory/embed/EmbedSettings.md)
- [check_settings](../../../functions/src/memory/embed/check_settings.md)
- [client](../../../functions/src/memory/embed/client.md)
- [finish](../../../functions/src/memory/embed/finish.md)
- [OllamaEmbedResponse](../../../classes/src/memory/embed/OllamaEmbedResponse.md)
- [OllamaEmbedder](../../../classes/src/memory/embed/OllamaEmbedder.md)
- [new](../../../functions/src/memory/embed/OllamaEmbedder/new.md)
- [model_id](../../../functions/src/memory/embed/OllamaEmbedder/embedder/model_id.md)
- [embed](../../../functions/src/memory/embed/OllamaEmbedder/embedder/embed.md)
- [OpenAiEmbedResponse](../../../classes/src/memory/embed/OpenAiEmbedResponse.md)
- [OpenAiEmbedding](../../../classes/src/memory/embed/OpenAiEmbedding.md)
- [OpenAiCompatibleEmbedder](../../../classes/src/memory/embed/OpenAiCompatibleEmbedder.md)
- [new](../../../functions/src/memory/embed/OpenAiCompatibleEmbedder/new.md)
- [model_id](../../../functions/src/memory/embed/OpenAiCompatibleEmbedder/embedder/model_id.md)
- [embed](../../../functions/src/memory/embed/OpenAiCompatibleEmbedder/embedder/embed.md)
- [mock_server](../../../functions/src/memory/embed/mock_server.md)
- [settings](../../../functions/src/memory/embed/settings.md)
- [nomic_model_gets_its_task_prefixes](../../../functions/src/memory/embed/nomic_model_gets_its_task_prefixes.md)
- [configured_prefixes_override_the_built_in_ones](../../../functions/src/memory/embed/configured_prefixes_override_the_built_in_ones.md)
- [settings_are_checked_before_any_request](../../../functions/src/memory/embed/settings_are_checked_before_any_request.md)
- [ollama_sends_prefixed_input_and_returns_unit_vectors](../../../functions/src/memory/embed/ollama_sends_prefixed_input_and_returns_unit_vectors.md)
- [ollama_reports_an_error_status_without_panicking](../../../functions/src/memory/embed/ollama_reports_an_error_status_without_panicking.md)
- [ollama_refuses_a_wrong_number_of_vectors](../../../functions/src/memory/embed/ollama_refuses_a_wrong_number_of_vectors.md)
- [openai_orders_by_index_and_sends_the_key](../../../functions/src/memory/embed/openai_orders_by_index_and_sends_the_key.md)
- [an_unreachable_server_is_a_transport_error](../../../functions/src/memory/embed/an_unreachable_server_is_a_transport_error.md)

# Imports

- `super::vector::normalize`
- `async_trait::async_trait`
- `serde::Deserialize`
- `std::fmt`
- `std::time::Duration`
- `super::*`
- `tokio::io::{AsyncReadExt, AsyncWriteExt}`
- `tokio::net::TcpListener`

# Member of

- [crustly](../../../packages/crustly.md)