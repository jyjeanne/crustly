-- Stores per-session summaries for cross-session context injection.

CREATE TABLE IF NOT EXISTS episodic_memories (
    id TEXT PRIMARY KEY NOT NULL,
    session_id TEXT NOT NULL,
    summary_text TEXT NOT NULL,
    token_count INTEGER NOT NULL DEFAULT 0,
    files_touched TEXT NOT NULL DEFAULT '[]',   -- JSON array of file paths
    decisions TEXT NOT NULL DEFAULT '[]',        -- JSON array of key decisions
    created_at INTEGER NOT NULL,                -- Unix timestamp

    FOREIGN KEY (session_id) REFERENCES sessions(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_episodic_memories_created_at
    ON episodic_memories(created_at DESC);
