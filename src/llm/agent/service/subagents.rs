//! Sub-agents: separate agent sessions the `agent` tool hands focused tasks
//! to.
//!
//! A sub-agent runs its own loop in its own session, with its own context
//! window, so the parent only pays for the answer. [`SubAgentManager`] runs
//! them - waiting for the answer, or in the background - and keeps their
//! reports for the rest of the session. At most `[subagents] max_concurrent`
//! run at a time (default 1): one local GPU serializes inference anyway, so
//! sub-agents are for isolating context, not for speed.
//!
//! A sub-agent inherits the parent's provider, tools, approval prompt and
//! limits, narrowed by its profile (see `profiles`). It cannot start
//! sub-agents of its own. Each one leaves `.crustly/agents/<id>.md` (task and
//! answer) and `<id>.json` (its report) in the working directory.

use super::{AgentService, ApprovalCallback, Fallback, ToolApprovalInfo};
use crate::config::ToolProfile;
use crate::llm::agent::error::AgentError;
use crate::llm::agent::profiles::{AgentProfile, DEFAULT_PROFILE};
use crate::llm::provider::Provider;
use crate::llm::tools::{
    SubAgentLauncher, SubAgentReport, SubAgentStatus, SubAgentTask, ToolRegistry,
};
use crate::services::{ServiceContext, SessionService};
use std::collections::{BTreeMap, HashMap};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use tokio::sync::Semaphore;
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;
use uuid::Uuid;

/// Longest answer handed back to the parent, in characters. The full answer
/// is in the sub-agent's report file.
const MAX_ANSWER_CHARS: usize = 6_000;

/// Where reports are written, relative to the working directory.
const AGENTS_DIR: &str = ".crustly/agents";

/// What a sub-agent inherits from its parent service.
#[derive(Clone)]
pub(crate) struct ChildTemplate {
    pub provider: Arc<dyn Provider>,
    pub fallback: Option<Fallback>,
    pub context: ServiceContext,
    pub tool_registry: Arc<ToolRegistry>,
    pub working_directory: PathBuf,
    pub system_prompt: Option<String>,
    pub core_system_prompt: Option<String>,
    pub tool_profile: ToolProfile,
    pub extra_tools: Vec<String>,
    pub approval_callback: Option<ApprovalCallback>,
    pub auto_approve_tools: bool,
    pub verification: bool,
    pub verification_command: Option<String>,
    pub project_notes: bool,
    pub profiles: Arc<BTreeMap<String, AgentProfile>>,
}

impl ChildTemplate {
    /// The agent service a sub-agent with `profile` runs in.
    fn build(&self, task: &SubAgentTask, profile: &AgentProfile) -> AgentService {
        let read_only = task.read_only || profile.read_only;
        let with_role = |base: &str| format!("{base}\n\n## Your role\n\n{}", profile.instructions);

        let mut service = AgentService::new(self.provider.clone(), self.context.clone())
            .with_tool_registry(self.tool_registry.clone())
            .with_working_directory(self.working_directory.clone())
            // No sub-agents of its own.
            .with_allow_sub_agents(false)
            .with_max_tool_iterations(profile.max_rounds)
            .with_tool_profile(self.tool_profile, self.extra_tools.clone())
            // Same approval rules as the parent: the user is asked (with the
            // sub-agent named) whenever the parent would ask. Without a
            // callback, approval-requiring tools are refused, never run.
            .with_auto_approve_tools(self.auto_approve_tools)
            .with_approval_callback(
                self.approval_callback
                    .as_ref()
                    .map(|callback| labelled(callback, &task.description)),
            )
            .with_verification(
                self.verification && !read_only,
                self.verification_command.clone(),
            )
            .with_project_notes(self.project_notes);
        if let Some(fallback) = &self.fallback {
            service = service.with_fallback(fallback.clone());
        }
        if let Some(tools) = &profile.tools {
            service = service.with_tool_allowlist(tools.clone());
        }
        if let Some(prompt) = &self.system_prompt {
            service = service.with_system_prompt(with_role(prompt));
        }
        if let Some(prompt) = &self.core_system_prompt {
            service = service.with_core_system_prompt(with_role(prompt));
        }
        service
    }
}

/// The parent's approval callback, with requests marked as coming from the
/// sub-agent working on `description`.
fn labelled(callback: &ApprovalCallback, description: &str) -> ApprovalCallback {
    let callback = callback.clone();
    let label = format!("[sub-agent: {description}] ");
    Arc::new(move |mut info: ToolApprovalInfo| {
        info.tool_description = format!("{label}{}", info.tool_description);
        callback(info)
    })
}

/// One sub-agent the manager knows about.
struct Slot {
    report: SubAgentReport,
    cancel: CancellationToken,
    /// For a background sub-agent: the task to wait on.
    task: Option<JoinHandle<()>>,
}

/// Runs the sub-agents of one agent service and keeps their reports.
pub(crate) struct SubAgentManager {
    slots: Arc<Semaphore>,
    agents: Mutex<HashMap<Uuid, Slot>>,
    /// Ids in start order, for `list`.
    order: Mutex<Vec<Uuid>>,
}

impl SubAgentManager {
    pub fn new(max_concurrent: usize) -> Self {
        Self {
            slots: Arc::new(Semaphore::new(max_concurrent.max(1))),
            agents: Mutex::new(HashMap::new()),
            order: Mutex::new(Vec::new()),
        }
    }

    fn agents(&self) -> std::sync::MutexGuard<'_, HashMap<Uuid, Slot>> {
        self.agents.lock().unwrap_or_else(|e| e.into_inner())
    }

    fn register(&self, task: &SubAgentTask, profile: &str, cancel: CancellationToken) {
        let report = SubAgentReport {
            id: task.id,
            description: task.description.clone(),
            profile: profile.to_string(),
            status: SubAgentStatus::Queued,
            answer: None,
            error: None,
            tools_used: None,
        };
        self.agents().insert(
            task.id,
            Slot {
                report,
                cancel,
                task: None,
            },
        );
        self.order
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .push(task.id);
    }

    fn update(&self, report: &SubAgentReport) {
        if let Some(slot) = self.agents().get_mut(&report.id) {
            slot.report = report.clone();
        }
    }

    fn current(&self, id: Uuid) -> Option<SubAgentReport> {
        self.agents().get(&id).map(|slot| slot.report.clone())
    }

    /// Run `task` and wait for its report.
    pub async fn run(
        self: &Arc<Self>,
        template: &ChildTemplate,
        task: SubAgentTask,
        cancel: CancellationToken,
    ) -> SubAgentReport {
        let profile = profile_name(&task);
        self.register(&task, &profile, cancel.clone());
        execute(self.clone(), template.clone(), task, profile, cancel).await
    }

    /// Start `task` in the background.
    pub fn start(
        self: &Arc<Self>,
        template: &ChildTemplate,
        task: SubAgentTask,
        cancel: CancellationToken,
    ) {
        let id = task.id;
        let profile = profile_name(&task);
        self.register(&task, &profile, cancel.clone());
        let handle = tokio::spawn({
            let manager = self.clone();
            let template = template.clone();
            async move {
                execute(manager, template, task, profile, cancel).await;
            }
        });
        if let Some(slot) = self.agents().get_mut(&id) {
            slot.task = Some(handle);
        }
    }

    /// The report of a sub-agent; with `wait`, once it has finished.
    pub async fn report(&self, id: Uuid, wait: bool) -> Option<SubAgentReport> {
        let handle = {
            let mut agents = self.agents();
            let slot = agents.get_mut(&id)?;
            if wait {
                slot.task.take()
            } else {
                None
            }
        };
        if let Some(handle) = handle {
            if let Err(e) = handle.await {
                tracing::warn!("Sub-agent {} task ended abnormally: {}", id, e);
                let mut report = self.current(id)?;
                if !report.status.is_finished() {
                    report.status = SubAgentStatus::Failed;
                    report.error = Some(format!("the sub-agent task ended abnormally: {e}"));
                    self.update(&report);
                }
            }
        }
        self.current(id)
    }

    /// Stop a sub-agent. Returns whether it was still running.
    pub fn cancel(&self, id: Uuid) -> bool {
        match self.agents().get(&id) {
            Some(slot) if !slot.report.status.is_finished() => {
                slot.cancel.cancel();
                true
            }
            _ => false,
        }
    }

    /// Every sub-agent, in start order.
    pub fn list(&self) -> Vec<SubAgentReport> {
        let order = self.order.lock().unwrap_or_else(|e| e.into_inner()).clone();
        order
            .into_iter()
            .filter_map(|id| self.current(id))
            .collect()
    }
}

/// The profile name `task` runs with: the one it asked for, or `general`.
/// Unknown names are kept, so the report says what was asked for.
fn profile_name(task: &SubAgentTask) -> String {
    task.profile
        .as_deref()
        .map(str::trim)
        .filter(|p| !p.is_empty())
        .unwrap_or(DEFAULT_PROFILE)
        .to_string()
}

/// Run one sub-agent from queue to report, keeping the manager's copy of
/// its report current at each step.
async fn execute(
    manager: Arc<SubAgentManager>,
    template: ChildTemplate,
    task: SubAgentTask,
    profile_name: String,
    cancel: CancellationToken,
) -> SubAgentReport {
    let mut report = manager.current(task.id).unwrap_or(SubAgentReport {
        id: task.id,
        description: task.description.clone(),
        profile: profile_name.clone(),
        status: SubAgentStatus::Queued,
        answer: None,
        error: None,
        tools_used: None,
    });

    let Some(profile) = template.profiles.get(&profile_name).cloned() else {
        report.status = SubAgentStatus::Failed;
        report.error = Some(format!(
            "unknown profile '{}'; available: {}",
            profile_name,
            template
                .profiles
                .keys()
                .cloned()
                .collect::<Vec<_>>()
                .join(", ")
        ));
        manager.update(&report);
        return report;
    };

    // Wait for a free slot.
    let _permit = tokio::select! {
        permit = manager.slots.clone().acquire_owned() => permit.ok(),
        _ = cancel.cancelled() => {
            report.status = SubAgentStatus::Cancelled;
            manager.update(&report);
            write_report(&template, &task, &report);
            return report;
        }
    };
    report.status = SubAgentStatus::Running;
    manager.update(&report);
    write_report(&template, &task, &report);
    tracing::info!(
        "Sub-agent {} ({}) started: {}",
        task.id,
        profile_name,
        task.description
    );

    let service = template.build(&task, &profile);
    let session = SessionService::new(template.context.clone())
        .create_session(Some(format!("sub-agent: {}", task.description)))
        .await;
    let outcome = match session {
        Ok(session) => {
            service
                .send_message_with_tools_cancellable(
                    session.id,
                    task.prompt.clone(),
                    profile.model.clone(),
                    task.read_only || profile.read_only,
                    cancel.clone(),
                )
                .await
        }
        Err(e) => Err(AgentError::Database(e.to_string())),
    };

    match outcome {
        Ok(response) => {
            report.status = SubAgentStatus::Completed;
            report.answer = Some(shorten(&response.content));
            report.tools_used = response.tool_summary;
            write_answer(&template, &task, &response.content);
        }
        Err(AgentError::Cancelled) => report.status = SubAgentStatus::Cancelled,
        Err(e) => {
            report.status = SubAgentStatus::Failed;
            report.error = Some(e.to_string());
        }
    }
    tracing::info!("Sub-agent {} finished: {:?}", task.id, report.status);
    manager.update(&report);
    write_report(&template, &task, &report);
    report
}

/// `answer`, cut to [`MAX_ANSWER_CHARS`] with a pointer to the full text.
fn shorten(answer: &str) -> String {
    if answer.chars().count() <= MAX_ANSWER_CHARS {
        return answer.to_string();
    }
    let kept: String = answer.chars().take(MAX_ANSWER_CHARS).collect();
    format!("{kept}\n\n[Answer cut here; the full answer is in {AGENTS_DIR}.]")
}

fn agents_dir(template: &ChildTemplate) -> PathBuf {
    template.working_directory.join(AGENTS_DIR)
}

/// Write the sub-agent's report next to its task. Failures are logged:
/// the files are a record, not part of the answer.
fn write_report(template: &ChildTemplate, task: &SubAgentTask, report: &SubAgentReport) {
    let dir = agents_dir(template);
    let result = std::fs::create_dir_all(&dir).and_then(|()| {
        let json = serde_json::to_string_pretty(report).map_err(std::io::Error::other)?;
        std::fs::write(dir.join(format!("{}.json", task.id)), json)?;
        let task_file = dir.join(format!("{}.md", task.id));
        if !task_file.exists() {
            std::fs::write(
                task_file,
                format!(
                    "# Sub-agent task\n\n- id: {}\n- description: {}\n- profile: {}\n\n## Prompt\n\n{}\n",
                    task.id, task.description, report.profile, task.prompt
                ),
            )?;
        }
        Ok(())
    });
    if let Err(e) = result {
        tracing::warn!("Could not write the report of sub-agent {}: {}", task.id, e);
    }
}

/// Append the full answer to the sub-agent's task file.
fn write_answer(template: &ChildTemplate, task: &SubAgentTask, answer: &str) {
    use std::io::Write;
    let path = agents_dir(template).join(format!("{}.md", task.id));
    let result = std::fs::OpenOptions::new()
        .append(true)
        .open(&path)
        .and_then(|mut file| write!(file, "\n## Answer\n\n{answer}\n"));
    if let Err(e) = result {
        tracing::warn!("Could not write the answer of sub-agent {}: {}", task.id, e);
    }
}

/// The `SubAgentLauncher` handed to the `agent` tool: the parent's manager
/// plus what its sub-agents inherit.
pub(crate) struct AgentServiceLauncher {
    manager: Arc<SubAgentManager>,
    template: ChildTemplate,
}

impl AgentServiceLauncher {
    pub fn new(manager: Arc<SubAgentManager>, template: ChildTemplate) -> Self {
        Self { manager, template }
    }
}

impl std::fmt::Debug for AgentServiceLauncher {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AgentServiceLauncher")
            .field("working_directory", &self.template.working_directory)
            .finish_non_exhaustive()
    }
}

#[async_trait::async_trait]
impl SubAgentLauncher for AgentServiceLauncher {
    async fn run(&self, task: SubAgentTask, cancel: CancellationToken) -> SubAgentReport {
        self.manager.run(&self.template, task, cancel).await
    }

    async fn start(
        &self,
        task: SubAgentTask,
        cancel: CancellationToken,
    ) -> std::result::Result<(), String> {
        self.manager.start(&self.template, task, cancel);
        Ok(())
    }

    async fn report(&self, id: Uuid, wait: bool) -> Option<SubAgentReport> {
        self.manager.report(id, wait).await
    }

    fn cancel(&self, id: Uuid) -> bool {
        self.manager.cancel(id)
    }

    fn list(&self) -> Vec<SubAgentReport> {
        self.manager.list()
    }

    fn profiles(&self) -> Vec<String> {
        self.template.profiles.keys().cloned().collect()
    }
}
