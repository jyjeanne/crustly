---
type: Rust Method
title: for_model
resource: src/memory/embed.rs#L81-L99
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/tui/events/EventHandler/next
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/memory/embed/settings
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/embed/nomic_model_gets_its_task_prefixes
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/embed/configured_prefixes_override_the_built_in_ones
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/embedder_from_config
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_benchmark_test/real_embedder_benchmark
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn for_model( model: &str, query_override: Option<&str>, document_override: Option<&str>, ) -> Self`

# Calls

- [next](../../../../../functions/src/tui/events/EventHandler/next.md)

# Called by

- [settings](../../../../../functions/src/memory/embed/settings.md)
- [nomic_model_gets_its_task_prefixes](../../../../../functions/src/memory/embed/nomic_model_gets_its_task_prefixes.md)
- [configured_prefixes_override_the_built_in_ones](../../../../../functions/src/memory/embed/configured_prefixes_override_the_built_in_ones.md)
- [embedder_from_config](../../../../../functions/src/memory/embedder_from_config.md)
- [real_embedder_benchmark](../../../../../functions/tests/memory_benchmark_test/real_embedder_benchmark.md)