//! CLI Module
//!
//! Command-line interface for Crustly using Clap v4.

use anyhow::Result;
use clap::{Parser, Subcommand};

/// Crustly - High-Performance Terminal AI Assistant
#[derive(Parser, Debug)]
#[command(name = "crustly")]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Enable debug mode
    #[arg(short, long, global = true)]
    pub debug: bool,

    /// Subcommand to execute
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Start interactive TUI mode
    Interactive,

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

    /// List all chat sessions
    Sessions,

    /// Show version information
    Version,
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

    match cli.command {
        None => {
            // Default: Interactive TUI mode
            tracing::info!("Starting Crustly in interactive mode...");
            println!("🥐 Crustly - Terminal AI Assistant");
            println!("Version: {}", env!("CARGO_PKG_VERSION"));
            println!();
            println!("Interactive TUI mode will be implemented in Sprint 9-10");
            println!("For now, try: crustly run \"hello world\"");
            Ok(())
        }
        Some(Commands::Interactive) => {
            tracing::info!("Starting interactive mode...");
            println!("Interactive TUI mode coming soon!");
            Ok(())
        }
        Some(Commands::Run {
            prompt,
            auto_approve,
            format,
        }) => {
            tracing::info!("Running prompt: {:?}", prompt);
            println!("Prompt: {}", prompt);
            println!("Auto-approve: {}", auto_approve);
            println!("Format: {:?}", format);
            println!();
            println!("Note: LLM integration will be implemented in Sprint 5-8");
            Ok(())
        }
        Some(Commands::Sessions) => {
            tracing::info!("Listing sessions...");
            println!("Session management will be implemented in Sprint 3-4");
            Ok(())
        }
        Some(Commands::Version) => {
            println!("Crustly v{}", env!("CARGO_PKG_VERSION"));
            println!("Authors: {}", env!("CARGO_PKG_AUTHORS"));
            println!("Description: {}", env!("CARGO_PKG_DESCRIPTION"));
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
}
