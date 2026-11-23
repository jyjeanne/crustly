CREATE TABLE IF NOT EXISTS memory_projects (
    id TEXT PRIMARY KEY NOT NULL,        -- hex SHA-256 of the canonical root path
    root_path TEXT NOT NULL,             -- canonical root, for display and relinking
    name TEXT NOT NULL,                  -- root directory name, for display
    remote_url TEXT,                     -- git "origin" URL, display only
    created_at INTEGER NOT NULL,
    last_seen_at INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS memories (
    seq INTEGER PRIMARY KEY,             -- stable rowid for memories_fts
    id TEXT NOT NULL UNIQUE,             -- UUID, the public id
    scope TEXT NOT NULL CHECK (scope IN ('global', 'project')),
    project_id TEXT REFERENCES memory_projects(id) ON DELETE CASCADE,
    category TEXT NOT NULL CHECK (category IN
        ('preference', 'convention', 'decision', 'fact', 'lesson', 'episode')),
    content TEXT NOT NULL CHECK (length(content) BETWEEN 1 AND 500),
    content_hash TEXT NOT NULL,          -- hex SHA-256 of the normalised content
    source TEXT NOT NULL CHECK (source IN
        ('user', 'tool', 'extracted', 'imported', 'consolidated')),
    source_session_id TEXT REFERENCES sessions(id) ON DELETE SET NULL,
    importance INTEGER NOT NULL DEFAULT 3 CHECK (importance BETWEEN 1 AND 5),
    confidence REAL NOT NULL DEFAULT 1.0 CHECK (confidence BETWEEN 0.0 AND 1.0),
    status TEXT NOT NULL DEFAULT 'active' CHECK (status IN
        ('active', 'pending', 'superseded', 'archived')),
    superseded_by TEXT REFERENCES memories(id) ON DELETE SET NULL,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL,
    last_recalled_at INTEGER,
    recall_count INTEGER NOT NULL DEFAULT 0,
    expires_at INTEGER,
    CHECK ((scope = 'global') = (project_id IS NULL))
);

CREATE INDEX IF NOT EXISTS idx_memories_recall
    ON memories(status, scope, project_id);

-- One active or pending copy of a fact per project (global memories use '').
CREATE UNIQUE INDEX IF NOT EXISTS idx_memories_active_hash
    ON memories(ifnull(project_id, ''), content_hash)
    WHERE status IN ('active', 'pending');

CREATE VIRTUAL TABLE IF NOT EXISTS memories_fts USING fts5(
    content,
    content = 'memories',
    content_rowid = 'seq',
    tokenize = 'porter unicode61 remove_diacritics 2'
);

-- Purge deleted tokens from the index immediately (SQLite 3.44+).
INSERT INTO memories_fts(memories_fts, rank) VALUES ('secure-delete', 1);

CREATE TRIGGER IF NOT EXISTS memories_fts_insert AFTER INSERT ON memories BEGIN
    INSERT INTO memories_fts(rowid, content) VALUES (new.seq, new.content);
END;

CREATE TRIGGER IF NOT EXISTS memories_fts_delete AFTER DELETE ON memories BEGIN
    INSERT INTO memories_fts(memories_fts, rowid, content)
        VALUES ('delete', old.seq, old.content);
END;

CREATE TRIGGER IF NOT EXISTS memories_fts_update AFTER UPDATE OF content ON memories BEGIN
    INSERT INTO memories_fts(memories_fts, rowid, content)
        VALUES ('delete', old.seq, old.content);
    INSERT INTO memories_fts(rowid, content) VALUES (new.seq, new.content);
END;

CREATE TABLE IF NOT EXISTS memory_events (
    id INTEGER PRIMARY KEY,
    memory_id TEXT NOT NULL,             -- not a foreign key: the event outlives the memory
    event TEXT NOT NULL CHECK (event IN ('created', 'updated', 'reinforced',
        'approved', 'superseded', 'archived', 'forgotten', 'promoted', 'relinked')),
    actor TEXT NOT NULL CHECK (actor IN
        ('user', 'model', 'extractor', 'consolidator', 'import')),
    session_id TEXT,
    created_at INTEGER NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_memory_events_memory ON memory_events(memory_id);
