---
type: Rust Method
title: query_symbol
resource: src/llm/agent/memory.rs#L92-L102
generated:
  by: okf-rs/0.3.0
relationships:
  called_by:
  - functions/tests/codebase_index_test/index_and_query_provider_trait
  - functions/tests/codebase_index_test/index_file_twice_no_duplicate
---

# Signature

`pub async fn query_symbol(&self, name: &str) -> Result<Vec<CodebaseIndexEntry>>`

# Called by

- [index_and_query_provider_trait](../../../../../../functions/tests/codebase_index_test/index_and_query_provider_trait.md)
- [index_file_twice_no_duplicate](../../../../../../functions/tests/codebase_index_test/index_file_twice_no_duplicate.md)