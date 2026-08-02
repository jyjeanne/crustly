---
type: Rust Module
title: doc_parser
resource: src/llm/tools/doc_parser.rs#L1-L779
generated:
  by: okf-rs/0.3.0
relationships:
  imports:
  - external/super-error-result-toolerror
  - external/super-r-trait-tool-toolcapability-toolexecutioncontext-toolresult
  - external/async-trait-async-trait
  - external/serde-deserialize-serialize
  - external/serde-json-value
  - external/std-io-read
  - external/std-path-path-pathbuf
  - external/super
  - external/std-io-write
  - external/tempfile-tempdir
  - external/uuid-uuid
  member_of:
  - packages/crustly
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