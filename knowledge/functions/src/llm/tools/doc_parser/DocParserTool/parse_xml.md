---
type: Rust Method
title: parse_xml
resource: src/llm/tools/doc_parser.rs#L561-L619
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/agent/service/tool_round/ToolRound/push
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/doc_parser/DocParserTool/resolve_general_ref
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/tools/doc_parser/DocParserTool/tool/execute
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn parse_xml(&self, path: &Path) -> Result<(String, ParsedMetadata)>`

# Calls

- [push](../../../../../../functions/src/llm/agent/service/tool_round/ToolRound/push.md)
- [resolve_general_ref](../../../../../../functions/src/llm/tools/doc_parser/DocParserTool/resolve_general_ref.md)

# Called by

- [execute](../../../../../../functions/src/llm/tools/doc_parser/DocParserTool/tool/execute.md)