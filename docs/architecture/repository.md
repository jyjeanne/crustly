# Repository Summary

Generated from `Cargo.toml` + `cargo metadata` by `scripts/generate-architecture-docs.sh`. Do not hand-edit - re-run the script instead.

## Crate

- **Name:** crustly
- **Version:** 0.4.1
- **Edition:** 2021
- **License:** FSL-1.1-MIT
- **Rust edition:** 2021

## Targets

- **lib** (1): crustly
- **bin** (1): crustly
- **test** (11): cli_test, codebase_index_test, compaction_test, error_scenarios_test, integration_test, mcp_contract_test, model_routing_test, plan_autorun_test, plan_crash_recovery_test, plan_mode_integration_test, streaming_test
- **bench** (2): database, parallel_tool_dispatch

## Features

- `all-llm` -> openai, aws-bedrock, ollama
- `async-openai` -> dep:async-openai
- `aws-bedrock` -> aws-sdk-bedrockruntime
- `aws-sdk-bedrockruntime` -> dep:aws-sdk-bedrockruntime
- `default`
- `ollama` -> dep:ollama-rs, dep:schemars
- `openai` -> async-openai
- `pprof` -> dep:pprof
- `profiling`

## Dependencies (by category, from Cargo.toml)

### Async Runtime

- `tokio`
- `tokio-stream`
- `tokio-util`
- `futures`
- `async-trait`
- `pin-project`

### CLI Framework

- `clap`
- `clap_complete`

### TUI

- `ratatui`
- `crossterm`
- `tui-textarea`
- `tui-tree-widget`
- `ratatui-image`
- `viuer`
- `arboard`

### Database

- `sqlx`

### Serialization

- `serde`
- `serde_json`
- `toml`

### Configuration

- `config`
- `dirs`
- `shellexpand`

### HTTP & LLM Clients

- `reqwest`
- `async-openai`
- `aws-sdk-bedrockruntime`
- `ollama-rs`
- `schemars`

### Provider Registry (Crabrace - replaces Catwalk)

- `crabrace`

### Error Handling

- `anyhow`
- `thiserror`
- `color-eyre`

### Logging & Tracing

- `tracing`
- `tracing-subscriber`
- `tracing-appender`

### Utilities

- `uuid`
- `chrono`
- `regex`
- `glob`
- `ignore`
- `which`
- `shell-words`
- `notify`
- `git2`
- `rand`
- `urlencoding`

### Syntax & Parsing

- `syntect`
- `tree-sitter`
- `pulldown-cmark`

### LSP

- `tower-lsp`
- `lsp-types`

### Tokenization (BPE-based, accurate to <2% for cl100k_base vocab)

- `tiktoken-rs`

### Concurrent Data Structures

- `dashmap`
- `parking_lot`
- `once_cell`
- `arc-swap`

### Security

- `zeroize`
- `keyring`

### Misc

- `bytes`
- `mime`
- `base64`

### Document Parsing

- `pdf-extract`
- `zip`
- `quick-xml`

