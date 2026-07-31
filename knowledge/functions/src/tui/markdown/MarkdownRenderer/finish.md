---
type: Rust Method
title: finish
resource: src/tui/markdown.rs#L204-L212
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/config/secrets/SecretString/is_empty
  called_by:
  - functions/benches/database/bench_session_create
  - functions/benches/database/bench_session_get
  - functions/benches/database/bench_session_list
  - functions/benches/database/bench_message_insert
  - functions/benches/database/bench_message_query
  - functions/benches/parallel_tool_dispatch/bench_parallel_dispatch
  - functions/src/llm/tools/cache/CacheKey/from_tool
  - functions/src/tui/markdown/parse_markdown
---

# Signature

`fn finish(mut self) -> Vec<Line<'static>>`

# Calls

- [is_empty](../../../../../functions/src/config/secrets/SecretString/is_empty.md)

# Called by

- [bench_session_create](../../../../../functions/benches/database/bench_session_create.md)
- [bench_session_get](../../../../../functions/benches/database/bench_session_get.md)
- [bench_session_list](../../../../../functions/benches/database/bench_session_list.md)
- [bench_message_insert](../../../../../functions/benches/database/bench_message_insert.md)
- [bench_message_query](../../../../../functions/benches/database/bench_message_query.md)
- [bench_parallel_dispatch](../../../../../functions/benches/parallel_tool_dispatch/bench_parallel_dispatch.md)
- [from_tool](../../../../../functions/src/llm/tools/cache/CacheKey/from_tool.md)
- [parse_markdown](../../../../../functions/src/tui/markdown/parse_markdown.md)