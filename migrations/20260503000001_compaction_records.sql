-- Stores one record per context compaction event for auditability.

CREATE TABLE IF NOT EXISTS compaction_records (
    id TEXT PRIMARY KEY NOT NULL,
    session_id TEXT NOT NULL,
    turn_range_start INTEGER NOT NULL,
    turn_range_end INTEGER NOT NULL,
    tokens_before INTEGER NOT NULL,
    tokens_after INTEGER NOT NULL DEFAULT 0,
    summary_text TEXT NOT NULL,
    created_at INTEGER NOT NULL,   -- Unix timestamp

    FOREIGN KEY (session_id) REFERENCES sessions(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_compaction_records_session_id
    ON compaction_records(session_id, created_at DESC);
