---
type: Rust Method
title: new
resource: src/memory/embed.rs#L180-L189
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/memory/embed/check_settings
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/embed/client
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/memory/embed/ollama_sends_prefixed_input_and_returns_unit_vectors
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/memory/embed/ollama_reports_an_error_status_without_panicking
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/memory/embed/ollama_refuses_a_wrong_number_of_vectors
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/memory/embed/an_unreachable_server_is_a_transport_error
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/memory/embed/settings_are_checked_before_any_request
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/memory/embedder_from_config
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_benchmark_test/real_embedder_benchmark
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub fn new(settings: EmbedSettings) -> Result<Self, EmbedError>`

# Calls

- [check_settings](../../../../../functions/src/memory/embed/check_settings.md)
- [client](../../../../../functions/src/memory/embed/client.md)

# Called by

- [ollama_sends_prefixed_input_and_returns_unit_vectors](../../../../../functions/src/memory/embed/ollama_sends_prefixed_input_and_returns_unit_vectors.md)
- [ollama_reports_an_error_status_without_panicking](../../../../../functions/src/memory/embed/ollama_reports_an_error_status_without_panicking.md)
- [ollama_refuses_a_wrong_number_of_vectors](../../../../../functions/src/memory/embed/ollama_refuses_a_wrong_number_of_vectors.md)
- [an_unreachable_server_is_a_transport_error](../../../../../functions/src/memory/embed/an_unreachable_server_is_a_transport_error.md)
- [settings_are_checked_before_any_request](../../../../../functions/src/memory/embed/settings_are_checked_before_any_request.md)
- [embedder_from_config](../../../../../functions/src/memory/embedder_from_config.md)
- [real_embedder_benchmark](../../../../../functions/tests/memory_benchmark_test/real_embedder_benchmark.md)