CREATE TABLE IF NOT EXISTS memory_paths (
    memory_id TEXT NOT NULL REFERENCES memories(id) ON DELETE CASCADE,
    glob TEXT NOT NULL,                  -- relative to the project root, forward slashes
    PRIMARY KEY (memory_id, glob)
);

CREATE TABLE IF NOT EXISTS memory_sources (
    memory_id TEXT NOT NULL REFERENCES memories(id) ON DELETE CASCADE,  -- consolidated memory
    source_id TEXT NOT NULL REFERENCES memories(id) ON DELETE CASCADE,  -- episode or duplicate it came from
    PRIMARY KEY (memory_id, source_id)
);
