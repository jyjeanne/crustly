---
type: Rust Module
title: ollama_models
resource: src/llm/provider/ollama_models.rs#L1-L349
generated:
  by: okf-rs/0.3.0
relationships:
  imports:
  - external/anyhow-context-result
  - external/futures-streamext-as
  - external/ollama-rs-generation-embeddings-request-embeddingsinput-generateembeddingsrequest
  - external/ollama-rs-ollama
  - external/tokio-sync-mpsc-unboundedsender
  - external/super
  - external/tokio-io-asyncreadext-asyncwriteext
  member_of:
  - packages/crustly
---

# Contains

- [LocalModelInfo](../../../../classes/src/llm/provider/ollama_models/LocalModelInfo.md)
- [PullProgress](../../../../classes/src/llm/provider/ollama_models/PullProgress.md)
- [is_success](../../../../functions/src/llm/provider/ollama_models/PullProgress/is_success.md)
- [fraction](../../../../functions/src/llm/provider/ollama_models/PullProgress/fraction.md)
- [ModelDetails](../../../../classes/src/llm/provider/ollama_models/ModelDetails.md)
- [client_for](../../../../functions/src/llm/provider/ollama_models/client_for.md)
- [list_models](../../../../functions/src/llm/provider/ollama_models/list_models.md)
- [show_model](../../../../functions/src/llm/provider/ollama_models/show_model.md)
- [delete_model](../../../../functions/src/llm/provider/ollama_models/delete_model.md)
- [pull_model](../../../../functions/src/llm/provider/ollama_models/pull_model.md)
- [generate_embeddings](../../../../functions/src/llm/provider/ollama_models/generate_embeddings.md)
- [pull_progress_fraction](../../../../functions/src/llm/provider/ollama_models/pull_progress_fraction.md)
- [pull_progress_fraction_missing_data](../../../../functions/src/llm/provider/ollama_models/pull_progress_fraction_missing_data.md)
- [pull_progress_is_success](../../../../functions/src/llm/provider/ollama_models/pull_progress_is_success.md)
- [invalid_host_returns_error](../../../../functions/src/llm/provider/ollama_models/invalid_host_returns_error.md)
- [embeddings_request_serializes_model_and_input](../../../../functions/src/llm/provider/ollama_models/embeddings_request_serializes_model_and_input.md)
- [embeddings_request_single_input_is_not_wrapped_in_array](../../../../functions/src/llm/provider/ollama_models/embeddings_request_single_input_is_not_wrapped_in_array.md)
- [mock_server](../../../../functions/src/llm/provider/ollama_models/mock_server.md)
- [list_models_parses_tags_response](../../../../functions/src/llm/provider/ollama_models/list_models_parses_tags_response.md)
- [show_model_parses_minimal_response](../../../../functions/src/llm/provider/ollama_models/show_model_parses_minimal_response.md)
- [delete_model_succeeds_on_2xx](../../../../functions/src/llm/provider/ollama_models/delete_model_succeeds_on_2xx.md)
- [pull_model_forwards_progress_and_completes](../../../../functions/src/llm/provider/ollama_models/pull_model_forwards_progress_and_completes.md)
- [generate_embeddings_parses_response](../../../../functions/src/llm/provider/ollama_models/generate_embeddings_parses_response.md)

# Imports

- `anyhow::{Context, Result}`
- `futures::StreamExt as _`
- `ollama_rs::generation::embeddings::request::{EmbeddingsInput, GenerateEmbeddingsRequest}`
- `ollama_rs::Ollama`
- `tokio::sync::mpsc::UnboundedSender`
- `super::*`
- `tokio::io::{AsyncReadExt, AsyncWriteExt}`

# Member of

- [crustly](../../../../packages/crustly.md)