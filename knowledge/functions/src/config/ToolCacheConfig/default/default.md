---
type: Rust Method
title: default
resource: src/config/mod.rs#L681-L688
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/config/default_read_file_ttl
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/default_glob_ttl
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/default_grep_ttl
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/default_web_search_ttl
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/config/Config/default/default
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`fn default() -> Self`

# Calls

- [default_read_file_ttl](../../../../../functions/src/config/default_read_file_ttl.md)
- [default_glob_ttl](../../../../../functions/src/config/default_glob_ttl.md)
- [default_grep_ttl](../../../../../functions/src/config/default_grep_ttl.md)
- [default_web_search_ttl](../../../../../functions/src/config/default_web_search_ttl.md)

# Called by

- [default](../../../../../functions/src/config/Config/default/default.md)