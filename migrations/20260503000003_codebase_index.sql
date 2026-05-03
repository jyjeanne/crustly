-- Persistent, incrementally-updated index of project symbols.
-- Updated by the file-watcher on Create/Modify events for *.rs files.

CREATE TABLE IF NOT EXISTS codebase_index (
    id TEXT PRIMARY KEY NOT NULL,
    file_path TEXT NOT NULL,
    symbol_name TEXT NOT NULL,
    symbol_kind TEXT NOT NULL,      -- 'fn' | 'struct' | 'enum' | 'trait' | 'mod' | 'impl' | 'const'
    line_number INTEGER NOT NULL,
    indexed_at INTEGER NOT NULL,    -- Unix timestamp

    UNIQUE(file_path, symbol_name, symbol_kind)
);

CREATE INDEX IF NOT EXISTS idx_codebase_index_symbol_name
    ON codebase_index(symbol_name);

CREATE INDEX IF NOT EXISTS idx_codebase_index_file_path
    ON codebase_index(file_path);
