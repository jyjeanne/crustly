---
type: Rust Module
title: web_search
resource: src/llm/tools/web_search.rs#L1-L226
generated:
  by: okf-rs/0.8.0
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
  member_of:
  - target: packages/crustly
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [WebSearchTool](../../../../classes/src/llm/tools/web_search/WebSearchTool.md)
- [SearchInput](../../../../classes/src/llm/tools/web_search/SearchInput.md)
- [default_max_results](../../../../functions/src/llm/tools/web_search/default_max_results.md)
- [DuckDuckGoResponse](../../../../classes/src/llm/tools/web_search/DuckDuckGoResponse.md)
- [RelatedTopic](../../../../classes/src/llm/tools/web_search/RelatedTopic.md)
- [TopicItem](../../../../classes/src/llm/tools/web_search/TopicItem.md)
- [name](../../../../functions/src/llm/tools/web_search/WebSearchTool/tool/name.md)
- [description](../../../../functions/src/llm/tools/web_search/WebSearchTool/tool/description.md)
- [input_schema](../../../../functions/src/llm/tools/web_search/WebSearchTool/tool/input_schema.md)
- [capabilities](../../../../functions/src/llm/tools/web_search/WebSearchTool/tool/capabilities.md)
- [requires_approval](../../../../functions/src/llm/tools/web_search/WebSearchTool/tool/requires_approval.md)
- [validate_input](../../../../functions/src/llm/tools/web_search/WebSearchTool/tool/validate_input.md)
- [execute](../../../../functions/src/llm/tools/web_search/WebSearchTool/tool/execute.md)

# Imports

- `super::error::{Result, ToolError}`
- `super::r#trait::{Tool, ToolCapability, ToolExecutionContext, ToolResult}`
- `async_trait::async_trait`
- `serde::{Deserialize, Serialize}`
- `serde_json::Value`

# Member of

- [crustly](../../../../packages/crustly.md)