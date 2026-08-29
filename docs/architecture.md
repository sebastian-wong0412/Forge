# Architecture

Forge is a local-first OKR and execution system. Phase 1A is the backend semantic model around Cycle. It does not include a frontend or Tauri.

## Layers

```
API (Axum)  →  Application  →  Domain
Infrastructure (SQLite) implements application repository traits
```

| Crate | Responsibility | May depend on |
|---|---|---|
| `forge-domain` | Entities, IDs, status enums, invariants | `thiserror`, `time`, `uuid` |
| `forge-application` | Commands, services, repository traits, `AppError` | `forge-domain` |
| `forge-server` | SQLite repos, migrations, Axum, config, tracing | application + domain + infra crates |

`forge-domain` must **not** depend on Axum, SQLx, Tokio, Tauri, HTTP, or SQLite.

Application services are generic over repository traits. The server crate is the composition root.

## Phase 1A product model

```
Cycle
├── Objective
│   ├── Key Result
│   │   └── Check-in
│   └── Project
│       └── Task
└── Review
```

**Cycle** is the current root planning unit. Dates are required. Status is `planning` → `active` → `closed`. Any non-archived cycle may be archived. Multiple active cycles are allowed; there is no unique constraint on active status.

Closed or archived cycles cannot gain Objective, Key Result, Project, Task, or Check-in records. They may still receive Reviews.

**Vision** is intentionally not implemented as a separate entity. Vision and Cycle are not the same concept. The architecture stays open to adding a long-term Vision above Cycle later. The `visions` table is leftover storage, not an active product surface.

**Objective** belongs to a Cycle (`cycle_id`). Status is `draft` → `active` → `completed`. Dates are optional; when present they must fall inside the parent Cycle. Only draft or active objectives may create Key Results and Projects.

**Key Result** belongs to an Objective. It stores `start_value` and optional `target_value`. `current_value` is not persisted and is not an authoritative write. It is derived from Check-ins:

- no Check-ins → current = `start_value`
- otherwise → value of the latest Check-in by `checked_on`, then `created_at`, then UUID v7

Progress is `(current - start) / (target - start)`, clamped to `[0, 1]`. The formula supports increasing and decreasing targets. Missing target or `target == start` yields `null`. Reaching `progress >= 1` does not auto-complete the Key Result.

**Check-in** is an append-only business record. `checked_on` is the date the progress occurred. `created_at` is when Forge recorded it. There is no update or delete API.

**Project** belongs directly to an Objective. Projects and Key Results are siblings. Only an active project may create Tasks.

**Task** remains under Project. Status is `todo` → `in_progress` → `done`, or `todo`/`in_progress` → `cancelled`. Done and cancelled are terminal. A completed task has `completed_at`. Generic PATCH updates title/description only.

**Review** belongs only to Cycle. There is no polymorphic subject and no ReviewType in this phase. Period dates are optional and must fall inside the Cycle when provided.

**DailyExecution** is frozen Phase 0 infrastructure. Existing table, repository, service, and routes remain for compatibility. Do not add tables or fields. Future daily execution should be derived primarily from Task activity.

Identifiers are UUID v7. Timestamps are UTC. API timestamps are RFC3339. Calendar dates are `YYYY-MM-DD`.

Status changes use explicit operations (`activate`, `close`, `complete`, `start`, `cancel`, `archive`). Do not accept arbitrary status mutation through generic PATCH.

Foreign keys use `ON DELETE RESTRICT`. No cascading wipes. Historical records are not hard-deleted.

## Persistence

SQLite is the source of truth. Schema changes go through `crates/forge-server/migrations/`. Never edit an applied migration; add a new numbered file.

Connections enable foreign keys, WAL, and a busy timeout. Queries that list children are indexed on the parent FK. The leftover `daily_executions.execution_date` index remains.

## API

JSON request/response DTOs live in `forge-server`. Domain entities are not the HTTP contract. Key Result responses expose derived `current_value` and `progress` as read-only fields.

Errors:

```json
{ "error": { "code": "not_found", "message": "..." } }
```

Codes: `bad_request`, `domain`, `not_found`, `conflict`, `persistence`.

See [api.md](api.md) for routes and [database.md](database.md) for tables.

## Tests

- Domain: constructors and invariants, no I/O
- Application: in-memory repositories (`#[cfg(test)]`)
- Server: tempfile SQLite + repository round-trips + Axum `oneshot`

## Not in Phase 1A

Auth, cloud, AI, notifications, calendar, mobile, analytics, permissions, collaboration, frontend, Tauri, ReviewType, polymorphic reviews, a Vision entity.

Known limitations: no user timezone for daily dates, no pagination, no KR scoring beyond derived progress, DailyExecution is still a leftover write surface.
