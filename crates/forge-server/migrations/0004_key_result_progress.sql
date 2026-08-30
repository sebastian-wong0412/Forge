-- Polymorphic Key Result progress: numeric, percentage, milestone, qualitative.
-- Existing rows migrate as numeric. Check-in value becomes nullable; milestone_state is added.
-- start_value is no longer required for every kind.

CREATE TABLE key_results_new (
    id TEXT PRIMARY KEY NOT NULL,
    objective_id TEXT NOT NULL REFERENCES objectives (id) ON DELETE RESTRICT,
    title TEXT NOT NULL,
    description TEXT,
    status TEXT NOT NULL,
    progress_kind TEXT NOT NULL,
    start_value REAL,
    target_value REAL,
    unit TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

INSERT INTO key_results_new (
    id,
    objective_id,
    title,
    description,
    status,
    progress_kind,
    start_value,
    target_value,
    unit,
    created_at,
    updated_at
)
SELECT
    id,
    objective_id,
    title,
    description,
    status,
    'numeric',
    start_value,
    target_value,
    unit,
    created_at,
    updated_at
FROM key_results;

CREATE TABLE check_ins_new (
    id TEXT PRIMARY KEY NOT NULL,
    key_result_id TEXT NOT NULL REFERENCES key_results_new (id) ON DELETE RESTRICT,
    value REAL,
    milestone_state TEXT,
    note TEXT,
    checked_on TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

INSERT INTO check_ins_new (
    id,
    key_result_id,
    value,
    milestone_state,
    note,
    checked_on,
    created_at,
    updated_at
)
SELECT
    id,
    key_result_id,
    value,
    NULL,
    note,
    checked_on,
    created_at,
    updated_at
FROM check_ins;

DROP TABLE check_ins;
DROP TABLE key_results;

ALTER TABLE key_results_new RENAME TO key_results;
ALTER TABLE check_ins_new RENAME TO check_ins;

CREATE INDEX idx_key_results_objective_id ON key_results (objective_id);
CREATE INDEX idx_check_ins_key_result_id ON check_ins (key_result_id);
