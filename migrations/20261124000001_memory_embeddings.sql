CREATE TABLE IF NOT EXISTS memory_embeddings (
    memory_id TEXT NOT NULL REFERENCES memories(id) ON DELETE CASCADE,
    model TEXT NOT NULL,                 -- Embedder::model_id(), e.g. 'ollama:nomic-embed-text'
    dimensions INTEGER NOT NULL,
    vector BLOB NOT NULL,                -- little-endian f32 x dimensions, L2-normalised
    created_at INTEGER NOT NULL,
    PRIMARY KEY (memory_id, model)
);
