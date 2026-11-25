---
type: Rust Method
title: parse_pdf
resource: src/llm/tools/doc_parser.rs#L231-L283
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/llm/tools/doc_parser/DocParserTool/tool/execute
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn parse_pdf( &self, path: &Path, input: &DocParserInput, ) -> Result<(String, ParsedMetadata)>`

# Called by

- [execute](../../../../../../functions/src/llm/tools/doc_parser/DocParserTool/tool/execute.md)