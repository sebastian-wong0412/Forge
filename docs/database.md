# Database

SQLite is the source of truth. Migrations live in `crates/forge-server/migrations/`.

- Never edit an existing migration.
- Every connection enables foreign keys, WAL, and a busy timeout.
- Foreign keys use `ON DELETE RESTRICT`.
- Timestamps are RFC3339 UTC text. Dates are `YYYY-MM-DD`.
- IDs are UUID v7 text.

## File location

The developer CLI defaults to `forge.db` in the process working directory (`FORGE_DATABASE_PATH`).

The packaged desktop app sets `FORGE_DATABASE_PATH` to:

- Production: `%LOCALAPPDATA%\app.forge.desktop\forge.db`
- `tauri dev`: `%LOCALAPPDATA%\app.forge.desktop\forge-dev.db`

This is outside the current-user install directory (`%LOCALAPPDATA%\Forge`). The installer does not ship a database. Uninstalling Forge leaves this folder in place unless the user checks **Delete app data**.

## Active Phase 1A schema

Applied by `0001_initial.sql` plus `0002_cycle_semantics.sql`.

### `cycles`

Root planning unit. `start_on` and `end_on` are required. No unique constraint on `status = active`.

| Column | Notes |
|---|---|
| `id` | PK |
| `name` | Display name |
| `start_on` | Required |
| `end_on` | Required, `>= start_on` |
| `status` | `planning`, `active`, `closed`, `archived` |
| `created_at` / `updated_at` | RFC3339 |

### `objectives`

Belongs to a cycle.

| Column | Notes |
|---|---|
| `id` | PK |
| `cycle_id` | FK → `cycles.id` |
| `title`, `description` | |
| `status` | `draft`, `active`, `completed`, `archived` |
| `start_on` / `end_on` | Optional; must fall inside the parent cycle |
| `created_at` / `updated_at` | RFC3339 |

Index: `idx_objectives_cycle_id`.

### `key_results`

Belongs to an objective. `current_value` is **not** stored.

| Column | Notes |
|---|---|
| `id` | PK |
| `objective_id` | FK → `objectives.id` |
| `title`, `description` | |
| `status` | `draft`, `active`, `completed`, `archived` |
| `progress_kind` | `numeric`, `percentage`, `milestone`, `qualitative` |
| `start_value` | Required for numeric / percentage |
| `target_value` | Optional for numeric; required for percentage |
| `unit` | Optional; numeric only |
| `created_at` / `updated_at` | RFC3339 |

Index: `idx_key_results_objective_id`.

### `check_ins`

Append-only history. `checked_on` is the date progress occurred. `created_at` is when Forge recorded the row.

| Column | Notes |
|---|---|
| `id` | PK |
| `key_result_id` | FK → `key_results.id` |
| `value` | Numeric / percentage measurement; null otherwise |
| `milestone_state` | `not_started` / `in_progress` / `achieved`; null otherwise |
| `note` | Optional except qualitative |
| `checked_on` | Progress date |
| `created_at` / `updated_at` | Recorded-at timestamps |

Index: `idx_check_ins_key_result_id`.

### `projects`

Sibling of Key Result under Objective.

| Column | Notes |
|---|---|
| `id` | PK |
| `objective_id` | FK → `objectives.id` |
| `title`, `description` | |
| `status` | `draft`, `active`, `completed`, `archived` |
| `created_at` / `updated_at` | RFC3339 |

Index: `idx_projects_objective_id`.

### `tasks`

Belongs to a project. `completed_at` is set when the task is completed.

| Column | Notes |
|---|---|
| `id` | PK |
| `project_id` | FK → `projects.id` |
| `title`, `description` | |
| `status` | `todo`, `in_progress`, `done`, `cancelled` |
| `scheduled_on` | Optional `YYYY-MM-DD` (from 0003). Intent to work that day, not a due date. |
| `completed_at` | Optional RFC3339 |
| `created_at` / `updated_at` | RFC3339 |

Indexes: `idx_tasks_project_id` (0001), `idx_tasks_scheduled_on` (0003).

## Migration 0003

Adds `tasks.scheduled_on TEXT` and an index. Existing rows stay `NULL`.
`daily_executions` is unchanged. There is no backfill from DailyExecution.

### `reviews`

Belongs only to a cycle. No polymorphic subject columns.

| Column | Notes |
|---|---|
| `id` | PK |
| `cycle_id` | FK → `cycles.id` |
| `content` | Required |
| `period_start` / `period_end` | Optional; must fall inside the cycle |
| `created_at` / `updated_at` | RFC3339 |

Index: `idx_reviews_cycle_id`.

## Frozen / leftover tables

### `visions`

Created in 0001. **Not dropped** in 0002. Not part of the active product model. Retained so a future Vision layer can sit above Cycle without inventing history.

### `daily_executions`

Created in 0001. Unchanged in 0002–0004. Frozen Phase 0 persistence. Do not add columns or tables for it. The table remains so existing rows and the leftover API surface keep working.

## Migration 0004

Rebuilds `key_results` and `check_ins`. Existing key results become `progress_kind = numeric` with their original `start_value` / `target_value` / `unit`. Existing check-ins keep `value` and get `milestone_state = NULL`. `start_value` and `value` become nullable so milestone and qualitative rows can omit them.

## Migration 0002 assumptions

Phase 0 had no meaningful production dataset. 0002 copies what it can with a simple mapping:

- Each `visions` row becomes a `cycles` row: `title` → `name`, both dates taken from `substr(created_at, 1, 10)`, archived stays archived, other statuses become `planning`.
- `objectives.vision_id` becomes `objectives.cycle_id`.
- Migrated key results get `start_value = 0`. `current_value` is dropped; history is not reconstructed.
- Projects move from `key_result_id` to the parent `objective_id`.
- Reviews are rewritten onto the nearest vision/cycle id. Periods stay as stored and become nullable.
- `tasks.completed_at` is added. Existing tasks are not rebuilt, so `daily_executions` foreign keys stay valid.
- Temporary `_tmp_*` tables are used; historical Phase 0 tables that remain (`visions`, `daily_executions`) are not hard-deleted.
