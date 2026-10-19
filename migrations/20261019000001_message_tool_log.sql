-- One-line summary of the tool calls that produced an assistant message
-- (e.g. "read_file src/main.rs; edit_file src/main.rs; bash cargo check").
-- Tool calls and their results are not stored as messages, so this is how a
-- later request learns what an earlier one read, changed or ran. NULL for
-- messages produced without tools and for rows written before this column.

ALTER TABLE messages ADD COLUMN tool_log TEXT;
