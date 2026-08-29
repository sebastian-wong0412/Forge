-- Phase 1A: Cycle becomes the planning root. Vision is no longer an active
-- product entity; the visions table is retained as unused legacy storage so a
-- future Vision layer can be introduced above Cycle without inventing history.
--
-- Assumption: Phase 0 has no meaningful production dataset. Existing rows are
-- copied where a lossless mapping exists (vision id → cycle id; project parent
-- via key_result → objective). start_value for migrated key results is 0.
-- daily_executions is untouched. visions is not dropped.

CREATE TABLE _tmp_objectives AS SELECT * FROM objectives;
CREATE TABLE _tmp_key_results AS SELECT * FROM key_results;
CREATE TABLE _tmp_projects AS SELECT * FROM projects;
CREATE TABLE _tmp_reviews AS SELECT * FROM reviews;

DROP TABLE reviews;
DROP TABLE projects;
DROP TABLE key_results;
DROP TABLE objectives;

CREATE TABLE cycles (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    start_on TEXT NOT NULL,
    end_on TEXT NOT NULL,
    status TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

INSERT INTO cycles (id, name, start_on, end_on, status, created_at, updated_at)
SELECT
    id,
    title,
    substr(created_at, 1, 10),
    substr(created_at, 1, 10),
    CASE status WHEN 'archived' THEN 'archived' ELSE 'planning' END,
    created_at,
    updated_at
FROM visions;

CREATE TABLE objectives (
    id TEXT PRIMARY KEY NOT NULL,
    cycle_id TEXT NOT NULL REFERENCES cycles (id) ON DELETE RESTRICT,
    title TEXT NOT NULL,
    description TEXT,
    status TEXT NOT NULL,
    start_on TEXT,
    end_on TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

INSERT INTO objectives (
    id, cycle_id, title, description, status, start_on, end_on, created_at, updated_at
)
SELECT
    id, vision_id, title, description, status, start_on, end_on, created_at, updated_at
FROM _tmp_objectives;

CREATE INDEX idx_objectives_cycle_id ON objectives (cycle_id);

CREATE TABLE key_results (
    id TEXT PRIMARY KEY NOT NULL,
    objective_id TEXT NOT NULL REFERENCES objectives (id) ON DELETE RESTRICT,
    title TEXT NOT NULL,
    description TEXT,
    status TEXT NOT NULL,
    start_value REAL NOT NULL,
    target_value REAL,
    unit TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

INSERT INTO key_results (
    id, objective_id, title, description, status, start_value, target_value, unit, created_at, updated_at
)
SELECT
    id, objective_id, title, description, status, 0, target_value, unit, created_at, updated_at
FROM _tmp_key_results;

CREATE INDEX idx_key_results_objective_id ON key_results (objective_id);

CREATE TABLE projects (
    id TEXT PRIMARY KEY NOT NULL,
    objective_id TEXT NOT NULL REFERENCES objectives (id) ON DELETE RESTRICT,
    title TEXT NOT NULL,
    description TEXT,
    status TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

INSERT INTO projects (id, objective_id, title, description, status, created_at, updated_at)
SELECT
    p.id,
    kr.objective_id,
    p.title,
    p.description,
    p.status,
    p.created_at,
    p.updated_at
FROM _tmp_projects AS p
INNER JOIN _tmp_key_results AS kr ON kr.id = p.key_result_id;

CREATE INDEX idx_projects_objective_id ON projects (objective_id);

CREATE TABLE check_ins (
    id TEXT PRIMARY KEY NOT NULL,
    key_result_id TEXT NOT NULL REFERENCES key_results (id) ON DELETE RESTRICT,
    value REAL NOT NULL,
    note TEXT,
    checked_on TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE INDEX idx_check_ins_key_result_id ON check_ins (key_result_id);

CREATE TABLE reviews (
    id TEXT PRIMARY KEY NOT NULL,
    cycle_id TEXT NOT NULL REFERENCES cycles (id) ON DELETE RESTRICT,
    content TEXT NOT NULL,
    period_start TEXT,
    period_end TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

INSERT INTO reviews (id, cycle_id, content, period_start, period_end, created_at, updated_at)
SELECT
    r.id,
    COALESCE(
        r.vision_id,
        obj.vision_id,
        obj_from_kr.vision_id,
        obj_from_project.vision_id
    ),
    r.content,
    r.period_start,
    r.period_end,
    r.created_at,
    r.updated_at
FROM _tmp_reviews AS r
LEFT JOIN _tmp_objectives AS obj ON obj.id = r.objective_id
LEFT JOIN _tmp_key_results AS kr ON kr.id = r.key_result_id
LEFT JOIN _tmp_objectives AS obj_from_kr ON obj_from_kr.id = kr.objective_id
LEFT JOIN _tmp_projects AS project ON project.id = r.project_id
LEFT JOIN _tmp_key_results AS kr_from_project ON kr_from_project.id = project.key_result_id
LEFT JOIN _tmp_objectives AS obj_from_project ON obj_from_project.id = kr_from_project.objective_id
WHERE COALESCE(
    r.vision_id,
    obj.vision_id,
    obj_from_kr.vision_id,
    obj_from_project.vision_id
) IS NOT NULL;

CREATE INDEX idx_reviews_cycle_id ON reviews (cycle_id);

ALTER TABLE tasks ADD COLUMN completed_at TEXT;

DROP TABLE _tmp_reviews;
DROP TABLE _tmp_projects;
DROP TABLE _tmp_key_results;
DROP TABLE _tmp_objectives;
