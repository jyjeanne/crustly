---
type: Rust Module
title: doc_parser
resource: src/llm/tools/doc_parser.rs#L1-L902
generated:
  by: okf-rs/0.7.0
relationships:
  imports:
  - target: external/super-error-result-toolerror
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super-r-trait-tool-toolcapability-toolexecutioncontext-toolresult
    resolved_by: tree-sitter
    confidence: exact
  - target: external/async-trait-async-trait
    resolved_by: tree-sitter
    confidence: exact
  - target: external/serde-deserialize-serialize
    resolved_by: tree-sitter
    confidence: exact
  - target: external/serde-json-value
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-io-read
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-path-path-pathbuf
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-io-write
    resolved_by: tree-sitter
    confidence: exact
  - target: external/tempfile-tempdir
    resolved_by: tree-sitter
    confidence: exact
  - target: external/uuid-uuid
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/crustly
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [DocParserTool](../../../../classes/src/llm/tools/doc_parser/DocParserTool.md)
- [DocParserInput](../../../../classes/src/llm/tools/doc_parser/DocParserInput.md)
- [DocumentMetadata](../../../../classes/src/llm/tools/doc_parser/DocumentMetadata.md)
- [name](../../../../functions/src/llm/tools/doc_parser/DocParserTool/tool/name.md)
- [description](../../../../functions/src/llm/tools/doc_parser/DocParserTool/tool/description.md)
- [input_schema](../../../../functions/src/llm/tools/doc_parser/DocParserTool/tool/input_schema.md)
- [capabilities](../../../../functions/src/llm/tools/doc_parser/DocParserTool/tool/capabilities.md)
- [requires_approval](../../../../functions/src/llm/tools/doc_parser/DocParserTool/tool/requires_approval.md)
- [validate_input](../../../../functions/src/llm/tools/doc_parser/DocParserTool/tool/validate_input.md)
- [execute](../../../../functions/src/llm/tools/doc_parser/DocParserTool/tool/execute.md)
- [ParsedMetadata](../../../../classes/src/llm/tools/doc_parser/ParsedMetadata.md)
- [parse_pdf](../../../../functions/src/llm/tools/doc_parser/DocParserTool/parse_pdf.md)
- [parse_docx](../../../../functions/src/llm/tools/doc_parser/DocParserTool/parse_docx.md)
- [resolve_general_ref](../../../../functions/src/llm/tools/doc_parser/DocParserTool/resolve_general_ref.md)
- [extract_text_from_docx_xml](../../../../functions/src/llm/tools/doc_parser/DocParserTool/extract_text_from_docx_xml.md)
- [extract_metadata_from_core_xml](../../../../functions/src/llm/tools/doc_parser/DocParserTool/extract_metadata_from_core_xml.md)
- [parse_text](../../../../functions/src/llm/tools/doc_parser/DocParserTool/parse_text.md)
- [parse_html](../../../../functions/src/llm/tools/doc_parser/DocParserTool/parse_html.md)
- [strip_html_tags](../../../../functions/src/llm/tools/doc_parser/DocParserTool/strip_html_tags.md)
- [extract_html_title](../../../../functions/src/llm/tools/doc_parser/DocParserTool/extract_html_title.md)
- [parse_json](../../../../functions/src/llm/tools/doc_parser/DocParserTool/parse_json.md)
- [parse_xml](../../../../functions/src/llm/tools/doc_parser/DocParserTool/parse_xml.md)
- [context_with_file](../../../../functions/src/llm/tools/doc_parser/context_with_file.md)
- [test_parse_text_file](../../../../functions/src/llm/tools/doc_parser/test_parse_text_file.md)
- [test_parse_markdown_file](../../../../functions/src/llm/tools/doc_parser/test_parse_markdown_file.md)
- [test_parse_json_file](../../../../functions/src/llm/tools/doc_parser/test_parse_json_file.md)
- [test_parse_html_file](../../../../functions/src/llm/tools/doc_parser/test_parse_html_file.md)
- [test_max_chars_truncation](../../../../functions/src/llm/tools/doc_parser/test_max_chars_truncation.md)
- [test_max_chars_truncation_does_not_panic_on_multibyte_text](../../../../functions/src/llm/tools/doc_parser/test_max_chars_truncation_does_not_panic_on_multibyte_text.md)
- [test_unsupported_format](../../../../functions/src/llm/tools/doc_parser/test_unsupported_format.md)
- [test_nonexistent_file](../../../../functions/src/llm/tools/doc_parser/test_nonexistent_file.md)
- [test_path_outside_working_directory_is_denied](../../../../functions/src/llm/tools/doc_parser/test_path_outside_working_directory_is_denied.md)
- [test_tool_schema](../../../../functions/src/llm/tools/doc_parser/test_tool_schema.md)
- [test_strip_html_tags](../../../../functions/src/llm/tools/doc_parser/test_strip_html_tags.md)
- [test_extract_html_title](../../../../functions/src/llm/tools/doc_parser/test_extract_html_title.md)
- [test_parse_xml_file](../../../../functions/src/llm/tools/doc_parser/test_parse_xml_file.md)
- [test_extract_text_from_docx_xml_reads_paragraphs_and_unescapes_entities](../../../../functions/src/llm/tools/doc_parser/test_extract_text_from_docx_xml_reads_paragraphs_and_unescapes_entities.md)
- [test_extract_text_from_docx_xml_ignores_text_outside_w_t](../../../../functions/src/llm/tools/doc_parser/test_extract_text_from_docx_xml_ignores_text_outside_w_t.md)
- [test_extract_metadata_from_core_xml](../../../../functions/src/llm/tools/doc_parser/test_extract_metadata_from_core_xml.md)
- [test_extract_metadata_from_core_xml_missing_fields](../../../../functions/src/llm/tools/doc_parser/test_extract_metadata_from_core_xml_missing_fields.md)

# Imports

- `super::error::{Result, ToolError}`
- `super::r#trait::{Tool, ToolCapability, ToolExecutionContext, ToolResult}`
- `async_trait::async_trait`
- `serde::{Deserialize, Serialize}`
- `serde_json::Value`
- `std::io::Read`
- `std::path::{Path, PathBuf}`
- `super::*`
- `std::io::Write`
- `tempfile::TempDir`
- `uuid::Uuid`

# Member of

- [crustly](../../../../packages/crustly.md)