CREATE TABLE IF NOT EXISTS memory_extraction_state (
    session_id TEXT PRIMARY KEY NOT NULL REFERENCES sessions(id) ON DELETE CASCADE,
    last_message_sequence INTEGER NOT NULL,   -- messages.sequence already processed
    failed_attempts INTEGER NOT NULL DEFAULT 0,
    updated_at INTEGER NOT NULL
);
