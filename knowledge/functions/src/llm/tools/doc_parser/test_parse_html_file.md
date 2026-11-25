---
type: Rust Function
title: test_parse_html_file
resource: src/llm/tools/doc_parser.rs#L696-L712
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/tools/doc_parser/context_with_file
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/doc_parser/DocParserTool/tool/execute
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`async fn test_parse_html_file()`

# Calls

- [context_with_file](../../../../../functions/src/llm/tools/doc_parser/context_with_file.md)
- [execute](../../../../../functions/src/llm/tools/doc_parser/DocParserTool/tool/execute.md)