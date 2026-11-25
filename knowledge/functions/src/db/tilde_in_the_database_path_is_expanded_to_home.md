---
type: Rust Function
title: tilde_in_the_database_path_is_expanded_to_home
resource: src/db/mod.rs#L305-L330
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/db/Database/connect
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/Database/is_connected
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/memory/code_knowledge/CodeKnowledge/exists
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn tilde_in_the_database_path_is_expanded_to_home()`

# Calls

- [connect](../../../functions/src/db/Database/connect.md)
- [is_connected](../../../functions/src/db/Database/is_connected.md)
- [exists](../../../functions/src/memory/code_knowledge/CodeKnowledge/exists.md)