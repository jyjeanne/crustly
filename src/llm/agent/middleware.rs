//! Round middleware: the rules the agent loop applies around every round.
//!
//! A round is one model call plus the tools it asked for. The loop consults
//! each middleware at fixed points:
//!
//! - [`RoundMiddleware::before_round`]: before the model is called (limits)
//! - [`RoundMiddleware::on_tool_calls`]: after the model asked for tools,
//!   before they run (loop guards)
//! - [`RoundMiddleware::after_tools`]: after the tools ran (bookkeeping)
//! - [`RoundMiddleware::on_final_answer`]: before a reply without tool calls
//!   is accepted as the answer (verification)
//!
//! Each answers with a [`Decision`]. Every middleware is consulted, so each
//! keeps its own bookkeeping, and the first answer other than
//! [`Decision::Continue`] wins. New rules are added as middleware instead of
//! more branches inside the loop.

use crate::llm::tools::ToolCapability;
use serde_json::Value;
use std::path::Path;
use std::time::{Duration, Instant};

/// What the loop should do next.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Decision {
    /// Carry on.
    Continue,
    /// Stop calling tools and ask the model for its final answer, telling it
    /// `reason`. `notice`, when set, is shown to the user under the answer.
    Finish {
        reason: String,
        notice: Option<String>,
    },
    /// Do not accept this reply as the final answer yet: send `message` to
    /// the model and run another round.
    Nudge { message: String },
}

/// One tool call the model asked for in the current round.
#[derive(Debug, Clone)]
pub struct ToolCallView {
    /// Registered (canonical) tool name.
    pub name: String,
    pub input: Value,
    pub capabilities: Vec<ToolCapability>,
}

/// One tool call that was handled in a round, in execution order.
#[derive(Debug, Clone, PartialEq)]
pub struct ToolCallRecord {
    /// Registered (canonical) tool name.
    pub name: String,
    pub input: Value,
    /// Whether the tool ran and reported success. Calls refused before
    /// running (hidden tool, denied, invalid arguments) count as failures.
    pub success: bool,
}

/// What a middleware sees of the run when it is consulted.
#[derive(Debug, Clone)]
pub struct RunState<'a> {
    /// The round being run, from 1.
    pub round: usize,
    /// When the request started.
    pub started: Instant,
    /// Input plus output tokens the provider reported so far.
    pub tokens_used: u32,
    /// Whether the model's reply in this round carried text for the user.
    pub produced_text: bool,
    /// The tool calls of this round (empty before the model is called and
    /// for a final answer).
    pub tool_calls: &'a [ToolCallView],
}

/// A rule applied around every round. Every hook defaults to doing nothing.
pub trait RoundMiddleware: Send {
    fn before_round(&mut self, _state: &RunState) -> Decision {
        Decision::Continue
    }

    fn on_tool_calls(&mut self, _state: &RunState) -> Decision {
        Decision::Continue
    }

    fn after_tools(&mut self, _records: &[ToolCallRecord]) {}

    fn on_final_answer(&mut self, _state: &RunState) -> Decision {
        Decision::Continue
    }
}

/// The middlewares of one run, consulted in order.
#[derive(Default)]
pub struct Pipeline {
    middlewares: Vec<Box<dyn RoundMiddleware>>,
}

impl Pipeline {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with(mut self, middleware: impl RoundMiddleware + 'static) -> Self {
        self.middlewares.push(Box::new(middleware));
        self
    }

    fn first_decision(
        &mut self,
        mut ask: impl FnMut(&mut dyn RoundMiddleware) -> Decision,
    ) -> Decision {
        let mut decision = Decision::Continue;
        for middleware in &mut self.middlewares {
            let answer = ask(middleware.as_mut());
            if decision == Decision::Continue {
                decision = answer;
            }
        }
        decision
    }

    pub fn before_round(&mut self, state: &RunState) -> Decision {
        self.first_decision(|m| m.before_round(state))
    }

    pub fn on_tool_calls(&mut self, state: &RunState) -> Decision {
        self.first_decision(|m| m.on_tool_calls(state))
    }

    pub fn after_tools(&mut self, records: &[ToolCallRecord]) {
        for middleware in &mut self.middlewares {
            middleware.after_tools(records);
        }
    }

    pub fn on_final_answer(&mut self, state: &RunState) -> Decision {
        self.first_decision(|m| m.on_final_answer(state))
    }
}

/// True when the capabilities can change files or system state.
pub(crate) fn has_mutating_capability(caps: &[ToolCapability]) -> bool {
    caps.iter().any(|c| {
        matches!(
            c,
            ToolCapability::WriteFiles
                | ToolCapability::ExecuteShell
                | ToolCapability::SystemModification
        )
    })
}

/// Build a loop-detection signature for one tool call: `<name>:<distinguishing
/// args>`. Two calls with the same signature seen back-to-back are treated as a
/// stuck loop. The signature MUST include whatever argument distinguishes one
/// legitimate call from another (path, command, pattern...), or genuinely
/// different calls collide and the loop detector aborts the second before it
/// runs.
///
/// For path-based tools the canonical input key is `path`, with `file_path` as
/// an alias (see the read/edit/write tool schemas). Reading only `file_path`
/// here yielded a bare `read_file:`/`edit_file:`/`write_file:` for every normal
/// call, so two operations on *different* paths shared a signature and the
/// second was falsely aborted as a loop.
pub(crate) fn tool_call_signature(name: &str, input: &Value) -> String {
    // First string-valued input under any of `keys`, path-normalized.
    let first_str = |keys: &[&str]| -> Option<String> {
        keys.iter()
            .find_map(|k| input.get(*k).and_then(|v| v.as_str()))
            .map(|s| s.replace('\\', "/"))
    };

    match name {
        "plan" => {
            if let Some(operation) = input.get("operation").and_then(|v| v.as_str()) {
                if operation == "add_task" {
                    if let Some(title) = input.get("title").and_then(|v| v.as_str()) {
                        format!("{name}:{operation}:{title}")
                    } else {
                        format!("{name}:{operation}")
                    }
                } else {
                    format!("{name}:{operation}")
                }
            } else {
                name.to_string()
            }
        }

        // File system exploration tools - include path to distinguish calls.
        "ls" => format!("ls:{}", first_str(&["path"]).unwrap_or_default()),
        "glob" => format!(
            "glob:{}",
            input
                .get("pattern")
                .and_then(|v| v.as_str())
                .unwrap_or_default()
        ),
        "grep" => {
            let pattern = input.get("pattern").and_then(|v| v.as_str()).unwrap_or("");
            let path = input.get("path").and_then(|v| v.as_str()).unwrap_or("");
            format!("grep:{pattern}:{path}")
        }

        // Path-based tools: canonical key `path`, alias `file_path`.
        "read_file" => format!(
            "read_file:{}",
            first_str(&["path", "file_path"]).unwrap_or_default()
        ),
        "write_file" | "edit_file" => {
            format!(
                "{name}:{}",
                first_str(&["path", "file_path"]).unwrap_or_default()
            )
        }

        // Command execution - include (truncated) command.
        "bash" => {
            let cmd = first_str(&["command"]).unwrap_or_default();
            let cmd_short: String = cmd.chars().take(100).collect();
            format!("bash:{cmd_short}")
        }

        // Other tools: name plus an input hash, so different calls to the same
        // tool never share a signature.
        _ => {
            use std::collections::hash_map::DefaultHasher;
            use std::hash::{Hash, Hasher};
            let mut h = DefaultHasher::new();
            input.to_string().hash(&mut h);
            format!("{name}:{:016x}", h.finish())
        }
    }
}

/// Consecutive tool-calling rounds with nothing said to the user before the
/// loop forces an answer.
const MAX_SILENT_TOOL_CALLS: usize = 6;

/// Recent round signatures kept for repeat detection.
const LOOP_HISTORY: usize = 15;

/// Stops a model that is stuck: one repeating the same call, or one that
/// keeps calling tools without ever answering.
#[derive(Debug, Default)]
pub struct LoopGuard {
    recent: Vec<String>,
    /// Consecutive tool-calling rounds in which the model said nothing to
    /// the user. Reset by any round that produces text.
    silent: usize,
}

impl LoopGuard {
    pub fn new() -> Self {
        Self::default()
    }
}

impl RoundMiddleware for LoopGuard {
    fn on_tool_calls(&mut self, state: &RunState) -> Decision {
        // Track the round's calls, arguments included, so `ls(./src)` and
        // `ls(./src/cli)` are different calls.
        let signature = state
            .tool_calls
            .iter()
            .map(|c| tool_call_signature(&c.name, &c.input))
            .collect::<Vec<_>>()
            .join(",");
        self.recent.push(signature.clone());
        if self.recent.len() > LOOP_HISTORY {
            self.recent.remove(0);
        }

        // Drift guard. The identical-call check below only fires when the
        // model repeats the *same* call, so a model that keeps picking
        // *different* tools sails past it and burns every round. That is the
        // common failure: the very first `bash` returns the answer, the model
        // ignores it and wanders off through ls, glob, write_file,
        // task_manager...
        //
        // Building a plan is an intentionally silent chain: create + one
        // add_task per step + finalize is 7+ consecutive tool calls with
        // nothing to say in between. Plan-tool rounds are therefore neutral:
        // they neither count toward drift nor reset it. A model genuinely
        // stuck repeating one plan call is still caught by the repeat check.
        let all_plan_calls =
            !state.tool_calls.is_empty() && state.tool_calls.iter().all(|c| c.name == "plan");
        if state.produced_text {
            self.silent = 0;
        } else if !all_plan_calls {
            self.silent += 1;
        }
        let drifting = self.silent >= MAX_SILENT_TOOL_CALLS;
        if drifting {
            tracing::warn!(
                "⚠️ {} consecutive tool calls with no answer to the user (last: '{}'). \
                 Forcing a text response.",
                self.silent,
                signature
            );
        }

        // Repeat threshold from the first tool's declared capabilities, so
        // the classification cannot drift from the registry's tool names.
        let first_caps = state
            .tool_calls
            .first()
            .map(|c| c.capabilities.as_slice())
            .unwrap_or_default();
        let is_modification_tool = has_mutating_capability(first_caps);
        let is_exploration_tool = !is_modification_tool
            && first_caps
                .iter()
                .any(|c| matches!(c, ToolCapability::ReadFiles | ToolCapability::Network));
        // Exploration may legitimately revisit a path; a repeated write or
        // command is dangerous.
        let threshold = if is_exploration_tool {
            10
        } else if is_modification_tool {
            2
        } else {
            3
        };

        let repeating = self.recent.len() >= threshold
            && self.recent[self.recent.len() - threshold..]
                .iter()
                .all(|s| s == &signature);

        if repeating {
            tracing::warn!(
                "⚠️ Detected tool loop: '{}' called {} times in a row. Injecting recovery hint.",
                signature,
                threshold
            );
            if is_exploration_tool {
                tracing::info!(
                    "💡 Hint: The model is stuck trying to access the same path {} times. \
                     This often means the path doesn't exist or the model is confused about \
                     the directory structure.",
                    threshold
                );
            } else if is_modification_tool {
                tracing::warn!(
                    "⚠️ Modification tool loop detected! This could be dangerous. The model \
                     tried to modify the same file/run the same command {} times.",
                    threshold
                );
            }
            Decision::Finish {
                reason: format!(
                    "⚠️ Loop detected: you called '{}' {} times in a row without making progress. \
                     Please reassess your approach. Summarise what you have found so far and \
                     explain what the next step should be without repeating the same tool call.",
                    signature, threshold
                ),
                notice: None,
            }
        } else if drifting {
            Decision::Finish {
                reason: format!(
                    "⚠️ You have made {} tool calls without answering the user. The results you \
                     already have are enough. Do NOT call any more tools. Answer the user's \
                     original question now, using only what those tools returned.",
                    self.silent
                ),
                notice: None,
            }
        } else {
            Decision::Continue
        }
    }
}

/// Caps on one request. When one is reached the loop stops calling tools and
/// asks the model to answer from the work done so far, instead of failing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Limits {
    /// Rounds that may call tools.
    pub max_rounds: usize,
    /// Wall-clock time for the whole request.
    pub max_wall_time: Option<Duration>,
    /// Input plus output tokens for the whole request.
    pub max_tokens: Option<u32>,
}

impl Default for Limits {
    fn default() -> Self {
        Self {
            max_rounds: 10,
            max_wall_time: None,
            max_tokens: None,
        }
    }
}

fn finish_at_limit(what: &str) -> Decision {
    Decision::Finish {
        reason: format!(
            "You have reached the limit of {what} for this request. Do NOT call any more tools. \
             Answer the user now with what you have found and done so far, and say what is \
             left to do."
        ),
        notice: Some(format!(
            "_⏱ Stopped at the limit of {what}; this answer is based on the work done so far._"
        )),
    }
}

impl RoundMiddleware for Limits {
    fn before_round(&mut self, state: &RunState) -> Decision {
        if state.round > self.max_rounds {
            return finish_at_limit(&format!("{} tool rounds", self.max_rounds));
        }
        if let Some(max) = self.max_wall_time {
            if state.started.elapsed() >= max {
                return finish_at_limit(&format!("{} seconds", max.as_secs()));
            }
        }
        if let Some(max) = self.max_tokens {
            if state.tokens_used >= max {
                return finish_at_limit(&format!("{max} tokens"));
            }
        }
        Decision::Continue
    }
}

/// Tools whose success means project files changed.
const FILE_EDIT_TOOLS: &[&str] = &["write_file", "edit_file", "apply_patch", "notebook_edit"];

/// Whether a successful call to the tool registered as `name` changed
/// project files.
pub(crate) fn is_file_edit(name: &str) -> bool {
    FILE_EDIT_TOOLS.contains(&name)
}

/// Shell commands that count as checking a change, besides the configured one.
const KNOWN_CHECK_COMMANDS: &[&str] = &[
    "cargo check",
    "cargo build",
    "cargo test",
    "cargo clippy",
    "go build",
    "go test",
    "go vet",
    "tsc",
    "npm test",
    "npm run test",
    "npm run build",
    "pnpm test",
    "yarn test",
    "pytest",
    "make test",
    "make check",
    "mvn test",
    "gradle test",
    "dotnet build",
    "dotnet test",
];

/// At most this many times per request is the model sent back to verify.
const MAX_VERIFY_NUDGES: usize = 2;

/// Makes the model check its own edits before it answers.
///
/// After a file-editing tool succeeds, a reply without tool calls is not
/// accepted until a check command (the configured one, or a known build or
/// test command) has passed through the shell tool. The model runs the
/// check itself, so it goes through the normal approval gate, and it sees
/// the errors it has to fix.
#[derive(Debug)]
pub struct Verification {
    command: String,
    edited: Vec<String>,
    verified: bool,
    nudges: usize,
}

impl Verification {
    pub fn new(command: impl Into<String>) -> Self {
        Self {
            command: command.into(),
            edited: Vec::new(),
            verified: true,
            nudges: 0,
        }
    }

    /// The check command for the project in `dir`: `configured` when set,
    /// otherwise one inferred from the build files present. `None` when
    /// nothing fits.
    pub fn detect_command(dir: &Path, configured: Option<&str>) -> Option<String> {
        if let Some(command) = configured.map(str::trim).filter(|c| !c.is_empty()) {
            return Some(command.to_string());
        }
        if dir.join("Cargo.toml").is_file() {
            Some("cargo check".to_string())
        } else if dir.join("go.mod").is_file() {
            Some("go build ./...".to_string())
        } else if dir.join("tsconfig.json").is_file() {
            Some("npx tsc --noEmit".to_string())
        } else {
            None
        }
    }

    fn is_check(&self, command: &str) -> bool {
        command.contains(self.command.as_str())
            || KNOWN_CHECK_COMMANDS.iter().any(|c| command.contains(c))
    }
}

impl RoundMiddleware for Verification {
    fn after_tools(&mut self, records: &[ToolCallRecord]) {
        for record in records.iter().filter(|r| r.success) {
            if FILE_EDIT_TOOLS.contains(&record.name.as_str()) {
                let path = ["path", "file_path"]
                    .iter()
                    .find_map(|k| record.input.get(*k).and_then(Value::as_str))
                    .unwrap_or(record.name.as_str());
                if !self.edited.iter().any(|p| p == path) {
                    self.edited.push(path.to_string());
                }
                self.verified = false;
            } else if matches!(record.name.as_str(), "bash" | "powershell") {
                let command = record
                    .input
                    .get("command")
                    .and_then(Value::as_str)
                    .unwrap_or_default();
                if self.is_check(command) {
                    self.verified = true;
                }
            }
        }
    }

    fn on_final_answer(&mut self, _state: &RunState) -> Decision {
        if self.verified || self.nudges >= MAX_VERIFY_NUDGES {
            return Decision::Continue;
        }
        self.nudges += 1;
        let shown: Vec<&str> = self.edited.iter().take(5).map(String::as_str).collect();
        Decision::Nudge {
            message: format!(
                "Before you answer: you changed {} but have not checked the result. Run `{}` \
                 with the bash tool, fix any errors it reports, then give your final answer.",
                shown.join(", "),
                self.command
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn state<'a>(round: usize, calls: &'a [ToolCallView], produced_text: bool) -> RunState<'a> {
        RunState {
            round,
            started: Instant::now(),
            tokens_used: 0,
            produced_text,
            tool_calls: calls,
        }
    }

    fn call(name: &str, input: Value, caps: Vec<ToolCapability>) -> ToolCallView {
        ToolCallView {
            name: name.to_string(),
            input,
            capabilities: caps,
        }
    }

    fn record(name: &str, input: Value, success: bool) -> ToolCallRecord {
        ToolCallRecord {
            name: name.to_string(),
            input,
            success,
        }
    }

    #[test]
    fn loop_guard_stops_a_repeated_write_on_the_second_call() {
        let mut guard = LoopGuard::new();
        let calls = [call(
            "write_file",
            json!({"path": "a.rs"}),
            vec![ToolCapability::WriteFiles],
        )];
        assert_eq!(
            guard.on_tool_calls(&state(1, &calls, true)),
            Decision::Continue
        );
        assert!(matches!(
            guard.on_tool_calls(&state(2, &calls, true)),
            Decision::Finish { .. }
        ));
    }

    #[test]
    fn loop_guard_lets_exploration_revisit_a_path() {
        let mut guard = LoopGuard::new();
        let calls = [call(
            "read_file",
            json!({"path": "a.rs"}),
            vec![ToolCapability::ReadFiles],
        )];
        for round in 1..10 {
            assert_eq!(
                guard.on_tool_calls(&state(round, &calls, true)),
                Decision::Continue
            );
        }
        assert!(matches!(
            guard.on_tool_calls(&state(10, &calls, true)),
            Decision::Finish { .. }
        ));
    }

    #[test]
    fn loop_guard_stops_drift_but_not_silent_plan_building() {
        let mut guard = LoopGuard::new();
        for round in 1..=8 {
            let plan = [call(
                "plan",
                json!({"operation": "add_task", "title": format!("task {round}")}),
                vec![],
            )];
            assert_eq!(
                guard.on_tool_calls(&state(round, &plan, false)),
                Decision::Continue,
                "plan building is neutral"
            );
        }
        let mut guard = LoopGuard::new();
        let mut decision = Decision::Continue;
        for round in 1..=6 {
            let calls = [call(
                "ls",
                json!({ "path": format!("dir{round}") }),
                vec![ToolCapability::ReadFiles],
            )];
            decision = guard.on_tool_calls(&state(round, &calls, false));
        }
        assert!(matches!(decision, Decision::Finish { .. }));
    }

    #[test]
    fn limits_finish_with_a_notice_after_the_last_round() {
        let mut limits = Limits {
            max_rounds: 3,
            ..Limits::default()
        };
        assert_eq!(
            limits.before_round(&state(3, &[], false)),
            Decision::Continue
        );
        match limits.before_round(&state(4, &[], false)) {
            Decision::Finish { notice, .. } => assert!(notice.unwrap().contains("3 tool rounds")),
            other => panic!("expected Finish, got {other:?}"),
        }
    }

    #[test]
    fn limits_finish_on_tokens_and_wall_time() {
        let mut limits = Limits {
            max_rounds: 100,
            max_wall_time: Some(Duration::ZERO),
            max_tokens: None,
        };
        assert!(matches!(
            limits.before_round(&state(1, &[], false)),
            Decision::Finish { .. }
        ));

        let mut limits = Limits {
            max_rounds: 100,
            max_wall_time: None,
            max_tokens: Some(1000),
        };
        let mut over = state(1, &[], false);
        over.tokens_used = 1000;
        assert!(matches!(
            limits.before_round(&over),
            Decision::Finish { .. }
        ));
    }

    #[test]
    fn verification_nudges_after_an_edit_until_a_check_passes() {
        let mut v = Verification::new("cargo check");
        // Nothing edited: the answer is accepted.
        assert_eq!(v.on_final_answer(&state(1, &[], true)), Decision::Continue);

        v.after_tools(&[record("edit_file", json!({"path": "src/lib.rs"}), true)]);
        match v.on_final_answer(&state(2, &[], true)) {
            Decision::Nudge { message } => {
                assert!(message.contains("src/lib.rs"));
                assert!(message.contains("cargo check"));
            }
            other => panic!("expected Nudge, got {other:?}"),
        }

        // A failing check does not count.
        v.after_tools(&[record("bash", json!({"command": "cargo check"}), false)]);
        assert!(matches!(
            v.on_final_answer(&state(3, &[], true)),
            Decision::Nudge { .. }
        ));

        // A passing one does.
        v.after_tools(&[record("bash", json!({"command": "cargo test --lib"}), true)]);
        assert_eq!(v.on_final_answer(&state(4, &[], true)), Decision::Continue);
    }

    #[test]
    fn verification_gives_up_after_two_nudges() {
        let mut v = Verification::new("cargo check");
        v.after_tools(&[record("write_file", json!({"path": "a.rs"}), true)]);
        assert!(matches!(
            v.on_final_answer(&state(1, &[], true)),
            Decision::Nudge { .. }
        ));
        assert!(matches!(
            v.on_final_answer(&state(2, &[], true)),
            Decision::Nudge { .. }
        ));
        assert_eq!(v.on_final_answer(&state(3, &[], true)), Decision::Continue);
    }

    #[test]
    fn verification_detects_the_check_command_from_build_files() {
        let dir = tempfile::tempdir().unwrap();
        assert_eq!(Verification::detect_command(dir.path(), None), None);
        assert_eq!(
            Verification::detect_command(dir.path(), Some("make lint")),
            Some("make lint".to_string())
        );
        std::fs::write(dir.path().join("Cargo.toml"), "[package]").unwrap();
        assert_eq!(
            Verification::detect_command(dir.path(), None),
            Some("cargo check".to_string())
        );
        assert_eq!(
            Verification::detect_command(dir.path(), Some("  ")),
            Some("cargo check".to_string())
        );
    }

    #[test]
    fn pipeline_consults_everyone_and_the_first_decision_wins() {
        struct Fixed(Decision, std::sync::Arc<std::sync::atomic::AtomicUsize>);
        impl RoundMiddleware for Fixed {
            fn before_round(&mut self, _state: &RunState) -> Decision {
                self.1.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                self.0.clone()
            }
        }
        let calls = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let nudge = Decision::Nudge {
            message: "first".to_string(),
        };
        let mut pipeline = Pipeline::new()
            .with(Fixed(Decision::Continue, calls.clone()))
            .with(Fixed(nudge.clone(), calls.clone()))
            .with(Fixed(
                Decision::Nudge {
                    message: "second".to_string(),
                },
                calls.clone(),
            ));
        assert_eq!(pipeline.before_round(&state(1, &[], false)), nudge);
        assert_eq!(calls.load(std::sync::atomic::Ordering::SeqCst), 3);
    }
}
