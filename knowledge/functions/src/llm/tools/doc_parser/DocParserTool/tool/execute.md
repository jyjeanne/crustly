---
type: Rust Method
title: execute
resource: src/llm/tools/doc_parser.rs#L102-L219
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/llm/tools/sandbox/check_path
  - functions/src/config/secrets/SecretString/len
  - functions/src/llm/tools/doc_parser/DocParserTool/parse_pdf
  - functions/src/llm/tools/doc_parser/DocParserTool/parse_docx
  - functions/src/llm/tools/doc_parser/DocParserTool/parse_text
  - functions/src/llm/tools/doc_parser/DocParserTool/parse_html
  - functions/src/llm/tools/doc_parser/DocParserTool/parse_json
  - functions/src/llm/tools/doc_parser/DocParserTool/parse_xml
  - functions/src/llm/tools/trait/ToolResult/with_metadata
---

# Signature

`async fn execute(&self, input: Value, context: &ToolExecutionContext) -> Result<ToolResult>`

# Calls

- [check_path](../../../../../../../functions/src/llm/tools/sandbox/check_path.md)
- [len](../../../../../../../functions/src/config/secrets/SecretString/len.md)
- [parse_pdf](../../../../../../../functions/src/llm/tools/doc_parser/DocParserTool/parse_pdf.md)
- [parse_docx](../../../../../../../functions/src/llm/tools/doc_parser/DocParserTool/parse_docx.md)
- [parse_text](../../../../../../../functions/src/llm/tools/doc_parser/DocParserTool/parse_text.md)
- [parse_html](../../../../../../../functions/src/llm/tools/doc_parser/DocParserTool/parse_html.md)
- [parse_json](../../../../../../../functions/src/llm/tools/doc_parser/DocParserTool/parse_json.md)
- [parse_xml](../../../../../../../functions/src/llm/tools/doc_parser/DocParserTool/parse_xml.md)
- [with_metadata](../../../../../../../functions/src/llm/tools/trait/ToolResult/with_metadata.md)