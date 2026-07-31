---
type: Rust Method
title: parse_json
resource: src/llm/tools/doc_parser.rs#L500-L518
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  called_by:
  - functions/src/llm/tools/doc_parser/DocParserTool/tool/execute
---

# Signature

`async fn parse_json(&self, path: &Path) -> Result<(String, ParsedMetadata)>`

# Called by

- [execute](../../../../../../functions/src/llm/tools/doc_parser/DocParserTool/tool/execute.md)