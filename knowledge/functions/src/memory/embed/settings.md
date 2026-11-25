---
type: Rust Function
title: settings
resource: src/memory/embed.rs#L348-L356
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/memory/embed/Prefixes/for_model
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/memory/embed/settings_are_checked_before_any_request
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/embed/ollama_sends_prefixed_input_and_returns_unit_vectors
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/embed/ollama_reports_an_error_status_without_panicking
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/embed/ollama_refuses_a_wrong_number_of_vectors
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/embed/openai_orders_by_index_and_sends_the_key
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/embed/an_unreachable_server_is_a_transport_error
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn settings(base_url: String, model: &str) -> EmbedSettings`

# Calls

- [for_model](../../../../functions/src/memory/embed/Prefixes/for_model.md)

# Called by

- [settings_are_checked_before_any_request](../../../../functions/src/memory/embed/settings_are_checked_before_any_request.md)
- [ollama_sends_prefixed_input_and_returns_unit_vectors](../../../../functions/src/memory/embed/ollama_sends_prefixed_input_and_returns_unit_vectors.md)
- [ollama_reports_an_error_status_without_panicking](../../../../functions/src/memory/embed/ollama_reports_an_error_status_without_panicking.md)
- [ollama_refuses_a_wrong_number_of_vectors](../../../../functions/src/memory/embed/ollama_refuses_a_wrong_number_of_vectors.md)
- [openai_orders_by_index_and_sends_the_key](../../../../functions/src/memory/embed/openai_orders_by_index_and_sends_the_key.md)
- [an_unreachable_server_is_a_transport_error](../../../../functions/src/memory/embed/an_unreachable_server_is_a_transport_error.md)