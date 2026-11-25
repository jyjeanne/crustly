---
type: Rust Method
title: parse_json
resource: src/llm/tools/doc_parser.rs#L540-L558
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

`async fn parse_json(&self, path: &Path) -> Result<(String, ParsedMetadata)>`

# Called by

- [execute](../../../../../../functions/src/llm/tools/doc_parser/DocParserTool/tool/execute.md)