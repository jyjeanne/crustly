-- A pending memory's replacement target (docs/improve-memory.md §12.4).
-- Extraction in review mode can propose a correction of an existing memory;
-- the target is held here and superseded only when the user approves.
ALTER TABLE memories ADD COLUMN pending_replaces TEXT;
