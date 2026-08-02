---
type: Rust Module
title: cli_test
resource: tests/cli_test.rs#L1-L273
generated:
  by: okf-rs/0.3.0
relationships:
  imports:
  - external/clap-parser
  - external/crustly-cli-cli-commands-dbcommands-outputformat
  member_of:
  - packages/crustly
---

# Contains

- [test_cli_parse_no_command](../../functions/tests/cli_test/test_cli_parse_no_command.md)
- [test_cli_parse_chat_command](../../functions/tests/cli_test/test_cli_parse_chat_command.md)
- [test_cli_parse_chat_with_session](../../functions/tests/cli_test/test_cli_parse_chat_with_session.md)
- [test_cli_parse_run_command](../../functions/tests/cli_test/test_cli_parse_run_command.md)
- [test_cli_parse_run_with_json_format](../../functions/tests/cli_test/test_cli_parse_run_with_json_format.md)
- [test_cli_parse_run_with_markdown_format](../../functions/tests/cli_test/test_cli_parse_run_with_markdown_format.md)
- [test_cli_parse_run_with_auto_approve](../../functions/tests/cli_test/test_cli_parse_run_with_auto_approve.md)
- [test_cli_parse_run_with_yolo_alias](../../functions/tests/cli_test/test_cli_parse_run_with_yolo_alias.md)
- [test_cli_parse_init_command](../../functions/tests/cli_test/test_cli_parse_init_command.md)
- [test_cli_parse_init_with_force](../../functions/tests/cli_test/test_cli_parse_init_with_force.md)
- [test_cli_parse_config_command](../../functions/tests/cli_test/test_cli_parse_config_command.md)
- [test_cli_parse_config_with_show_secrets](../../functions/tests/cli_test/test_cli_parse_config_with_show_secrets.md)
- [test_cli_parse_db_init](../../functions/tests/cli_test/test_cli_parse_db_init.md)
- [test_cli_parse_db_stats](../../functions/tests/cli_test/test_cli_parse_db_stats.md)
- [test_cli_parse_debug_flag](../../functions/tests/cli_test/test_cli_parse_debug_flag.md)
- [test_cli_parse_debug_flag_short](../../functions/tests/cli_test/test_cli_parse_debug_flag_short.md)
- [test_cli_parse_config_path](../../functions/tests/cli_test/test_cli_parse_config_path.md)
- [test_cli_parse_config_path_short](../../functions/tests/cli_test/test_cli_parse_config_path_short.md)
- [test_cli_parse_combined_flags](../../functions/tests/cli_test/test_cli_parse_combined_flags.md)
- [test_cli_invalid_format](../../functions/tests/cli_test/test_cli_invalid_format.md)
- [test_cli_missing_prompt_for_run](../../functions/tests/cli_test/test_cli_missing_prompt_for_run.md)
- [test_cli_invalid_subcommand](../../functions/tests/cli_test/test_cli_invalid_subcommand.md)
- [test_cli_db_missing_operation](../../functions/tests/cli_test/test_cli_db_missing_operation.md)
- [test_cli_db_invalid_operation](../../functions/tests/cli_test/test_cli_db_invalid_operation.md)

# Imports

- `clap::Parser`
- `crustly::cli::{Cli, Commands, DbCommands, OutputFormat}`

# Member of

- [crustly](../../packages/crustly.md)