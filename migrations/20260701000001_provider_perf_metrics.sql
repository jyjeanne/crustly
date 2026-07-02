-- Adds provider identity + runtime performance metrics columns, populated by
-- the native Ollama provider (ollama-rs). All columns are nullable and
-- default to NULL for existing rows and for providers that don't expose
-- this level of detail (Anthropic, OpenAI, Qwen, Azure).

ALTER TABLE sessions ADD COLUMN provider TEXT;              -- Provider name (e.g. "ollama", "anthropic")

ALTER TABLE messages ADD COLUMN provider_name TEXT;         -- Provider that generated this message
ALTER TABLE messages ADD COLUMN perf_metrics_json TEXT;     -- Serialized PerfMetrics (load/prompt/eval durations)
