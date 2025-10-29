//! CLI Module
//!
//! Command-line interface for Crustly using Clap v4.

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::sync::Arc;

/// Crustly - High-Performance Terminal AI Assistant
#[derive(Parser, Debug)]
#[command(name = "crustly")]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Enable debug mode
    #[arg(short, long, global = true)]
    pub debug: bool,

    /// Configuration file path
    #[arg(short, long, global = true)]
    pub config: Option<String>,

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
}

#[derive(Subcommand, Debug)]
pub enum DbCommands {
    /// Initialize database
    Init,
    /// Show database statistics
    Stats,
}

#[derive(Debug, Clone, Copy, clap::ValueEnum)]
pub enum OutputFormat {
    Text,
    Json,
    Markdown,
}

/// Main CLI entry point
pub async fn run() -> Result<()> {
    let cli = Cli::parse();

    // Set up logging level based on debug flag
    if cli.debug {
        tracing::info!("Debug mode enabled");
    }

    // Load configuration
    let config = load_config(cli.config.as_deref()).await?;

    match cli.command {
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
        Some(Commands::Run {
            prompt,
            auto_approve,
            format,
        }) => cmd_run(&config, prompt, auto_approve, format).await,
    }
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

            let file_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM tracked_files")
                .fetch_one(db.pool())
                .await?;

            println!("Sessions: {}", session_count);
            println!("Messages: {}", message_count);
            println!("Tracked files: {}", file_count);

            Ok(())
        }
    }
}

/// Start interactive chat session
async fn cmd_chat(config: &crate::config::Config, _session_id: Option<String>) -> Result<()> {
    use crate::{
        db::Database,
        llm::{
            agent::AgentService,
            provider::{anthropic::AnthropicProvider, openai::OpenAIProvider, Provider},
            tools::{bash::BashTool, read::ReadTool, registry::ToolRegistry, write::WriteTool},
        },
        services::ServiceContext,
        tui,
    };

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

    // Select provider based on configuration
    let provider: Arc<dyn Provider> = if let Some(openai_config) = &config.providers.openai {
        // OpenAI provider is configured
        if let Some(base_url) = &openai_config.base_url {
            // Local LLM (LM Studio, Ollama, etc.)
            tracing::info!("Using local LLM at: {}", base_url);
            println!("🏠 Using local LLM at: {}\n", base_url);
            Arc::new(OpenAIProvider::local(base_url.clone()))
        } else if let Some(api_key) = &openai_config.api_key {
            // Official OpenAI API
            tracing::info!("Using OpenAI provider");
            println!("🤖 Using OpenAI provider\n");
            Arc::new(OpenAIProvider::new(api_key.clone()))
        } else {
            // OpenAI configured but no credentials - fall back to Anthropic
            tracing::debug!("OpenAI configured but no credentials, falling back to Anthropic");
            let anthropic_config = config.providers.anthropic.as_ref().context(
                "No provider configured. Please set ANTHROPIC_API_KEY or OPENAI_API_KEY",
            )?;

            let api_key = anthropic_config
                .api_key
                .as_ref()
                .context("Anthropic API key not set")?
                .clone();

            tracing::info!("Using Anthropic provider");
            println!("🤖 Using Anthropic Claude\n");
            Arc::new(AnthropicProvider::new(api_key))
        }
    } else {
        // No OpenAI config, use Anthropic
        let anthropic_config = config
            .providers
            .anthropic
            .as_ref()
            .context("No provider configured.\n\nPlease set one of:\n  - ANTHROPIC_API_KEY for Claude\n  - OPENAI_API_KEY for OpenAI/GPT\n  - OPENAI_BASE_URL for local LLMs (LM Studio, Ollama)\n\nExample for LM Studio:\n  export OPENAI_BASE_URL=\"http://localhost:1234/v1\"")?;

        let api_key = anthropic_config
            .api_key
            .as_ref()
            .context("Anthropic API key not set")?
            .clone();

        tracing::info!("Using Anthropic provider");
        println!("🤖 Using Anthropic Claude\n");
        Arc::new(AnthropicProvider::new(api_key))
    };

    // Create tool registry
    tracing::debug!("Setting up tool registry");
    let mut tool_registry = ToolRegistry::new();
    tool_registry.register(Arc::new(ReadTool));
    tool_registry.register(Arc::new(WriteTool));
    tool_registry.register(Arc::new(BashTool));

    // Create service context
    let service_context = ServiceContext::new(db.pool().clone());

    // Create TUI app first (so we can get the event sender)
    tracing::debug!("Creating TUI app");
    let mut app = tui::App::new(
        Arc::new(AgentService::new(provider.clone(), service_context.clone())),
        service_context.clone(),
    );

    // Get event sender from app
    let event_sender = app.event_sender();

    // Create approval callback that sends requests to TUI
    let approval_callback: crate::llm::agent::ApprovalCallback = Arc::new(move |tool_info| {
        let sender = event_sender.clone();
        Box::pin(async move {
            use crate::tui::events::{ToolApprovalRequest, TuiEvent};
            use tokio::sync::mpsc;

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
    });

    // Create agent service with approval callback
    tracing::debug!("Creating agent service with approval callback");
    let agent_service = Arc::new(
        AgentService::new(provider.clone(), service_context.clone())
            .with_tool_registry(Arc::new(tool_registry))
            .with_approval_callback(Some(approval_callback)),
    );

    // Update app with the configured agent service
    app = tui::App::new(agent_service, service_context.clone());

    // Run TUI
    tracing::debug!("Launching TUI");
    tui::run(app).await.context("TUI error")?;

    println!("\n👋 Goodbye!");

    Ok(())
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
        llm::{
            agent::AgentService,
            provider::{anthropic::AnthropicProvider, openai::OpenAIProvider, Provider},
            tools::{bash::BashTool, read::ReadTool, registry::ToolRegistry, write::WriteTool},
        },
        services::{ServiceContext, SessionService},
    };

    tracing::info!("Running non-interactive command: {}", prompt);

    // Initialize database
    let db = Database::connect(&config.database.path).await?;
    db.run_migrations().await?;

    // Select provider based on configuration
    let provider: Arc<dyn Provider> = if let Some(openai_config) = &config.providers.openai {
        // OpenAI provider is configured
        if let Some(base_url) = &openai_config.base_url {
            // Local LLM (LM Studio, Ollama, etc.)
            tracing::info!("Using local LLM at: {}", base_url);
            Arc::new(OpenAIProvider::local(base_url.clone()))
        } else if let Some(api_key) = &openai_config.api_key {
            // Official OpenAI API
            tracing::info!("Using OpenAI provider");
            Arc::new(OpenAIProvider::new(api_key.clone()))
        } else {
            // Fall back to Anthropic
            let anthropic_config = config
                .providers
                .anthropic
                .as_ref()
                .context("No provider configured")?;
            let api_key = anthropic_config
                .api_key
                .as_ref()
                .context("Anthropic API key not set")?
                .clone();
            tracing::info!("Using Anthropic provider");
            Arc::new(AnthropicProvider::new(api_key))
        }
    } else {
        // No OpenAI config, use Anthropic
        let anthropic_config = config
            .providers
            .anthropic
            .as_ref()
            .context("No provider configured")?;
        let api_key = anthropic_config
            .api_key
            .as_ref()
            .context("Anthropic API key not set")?
            .clone();
        tracing::info!("Using Anthropic provider");
        Arc::new(AnthropicProvider::new(api_key))
    };

    // Create tool registry
    let mut tool_registry = ToolRegistry::new();
    tool_registry.register(Arc::new(ReadTool));
    tool_registry.register(Arc::new(WriteTool));
    tool_registry.register(Arc::new(BashTool));

    // Create service context and agent service
    let service_context = ServiceContext::new(db.pool().clone());
    let agent_service = AgentService::new(provider.clone(), service_context.clone())
        .with_tool_registry(Arc::new(tool_registry));

    // Create or get session
    let session_service = SessionService::new(service_context);

    let session = session_service
        .create_session(Some("CLI Run".to_string()))
        .await?;

    // Send message
    println!("🤔 Processing...\n");
    let response = agent_service.send_message(session.id, prompt, None).await?;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_parse() {
        use clap::CommandFactory;
        Cli::command().debug_assert();
    }
}
