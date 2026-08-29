-- Phase 1B-1: Task scheduling. Existing rows stay unscheduled (NULL).
-- DailyExecution is not used as a source and is left unchanged.

ALTER TABLE tasks ADD COLUMN scheduled_on TEXT;

CREATE INDEX idx_tasks_scheduled_on ON tasks (scheduled_on);
