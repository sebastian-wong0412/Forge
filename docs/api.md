# API

Base URL: `http://127.0.0.1:8080` by default.

JSON DTOs are the HTTP contract. Domain entities are not serialized directly.

Errors:

```json
{ "error": { "code": "not_found", "message": "..." } }
```

| Code | Status |
|---|---|
| `bad_request` | 400 |
| `domain` | 422 |
| `not_found` | 404 |
| `conflict` | 409 |
| `persistence` | 500 |

IDs are UUID v7 strings. Timestamps are RFC3339. Dates are `YYYY-MM-DD`.

Lifecycle status is never accepted on generic `PATCH`. Use the explicit `POST` operations below.

## Active Phase 1A routes

### Cycles

| Method | Path | Notes |
|---|---|---|
| `GET` | `/api/v1/cycles` | List |
| `POST` | `/api/v1/cycles` | Create (`planning`) |
| `GET` | `/api/v1/cycles/:id` | |
| `PATCH` | `/api/v1/cycles/:id` | Name and dates only |
| `POST` | `/api/v1/cycles/:id/activate` | `planning` → `active` |
| `POST` | `/api/v1/cycles/:id/close` | `active` → `closed` |
| `POST` | `/api/v1/cycles/:id/archive` | Any non-archived → `archived` |

Create body: `{ "name", "start_on", "end_on" }`.

### Objectives

| Method | Path | Notes |
|---|---|---|
| `GET` | `/api/v1/cycles/:cycle_id/objectives` | |
| `POST` | `/api/v1/cycles/:cycle_id/objectives` | Parent must allow tree mutation |
| `GET` | `/api/v1/objectives/:id` | |
| `PATCH` | `/api/v1/objectives/:id` | Title, description, dates |
| `POST` | `/api/v1/objectives/:id/activate` | |
| `POST` | `/api/v1/objectives/:id/complete` | |
| `POST` | `/api/v1/objectives/:id/archive` | |

### Key Results

| Method | Path | Notes |
|---|---|---|
| `GET` | `/api/v1/objectives/:objective_id/key-results` | |
| `POST` | `/api/v1/objectives/:objective_id/key-results` | |
| `GET` | `/api/v1/key-results/:id` | Includes derived fields |
| `PATCH` | `/api/v1/key-results/:id` | Title, description, start/target/unit |
| `POST` | `/api/v1/key-results/:id/activate` | |
| `POST` | `/api/v1/key-results/:id/complete` | Explicit; not triggered by progress |
| `POST` | `/api/v1/key-results/:id/archive` | |

Create/update accept `start_value` and optional `target_value`. They do **not** accept `current_value` as an authoritative write.

Response includes:

- `start_value`
- `current_value` (derived, read-only)
- `target_value`
- `progress` (derived, read-only, `null` when undefined)

### Check-ins

| Method | Path | Notes |
|---|---|---|
| `POST` | `/api/v1/key-results/:id/check-ins` | Append only |
| `GET` | `/api/v1/key-results/:id/check-ins` | History |

There is no PATCH or DELETE. A new value always creates a new row.

Body: `{ "value", "note?", "checked_on" }`.

`checked_on` is the date the progress occurred. `created_at` is when Forge stored the check-in.

Allowed only when the Key Result is draft or active, the Objective is not completed/archived, and the Cycle is not closed/archived.

### Projects

| Method | Path | Notes |
|---|---|---|
| `GET` | `/api/v1/objectives/:objective_id/projects` | |
| `POST` | `/api/v1/objectives/:objective_id/projects` | |
| `GET` | `/api/v1/projects/:id` | |
| `PATCH` | `/api/v1/projects/:id` | Title and description |
| `POST` | `/api/v1/projects/:id/activate` | Required before creating tasks |
| `POST` | `/api/v1/projects/:id/complete` | |
| `POST` | `/api/v1/projects/:id/archive` | |

`/api/v1/key-results/:id/projects` is no longer an active route.

### Tasks

| Method | Path | Notes |
|---|---|---|
| `GET` | `/api/v1/projects/:project_id/tasks` | |
| `POST` | `/api/v1/projects/:project_id/tasks` | Project must be active |
| `GET` | `/api/v1/tasks/:id` | |
| `PATCH` | `/api/v1/tasks/:id` | Title and description only |
| `POST` | `/api/v1/tasks/:id/start` | `todo` → `in_progress` |
| `POST` | `/api/v1/tasks/:id/complete` | `in_progress` → `done`, sets `completed_at` |
| `POST` | `/api/v1/tasks/:id/cancel` | `todo` or `in_progress` → `cancelled` |

Cancelled tasks are not active work.

### Reviews

| Method | Path | Notes |
|---|---|---|
| `GET` | `/api/v1/cycles/:cycle_id/reviews` | |
| `POST` | `/api/v1/cycles/:cycle_id/reviews` | Allowed after close/archive |

Body: `{ "content", "period_start?", "period_end?" }`.

No ReviewType. No polymorphic subject. Phase 1A does not expose get/update/archive for reviews.

## Removed / deprecated active surfaces

These are not registered on the Phase 1A router:

- `/api/v1/visions/...`
- `/api/v1/key-results/:id/projects`
- `/api/v1/reviews` as a polymorphic collection

## Frozen Phase 0 DailyExecution surface

Still mounted for compatibility. Not part of the active Phase 1A product model. Do not extend.

| Method | Path |
|---|---|
| `GET` | `/api/v1/tasks/:task_id/daily-executions` |
| `POST` | `/api/v1/tasks/:task_id/daily-executions` |
| `GET` | `/api/v1/daily-executions?date=YYYY-MM-DD` |
| `GET` | `/api/v1/daily-executions/:id` |
| `PATCH` | `/api/v1/daily-executions/:id` |

## Health

`GET /health` → `{ "status": "ok" }`
