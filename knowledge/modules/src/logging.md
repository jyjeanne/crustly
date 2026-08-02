---
type: Rust Module
title: logging
resource: src/logging.rs#L1-L393
generated:
  by: okf-rs/0.3.0
relationships:
  imports:
  - external/std-path-pathbuf
  - external/tracing-level
  - external/tracing-appender-non-blocking-workerguard
  - external/tracing-subscriber-layer-subscriberext-util-subscriberinitext-envfilter
  - external/super
  - external/std-io-write
  member_of:
  - packages/crustly
---

# Contains

- [LogConfig](../../classes/src/logging/LogConfig.md)
- [default](../../functions/src/logging/LogConfig/default/default.md)
- [new](../../functions/src/logging/LogConfig/new.md)
- [with_debug_mode](../../functions/src/logging/LogConfig/with_debug_mode.md)
- [with_log_dir](../../functions/src/logging/LogConfig/with_log_dir.md)
- [with_log_level](../../functions/src/logging/LogConfig/with_log_level.md)
- [with_console_output](../../functions/src/logging/LogConfig/with_console_output.md)
- [with_log_prefix](../../functions/src/logging/LogConfig/with_log_prefix.md)
- [LoggerGuard](../../classes/src/logging/LoggerGuard.md)
- [with_guard](../../functions/src/logging/LoggerGuard/with_guard.md)
- [empty](../../functions/src/logging/LoggerGuard/empty.md)
- [init_logging](../../functions/src/logging/init_logging.md)
- [init_debug_logging](../../functions/src/logging/init_debug_logging.md)
- [init_minimal_logging](../../functions/src/logging/init_minimal_logging.md)
- [setup_from_cli](../../functions/src/logging/setup_from_cli.md)
- [get_log_path](../../functions/src/logging/get_log_path.md)
- [cleanup_old_logs](../../functions/src/logging/cleanup_old_logs.md)
- [test_log_config_default](../../functions/src/logging/test_log_config_default.md)
- [test_log_config_with_debug](../../functions/src/logging/test_log_config_with_debug.md)
- [test_log_config_builder](../../functions/src/logging/test_log_config_builder.md)
- [test_log_dir_in_crustly_folder](../../functions/src/logging/test_log_dir_in_crustly_folder.md)
- [debug_log_files_are_findable_by_the_readers](../../functions/src/logging/debug_log_files_are_findable_by_the_readers.md)
- [debug_filter_is_scoped_to_crustly](../../functions/src/logging/debug_filter_is_scoped_to_crustly.md)

# Imports

- `std::path::PathBuf`
- `tracing::Level`
- `tracing_appender::non_blocking::WorkerGuard`
- `tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter}`
- `super::*`
- `std::io::Write`

# Member of

- [crustly](../../packages/crustly.md)