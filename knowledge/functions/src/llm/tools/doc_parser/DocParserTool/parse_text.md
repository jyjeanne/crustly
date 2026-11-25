---
type: Rust Method
title: parse_text
resource: src/llm/tools/doc_parser.rs#L451-L463
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/llm/tools/doc_parser/DocParserTool/tool/execute
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn parse_text(&self, path: &Path, _format: &str) -> Result<(String, ParsedMetadata)>`

# Called by

- [execute](../../../../../../functions/src/llm/tools/doc_parser/DocParserTool/tool/execute.md)