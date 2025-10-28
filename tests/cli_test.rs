//! CLI Command Parsing Tests
//!
//! Tests for command-line argument parsing using Clap.

use clap::Parser;
use crustly::cli::{Cli, Commands, DbCommands, OutputFormat};

#[test]
fn test_cli_parse_no_command() {
    // When no command is given, command should be None (defaults to chat)
    let cli = Cli::try_parse_from(&["crustly"]).unwrap();
    assert!(cli.command.is_none());
    assert!(!cli.debug);
    assert!(cli.config.is_none());
}

#[test]
fn test_cli_parse_chat_command() {
    let cli = Cli::try_parse_from(&["crustly", "chat"]).unwrap();
    match cli.command {
        Some(Commands::Chat { session }) => {
            assert!(session.is_none());
        }
        _ => panic!("Expected Chat command"),
    }
}

#[test]
fn test_cli_parse_chat_with_session() {
    let cli = Cli::try_parse_from(&["crustly", "chat", "--session", "test-session-id"]).unwrap();
    match cli.command {
        Some(Commands::Chat { session }) => {
            assert_eq!(session, Some("test-session-id".to_string()));
        }
        _ => panic!("Expected Chat command with session"),
    }
}

#[test]
fn test_cli_parse_run_command() {
    let cli = Cli::try_parse_from(&["crustly", "run", "Hello, how are you?"]).unwrap();
    match cli.command {
        Some(Commands::Run {
            prompt,
            auto_approve,
            format,
        }) => {
            assert_eq!(prompt, "Hello, how are you?");
            assert!(!auto_approve);
            assert!(matches!(format, OutputFormat::Text));
        }
        _ => panic!("Expected Run command"),
    }
}

#[test]
fn test_cli_parse_run_with_json_format() {
    let cli = Cli::try_parse_from(&["crustly", "run", "--format", "json", "Test prompt"]).unwrap();
    match cli.command {
        Some(Commands::Run {
            prompt,
            auto_approve,
            format,
        }) => {
            assert_eq!(prompt, "Test prompt");
            assert!(!auto_approve);
            assert!(matches!(format, OutputFormat::Json));
        }
        _ => panic!("Expected Run command with JSON format"),
    }
}

#[test]
fn test_cli_parse_run_with_markdown_format() {
    let cli =
        Cli::try_parse_from(&["crustly", "run", "--format", "markdown", "Test prompt"]).unwrap();
    match cli.command {
        Some(Commands::Run {
            prompt,
            auto_approve,
            format,
        }) => {
            assert_eq!(prompt, "Test prompt");
            assert!(!auto_approve);
            assert!(matches!(format, OutputFormat::Markdown));
        }
        _ => panic!("Expected Run command with Markdown format"),
    }
}

#[test]
fn test_cli_parse_run_with_auto_approve() {
    let cli = Cli::try_parse_from(&["crustly", "run", "--auto-approve", "Test prompt"]).unwrap();
    match cli.command {
        Some(Commands::Run {
            prompt,
            auto_approve,
            format: _,
        }) => {
            assert_eq!(prompt, "Test prompt");
            assert!(auto_approve);
        }
        _ => panic!("Expected Run command with auto-approve"),
    }
}

#[test]
fn test_cli_parse_run_with_yolo_alias() {
    let cli = Cli::try_parse_from(&["crustly", "run", "--yolo", "Test prompt"]).unwrap();
    match cli.command {
        Some(Commands::Run {
            prompt,
            auto_approve,
            format: _,
        }) => {
            assert_eq!(prompt, "Test prompt");
            assert!(auto_approve);
        }
        _ => panic!("Expected Run command with yolo alias"),
    }
}

#[test]
fn test_cli_parse_init_command() {
    let cli = Cli::try_parse_from(&["crustly", "init"]).unwrap();
    match cli.command {
        Some(Commands::Init { force }) => {
            assert!(!force);
        }
        _ => panic!("Expected Init command"),
    }
}

#[test]
fn test_cli_parse_init_with_force() {
    let cli = Cli::try_parse_from(&["crustly", "init", "--force"]).unwrap();
    match cli.command {
        Some(Commands::Init { force }) => {
            assert!(force);
        }
        _ => panic!("Expected Init command with force"),
    }
}

#[test]
fn test_cli_parse_config_command() {
    let cli = Cli::try_parse_from(&["crustly", "config"]).unwrap();
    match cli.command {
        Some(Commands::Config { show_secrets }) => {
            assert!(!show_secrets);
        }
        _ => panic!("Expected Config command"),
    }
}

#[test]
fn test_cli_parse_config_with_show_secrets() {
    let cli = Cli::try_parse_from(&["crustly", "config", "--show-secrets"]).unwrap();
    match cli.command {
        Some(Commands::Config { show_secrets }) => {
            assert!(show_secrets);
        }
        _ => panic!("Expected Config command with show-secrets"),
    }
}

#[test]
fn test_cli_parse_db_init() {
    let cli = Cli::try_parse_from(&["crustly", "db", "init"]).unwrap();
    match cli.command {
        Some(Commands::Db { operation }) => {
            assert!(matches!(operation, DbCommands::Init));
        }
        _ => panic!("Expected Db Init command"),
    }
}

#[test]
fn test_cli_parse_db_stats() {
    let cli = Cli::try_parse_from(&["crustly", "db", "stats"]).unwrap();
    match cli.command {
        Some(Commands::Db { operation }) => {
            assert!(matches!(operation, DbCommands::Stats));
        }
        _ => panic!("Expected Db Stats command"),
    }
}

#[test]
fn test_cli_parse_debug_flag() {
    let cli = Cli::try_parse_from(&["crustly", "--debug"]).unwrap();
    assert!(cli.debug);
}

#[test]
fn test_cli_parse_debug_flag_short() {
    let cli = Cli::try_parse_from(&["crustly", "-d"]).unwrap();
    assert!(cli.debug);
}

#[test]
fn test_cli_parse_config_path() {
    let cli = Cli::try_parse_from(&["crustly", "--config", "/path/to/config.toml"]).unwrap();
    assert_eq!(cli.config, Some("/path/to/config.toml".to_string()));
}

#[test]
fn test_cli_parse_config_path_short() {
    let cli = Cli::try_parse_from(&["crustly", "-c", "/path/to/config.toml"]).unwrap();
    assert_eq!(cli.config, Some("/path/to/config.toml".to_string()));
}

#[test]
fn test_cli_parse_combined_flags() {
    let cli = Cli::try_parse_from(&[
        "crustly",
        "--debug",
        "--config",
        "/path/config.toml",
        "run",
        "--format",
        "json",
        "--auto-approve",
        "Test prompt",
    ])
    .unwrap();

    assert!(cli.debug);
    assert_eq!(cli.config, Some("/path/config.toml".to_string()));

    match cli.command {
        Some(Commands::Run {
            prompt,
            auto_approve,
            format,
        }) => {
            assert_eq!(prompt, "Test prompt");
            assert!(auto_approve);
            assert!(matches!(format, OutputFormat::Json));
        }
        _ => panic!("Expected Run command with all flags"),
    }
}

#[test]
fn test_cli_invalid_format() {
    let result = Cli::try_parse_from(&["crustly", "run", "--format", "invalid", "Test"]);
    assert!(result.is_err());
}

#[test]
fn test_cli_missing_prompt_for_run() {
    let result = Cli::try_parse_from(&["crustly", "run"]);
    assert!(result.is_err());
}

#[test]
fn test_cli_invalid_subcommand() {
    let result = Cli::try_parse_from(&["crustly", "invalid"]);
    assert!(result.is_err());
}

#[test]
fn test_cli_db_missing_operation() {
    let result = Cli::try_parse_from(&["crustly", "db"]);
    assert!(result.is_err());
}

#[test]
fn test_cli_db_invalid_operation() {
    let result = Cli::try_parse_from(&["crustly", "db", "invalid"]);
    assert!(result.is_err());
}
