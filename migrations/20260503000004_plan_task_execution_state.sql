-- Adds execution-tracking columns to the existing plan_tasks table.
-- These enable crash recovery: the service reads started_at/status on restart.

ALTER TABLE plan_tasks ADD COLUMN started_at INTEGER;       -- Unix timestamp; set when status -> InProgress
ALTER TABLE plan_tasks ADD COLUMN output_summary TEXT;      -- Brief summary of task output; set on Completed
ALTER TABLE plan_tasks ADD COLUMN error_text TEXT;          -- Error message; set on Failed

CREATE INDEX IF NOT EXISTS idx_plan_tasks_started_at
    ON plan_tasks(started_at);
