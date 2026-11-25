---
type: Rust Module
title: tools
resource: src/llm/tools/mod.rs#L1-L56
generated:
  by: okf-rs/0.7.0
relationships:
  imports:
  - target: external/pub-use-error-result-toolerror
    resolved_by: tree-sitter
    confidence: exact
  - target: external/pub-use-file-read-cache-filefingerprint-filereadcache-readgate
    resolved_by: tree-sitter
    confidence: exact
  - target: external/pub-use-r-trait-subagentlauncher-subagentreport-subagentstatus-subagenttask-tool-toolcapability-toolexecutioncontext-toolresult
    resolved_by: tree-sitter
    confidence: exact
  - target: external/pub-use-registry-toolregistry
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/crustly
    resolved_by: tree-sitter
    confidence: exact
---

# Imports

- `pub use error::{Result, ToolError}`
- `pub use file_read_cache::{FileFingerprint, FileReadCache, ReadGate}`
- `pub use r#trait::{
    SubAgentLauncher, SubAgentReport, SubAgentStatus, SubAgentTask, Tool, ToolCapability,
    ToolExecutionContext, ToolResult,
}`
- `pub use registry::ToolRegistry`

# Member of

- [crustly](../../../packages/crustly.md)