-- Memories anchored to code concepts of an okf-rs bundle
-- (docs/improve-memory.md §23.3, task P6.2). Only project memories are
-- anchored, at most 3 anchors each (enforced by the memory module).
CREATE TABLE IF NOT EXISTS memory_anchors (
    memory_id TEXT NOT NULL REFERENCES memories(id) ON DELETE CASCADE,
    concept_id TEXT NOT NULL,            -- okf concept id, e.g. 'functions/src/db/connect_pool'
    anchored_at INTEGER NOT NULL,        -- bundle modification time when the anchor was made
    PRIMARY KEY (memory_id, concept_id)
);

CREATE INDEX IF NOT EXISTS idx_memory_anchors_concept ON memory_anchors(concept_id);
