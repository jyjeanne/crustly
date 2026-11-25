---
type: Rust Function
title: test_max_chars_truncation_does_not_panic_on_multibyte_text
resource: src/llm/tools/doc_parser.rs#L737-L749
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

`async fn test_max_chars_truncation_does_not_panic_on_multibyte_text()`

# Calls

- [context_with_file](../../../../../functions/src/llm/tools/doc_parser/context_with_file.md)
- [execute](../../../../../functions/src/llm/tools/doc_parser/DocParserTool/tool/execute.md)