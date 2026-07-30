---
type: Rust Method
title: parse_text
resource: src/llm/tools/doc_parser.rs#L411-L423
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  called_by:
  - functions/src/llm/tools/doc_parser/DocParserTool/tool/execute
---

# Signature

`async fn parse_text(&self, path: &Path, _format: &str) -> Result<(String, ParsedMetadata)>`

# Called by

- [execute](../../../../../../functions/src/llm/tools/doc_parser/DocParserTool/tool/execute.md)