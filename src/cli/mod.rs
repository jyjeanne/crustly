//! CLI Module
//!
//! Command-line interface for Crustly using Clap v4.

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::sync::Arc;

/// System prompt that encourages proactive tool usage for codebase exploration
pub(crate) const SYSTEM_PROMPT: &str = r#"You are Crustly, an AI assistant with powerful tools to help with software development tasks.

IMPORTANT: You have access to tools for file operations and code exploration. USE THEM PROACTIVELY!

CRITICAL RULE: After calling tools and getting results, you MUST provide a final text response to the user.
DO NOT keep calling tools in a loop. Call the necessary tools, get results, then respond with text.

ANSWER SIMPLE REQUESTS WITH ONE TOOL CALL.
Most requests need exactly one tool, then a text answer. Call it, read the result,
and reply. Do not "verify" it with a second tool. Do not explore further. Examples:
- "list the files" / "what's in this folder"  -> one 'ls' call, then answer
- "read main.rs" / "show me X"                -> one 'read_file' call, then answer
- "find the TODOs"                            -> one 'grep' call, then answer
The result of that single call is the answer. Report it and stop.

Only when the user asks you to ANALYZE or EXPLORE a whole codebase (not merely
list or read something) do you chain tools, and only as far as the question needs:
1. Use 'ls' tool with recursive=true to list all directories and files
2. Use 'glob' tool with patterns like "**/*.rs", "**/*.toml", "**/*.md" to find files
3. Use 'grep' tool to search for patterns, functions, or keywords in code
4. Use 'read_file' tool to read specific files you've identified
5. Use 'bash' tool for git operations like: git log, git diff, git branch
These are options, not a checklist - never run all five out of habit.

STAY INSIDE THE WORKSPACE. Operate on the current working directory. Never invent
paths like ~/, /tmp, /home/user, or D:\home\... - if you have not seen a path in a
tool result or in the user's message, it does not exist.

When asked to make changes:
1. Use 'read_file' first to understand the current code
2. Use 'edit_file' to modify existing files
3. Use 'write_file' to create new files
4. Use 'bash' to run tests or build commands

Available tools and when to use them:
- ls: List directory contents (use recursive=true for deep exploration)
- glob: Find files matching patterns (e.g., "**/*.rs" for all Rust files)
- grep: Search for text/patterns in files (use for finding functions, TODOs, etc.)
- read_file: Read file contents
- edit_file: Modify existing files
- write_file: Create new files
- bash: Run shell commands (git, cargo, npm, etc.)
- execute_code: Test code snippets
- web_search: Search the internet for documentation
- http_request: Call external APIs
- task_manager: Track multi-step work
- session_context: Remember important facts
- plan: Create structured plans for complex tasks (use when user requests require multiple coordinated steps)

CRITICAL: PLAN TOOL USAGE

WHEN NOT TO USE THE PLAN TOOL (check this FIRST):
- The request is satisfied by one or two tool calls ("list the files", "read main.rs",
  "what's in this folder", "run the tests"). Just call that tool and answer.
- The user did not ask for a plan and the work is not multi-step.
- A tool call was denied or failed. Creating a plan is NOT a recovery strategy.
  Report what failed and stop. Do NOT escalate to a different tool, and do NOT
  invent unrelated work.
If you are unsure, do NOT create a plan - answer directly.

When a user says "create a plan", "make a plan", or describes a genuinely complex
multi-step task, you MUST use the plan tool immediately.
DO NOT write a text description of a plan. DO NOT explain what should be done. CALL THE TOOL.
Every task in a plan MUST come from what the user actually asked for. Never
invent a task the user did not request.

Mandatory steps for plan creation:
1. IMMEDIATELY call plan tool with operation='create' to create a new plan
2. Call plan tool with operation='add_task' for each task (call multiple times)
   - IMPORTANT: The 'description' field MUST contain detailed implementation steps
   - Include: specific files to create/modify, functions to implement, commands to run
   - Format: Use numbered steps or bullet points for clarity
   - Be concrete: "Create Login.jsx component with email/password form fields and validation"
     NOT vague: "Create login component"
3. Call plan tool with operation='finalize' to present the plan for user approval
4. **STOP CALLING TOOLS** - After 'finalize', DO NOT call any more plan operations!
5. INFORM the user that the plan is ready for review:
   "✅ Plan finalized! The plan is now displayed in Plan Mode for your review.

   To proceed:
   • Press Ctrl+A to approve and execute the plan
   • Press Ctrl+R to reject and revise the plan
   • Press Esc to cancel and return to chat

   When you approve, the plan will be automatically exported to PLAN.md and execution will begin."
6. WAIT for the user to approve the plan via Ctrl+A before execution begins
   - The TUI will automatically switch to Plan Mode and display the plan
   - User controls the approval through keyboard shortcuts, not text responses
   - Your job is DONE after calling finalize and informing the user

IMPORTANT: Do NOT call plan tool with operation='export_markdown' after finalize.
The markdown export happens automatically when the user presses Ctrl+A to approve the plan.

Example: If user says "create a plan to implement a login page"
- FIRST TOOL CALL: plan(operation="create", title="Implement Login Page", description="Build a React login page with email/password authentication", context="React app needs user authentication. Backend API endpoint /auth/login exists.")
- NEXT TOOL CALL: plan(operation="add_task", title="Create Login Component", description="1. Create src/components/Login.jsx file\n2. Add email input field with type='email' validation\n3. Add password input field with type='password'\n4. Add submit button that calls handleSubmit()\n5. Import useState for form state management\n6. Add basic CSS styling for form layout", task_type="create", complexity=2)
- NEXT TOOL CALL: plan(operation="add_task", title="Implement Authentication Logic", description="1. Create handleSubmit() function in Login.jsx\n2. Validate email format using regex\n3. Make POST request to /auth/login endpoint\n4. Include email/password in request body\n5. Handle success response - store JWT token in localStorage\n6. Handle error response - display error message to user\n7. Redirect to dashboard on successful login", task_type="edit", complexity=3, dependencies=[1])
- TOOL CALL: plan(operation="finalize")
- THEN SAY: "✅ Plan finalized! The plan is now displayed in Plan Mode. Press Ctrl+A to approve and execute, Ctrl+R to reject, or Esc to cancel. The plan will be exported to PLAN.md when you approve it."

TASK DESCRIPTION QUALITY REQUIREMENTS:
- Each task description MUST be detailed enough to execute without further clarification
- Include specific file paths, function names, and concrete implementation steps
- Mention required libraries, APIs, or dependencies
- Specify error handling and edge cases
- Add configuration or setup requirements

NEVER generate text plans. ALWAYS use the plan tool for planning requests.

ALWAYS explore first before answering questions about a codebase. Don't guess - use the tools!

## Reasoning Pattern (ReAct)

For complex tasks (multi-file analysis, debugging, architecture questions), structure your internal reasoning before acting:

**THINK:** What is the user asking? What information do I need? What is the dependency order of my tool calls?
**ACT:** Call the minimum set of tools needed to answer the question.
**OBSERVE:** What did the tool results reveal? Did anything surprise me or contradict my hypothesis?
**UPDATE:** Revise my understanding. Is my plan still correct, or do I need different tool calls?

Apply this pattern silently — do not output the THINK/OBSERVE/UPDATE labels to the user unless they ask to see your reasoning. Emit **ACT:** labels only when you want to explain which tool you are about to call and why."#;

/// Crustly - High-Performance Terminal AI Assistant
#[derive(Parser, Debug)]
#[command(name = "crustly")]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Enable debug mode (creates log files in .crustly/logs/)
    #[arg(short, long, global = true)]
    pub debug: bool,

    /// Configuration file path
    #[arg(short, long, global = true)]
    pub config: Option<String>,

    /// Override the model for this run, e.g. `qwen2.5-coder:7b`.
    ///
    /// Applies to whichever provider is active, without editing config.toml.
    /// Long-only: `auto-plan` already uses `-m` for `--max-iterations`, and a
    /// global short `-m` would collide with it.
    #[arg(long, global = true)]
    pub model: Option<String>,

    /// Subcommand to execute
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Start interactive TUI mode (default)
    Chat {
        /// Session ID to resume
        #[arg(short, long)]
        session: Option<String>,
    },

    /// Run a single command non-interactively
    Run {
        /// The prompt to execute
        prompt: String,

        /// Auto-approve all tool executions (dangerous!)
        #[arg(long, alias = "yolo")]
        auto_approve: bool,

        /// Output format
        #[arg(short, long, default_value = "text")]
        format: OutputFormat,
    },

    /// Initialize configuration
    Init {
        /// Force overwrite existing configuration
        #[arg(short, long)]
        force: bool,
    },

    /// Show configuration
    Config {
        /// Show full configuration including secrets
        #[arg(short, long)]
        show_secrets: bool,
    },

    /// Database operations
    Db {
        #[command(subcommand)]
        operation: DbCommands,
    },

    /// Log management operations
    Logs {
        #[command(subcommand)]
        operation: LogCommands,
    },

    /// Manage API keys in OS keyring (secure storage)
    Keyring {
        #[command(subcommand)]
        operation: KeyringCommands,
    },

    /// Run in fully-autonomous plan mode (FullAuto — no approval gates).
    ///
    /// Example: crustly autoplan "Refactor the auth module to use the repository pattern"
    AutoPlan {
        /// The goal to pursue autonomously.
        goal: String,

        /// Maximum number of agent iterations before stopping.
        #[arg(short, long, default_value = "20")]
        max_iterations: u32,
    },

    /// Manage local Ollama models (native provider, requires the crate's
    /// 'ollama' build feature)
    Ollama {
        #[command(subcommand)]
        operation: OllamaCommands,
    },

    /// Manage local .gguf model files (native llama.cpp provider, requires
    /// the crate's 'llama-cpp' build feature)
    LlamaCpp {
        #[command(subcommand)]
        operation: LlamaCppCommands,
    },
}

#[derive(Subcommand, Debug)]
pub enum OllamaCommands {
    /// List models already pulled/installed locally
    List,
    /// Pull (download) a model by name, e.g. `qwen2.5-coder:7b`
    Pull {
        /// Model name/tag, exactly as understood by `ollama pull <name>`
        model: String,
    },
    /// Delete a locally-installed model
    Rm {
        /// Model name/tag to delete
        model: String,
    },
    /// Show details about a model (license, parameters, template, capabilities)
    Show {
        /// Model name/tag to inspect
        model: String,
    },
    /// Generate an embedding vector for a piece of text using an
    /// embedding-capable model (e.g. `nomic-embed-text`)
    Embed {
        /// Embedding model name/tag
        model: String,
        /// Text to embed
        text: String,
    },
}

#[derive(Subcommand, Debug)]
pub enum LlamaCppCommands {
    /// List .gguf files in the configured models directory, plus any
    /// extra_model_paths/scan_ollama_models sources
    ///
    /// Exit codes: 0 success, 1 unrecognized error. `--json` output is a
    /// stable, versioned schema (`schema_version` field) suitable for
    /// scripting - see docs/guides/LLAMA_CPP_GUIDE.md.
    List {
        /// Print a stable, versioned JSON schema instead of a human-readable
        /// table - no emoji header, just the JSON, for scripting/agents.
        #[arg(long)]
        json: bool,
        /// Sort by fit against this machine's detected GPU VRAM/system RAM
        /// (best-effort, best-effort-cached per invocation) instead of by
        /// path, and annotate each entry Fits/Tight/Won't fit. Composable
        /// with --json (adds `fit`/`estimated_memory_context_length`
        /// fields). Never used for anything but already-downloaded models -
        /// this does not search Hugging Face for new ones.
        #[arg(long)]
        best_fit: bool,
    },
    /// Download a model: a direct URL, or an `hf:org/repo/file.gguf[@revision]`
    /// shorthand resolved against Hugging Face
    ///
    /// Exit codes: 0 success, 11 not enough disk space, 12 checksum mismatch,
    /// 13 network/download failure, 14 build missing the 'gguf-management'
    /// feature, 1 any other error.
    Pull {
        /// Direct URL or `hf:org/repo/file.gguf[@revision]` shorthand
        source: String,
    },
    /// Delete a local .gguf file by name or path
    ///
    /// Exit codes: 0 success, 10 no such file, 14 build missing the
    /// 'gguf-management' feature, 1 any other error.
    Rm {
        /// Filename (resolved inside the models directory) or full path.
        /// Named `name`, not `model` - a positional field sharing an ident
        /// with the top-level `global = true` `--model` flag causes clap
        /// to route the value into the wrong one (confirmed independently
        /// of this plan's own changes - `ollama rm` has the identical
        /// pre-existing bug via the same collision, left unfixed here as
        /// out of this phase's scope, flagged separately).
        name: String,
    },
    /// Diagnose local .gguf model management setup: build features,
    /// models directory, disk space, configured extra sources
    ///
    /// Always exits 0 - this reports findings as text, it never fails
    /// the way `list`/`pull`/`rm` can.
    Doctor,
}

/// `crustly llama-cpp list --json`'s schema version. Bump only on a
/// breaking change to `LlamaCppModelJson`'s field set/types - additive
/// fields don't need a bump, per the usual "additive is non-breaking"
/// convention for a versioned JSON contract.
#[cfg(feature = "gguf-management")]
const LLAMA_CPP_LIST_JSON_SCHEMA_VERSION: u32 = 1;

/// Stable, versioned wire format for `crustly llama-cpp list --json` -
/// deliberately a separate type from `LocalGgufModel`
/// (`src/llm/provider/llama_cpp_models.rs`), not `#[derive(Serialize)]` on
/// it directly, so the internal struct can keep evolving without silently
/// breaking this contract. snake_case field names, no `rename_all` needed -
/// matches the dominant convention for Crustly's own domain types (e.g.
/// `src/config/mod.rs`); camelCase in this codebase is reserved for structs
/// mirroring an external API, which this isn't. Gated on `gguf-management`
/// since it references `LocalGgufModel`, which is itself gated - the not-
/// compiled-in build of `cmd_llama_cpp` never needs this type at all.
#[cfg(feature = "gguf-management")]
#[derive(Debug, Clone, serde::Serialize)]
struct LlamaCppModelJson {
    path: std::path::PathBuf,
    display_name: Option<String>,
    size_bytes: u64,
    modified_at: String,
    architecture: Option<String>,
    parameter_count: Option<u64>,
    quantization: Option<String>,
    context_length: Option<u64>,
    has_chat_template: bool,
    estimated_memory_bytes: Option<u64>,
    estimated_memory_includes_kv_cache: bool,
    /// The context length `estimated_memory_bytes` was actually computed
    /// at - see `LocalGgufModel::estimated_memory_context_length`.
    estimated_memory_context_length: Option<u64>,
    is_mmproj: bool,
    mmproj_path: Option<std::path::PathBuf>,
    /// Fit against this machine's detected hardware ("Fits"/"Tight"/"Won't
    /// fit"/"unknown") - Phase M12. Only set (present in the serialized
    /// JSON) when `--best-fit` was passed; omitted entirely on a plain
    /// `list --json`, matching that flag's opt-in, detection-costs-a-
    /// subprocess-spawn framing (`hardware_detect`'s own module doc).
    #[serde(skip_serializing_if = "Option::is_none")]
    fit: Option<String>,
}

#[cfg(feature = "gguf-management")]
impl From<&crate::llm::provider::llama_cpp_models::LocalGgufModel> for LlamaCppModelJson {
    fn from(m: &crate::llm::provider::llama_cpp_models::LocalGgufModel) -> Self {
        // Destructured with every field named and no `..` rest pattern -
        // deliberately, as a poor man's exhaustiveness check: a field added
        // to `LocalGgufModel` fails to compile right here instead of
        // silently never reaching the `--json` output. See
        // `llama_cpp_download.rs`'s `list_local` for the identical
        // safeguard on the TUI's own mirror of this same struct.
        let crate::llm::provider::llama_cpp_models::LocalGgufModel {
            path,
            size_bytes,
            modified_at,
            quantization_hint,
            architecture,
            parameter_count,
            context_length,
            has_chat_template,
            display_name,
            estimated_memory_bytes,
            estimated_memory_includes_kv_cache,
            estimated_memory_context_length,
            is_mmproj,
            mmproj_path,
        } = m;
        Self {
            path: path.clone(),
            display_name: display_name.clone(),
            size_bytes: *size_bytes,
            modified_at: modified_at.clone(),
            architecture: architecture.clone(),
            parameter_count: *parameter_count,
            quantization: quantization_hint.clone(),
            context_length: *context_length,
            has_chat_template: *has_chat_template,
            estimated_memory_bytes: *estimated_memory_bytes,
            estimated_memory_includes_kv_cache: *estimated_memory_includes_kv_cache,
            estimated_memory_context_length: *estimated_memory_context_length,
            is_mmproj: *is_mmproj,
            mmproj_path: mmproj_path.clone(),
            fit: None,
        }
    }
}

#[cfg(feature = "gguf-management")]
#[derive(Debug, Clone, serde::Serialize)]
struct LlamaCppListJson {
    schema_version: u32,
    models: Vec<LlamaCppModelJson>,
}

/// Human/JSON display label for a `HardwareFit` value - Phase M12. A
/// standalone function (not a `Display` impl on `HardwareFit` itself,
/// which lives in `llama_cpp_models.rs` and has no reason to know about
/// this CLI's specific wording) so `--best-fit`'s human table and its
/// `--json` `fit` field use one identical wording, not two that could
/// drift apart.
#[cfg(feature = "gguf-management")]
fn hardware_fit_label(fit: crate::llm::provider::llama_cpp_models::HardwareFit) -> &'static str {
    use crate::llm::provider::llama_cpp_models::HardwareFit;
    match fit {
        HardwareFit::Fits => "Fits",
        HardwareFit::Tight => "Tight",
        HardwareFit::WontFit => "Won't fit",
        HardwareFit::Unknown => "unknown",
    }
}

/// Error-message prefixes this file's own `bail!` sites produce, that
/// `llama_cpp_exit_code` below matches against - declared once and
/// referenced at both the producing and matching sites for the same
/// single-source-of-truth reason as `llama_cpp_models`'s
/// `DISK_SPACE_ERROR_PREFIX`/etc. (see that module's doc comment on those
/// constants). `FEATURE_NOT_COMPILED_ERROR_PREFIX` is shared by every
/// provider's "not compiled in" `bail!` (`cmd_ollama`'s included), not just
/// `llama-cpp`'s - only the latter is reachable through this exit-code
/// mapping today, but the wording itself is meant to stay identical across
/// providers.
const NO_SUCH_FILE_ERROR_PREFIX: &str = "No such file:";
const FEATURE_NOT_COMPILED_ERROR_PREFIX: &str = "This build of crustly was compiled without";

/// `llama_cpp_exit_code` below is deliberately *not* `#[cfg(feature =
/// "gguf-management")]`-gated itself - it still has to run (and correctly
/// return 14) against `cmd_llama_cpp`'s `not(gguf-management)` stub error.
/// That means its body must compile either way, but
/// `llama_cpp_models::{DISK_SPACE_ERROR_PREFIX, ...}` only exist when the
/// module they live in is compiled in (`provider/mod.rs` gates the whole
/// `mod llama_cpp_models;` declaration on this same feature) - so the
/// import is gated, with a same-named, same-valued local fallback for the
/// off build below. That fallback never actually matches anything real:
/// codes 11-13 can only be produced by `download_model`, which doesn't
/// exist without this feature either.
#[cfg(feature = "gguf-management")]
use crate::llm::provider::llama_cpp_models::{
    CHECKSUM_MISMATCH_ERROR_PREFIX, DISK_SPACE_ERROR_PREFIX, DOWNLOAD_FAILED_ERROR_PREFIX,
    DOWNLOAD_START_ERROR_PREFIX,
};
#[cfg(not(feature = "gguf-management"))]
const DISK_SPACE_ERROR_PREFIX: &str = "Not enough disk space";
#[cfg(not(feature = "gguf-management"))]
const CHECKSUM_MISMATCH_ERROR_PREFIX: &str = "Checksum mismatch for";
#[cfg(not(feature = "gguf-management"))]
const DOWNLOAD_START_ERROR_PREFIX: &str = "Failed to start download from";
#[cfg(not(feature = "gguf-management"))]
const DOWNLOAD_FAILED_ERROR_PREFIX: &str = "Download failed for";

/// Maps an error from a `crustly llama-cpp` subcommand to a specific,
/// documented exit code, by matching known message prefixes anywhere in
/// the error's context chain (`anyhow::Error::chain()`, not just
/// `.to_string()`'s outermost layer - `Pull`'s errors are wrapped in
/// `"Failed to download '...'"`, so the more specific cause is a layer
/// deeper). Reuses Crustly's own existing error *messages* as the taxonomy
/// (every prefix here is a string this codebase already produces
/// elsewhere, unchanged by this function) rather than inventing new ones
/// or importing llamastash's specific numeric meanings - see
/// `ccguf-managment-imrpoment-plan.md` Phase M7.
///
/// An unmatched error falls back to `1`, identical to every other Crustly
/// command's behavior today - this only adds finer-grained codes on top of
/// that default for failure messages that already exist, it never changes
/// what a *new*, unrecognized error does.
fn llama_cpp_exit_code(err: &anyhow::Error) -> i32 {
    for cause in err.chain() {
        let msg = cause.to_string();
        if msg.starts_with(NO_SUCH_FILE_ERROR_PREFIX) {
            return 10; // model/file not found
        }
        if msg.starts_with(DISK_SPACE_ERROR_PREFIX) {
            return 11;
        }
        if msg.starts_with(CHECKSUM_MISMATCH_ERROR_PREFIX) {
            return 12;
        }
        if msg.starts_with(DOWNLOAD_START_ERROR_PREFIX)
            || msg.starts_with(DOWNLOAD_FAILED_ERROR_PREFIX)
        {
            return 13; // network/HTTP failure
        }
        if msg.starts_with(FEATURE_NOT_COMPILED_ERROR_PREFIX) {
            return 14; // feature not compiled in
        }
    }
    1
}

#[derive(Subcommand, Debug)]
pub enum LogCommands {
    /// Show log file location and status
    Status,
    /// View recent log entries (requires debug mode)
    View {
        /// Number of lines to show (default: 50)
        #[arg(short, long, default_value = "50")]
        lines: usize,
    },
    /// Clean up old log files
    Clean {
        /// Maximum age in days (default: 7)
        #[arg(short = 'a', long, default_value = "7")]
        days: u64,
    },
    /// Open log directory in file manager
    Open,
    /// Show prompt cache hit/miss statistics for the current session.
    ShowCacheStats,
}

#[derive(Subcommand, Debug)]
pub enum DbCommands {
    /// Initialize database
    Init,
    /// Show database statistics
    Stats,
    /// Clear all sessions and messages from database
    Clear {
        /// Skip confirmation prompt (use with caution)
        #[arg(short, long)]
        force: bool,
    },
}

#[derive(Subcommand, Debug)]
pub enum KeyringCommands {
    /// Store an API key in OS keyring
    Set {
        /// Provider name (anthropic, openai, gemini, azure)
        provider: String,
        /// API key to store
        api_key: String,
    },
    /// Retrieve an API key from OS keyring
    Get {
        /// Provider name
        provider: String,
    },
    /// Delete an API key from OS keyring
    Delete {
        /// Provider name
        provider: String,
    },
    /// List all stored providers
    List,
}

#[derive(Debug, Clone, Copy, clap::ValueEnum)]
pub enum OutputFormat {
    Text,
    Json,
    Markdown,
}

/// Main CLI entry point
pub async fn run() -> Result<std::process::ExitCode> {
    let cli = Cli::parse();

    // Set up logging level based on debug flag
    if cli.debug {
        tracing::info!("Debug mode enabled");
    }

    // Load configuration
    let mut config = load_config(cli.config.as_deref()).await?;

    // Apply `--model` before any provider is built. `create_provider` reads
    // `default_model` off whichever provider config it selects, so overriding the
    // field here reaches every provider and every subcommand with no further
    // plumbing - and nothing is written back to config.toml.
    if let Some(model) = &cli.model {
        match config.providers.override_default_model(model) {
            Some(provider) => {
                tracing::info!("Model overridden via --model: {model} (provider: {provider})");
            }
            None => {
                anyhow::bail!(
                    "--model {model} was given, but no provider is configured to run it.\n\
                     Configure a provider in config.toml first (see `crustly init`)."
                );
            }
        }
    }
    let config = config;

    // `LlamaCpp` needs a documented, non-1 exit code on failure (10-14 -
    // see `llama_cpp_exit_code`); every other command keeps Rust's default
    // `Result<(), E>` `Termination` behavior (print `Error: {e:?}`, exit 1)
    // completely unchanged below. `LlamaCpp` used to get its custom code via
    // `std::process::exit()` directly - which terminates the process
    // immediately, skipping every live destructor, including `main()`'s
    // `_guard` (a `tracing_appender` `WorkerGuard`) that flushes buffered
    // debug-mode log lines on drop. Returning a `std::process::ExitCode`
    // instead (an early `return` here, not a `process::exit` call) lets
    // `main()`'s async body finish and its locals drop normally before the
    // process actually exits - the standard, destructor-safe way to produce
    // a custom exit code, and the only change from before: the printed
    // message and the exit code itself are identical to what
    // `std::process::exit` produced.
    if let Some(Commands::LlamaCpp { operation }) = cli.command {
        return match cmd_llama_cpp(&config, operation).await {
            Ok(()) => Ok(std::process::ExitCode::SUCCESS),
            Err(e) => {
                eprintln!("Error: {e:?}");
                Ok(std::process::ExitCode::from(llama_cpp_exit_code(&e) as u8))
            }
        };
    }

    let result: Result<()> = match cli.command {
        None | Some(Commands::Chat { session: _ }) => {
            // Default: Interactive TUI mode
            let session = match &cli.command {
                Some(Commands::Chat { session }) => session.clone(),
                _ => None,
            };
            cmd_chat(&config, session).await
        }
        Some(Commands::Init { force }) => cmd_init(&config, force).await,
        Some(Commands::Config { show_secrets }) => cmd_config(&config, show_secrets).await,
        Some(Commands::Db { operation }) => cmd_db(&config, operation).await,
        Some(Commands::Logs { operation }) => cmd_logs(operation).await,
        Some(Commands::Keyring { operation }) => cmd_keyring(operation).await,
        Some(Commands::Run {
            prompt,
            auto_approve,
            format,
        }) => cmd_run(&config, prompt, auto_approve, format).await,
        Some(Commands::AutoPlan {
            goal,
            max_iterations,
        }) => cmd_autoplan(&config, goal, max_iterations).await,
        Some(Commands::Ollama { operation }) => cmd_ollama(&config, operation).await,
        Some(Commands::LlamaCpp { .. }) => unreachable!("handled by the early return above"),
    };
    result.map(|()| std::process::ExitCode::SUCCESS)
}

/// Load configuration from file or defaults
async fn load_config(config_path: Option<&str>) -> Result<crate::config::Config> {
    use crate::config::Config;

    let config = if let Some(path) = config_path {
        tracing::info!("Loading configuration from custom path: {}", path);
        Config::load_from_path(path)?
    } else {
        tracing::debug!("Loading default configuration");
        Config::load()?
    };

    // Validate configuration
    config.validate()?;

    Ok(config)
}

/// Initialize configuration file
async fn cmd_init(_config: &crate::config::Config, force: bool) -> Result<()> {
    use crate::config::Config;

    println!("🦀 Crustly Configuration Initialization\n");

    let config_path = dirs::config_dir()
        .context("Could not determine config directory")?
        .join("crustly")
        .join("config.toml");

    // Check if config already exists
    if config_path.exists() && !force {
        anyhow::bail!(
            "Configuration file already exists at: {}\nUse --force to overwrite",
            config_path.display()
        );
    }

    // Save default configuration
    let default_config = Config::default();
    default_config.save(&config_path)?;

    println!("✅ Configuration initialized at: {}", config_path.display());
    println!("\n📝 Next steps:");
    println!("   1. Edit the config file to add your API keys");
    println!("   2. Set ANTHROPIC_API_KEY environment variable");
    println!("   3. Run 'crustly' or 'crustly chat' to start");

    Ok(())
}

/// Show configuration
async fn cmd_config(config: &crate::config::Config, show_secrets: bool) -> Result<()> {
    println!("🦀 Crustly Configuration\n");

    if show_secrets {
        println!("{:#?}", config);
    } else {
        println!("Database: {}", config.database.path.display());
        println!("Log level: {}", config.logging.level);
        println!("\nProviders:");

        if let Some(ref anthropic) = config.providers.anthropic {
            println!(
                "  - anthropic: {}",
                anthropic
                    .default_model
                    .as_ref()
                    .unwrap_or(&"claude-3-5-sonnet-20240620".to_string())
            );
            println!(
                "    API Key: {}",
                if anthropic.api_key.is_some() {
                    "[SET]"
                } else {
                    "[NOT SET]"
                }
            );
        }

        if let Some(ref openai) = config.providers.openai {
            println!(
                "  - openai: {}",
                openai
                    .default_model
                    .as_ref()
                    .unwrap_or(&"gpt-4".to_string())
            );
            println!(
                "    API Key: {}",
                if openai.api_key.is_some() {
                    "[SET]"
                } else {
                    "[NOT SET]"
                }
            );
        }

        println!("\n💡 Use --show-secrets to display API keys");
    }

    Ok(())
}

/// Database operations
async fn cmd_db(config: &crate::config::Config, operation: DbCommands) -> Result<()> {
    use crate::db::Database;

    match operation {
        DbCommands::Init => {
            println!("🗄️  Initializing database...");
            let db = Database::connect(&config.database.path).await?;
            db.run_migrations().await?;
            println!(
                "✅ Database initialized at: {}",
                config.database.path.display()
            );
            Ok(())
        }
        DbCommands::Stats => {
            println!("📊 Database Statistics\n");
            let db = Database::connect(&config.database.path).await?;

            // Get counts using raw SQL for simplicity
            let session_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM sessions")
                .fetch_one(db.pool())
                .await?;

            let message_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM messages")
                .fetch_one(db.pool())
                .await?;

            let file_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM files")
                .fetch_one(db.pool())
                .await?;

            println!("Sessions: {}", session_count);
            println!("Messages: {}", message_count);
            println!("Files: {}", file_count);

            Ok(())
        }
        DbCommands::Clear { force } => {
            let db = Database::connect(&config.database.path).await?;

            // Get counts before clearing
            let session_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM sessions")
                .fetch_one(db.pool())
                .await?;

            let message_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM messages")
                .fetch_one(db.pool())
                .await?;

            let file_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM files")
                .fetch_one(db.pool())
                .await?;

            if session_count == 0 && message_count == 0 && file_count == 0 {
                println!("✨ Database is already empty");
                return Ok(());
            }

            println!("⚠️  WARNING: This will permanently delete ALL data:\n");
            println!("   • {} sessions", session_count);
            println!("   • {} messages", message_count);
            println!("   • {} files", file_count);
            println!();

            // Confirmation prompt
            if !force {
                use std::io::{self, Write};
                print!("Type 'yes' to confirm deletion: ");
                io::stdout().flush()?;

                let mut input = String::new();
                io::stdin().read_line(&mut input)?;

                if input.trim().to_lowercase() != "yes" {
                    println!("❌ Cancelled - no data was deleted");
                    return Ok(());
                }
            }

            // Clear all tables
            println!("\n🗑️  Clearing database...");

            // Delete in correct order to respect foreign key constraints
            sqlx::query("DELETE FROM messages")
                .execute(db.pool())
                .await?;

            sqlx::query("DELETE FROM files").execute(db.pool()).await?;

            sqlx::query("DELETE FROM sessions")
                .execute(db.pool())
                .await?;

            println!(
                "✅ Successfully cleared {} sessions, {} messages, and {} files",
                session_count, message_count, file_count
            );

            Ok(())
        }
    }
}

/// Build the tool registry with the full set of built-in tools available to
/// the interactive chat agent (MCP servers are registered separately, since
/// that requires network I/O and per-server config - see
/// `connect_configured_mcp_servers`).
fn build_tool_registry() -> crate::llm::tools::registry::ToolRegistry {
    use crate::llm::tools::{
        agent::AgentTool, apply_patch::ApplyPatchTool, ask_user::AskUserTool, bash::BashTool,
        code_exec::CodeExecTool, context::ContextTool, doc_parser::DocParserTool, edit::EditTool,
        glob::GlobTool, grep::GrepTool, http::HttpClientTool, ls::LsTool,
        notebook::NotebookEditTool, plan_tool::PlanTool, powershell::PowerShellTool,
        read::ReadTool, registry::ToolRegistry, save_memory::SaveMemoryTool, skill::SkillTool,
        task::TaskTool, todo_write::TodoWriteTool, web_fetch::WebFetchTool,
        web_search::WebSearchTool, write::WriteTool,
    };

    let mut tool_registry = ToolRegistry::new();
    // Phase 1: Essential file operations
    tool_registry.register(Arc::new(ReadTool));
    tool_registry.register(Arc::new(WriteTool));
    tool_registry.register(Arc::new(EditTool));
    tool_registry.register(Arc::new(ApplyPatchTool));
    tool_registry.register(Arc::new(BashTool));
    tool_registry.register(Arc::new(LsTool));
    tool_registry.register(Arc::new(GlobTool));
    tool_registry.register(Arc::new(GrepTool));
    // Phase 2: Advanced features
    tool_registry.register(Arc::new(WebSearchTool));
    tool_registry.register(Arc::new(CodeExecTool));
    tool_registry.register(Arc::new(NotebookEditTool));
    tool_registry.register(Arc::new(DocParserTool));
    // Phase 3: Workflow & integration
    tool_registry.register(Arc::new(TaskTool));
    tool_registry.register(Arc::new(ContextTool));
    tool_registry.register(Arc::new(SaveMemoryTool));
    tool_registry.register(Arc::new(HttpClientTool));
    tool_registry.register(Arc::new(PlanTool));
    // Phase 4: Claw Code parity
    tool_registry.register(Arc::new(WebFetchTool));
    tool_registry.register(Arc::new(TodoWriteTool));
    tool_registry.register(Arc::new(AskUserTool));
    tool_registry.register(Arc::new(SkillTool));
    tool_registry.register(Arc::new(AgentTool));
    tool_registry.register(Arc::new(PowerShellTool));

    tool_registry
}

/// Connect to every configured MCP server (`[[mcp.servers]]`) and register
/// their tools. Fixes a real gap: config.mcp.servers was previously parsed
/// but never consumed anywhere, so configured servers had zero runtime
/// effect. Failures are caught per-server (recorded in the status snapshot
/// for the TUI's `/mcp` view) rather than aborting startup - one broken MCP
/// server shouldn't block the whole TUI.
async fn connect_configured_mcp_servers(
    tool_registry: &mut crate::llm::tools::registry::ToolRegistry,
    config: &crate::config::Config,
) -> Vec<crate::mcp::McpServerStatus> {
    let mut mcp_status = Vec::new();
    for server in &config.mcp.servers {
        let args: Vec<&str> = server.args.iter().map(String::as_str).collect();
        match tool_registry
            .register_mcp_server(&server.name, &server.command, &args)
            .await
        {
            Ok(tool_count) => {
                tracing::info!(
                    "Connected to MCP server '{}' ({} tools)",
                    server.name,
                    tool_count
                );
                mcp_status.push(crate::mcp::McpServerStatus {
                    name: server.name.clone(),
                    command: server.command.clone(),
                    connected: true,
                    tool_count,
                    error: None,
                });
            }
            Err(e) => {
                tracing::warn!("Failed to connect to MCP server '{}': {}", server.name, e);
                mcp_status.push(crate::mcp::McpServerStatus {
                    name: server.name.clone(),
                    command: server.command.clone(),
                    connected: false,
                    tool_count: 0,
                    error: Some(e.to_string()),
                });
            }
        }
    }
    mcp_status
}

/// Build the tool-approval callback the agent invokes before executing a
/// tool. Auto Mode (see `PlanExecMode` doc comments) lets low-risk tools
/// through without prompting; everything else round-trips through the TUI
/// event channel and blocks on the user's response. This does NOT touch
/// `AgentService::auto_approve_tools` or the `SecurityConfig` policy chain in
/// `ToolRegistry::execute` - both remain fully enforced regardless of Auto
/// Mode, so deny-listed tools/paths/bash patterns stay blocked no matter
/// what, and every auto-approved call still produces the same "User approved
/// tool" log line as a manually-approved one (logged downstream in
/// `AgentService`, not here).
fn build_approval_callback(
    event_sender: tokio::sync::mpsc::UnboundedSender<crate::tui::events::TuiEvent>,
    auto_mode: Arc<std::sync::Mutex<crate::config::PlanExecMode>>,
) -> crate::llm::agent::ApprovalCallback {
    Arc::new(move |tool_info| {
        let sender = event_sender.clone();
        let auto_mode = auto_mode.clone();
        Box::pin(async move {
            use crate::tui::events::{ToolApprovalRequest, TuiEvent};
            use tokio::sync::mpsc;

            let mode = auto_mode.lock().expect("auto_mode mutex poisoned").clone();
            if auto_mode_bypasses_approval(&mode, &tool_info.tool_name) {
                tracing::debug!(
                    "Auto Mode ({:?}) approved tool '{}' without prompting",
                    mode,
                    tool_info.tool_name
                );
                return Ok(true);
            }

            // Create response channel
            let (response_tx, mut response_rx) = mpsc::unbounded_channel();

            // Create approval request
            let request = ToolApprovalRequest {
                request_id: uuid::Uuid::new_v4(),
                tool_name: tool_info.tool_name,
                tool_description: tool_info.tool_description,
                tool_input: tool_info.tool_input,
                capabilities: tool_info.capabilities,
                response_tx,
                requested_at: std::time::Instant::now(),
            };

            // Send to TUI
            sender
                .send(TuiEvent::ToolApprovalRequested(request))
                .map_err(|e| {
                    crate::llm::agent::AgentError::Internal(format!(
                        "Failed to send approval request: {}",
                        e
                    ))
                })?;

            // Wait for response
            let response = response_rx.recv().await.ok_or_else(|| {
                crate::llm::agent::AgentError::Internal(
                    "Approval response channel closed".to_string(),
                )
            })?;

            Ok(response.approved)
        })
    })
}

/// Start interactive chat session
async fn cmd_chat(config: &crate::config::Config, _session_id: Option<String>) -> Result<()> {
    use crate::{db::Database, llm::agent::AgentService, services::ServiceContext, tui};

    println!("🦀 Starting Crustly AI Assistant...\n");

    // Initialize database
    tracing::info!("Connecting to database: {}", config.database.path.display());
    let db = Database::connect(&config.database.path)
        .await
        .context("Failed to connect to database")?;

    // Run migrations
    db.run_migrations()
        .await
        .context("Failed to run database migrations")?;

    // Select provider based on configuration using factory
    let provider = crate::llm::provider::create_provider(config)?;

    // Create tool registry
    tracing::debug!("Setting up tool registry");
    let mut tool_registry = build_tool_registry();
    let mcp_status = connect_configured_mcp_servers(&mut tool_registry, config).await;

    // Create service context
    let service_context = ServiceContext::new(db.pool().clone());

    // Get working directory
    let working_directory = std::env::current_dir().unwrap_or_default();

    // Create agent service with system prompt and working directory
    let agent_service = Arc::new(
        AgentService::new(provider.clone(), service_context.clone())
            .with_system_prompt(SYSTEM_PROMPT.to_string())
            .with_max_tool_iterations(20)
            .with_working_directory(working_directory.clone()),
    );

    // Create TUI app first (so we can get the event sender)
    tracing::debug!("Creating TUI app");
    let mut app = tui::App::new(agent_service, service_context.clone());
    app.set_ollama_host(ollama_host(config));
    // Hand the [providers.ollama] section to the TUI so the Ctrl+W model
    // switch rebuilds providers with the SAME settings as startup (per-model
    // num_ctx/sampling, keep_alive) instead of a bare unconfigured provider.
    if let Some(ollama_cfg) = &config.providers.ollama {
        app.set_ollama_config(ollama_cfg.clone());
    }
    app.set_llama_cpp_models_dir(config.providers.llama_cpp_models_dir());
    app.set_llama_cpp_discovery_sources(
        config.providers.llama_cpp_extra_model_paths(),
        config.providers.llama_cpp_ollama_models_dir(),
    );
    // Hand the [providers.llama_cpp] section to the TUI for the same reason
    // as ollama_config above: the Ctrl+G model switch needs the same
    // n_gpu_layers/n_ctx/sampling settings as the one built at startup, and
    // the Model Info panel needs it to show GPU-layers/quantization.
    if let Some(llama_cpp_cfg) = &config.providers.llama_cpp {
        app.set_llama_cpp_config(llama_cpp_cfg.clone());
    }
    app.set_mcp_status(mcp_status);

    // Get event sender from app
    let event_sender = app.event_sender();

    // Shared Auto Mode level (Interactive/AutoPlan/FullAuto), seeded from
    // config and toggled at runtime by the TUI's Shift+Tab handler
    // (App::cycle_auto_mode). Cloned into the approval callback below so
    // toggling it in the TUI takes effect on the very next tool call.
    let auto_mode = Arc::new(std::sync::Mutex::new(config.plan_mode.mode.clone()));
    app.set_auto_mode_state(auto_mode.clone());

    // Create approval callback that sends requests to TUI - unless Auto Mode
    // bypasses it (see `build_approval_callback` doc comment).
    let approval_callback = build_approval_callback(event_sender, auto_mode);

    // Install the [security] policy chain (deny_tools/deny_paths/allow_bash).
    // Without this the whole section is inert: nothing is denied, and nothing
    // is trusted, so every bash call re-prompts.
    tool_registry.set_policy(config.security.to_policy().into());

    // Create agent service with approval callback
    tracing::debug!("Creating agent service with approval callback");
    let agent_service = Arc::new(
        AgentService::new(provider.clone(), service_context.clone())
            .with_system_prompt(SYSTEM_PROMPT.to_string())
            .with_tool_registry(Arc::new(tool_registry))
            .with_approval_callback(Some(approval_callback))
            .with_max_tool_iterations(20)
            .with_working_directory(working_directory),
    );

    // Update app with the configured agent service (preserve event channels!)
    app.set_agent_service(agent_service);

    // Run TUI
    tracing::debug!("Launching TUI");
    tui::run(app).await.context("TUI error")?;

    println!("\n👋 Goodbye!");

    Ok(())
}

/// Whether the given Auto Mode level should approve `tool_name` without
/// prompting. Extracted as a pure function (rather than left inline in the
/// approval callback closure) specifically so this security-relevant
/// decision has direct unit test coverage independent of the surrounding
/// channel/TUI plumbing.
///
/// - `Interactive`: never bypasses - always prompts.
/// - `AutoPlan`: bypasses everything except high-risk tools
///   (`PlanModeState::is_high_risk_tool`: `bash`, `write_file`,
///   `edit_file`, `code_exec`), which still prompt.
/// - `FullAuto`: bypasses everything, including high-risk tools.
///
/// Callers must not treat this as the only safety layer: the
/// `SecurityConfig` policy chain (`deny_tools`/`deny_paths`/`allow_bash`)
/// is installed via `ToolRegistry::set_policy` and evaluated independently in
/// `ToolRegistry::execute()`, so it stays enforced regardless of this
/// function's result. A `Deny` there overrides any bypass decided here.
fn auto_mode_bypasses_approval(mode: &crate::config::PlanExecMode, tool_name: &str) -> bool {
    use crate::config::PlanExecMode;
    use crate::plan::PlanModeState;

    match mode {
        PlanExecMode::Interactive => false,
        PlanExecMode::AutoPlan => !PlanModeState::is_high_risk_tool(tool_name),
        PlanExecMode::FullAuto => true,
    }
}

/// Run a single command non-interactively
async fn cmd_run(
    config: &crate::config::Config,
    prompt: String,
    auto_approve: bool,
    format: OutputFormat,
) -> Result<()> {
    use crate::{
        db::Database,
        llm::agent::AgentService,
        services::{ServiceContext, SessionService},
    };

    tracing::info!("Running non-interactive command: {}", prompt);

    // Initialize database
    let db = Database::connect(&config.database.path).await?;
    db.run_migrations().await?;

    // Select provider based on configuration using factory
    let provider = crate::llm::provider::create_provider(config)?;

    // Create tool registry (same built-in set as the interactive chat agent
    // - see build_tool_registry's doc comment).
    let mut tool_registry = build_tool_registry();

    // Connect to configured MCP servers, same as cmd_chat - see its
    // comment for why this needs to happen at all (config.mcp.servers was
    // previously parsed but never consumed). No status snapshot needed
    // here since there's no TUI to display it in.
    for server in &config.mcp.servers {
        let args: Vec<&str> = server.args.iter().map(String::as_str).collect();
        match tool_registry
            .register_mcp_server(&server.name, &server.command, &args)
            .await
        {
            Ok(tool_count) => {
                tracing::info!(
                    "Connected to MCP server '{}' ({} tools)",
                    server.name,
                    tool_count
                );
            }
            Err(e) => {
                tracing::warn!("Failed to connect to MCP server '{}': {}", server.name, e);
            }
        }
    }

    // Install the [security] policy chain, as in the interactive path above.
    tool_registry.set_policy(config.security.to_policy().into());

    // Create service context and agent service
    let service_context = ServiceContext::new(db.pool().clone());
    let agent_service = AgentService::new(provider.clone(), service_context.clone())
        .with_tool_registry(Arc::new(tool_registry))
        .with_system_prompt(SYSTEM_PROMPT.to_string())
        .with_max_tool_iterations(20)
        .with_auto_approve_tools(auto_approve);

    // Create or get session
    let session_service = SessionService::new(service_context);

    let session = session_service
        .create_session(Some("CLI Run".to_string()))
        .await?;

    // Send message. Must go through the tool-aware path: the service above is
    // built with a tool registry, an iteration cap, and an auto-approve flag,
    // none of which `send_message` consults - it would send `tools=0` and the
    // model could never call anything.
    println!("🤔 Processing...\n");
    let response = agent_service
        .send_message_with_tools(session.id, prompt, None)
        .await?;

    // Format and display output
    match format {
        OutputFormat::Text => {
            println!("{}", response.content);
            println!();
            println!(
                "📊 Tokens: {}",
                response.usage.input_tokens + response.usage.output_tokens
            );
            println!("💰 Cost: ${:.6}", response.cost);
        }
        OutputFormat::Json => {
            let output = serde_json::json!({
                "content": response.content,
                "usage": {
                    "input_tokens": response.usage.input_tokens,
                    "output_tokens": response.usage.output_tokens,
                },
                "cost": response.cost,
                "model": response.model,
            });
            println!("{}", serde_json::to_string_pretty(&output)?);
        }
        OutputFormat::Markdown => {
            println!("# Response\n");
            println!("{}\n", response.content);
            println!("---");
            println!(
                "**Tokens:** {}",
                response.usage.input_tokens + response.usage.output_tokens
            );
            println!("**Cost:** ${:.6}", response.cost);
        }
    }

    if auto_approve {
        println!("\n⚠️  Auto-approve mode was enabled");
    }

    Ok(())
}

/// Keyring management commands
async fn cmd_keyring(operation: KeyringCommands) -> Result<()> {
    use crate::config::secrets::SecretString;

    match operation {
        KeyringCommands::Set { provider, api_key } => {
            println!("🔐 Saving API key for {} to OS keyring...\n", provider);

            let secret = SecretString::from_str(&api_key);
            let key_name = format!("{}_api_key", provider.to_lowercase());

            secret
                .save_to_keyring(&key_name)
                .with_context(|| format!("Failed to save {} API key to keyring", provider))?;

            println!("✅ Successfully saved {} API key to OS keyring", provider);
            println!("\n💡 The key is now securely stored in your system's credential manager:");
            #[cfg(target_os = "windows")]
            println!("   - Windows Credential Manager");
            #[cfg(target_os = "macos")]
            println!("   - macOS Keychain");
            #[cfg(target_os = "linux")]
            println!("   - Linux Secret Service");

            println!("\n🔒 Security benefits:");
            println!("   ✓ Encrypted by the operating system");
            println!("   ✓ Not stored in plaintext files");
            println!("   ✓ Automatically cleared from memory");

            Ok(())
        }

        KeyringCommands::Get { provider } => {
            let key_name = format!("{}_api_key", provider.to_lowercase());

            match SecretString::from_keyring_optional(&key_name) {
                Some(secret) => {
                    println!("🔐 API key for {}: {}", provider, secret.expose_secret());
                    println!(
                        "\n⚠️  Warning: API key displayed in plain text. Clear your terminal history."
                    );
                }
                None => {
                    println!("❌ No API key found for {} in OS keyring", provider);
                    println!("\n💡 To store an API key, use:");
                    println!("   crustly keyring set {} YOUR_API_KEY", provider);
                }
            }

            Ok(())
        }

        KeyringCommands::Delete { provider } => {
            let key_name = format!("{}_api_key", provider.to_lowercase());

            SecretString::delete_from_keyring(&key_name)
                .with_context(|| format!("Failed to delete {} API key from keyring", provider))?;

            println!("✅ Deleted {} API key from OS keyring", provider);
            Ok(())
        }

        KeyringCommands::List => {
            println!("🔐 API Keys in OS Keyring\n");

            let providers = ["anthropic", "openai", "gemini", "azure"];
            let mut found_any = false;

            for provider in &providers {
                let key_name = format!("{}_api_key", provider);
                if let Some(secret) = SecretString::from_keyring_optional(&key_name) {
                    let masked = format!(
                        "{}...{}",
                        &secret.expose_secret()[..4.min(secret.len())],
                        if secret.len() > 8 {
                            &secret.expose_secret()[secret.len() - 4..]
                        } else {
                            ""
                        }
                    );
                    println!("  ✓ {:<12} {}", provider, masked);
                    found_any = true;
                } else {
                    println!("  ✗ {:<12} (not configured)", provider);
                }
            }

            if !found_any {
                println!("\n💡 No API keys found in keyring.");
                println!("   To store an API key, use:");
                println!("   crustly keyring set <provider> <api-key>");
            }

            Ok(())
        }
    }
}

/// Resolve the configured Ollama host, falling back to the local default.
/// Used both by `crustly ollama <...>` and to point the TUI's Model
/// Download dialog (Ctrl+D) at the right instance.
fn ollama_host(config: &crate::config::Config) -> String {
    config
        .providers
        .ollama
        .as_ref()
        .map(|c| c.host.clone())
        .unwrap_or_else(|| "http://localhost:11434".to_string())
}

#[cfg(feature = "ollama")]
async fn cmd_ollama(config: &crate::config::Config, operation: OllamaCommands) -> Result<()> {
    use crate::llm::provider::ollama_models;

    let host = ollama_host(config);

    match operation {
        OllamaCommands::List => {
            println!("🦙 Models installed at {}\n", host);
            let models = ollama_models::list_models(&host).await?;

            if models.is_empty() {
                println!("  (none) - pull one with: crustly ollama pull <model>");
            } else {
                for m in models {
                    let size_gb = m.size_bytes as f64 / 1_073_741_824.0;
                    println!("  {:<30} {:>7.2} GB   {}", m.name, size_gb, m.modified_at);
                }
            }
            Ok(())
        }

        OllamaCommands::Pull { model } => {
            println!("🦙 Pulling '{}' from {}...\n", model, host);

            let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();
            let host_clone = host.clone();
            let model_clone = model.clone();
            let pull_task = tokio::spawn(async move {
                ollama_models::pull_model(&host_clone, &model_clone, tx).await
            });

            let mut last_status = String::new();
            while let Some(progress) = rx.recv().await {
                if progress.status != last_status {
                    println!("  {}", progress.status);
                    last_status = progress.status.clone();
                }
                if let Some(fraction) = progress.fraction() {
                    print!("\r  {:.0}%", fraction * 100.0);
                    use std::io::Write as _;
                    let _ = std::io::stdout().flush();
                }
            }
            println!();

            pull_task
                .await
                .context("Pull task panicked")?
                .with_context(|| format!("Failed to pull model '{}'", model))?;

            println!("✅ Pulled '{}'", model);
            Ok(())
        }

        OllamaCommands::Rm { model } => {
            ollama_models::delete_model(&host, &model).await?;
            println!("✅ Deleted '{}'", model);
            Ok(())
        }

        OllamaCommands::Show { model } => {
            let info = ollama_models::show_model(&host, &model).await?;
            println!("🦙 {}\n", model);
            println!("License:\n{}\n", info.license);
            println!("Parameters:\n{}\n", info.parameters);
            println!("Template:\n{}\n", info.template);
            println!("Capabilities: {}", info.capabilities.join(", "));
            Ok(())
        }

        OllamaCommands::Embed { model, text } => {
            let embeddings = ollama_models::generate_embeddings(&host, &model, vec![text]).await?;
            let embedding = embeddings
                .into_iter()
                .next()
                .context("Ollama returned no embedding vector")?;

            println!(
                "🦙 Embedding from '{}' ({} dimensions)\n",
                model,
                embedding.len()
            );
            let preview: Vec<String> = embedding
                .iter()
                .take(8)
                .map(|v| format!("{v:.4}"))
                .collect();
            println!("[{}, ...]", preview.join(", "));
            Ok(())
        }
    }
}

#[cfg(not(feature = "ollama"))]
async fn cmd_ollama(_config: &crate::config::Config, _operation: OllamaCommands) -> Result<()> {
    anyhow::bail!(
        "{FEATURE_NOT_COMPILED_ERROR_PREFIX} the 'ollama' feature. \
         Rebuild with `--features ollama` (or `all-llm`) to use `crustly ollama`."
    );
}

/// Resolve a user-supplied `model` argument (from `crustly llama-cpp rm`)
/// to a path: an absolute path or one containing a separator is used as-is,
/// otherwise it's treated as a filename inside `models_dir`.
#[cfg(feature = "gguf-management")]
fn resolve_llama_cpp_model_path(models_dir: &std::path::Path, model: &str) -> std::path::PathBuf {
    let candidate = std::path::Path::new(model);
    if candidate.is_absolute() || model.contains(std::path::MAIN_SEPARATOR) {
        candidate.to_path_buf()
    } else {
        models_dir.join(model)
    }
}

#[cfg(feature = "gguf-management")]
async fn cmd_llama_cpp(config: &crate::config::Config, operation: LlamaCppCommands) -> Result<()> {
    use crate::llm::provider::llama_cpp_models;

    let models_dir = config.providers.llama_cpp_models_dir();

    match operation {
        LlamaCppCommands::List { json, best_fit } => {
            let extra_model_paths = config.providers.llama_cpp_extra_model_paths();
            let ollama_models_dir = config.providers.llama_cpp_ollama_models_dir();
            let mut models = llama_cpp_models::list_all_local_models(
                &models_dir,
                &extra_model_paths,
                ollama_models_dir.as_deref(),
            )?;

            // Detection spawns subprocesses - only paid for when actually
            // asked for (`hardware_detect`'s own module doc), never as a
            // side effect of a plain `list`.
            let budget_bytes = if best_fit {
                crate::llm::provider::hardware_detect::detect_hardware().budget_bytes()
            } else {
                None
            };
            if best_fit {
                llama_cpp_models::sort_by_fit(&mut models, budget_bytes);
            }

            if json {
                let mut payload = LlamaCppListJson {
                    schema_version: LLAMA_CPP_LIST_JSON_SCHEMA_VERSION,
                    models: models.iter().map(LlamaCppModelJson::from).collect(),
                };
                if best_fit {
                    for (m, j) in models.iter().zip(payload.models.iter_mut()) {
                        j.fit = Some(
                            hardware_fit_label(llama_cpp_models::hardware_fit(
                                m.estimated_memory_bytes,
                                budget_bytes,
                            ))
                            .to_string(),
                        );
                    }
                }
                println!(
                    "{}",
                    serde_json::to_string_pretty(&payload)
                        .context("Failed to serialize model list as JSON")?
                );
                return Ok(());
            }

            println!("🦙 Local .gguf models\n");
            if models.is_empty() {
                println!("  (none) - pull one with: crustly llama-cpp pull <source>");
            } else {
                for m in models {
                    let size_gb = m.size_bytes as f64 / 1_073_741_824.0;
                    let quant = m.quantization_hint.as_deref().unwrap_or("unknown");
                    let mut name = m.display_name.clone().unwrap_or_else(|| {
                        m.path
                            .file_name()
                            .map(|n| n.to_string_lossy().into_owned())
                            .unwrap_or_default()
                    });
                    // A paired base model's projector is folded into this
                    // entry (its own row was removed by `pair_mmproj_files`);
                    // an unpaired projector keeps its own row, labeled so
                    // it's not just a mysteriously-named model.
                    if m.mmproj_path.is_some() {
                        name.push_str(" (+ mmproj)");
                    } else if m.is_mmproj {
                        name.push_str(" [mmproj]");
                    }
                    let memory = match m.estimated_memory_bytes {
                        Some(bytes) => {
                            let gb = bytes as f64 / 1_073_741_824.0;
                            if m.estimated_memory_includes_kv_cache {
                                format!("~{gb:.1} GB")
                            } else {
                                format!("~{gb:.1} GB (weights only)")
                            }
                        }
                        None => "-".to_string(),
                    };
                    print!(
                        "  {:<45} {:>7.2} GB   {:<10} {:<22} {}",
                        name, size_gb, quant, memory, m.modified_at
                    );
                    if best_fit {
                        let fit =
                            llama_cpp_models::hardware_fit(m.estimated_memory_bytes, budget_bytes);
                        match m.estimated_memory_context_length {
                            Some(ctx) => {
                                print!("   {} (ctx {ctx})", hardware_fit_label(fit))
                            }
                            None => print!("   {}", hardware_fit_label(fit)),
                        }
                    }
                    println!();
                }
                if best_fit && budget_bytes.is_none() {
                    println!(
                        "\n  (hardware detection found no usable GPU/RAM reading on this \
                         machine - every entry above is \"unknown\")"
                    );
                }
            }
            Ok(())
        }

        LlamaCppCommands::Pull { source } => {
            println!("🦙 Resolving '{}'...\n", source);
            let (url, expected_sha256) = llama_cpp_models::resolve_download_source(&source).await?;
            if expected_sha256.is_none() {
                println!(
                    "⚠️  No integrity hash available for this download - the file will \
                     not be checksum-verified.\n"
                );
            }
            println!("Downloading {}...\n", url);

            let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();
            let models_dir_clone = models_dir.clone();
            let pull_task = tokio::spawn(async move {
                llama_cpp_models::download_model(
                    &url,
                    &models_dir_clone,
                    expected_sha256.as_deref(),
                    tx,
                )
                .await
            });

            let mut last_pct: i64 = -1;
            while let Some(progress) = rx.recv().await {
                use std::io::Write as _;
                if let Some(fraction) = progress.fraction() {
                    let pct = (fraction * 100.0) as i64;
                    if pct != last_pct {
                        print!("\r  {pct}%");
                        let _ = std::io::stdout().flush();
                        last_pct = pct;
                    }
                } else {
                    print!("\r  {} bytes", progress.bytes_downloaded);
                    let _ = std::io::stdout().flush();
                }
            }
            println!();

            let path = pull_task
                .await
                .context("Download task panicked")?
                .with_context(|| format!("Failed to download '{}'", source))?;

            println!("✅ Downloaded to {}", path.display());
            Ok(())
        }

        LlamaCppCommands::Rm { name } => {
            let path = resolve_llama_cpp_model_path(&models_dir, &name);
            if !path.exists() {
                anyhow::bail!("{NO_SUCH_FILE_ERROR_PREFIX} {}", path.display());
            }

            let size_gb = std::fs::metadata(&path)
                .map(|m| m.len() as f64 / 1_073_741_824.0)
                .unwrap_or(0.0);
            print!("Delete '{}' ({:.2} GB)? [y/N] ", path.display(), size_gb);
            use std::io::Write as _;
            std::io::stdout().flush().ok();
            let mut answer = String::new();
            std::io::stdin().read_line(&mut answer).ok();
            if !answer.trim().eq_ignore_ascii_case("y") {
                println!("Cancelled.");
                return Ok(());
            }

            llama_cpp_models::delete_model(&path)?;
            println!("✅ Deleted '{}'", path.display());
            Ok(())
        }

        LlamaCppCommands::Doctor => {
            println!("🩺 crustly llama-cpp doctor\n");
            let findings = llama_cpp_doctor_findings(config, &models_dir);
            let (mut ok, mut warn, mut fail) = (0, 0, 0);
            for f in &findings {
                match f.status {
                    DoctorStatus::Ok => ok += 1,
                    DoctorStatus::Warn => warn += 1,
                    DoctorStatus::Fail => fail += 1,
                }
                println!("  {} {}", f.status.icon(), f.message);
            }
            println!("\n{ok} ok, {warn} warning(s), {fail} failure(s)");
            // Always exits 0 - this reports findings, it doesn't fail like
            // `list`/`pull`/`rm` can (see the subcommand's own doc comment).
            Ok(())
        }
    }
}

/// A single `crustly llama-cpp doctor` finding's severity - not a hard
/// pass/fail gate, since `doctor` always exits 0 regardless (structured
/// findings for a human/agent to read, not a check the process itself
/// lives or dies by). Gated on `gguf-management`, same as `Doctor`'s real
/// implementation - the not-compiled-in stub never constructs one.
#[cfg(feature = "gguf-management")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DoctorStatus {
    Ok,
    Warn,
    Fail,
}

#[cfg(feature = "gguf-management")]
impl DoctorStatus {
    fn icon(self) -> &'static str {
        match self {
            Self::Ok => "✅",
            Self::Warn => "⚠️ ",
            Self::Fail => "❌",
        }
    }
}

#[cfg(feature = "gguf-management")]
#[derive(Debug, Clone, PartialEq, Eq)]
struct DoctorFinding {
    status: DoctorStatus,
    message: String,
}

/// Computes `doctor`'s findings - a pure function over `config`/
/// `models_dir` (plus the filesystem/disk they point at) so the checklist
/// logic is testable without going through the CLI dispatch machinery.
/// Every check degrades to a `Warn`, never panics or errors the whole
/// function, on anything it can't determine - same "honest unknown, not a
/// crash" posture the rest of this plan's phases already established.
#[cfg(feature = "gguf-management")]
fn llama_cpp_doctor_findings(
    config: &crate::config::Config,
    models_dir: &std::path::Path,
) -> Vec<DoctorFinding> {
    use crate::llm::provider::llama_cpp_models;

    let mut findings = vec![DoctorFinding {
        status: DoctorStatus::Ok,
        message: "gguf-management feature compiled in - list/pull/rm/doctor available".to_string(),
    }];

    if cfg!(feature = "llama-cpp") {
        findings.push(DoctorFinding {
            status: DoctorStatus::Ok,
            message: "llama-cpp feature compiled in - in-process inference available".to_string(),
        });
        let gpu_backend_compiled_in = cfg!(any(
            feature = "llama-cpp-cuda",
            feature = "llama-cpp-metal",
            feature = "llama-cpp-vulkan",
            feature = "llama-cpp-rocm",
            feature = "llama-cpp-opencl",
            feature = "llama-cpp-mkl",
        ));
        findings.push(if gpu_backend_compiled_in {
            DoctorFinding {
                status: DoctorStatus::Ok,
                message: "a GPU backend feature is compiled in".to_string(),
            }
        } else {
            DoctorFinding {
                status: DoctorStatus::Warn,
                message: "no GPU backend feature compiled in - n_gpu_layers > 0 in config \
                          will be a silent no-op (CPU only)"
                    .to_string(),
            }
        });
    } else {
        findings.push(DoctorFinding {
            status: DoctorStatus::Warn,
            message: "llama-cpp feature not compiled in - in-process inference unavailable; \
                      management (list/pull/rm) still works"
                .to_string(),
        });
    }

    if !models_dir.exists() {
        findings.push(DoctorFinding {
            status: DoctorStatus::Warn,
            message: format!(
                "models_dir does not exist yet: {} (created automatically on first `pull`)",
                models_dir.display()
            ),
        });
    } else if !models_dir.is_dir() {
        findings.push(DoctorFinding {
            status: DoctorStatus::Fail,
            message: format!(
                "models_dir exists but is not a directory: {}",
                models_dir.display()
            ),
        });
    } else {
        let probe = models_dir.join(".crustly-doctor-write-test");
        match std::fs::write(&probe, b"x") {
            Ok(()) => {
                let _ = std::fs::remove_file(&probe);
                findings.push(DoctorFinding {
                    status: DoctorStatus::Ok,
                    message: format!(
                        "models_dir exists and is writable: {}",
                        models_dir.display()
                    ),
                });
            }
            Err(e) => {
                findings.push(DoctorFinding {
                    status: DoctorStatus::Fail,
                    message: format!(
                        "models_dir exists but is not writable: {} ({e})",
                        models_dir.display()
                    ),
                });
            }
        }
    }

    findings.push(match llama_cpp_models::available_space_at(models_dir) {
        Some(bytes) => DoctorFinding {
            status: DoctorStatus::Ok,
            message: format!(
                "~{:.1} GB free at models_dir's filesystem",
                bytes as f64 / 1_073_741_824.0
            ),
        },
        None => DoctorFinding {
            status: DoctorStatus::Warn,
            message: "could not determine free disk space at models_dir".to_string(),
        },
    });

    for path in config.providers.llama_cpp_extra_model_paths() {
        findings.push(if path.exists() {
            DoctorFinding {
                status: DoctorStatus::Ok,
                message: format!("extra_model_paths entry exists: {}", path.display()),
            }
        } else {
            DoctorFinding {
                status: DoctorStatus::Warn,
                message: format!(
                    "extra_model_paths entry does not exist: {} (typo, or not mounted yet?)",
                    path.display()
                ),
            }
        });
    }

    if let Some(ollama_dir) = config.providers.llama_cpp_ollama_models_dir() {
        findings.push(if ollama_dir.join("manifests").exists() {
            DoctorFinding {
                status: DoctorStatus::Ok,
                message: format!(
                    "scan_ollama_models is on and a manifest tree was found: {}",
                    ollama_dir.display()
                ),
            }
        } else {
            DoctorFinding {
                status: DoctorStatus::Warn,
                message: format!(
                    "scan_ollama_models is on but no manifests found at: {} (has Ollama \
                     pulled anything, or is $OLLAMA_MODELS set correctly?)",
                    ollama_dir.display()
                ),
            }
        });
    }

    // Phase M11/M12: surface the detected hardware as text - this is the
    // natural home for that output (M9's own doc comment already called it
    // out), not a reason to duplicate detection logic here. Always `Ok` -
    // "CPU-only, RAM unknown" is a legitimate, non-fatal reading on a
    // machine/CI image with no vendor tools installed, same as every other
    // "unknown" this function already reports without failing.
    let hardware = crate::llm::provider::hardware_detect::detect_hardware();
    let hardware_message = match &hardware.gpu {
        Some(gpu) => {
            let name = gpu.name.as_deref().unwrap_or("unknown GPU");
            match gpu.vram_available_bytes() {
                Some(vram) => format!(
                    "detected GPU: {name} (~{:.1} GB VRAM available)",
                    vram as f64 / 1_073_741_824.0
                ),
                None => format!("detected GPU: {name} (VRAM unknown)"),
            }
        }
        None => "no GPU detected (or no supported vendor tool installed) - CPU-only".to_string(),
    };
    let ram_message = match hardware.system_ram_total_bytes {
        Some(bytes) => format!("system RAM: ~{:.1} GB", bytes as f64 / 1_073_741_824.0),
        None => "system RAM: unknown".to_string(),
    };
    findings.push(DoctorFinding {
        status: DoctorStatus::Ok,
        message: format!("{hardware_message}; {ram_message}"),
    });

    // Best-effort - a models scan failure here (e.g. models_dir genuinely
    // unreadable, already reported above) just means this bonus finding is
    // skipped, not that `doctor` itself fails.
    if let Ok(models) = llama_cpp_models::list_all_local_models(
        models_dir,
        &config.providers.llama_cpp_extra_model_paths(),
        config.providers.llama_cpp_ollama_models_dir().as_deref(),
    ) {
        if !models.is_empty() {
            let budget_bytes = hardware.budget_bytes();
            findings.push(
                match llama_cpp_models::best_fitting_model(&models, budget_bytes) {
                    Some(best) => {
                        let name = best.display_name.clone().unwrap_or_else(|| {
                            best.path
                                .file_name()
                                .map(|n| n.to_string_lossy().into_owned())
                                .unwrap_or_default()
                        });
                        let gb = best.estimated_memory_bytes.unwrap_or(0) as f64 / 1_073_741_824.0;
                        DoctorFinding {
                            status: DoctorStatus::Ok,
                            message: format!(
                                "largest already-downloaded model this hardware can hold: \
                             {name} (~{gb:.1} GB estimated)"
                            ),
                        }
                    }
                    None if budget_bytes.is_some() => DoctorFinding {
                        status: DoctorStatus::Warn,
                        message: "no already-downloaded model comfortably fits this hardware's \
                              detected budget - consider a smaller quantization"
                            .to_string(),
                    },
                    None => DoctorFinding {
                        status: DoctorStatus::Warn,
                        message: "could not determine a hardware budget to compare downloaded \
                              models against"
                            .to_string(),
                    },
                },
            );
        }
    }

    findings
}

#[cfg(not(feature = "gguf-management"))]
async fn cmd_llama_cpp(
    _config: &crate::config::Config,
    _operation: LlamaCppCommands,
) -> Result<()> {
    anyhow::bail!(
        "{FEATURE_NOT_COMPILED_ERROR_PREFIX} the 'gguf-management' feature. \
         Rebuild with `--features gguf-management` to use `crustly llama-cpp`."
    );
}

/// FullAuto plan mode: no approval gates, runs to completion or max_iterations.
async fn cmd_autoplan(
    config: &crate::config::Config,
    goal: String,
    max_iterations: u32,
) -> Result<()> {
    use crate::plan::PlanModeState;

    println!("🤖 Crustly AutoPlan — FullAuto mode");
    println!("Goal: {}", goal);
    println!("Max iterations: {}", max_iterations);
    println!();

    let state = PlanModeState::FullAuto {
        goal: goal.clone(),
        iteration: 0,
        max_iterations,
    };

    // Launch the non-interactive agent runner with FullAuto state.
    // TODO: wire into AgentService once that layer supports session injection.
    // For now, fall back to the existing `run` path with auto-approve.
    let _ = state; // suppress unused warning until wired
    cmd_run(config, goal, true, OutputFormat::Text).await
}

/// Log management commands
async fn cmd_logs(operation: LogCommands) -> Result<()> {
    use crate::logging;
    use std::io::{BufRead, BufReader};

    let log_dir = std::env::current_dir()?.join(".crustly").join("logs");

    match operation {
        LogCommands::Status => {
            println!("📊 Crustly Logging Status\n");
            println!("Log directory: {}", log_dir.display());

            if log_dir.exists() {
                // Count log files and total size
                let mut file_count = 0;
                let mut total_size = 0u64;
                let mut newest_file: Option<std::path::PathBuf> = None;
                let mut newest_time = std::time::UNIX_EPOCH;

                for entry in std::fs::read_dir(&log_dir)? {
                    let entry = entry?;
                    let path = entry.path();
                    if path.extension().map(|e| e == "log").unwrap_or(false) {
                        file_count += 1;
                        if let Ok(metadata) = entry.metadata() {
                            total_size += metadata.len();
                            if let Ok(modified) = metadata.modified() {
                                if modified > newest_time {
                                    newest_time = modified;
                                    newest_file = Some(path);
                                }
                            }
                        }
                    }
                }

                println!("Status: ✅ Active");
                println!("Log files: {}", file_count);
                println!(
                    "Total size: {:.2} MB",
                    total_size as f64 / (1024.0 * 1024.0)
                );

                if let Some(newest) = newest_file {
                    println!("Latest log: {}", newest.display());
                }

                println!("\n💡 To enable debug logging, run with -d flag:");
                println!("   crustly -d");
            } else {
                println!("Status: ❌ No logs found");
                println!("\n💡 To enable debug logging, run with -d flag:");
                println!("   crustly -d");
                println!("\nThis will create log files in:");
                println!("   {}", log_dir.display());
            }

            Ok(())
        }

        LogCommands::View { lines } => {
            if let Some(log_path) = logging::get_log_path() {
                println!(
                    "📜 Viewing last {} lines of: {}\n",
                    lines,
                    log_path.display()
                );

                let file = std::fs::File::open(&log_path)?;
                let reader = BufReader::new(file);

                // Collect all lines then show last N
                let all_lines: Vec<String> = reader.lines().map_while(Result::ok).collect();
                let start = all_lines.len().saturating_sub(lines);

                for line in &all_lines[start..] {
                    println!("{}", line);
                }

                if all_lines.is_empty() {
                    println!("(empty log file)");
                }
            } else {
                println!("❌ No log files found.\n");
                println!("💡 Run Crustly with -d flag to enable debug logging:");
                println!("   crustly -d");
            }

            Ok(())
        }

        LogCommands::Clean { days } => {
            println!("🧹 Cleaning up log files older than {} days...\n", days);

            match logging::cleanup_old_logs(days) {
                Ok(removed) => {
                    if removed > 0 {
                        println!("✅ Removed {} old log file(s)", removed);
                    } else {
                        println!("✅ No old log files to remove");
                    }
                }
                Err(e) => {
                    println!("❌ Error cleaning logs: {}", e);
                }
            }

            Ok(())
        }

        LogCommands::Open => {
            if !log_dir.exists() {
                println!("❌ Log directory does not exist: {}", log_dir.display());
                println!("\n💡 Run Crustly with -d flag to enable debug logging:");
                println!("   crustly -d");
                return Ok(());
            }

            println!("📂 Opening log directory: {}", log_dir.display());

            #[cfg(target_os = "macos")]
            {
                std::process::Command::new("open")
                    .arg(&log_dir)
                    .spawn()
                    .context("Failed to open directory")?;
            }

            #[cfg(target_os = "linux")]
            {
                std::process::Command::new("xdg-open")
                    .arg(&log_dir)
                    .spawn()
                    .context("Failed to open directory")?;
            }

            #[cfg(target_os = "windows")]
            {
                std::process::Command::new("explorer")
                    .arg(&log_dir)
                    .spawn()
                    .context("Failed to open directory")?;
            }

            Ok(())
        }

        LogCommands::ShowCacheStats => {
            println!("📊 Prompt Cache Statistics\n");
            println!(
                "Session cache metrics are accumulated in AgentContext during a live session."
            );
            println!("Start a chat session with -d to see per-turn cache telemetry in the log.");
            println!(
                "\nLog location: {}",
                std::env::current_dir()?
                    .join(".crustly")
                    .join("logs")
                    .display()
            );
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_parse() {
        use clap::CommandFactory;
        Cli::command().debug_assert();
    }

    #[test]
    fn test_ollama_host_defaults_when_unconfigured() {
        let config = crate::config::Config::default();
        assert_eq!(ollama_host(&config), "http://localhost:11434");
    }

    #[test]
    fn test_ollama_host_uses_configured_value() {
        let mut config = crate::config::Config::default();
        config.providers.ollama = Some(crate::config::OllamaProviderConfig {
            enabled: true,
            host: "http://my-ollama-box:11434".to_string(),
            ..Default::default()
        });
        assert_eq!(ollama_host(&config), "http://my-ollama-box:11434");
    }

    #[test]
    fn auto_mode_bypasses_approval_interactive_never_bypasses() {
        use crate::config::PlanExecMode;

        for tool in [
            "read_file",
            "grep",
            "bash",
            "write_file",
            "edit_file",
            "code_exec",
        ] {
            assert!(
                !auto_mode_bypasses_approval(&PlanExecMode::Interactive, tool),
                "Interactive must always prompt, including for {tool}"
            );
        }
    }

    #[test]
    fn auto_mode_bypasses_approval_autoplan_gates_high_risk_tools_only() {
        use crate::config::PlanExecMode;

        // Low-risk tools: bypassed without a prompt.
        for tool in [
            "read_file",
            "grep",
            "glob",
            "ls",
            "web_search",
            "todo_write",
        ] {
            assert!(
                auto_mode_bypasses_approval(&PlanExecMode::AutoPlan, tool),
                "AutoPlan should bypass the low-risk tool {tool}"
            );
        }

        // High-risk tools: this is the load-bearing safety property of
        // AutoPlan - these must still prompt.
        for tool in ["bash", "write_file", "edit_file", "code_exec"] {
            assert!(
                !auto_mode_bypasses_approval(&PlanExecMode::AutoPlan, tool),
                "AutoPlan must still prompt for the high-risk tool {tool}"
            );
        }
    }

    #[test]
    fn known_gap_powershell_is_not_classified_as_high_risk() {
        use crate::config::PlanExecMode;

        // Documents a pre-existing gap (flagged as an open decision in
        // ergonomy-improvment.md, not something introduced or silently
        // fixed here): PlanModeState::is_high_risk_tool() only lists
        // bash/write_file/edit_file/code_exec, so PowerShell - an
        // equally-capable arbitrary command execution tool on Windows -
        // is bypassed by AutoPlan the same as a read-only tool. Whether to
        // add it (and http/notebook) to the high-risk list is left as an
        // explicit follow-up decision rather than expanded here, since
        // that shared classification is also used by pre-existing plan
        // auto-run code this phase didn't otherwise touch.
        assert!(auto_mode_bypasses_approval(
            &PlanExecMode::AutoPlan,
            "powershell"
        ));
    }

    #[test]
    fn auto_mode_bypasses_approval_fullauto_bypasses_everything() {
        use crate::config::PlanExecMode;

        for tool in [
            "read_file",
            "grep",
            "bash",
            "write_file",
            "edit_file",
            "code_exec",
            "powershell",
        ] {
            assert!(
                auto_mode_bypasses_approval(&PlanExecMode::FullAuto, tool),
                "FullAuto should bypass every tool, including {tool}"
            );
        }
    }

    #[test]
    fn test_ollama_command_parses() {
        let cli = Cli::parse_from(["crustly", "ollama", "pull", "qwen2.5-coder:7b"]);
        match cli.command {
            Some(Commands::Ollama {
                operation: OllamaCommands::Pull { model },
            }) => assert_eq!(model, "qwen2.5-coder:7b"),
            other => panic!("expected Ollama Pull command, got {other:?}"),
        }
    }

    #[test]
    fn test_llama_cpp_command_parses() {
        let cli = Cli::parse_from([
            "crustly",
            "llama-cpp",
            "pull",
            "hf:TheBloke/Llama-2-7B-GGUF/llama-2-7b.Q4_K_M.gguf",
        ]);
        match cli.command {
            Some(Commands::LlamaCpp {
                operation: LlamaCppCommands::Pull { source },
            }) => assert_eq!(source, "hf:TheBloke/Llama-2-7B-GGUF/llama-2-7b.Q4_K_M.gguf"),
            other => panic!("expected LlamaCpp Pull command, got {other:?}"),
        }
    }

    #[test]
    fn test_llama_cpp_list_and_rm_parse() {
        let cli = Cli::parse_from(["crustly", "llama-cpp", "list"]);
        assert!(matches!(
            cli.command,
            Some(Commands::LlamaCpp {
                operation: LlamaCppCommands::List {
                    json: false,
                    best_fit: false
                }
            })
        ));

        let cli = Cli::parse_from(["crustly", "llama-cpp", "list", "--json"]);
        assert!(matches!(
            cli.command,
            Some(Commands::LlamaCpp {
                operation: LlamaCppCommands::List {
                    json: true,
                    best_fit: false
                }
            })
        ));

        let cli = Cli::parse_from(["crustly", "llama-cpp", "list", "--best-fit"]);
        assert!(matches!(
            cli.command,
            Some(Commands::LlamaCpp {
                operation: LlamaCppCommands::List {
                    json: false,
                    best_fit: true
                }
            })
        ));

        let cli = Cli::parse_from(["crustly", "llama-cpp", "list", "--json", "--best-fit"]);
        assert!(matches!(
            cli.command,
            Some(Commands::LlamaCpp {
                operation: LlamaCppCommands::List {
                    json: true,
                    best_fit: true
                }
            })
        ));

        let cli = Cli::parse_from(["crustly", "llama-cpp", "rm", "model.gguf"]);
        match cli.command {
            Some(Commands::LlamaCpp {
                operation: LlamaCppCommands::Rm { name },
            }) => assert_eq!(name, "model.gguf"),
            other => panic!("expected LlamaCpp Rm command, got {other:?}"),
        }

        // Regression test for the actual bug this rename fixed: a bare
        // positional value must resolve to `Rm.name`, not silently be
        // captured by the global `--model` flag (which shares its ident
        // with the *old* field name and confused clap's arg matching -
        // reproduced independently of this test file, confirmed against
        // the real built binary before this fix).
        let cli = Cli::parse_from(["crustly", "llama-cpp", "rm", "some-model.gguf"]);
        assert!(
            cli.model.is_none(),
            "the positional Rm argument must not be captured by the global --model flag"
        );

        let cli = Cli::parse_from(["crustly", "llama-cpp", "doctor"]);
        assert!(matches!(
            cli.command,
            Some(Commands::LlamaCpp {
                operation: LlamaCppCommands::Doctor
            })
        ));
    }

    #[cfg(feature = "gguf-management")]
    #[test]
    fn llama_cpp_list_json_round_trips_with_the_documented_schema() {
        let model = LlamaCppModelJson {
            path: std::path::PathBuf::from("/models/a.gguf"),
            display_name: Some("qwen2.5-coder:7b".to_string()),
            size_bytes: 4_294_967_296,
            modified_at: "2026-08-07T13:38:28Z".to_string(),
            architecture: Some("qwen2".to_string()),
            parameter_count: Some(7_000_000_000),
            quantization: Some("Q4_K_M".to_string()),
            context_length: Some(32768),
            has_chat_template: true,
            estimated_memory_bytes: Some(5_200_000_000),
            estimated_memory_includes_kv_cache: true,
            estimated_memory_context_length: Some(8_192),
            is_mmproj: false,
            mmproj_path: None,
            fit: None,
        };
        let payload = LlamaCppListJson {
            schema_version: LLAMA_CPP_LIST_JSON_SCHEMA_VERSION,
            models: vec![model],
        };

        let json = serde_json::to_value(&payload).expect("must serialize");
        assert_eq!(json["schema_version"], 1);
        assert_eq!(json["models"][0]["display_name"], "qwen2.5-coder:7b");
        assert_eq!(json["models"][0]["parameter_count"], 7_000_000_000_u64);
        assert_eq!(json["models"][0]["has_chat_template"], true);
        assert_eq!(json["models"][0]["is_mmproj"], false);
        assert!(json["models"][0]["mmproj_path"].is_null());
        assert_eq!(json["models"][0]["estimated_memory_context_length"], 8192);
        // `fit` is only present when `--best-fit` was passed - omitted
        // entirely (not even `null`) on a plain `list --json`, per its
        // `#[serde(skip_serializing_if = "Option::is_none")]`.
        assert!(!json["models"][0].as_object().unwrap().contains_key("fit"));
    }

    #[cfg(feature = "gguf-management")]
    #[test]
    fn llama_cpp_model_json_from_local_gguf_model_carries_every_field() {
        use crate::llm::provider::llama_cpp_models::LocalGgufModel;

        let model = LocalGgufModel {
            path: std::path::PathBuf::from("/models/a.gguf"),
            size_bytes: 100,
            modified_at: "now".to_string(),
            quantization_hint: Some("Q4_K_M".to_string()),
            architecture: Some("qwen2".to_string()),
            parameter_count: Some(7_000_000_000),
            context_length: Some(32768),
            has_chat_template: true,
            display_name: Some("nice-name".to_string()),
            estimated_memory_bytes: Some(5_000_000_000),
            estimated_memory_includes_kv_cache: true,
            estimated_memory_context_length: Some(8_192),
            is_mmproj: false,
            mmproj_path: None,
        };
        let json = LlamaCppModelJson::from(&model);
        assert_eq!(json.quantization.as_deref(), Some("Q4_K_M"));
        assert_eq!(json.display_name.as_deref(), Some("nice-name"));
        assert_eq!(json.parameter_count, Some(7_000_000_000));
        assert_eq!(json.estimated_memory_context_length, Some(8_192));
        assert_eq!(
            json.fit, None,
            "From<&LocalGgufModel> never sets `fit` - only --best-fit's own \
             handler does, after conversion"
        );
    }

    #[test]
    fn llama_cpp_exit_code_maps_each_known_message_prefix() {
        let cases: &[(&str, i32)] = &[
            ("No such file: /models/x.gguf", 10),
            ("Not enough disk space to download 'x.gguf': ...", 11),
            ("Checksum mismatch for x.gguf: ...", 12),
            ("Failed to start download from https://example.com", 13),
            ("Download failed for https://example.com", 13),
            (
                "This build of crustly was compiled without the 'gguf-management' feature.",
                14,
            ),
        ];
        for (msg, expected) in cases {
            let err = anyhow::anyhow!("{msg}");
            assert_eq!(llama_cpp_exit_code(&err), *expected, "for message: {msg}");
        }
    }

    #[test]
    fn llama_cpp_exit_code_checks_the_whole_chain_not_just_the_outermost_context() {
        // Mirrors how `Pull` actually wraps its errors: the outermost
        // context is always "Failed to download '...'", so the specific
        // cause (checksum mismatch, here) is a layer deeper.
        let root = anyhow::anyhow!("Checksum mismatch for model.gguf: expected a, got b.");
        let wrapped: anyhow::Error = Err::<(), _>(root)
            .context("Failed to download 'model.gguf'")
            .unwrap_err();
        assert_eq!(llama_cpp_exit_code(&wrapped), 12);
    }

    #[test]
    fn llama_cpp_exit_code_falls_back_to_one_for_an_unrecognized_error() {
        let err = anyhow::anyhow!("some entirely new failure mode nobody has seen before");
        assert_eq!(llama_cpp_exit_code(&err), 1);
    }

    #[cfg(feature = "gguf-management")]
    #[test]
    fn llama_cpp_doctor_findings_reports_a_writable_models_dir_as_ok() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let config = crate::config::Config::default();

        let findings = llama_cpp_doctor_findings(&config, tmp.path());

        let models_dir_finding = findings
            .iter()
            .find(|f| f.message.contains("models_dir exists and is writable"))
            .expect("must report the writable models_dir");
        assert_eq!(models_dir_finding.status, DoctorStatus::Ok);
        // The write-probe file must not be left behind.
        assert!(!tmp.path().join(".crustly-doctor-write-test").exists());
    }

    #[cfg(feature = "gguf-management")]
    #[test]
    fn llama_cpp_doctor_findings_warns_on_a_missing_models_dir() {
        let config = crate::config::Config::default();
        let missing = std::path::Path::new("/definitely/does/not/exist/crustly-doctor-test");

        let findings = llama_cpp_doctor_findings(&config, missing);

        let finding = findings
            .iter()
            .find(|f| f.message.contains("does not exist yet"))
            .expect("must report the missing models_dir");
        assert_eq!(finding.status, DoctorStatus::Warn);
    }

    #[cfg(feature = "gguf-management")]
    #[test]
    fn llama_cpp_doctor_findings_reports_extra_model_paths_existence() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let mut config = crate::config::Config::default();
        config.providers.llama_cpp = Some(crate::config::LlamaCppProviderConfig {
            extra_model_paths: vec![
                tmp.path().to_path_buf(),
                std::path::PathBuf::from("/definitely/does/not/exist/crustly-extra"),
            ],
            ..Default::default()
        });

        let findings = llama_cpp_doctor_findings(&config, tmp.path());

        assert!(findings.iter().any(|f| f.status == DoctorStatus::Ok
            && f.message.contains("extra_model_paths entry exists")));
        assert!(findings.iter().any(|f| f.status == DoctorStatus::Warn
            && f.message.contains("extra_model_paths entry does not exist")));
    }

    #[cfg(feature = "gguf-management")]
    #[test]
    fn llama_cpp_doctor_findings_says_nothing_about_ollama_when_not_opted_in() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let config = crate::config::Config::default(); // scan_ollama_models defaults false

        let findings = llama_cpp_doctor_findings(&config, tmp.path());

        assert!(
            !findings
                .iter()
                .any(|f| f.message.contains("scan_ollama_models")),
            "opting out of Ollama scanning should produce no finding about it at all"
        );
    }

    #[cfg(feature = "gguf-management")]
    #[test]
    fn llama_cpp_doctor_findings_reports_build_features() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let config = crate::config::Config::default();

        let findings = llama_cpp_doctor_findings(&config, tmp.path());

        assert!(findings.iter().any(
            |f| f.message.contains("gguf-management feature compiled in")
                && f.status == DoctorStatus::Ok
        ));
    }

    #[cfg(feature = "gguf-management")]
    #[test]
    fn llama_cpp_doctor_findings_always_reports_detected_hardware() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let config = crate::config::Config::default();

        let findings = llama_cpp_doctor_findings(&config, tmp.path());

        // This sandbox has no nvidia-smi/rocm-smi/vulkaninfo installed, so
        // this exercises the real "no vendor tool found" degrade-to-
        // CPU-only path end-to-end, not a mocked one - the same "missing
        // tool -> unknown, never fatal" contract `hardware_detect`'s own
        // tests establish in isolation.
        let hardware_finding = findings
            .iter()
            .find(|f| f.message.contains("system RAM"))
            .expect("must report a hardware finding");
        assert_eq!(hardware_finding.status, DoctorStatus::Ok);
        assert!(
            hardware_finding.message.contains("GPU"),
            "expected the GPU half of the hardware finding, got: {}",
            hardware_finding.message
        );
    }

    #[cfg(feature = "gguf-management")]
    #[test]
    fn llama_cpp_doctor_findings_skips_the_best_fit_finding_when_no_models_present() {
        let tmp = tempfile::tempdir().expect("tempdir"); // empty - no .gguf files
        let config = crate::config::Config::default();

        let findings = llama_cpp_doctor_findings(&config, tmp.path());

        assert!(
            !findings
                .iter()
                .any(|f| f.message.contains("already-downloaded model")),
            "no local models means no best-fit bonus finding at all, not a Warn about it"
        );
    }

    #[cfg(feature = "gguf-management")]
    #[test]
    fn hardware_fit_label_matches_the_documented_wording() {
        use crate::llm::provider::llama_cpp_models::HardwareFit;
        assert_eq!(hardware_fit_label(HardwareFit::Fits), "Fits");
        assert_eq!(hardware_fit_label(HardwareFit::Tight), "Tight");
        assert_eq!(hardware_fit_label(HardwareFit::WontFit), "Won't fit");
        assert_eq!(hardware_fit_label(HardwareFit::Unknown), "unknown");
    }

    #[cfg(feature = "gguf-management")]
    #[test]
    fn resolve_llama_cpp_model_path_treats_bare_names_as_relative_to_models_dir() {
        let dir = std::path::Path::new("/models");
        assert_eq!(
            resolve_llama_cpp_model_path(dir, "model.gguf"),
            std::path::PathBuf::from("/models/model.gguf")
        );
        assert_eq!(
            resolve_llama_cpp_model_path(dir, "/elsewhere/model.gguf"),
            std::path::PathBuf::from("/elsewhere/model.gguf")
        );
    }

    #[test]
    fn build_tool_registry_registers_every_built_in_tool() {
        let registry = build_tool_registry();

        // One entry per Phase 1-4 tool registered in build_tool_registry.
        assert_eq!(registry.count(), 23);
        for name in [
            "read_file",
            "write_file",
            "edit_file",
            "apply_patch",
            "bash",
            "ls",
            "glob",
            "grep",
            "web_search",
            "execute_code",
            "notebook_edit",
            "parse_document",
            "task_manager",
            "session_context",
            "save_memory",
            "http_request",
            "plan",
            "web_fetch",
            "todo_write",
            "ask_user",
            "skill",
            "agent",
            "powershell",
        ] {
            assert!(registry.has_tool(name), "missing tool: {name}");
        }
    }

    #[tokio::test]
    async fn connect_configured_mcp_servers_returns_empty_status_with_no_servers() {
        let config = crate::config::Config::default();
        let mut registry = build_tool_registry();

        let status = connect_configured_mcp_servers(&mut registry, &config).await;

        assert!(status.is_empty());
    }

    #[tokio::test]
    async fn connect_configured_mcp_servers_records_failure_for_unreachable_server() {
        let mut config = crate::config::Config::default();
        config.mcp.servers.push(crate::config::McpServerConfig {
            name: "broken".to_string(),
            command: "definitely-not-a-real-binary".to_string(),
            args: vec![],
        });
        let mut registry = build_tool_registry();

        let status = connect_configured_mcp_servers(&mut registry, &config).await;

        assert_eq!(status.len(), 1);
        assert_eq!(status[0].name, "broken");
        assert!(!status[0].connected);
        assert!(status[0].error.is_some());
    }
}
