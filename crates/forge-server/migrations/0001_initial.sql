CREATE TABLE visions (
    id TEXT PRIMARY KEY NOT NULL,
    title TEXT NOT NULL,
    description TEXT,
    status TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE objectives (
    id TEXT PRIMARY KEY NOT NULL,
    vision_id TEXT NOT NULL REFERENCES visions (id) ON DELETE RESTRICT,
    title TEXT NOT NULL,
    description TEXT,
    status TEXT NOT NULL,
    start_on TEXT,
    end_on TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE INDEX idx_objectives_vision_id ON objectives (vision_id);

CREATE TABLE key_results (
    id TEXT PRIMARY KEY NOT NULL,
    objective_id TEXT NOT NULL REFERENCES objectives (id) ON DELETE RESTRICT,
    title TEXT NOT NULL,
    description TEXT,
    status TEXT NOT NULL,
    target_value REAL,
    current_value REAL,
    unit TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE INDEX idx_key_results_objective_id ON key_results (objective_id);

CREATE TABLE projects (
    id TEXT PRIMARY KEY NOT NULL,
    key_result_id TEXT NOT NULL REFERENCES key_results (id) ON DELETE RESTRICT,
    title TEXT NOT NULL,
    description TEXT,
    status TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE INDEX idx_projects_key_result_id ON projects (key_result_id);

CREATE TABLE tasks (
    id TEXT PRIMARY KEY NOT NULL,
    project_id TEXT NOT NULL REFERENCES projects (id) ON DELETE RESTRICT,
    title TEXT NOT NULL,
    description TEXT,
    status TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE INDEX idx_tasks_project_id ON tasks (project_id);

CREATE TABLE daily_executions (
    id TEXT PRIMARY KEY NOT NULL,
    task_id TEXT NOT NULL REFERENCES tasks (id) ON DELETE RESTRICT,
    execution_date TEXT NOT NULL,
    notes TEXT,
    status TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE INDEX idx_daily_executions_task_id ON daily_executions (task_id);
CREATE INDEX idx_daily_executions_execution_date ON daily_executions (execution_date);

CREATE TABLE reviews (
    id TEXT PRIMARY KEY NOT NULL,
    vision_id TEXT REFERENCES visions (id) ON DELETE RESTRICT,
    objective_id TEXT REFERENCES objectives (id) ON DELETE RESTRICT,
    key_result_id TEXT REFERENCES key_results (id) ON DELETE RESTRICT,
    project_id TEXT REFERENCES projects (id) ON DELETE RESTRICT,
    period_start TEXT NOT NULL,
    period_end TEXT NOT NULL,
    content TEXT NOT NULL,
    status TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    CHECK (
        (
            CASE WHEN vision_id IS NOT NULL THEN 1 ELSE 0 END
        ) + (
            CASE WHEN objective_id IS NOT NULL THEN 1 ELSE 0 END
        ) + (
            CASE WHEN key_result_id IS NOT NULL THEN 1 ELSE 0 END
        ) + (
            CASE WHEN project_id IS NOT NULL THEN 1 ELSE 0 END
        ) = 1
    )
);

CREATE INDEX idx_reviews_vision_id ON reviews (vision_id);
CREATE INDEX idx_reviews_objective_id ON reviews (objective_id);
CREATE INDEX idx_reviews_key_result_id ON reviews (key_result_id);
CREATE INDEX idx_reviews_project_id ON reviews (project_id);
