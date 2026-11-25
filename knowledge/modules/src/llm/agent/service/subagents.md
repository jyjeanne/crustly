---
type: Rust Module
title: subagents
resource: src/llm/agent/service/subagents.rs#L1-L464
generated:
  by: okf-rs/0.8.0
relationships:
  imports:
  - target: external/super-agentservice-approvalcallback-fallback-toolapprovalinfo
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-config-toolprofile
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-llm-agent-error-agenterror
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-llm-agent-profiles-agentprofile-default-profile
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-llm-provider-provider
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-llm-tools-subagentlauncher-subagentreport-subagentstatus-subagenttask-toolregistry
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-services-servicecontext-sessionservice
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-collections-btreemap-hashmap
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-path-pathbuf
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-sync-arc-mutex
    resolved_by: tree-sitter
    confidence: exact
  - target: external/tokio-sync-semaphore
    resolved_by: tree-sitter
    confidence: exact
  - target: external/tokio-task-joinhandle
    resolved_by: tree-sitter
    confidence: exact
  - target: external/tokio-util-sync-cancellationtoken
    resolved_by: tree-sitter
    confidence: exact
  - target: external/uuid-uuid
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-io-write
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/crustly
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [ChildTemplate](../../../../../classes/src/llm/agent/service/subagents/ChildTemplate.md)
- [build](../../../../../functions/src/llm/agent/service/subagents/ChildTemplate/build.md)
- [labelled](../../../../../functions/src/llm/agent/service/subagents/labelled.md)
- [Slot](../../../../../classes/src/llm/agent/service/subagents/Slot.md)
- [SubAgentManager](../../../../../classes/src/llm/agent/service/subagents/SubAgentManager.md)
- [new](../../../../../functions/src/llm/agent/service/subagents/SubAgentManager/new.md)
- [agents](../../../../../functions/src/llm/agent/service/subagents/SubAgentManager/agents.md)
- [register](../../../../../functions/src/llm/agent/service/subagents/SubAgentManager/register.md)
- [update](../../../../../functions/src/llm/agent/service/subagents/SubAgentManager/update.md)
- [current](../../../../../functions/src/llm/agent/service/subagents/SubAgentManager/current.md)
- [run](../../../../../functions/src/llm/agent/service/subagents/SubAgentManager/run.md)
- [start](../../../../../functions/src/llm/agent/service/subagents/SubAgentManager/start.md)
- [report](../../../../../functions/src/llm/agent/service/subagents/SubAgentManager/report.md)
- [cancel](../../../../../functions/src/llm/agent/service/subagents/SubAgentManager/cancel.md)
- [list](../../../../../functions/src/llm/agent/service/subagents/SubAgentManager/list.md)
- [profile_name](../../../../../functions/src/llm/agent/service/subagents/profile_name.md)
- [execute](../../../../../functions/src/llm/agent/service/subagents/execute.md)
- [shorten](../../../../../functions/src/llm/agent/service/subagents/shorten.md)
- [agents_dir](../../../../../functions/src/llm/agent/service/subagents/agents_dir.md)
- [write_report](../../../../../functions/src/llm/agent/service/subagents/write_report.md)
- [write_answer](../../../../../functions/src/llm/agent/service/subagents/write_answer.md)
- [AgentServiceLauncher](../../../../../classes/src/llm/agent/service/subagents/AgentServiceLauncher.md)
- [new](../../../../../functions/src/llm/agent/service/subagents/AgentServiceLauncher/new.md)
- [fmt](../../../../../functions/src/llm/agent/service/subagents/AgentServiceLauncher/std-fmt-debug/fmt.md)
- [run](../../../../../functions/src/llm/agent/service/subagents/AgentServiceLauncher/subagentlauncher/run.md)
- [start](../../../../../functions/src/llm/agent/service/subagents/AgentServiceLauncher/subagentlauncher/start.md)
- [report](../../../../../functions/src/llm/agent/service/subagents/AgentServiceLauncher/subagentlauncher/report.md)
- [cancel](../../../../../functions/src/llm/agent/service/subagents/AgentServiceLauncher/subagentlauncher/cancel.md)
- [list](../../../../../functions/src/llm/agent/service/subagents/AgentServiceLauncher/subagentlauncher/list.md)
- [profiles](../../../../../functions/src/llm/agent/service/subagents/AgentServiceLauncher/subagentlauncher/profiles.md)

# Imports

- `super::{AgentService, ApprovalCallback, Fallback, ToolApprovalInfo}`
- `crate::config::ToolProfile`
- `crate::llm::agent::error::AgentError`
- `crate::llm::agent::profiles::{AgentProfile, DEFAULT_PROFILE}`
- `crate::llm::provider::Provider`
- `crate::llm::tools::{
    SubAgentLauncher, SubAgentReport, SubAgentStatus, SubAgentTask, ToolRegistry,
}`
- `crate::services::{ServiceContext, SessionService}`
- `std::collections::{BTreeMap, HashMap}`
- `std::path::PathBuf`
- `std::sync::{Arc, Mutex}`
- `tokio::sync::Semaphore`
- `tokio::task::JoinHandle`
- `tokio_util::sync::CancellationToken`
- `uuid::Uuid`
- `std::io::Write`

# Member of

- [crustly](../../../../../packages/crustly.md)