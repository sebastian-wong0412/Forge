# Agent working agreement

Read this before changing Forge. Also see `docs/architecture.md`, `docs/database.md`, and `docs/api.md`.

## Product

Forge is a **local-first** personal OKR and execution system. SQLite is the source of truth in this version. Do not add cloud infrastructure, authentication, sync, AI, a frontend, or Tauri unless the user explicitly asks.

Phase 1A planning root is **Cycle**. Vision is intentionally not implemented as a separate entity yet. Vision and Cycle are not the same concept; a long-term Vision layer may sit above Cycle later.

```
Cycle
├── Objective
│   ├── Key Result
│   │   └── Check-in
│   └── Project
│       └── Task
└── Review
```

`DailyExecution` is frozen Phase 0 legacy infrastructure. Do not extend it. Future daily execution should be derived primarily from Task activity.

## Before changing code

1. Inspect the existing repository and the layer you are in.
2. Understand the current architecture.
3. Explain the intended change briefly.
4. Make the smallest reasonable change.
5. Run `cargo fmt`.
6. Run `cargo clippy --workspace --all-targets -- -D warnings`.
7. Run `cargo test --workspace`.
8. Report what changed and any remaining issues.

Never rewrite large parts of the repository without necessity. Do not invent abstractions for a hypothetical future. Prefer explicit code.

## Layers

- `crates/forge-domain`: no Axum, SQLx, Tokio, HTTP, or SQLite details.
- `crates/forge-application`: no HTTP or SQLx types. Repository traits live here. Services take repositories as generics.
- `crates/forge-server`: SQLite implementations, migrations, Axum, config, tracing, `main`.

Dependency direction: server → application → domain.

## Coding

- Production code: no `unwrap()` / `expect()` unless there is a documented reason. Use `Result` and typed errors (`DomainError`, `AppError`, `ApiError`).
- Strong domain types where they prevent invalid states (`Title`, status enums, date containment).
- API DTOs are the JSON contract; do not serialize domain entities directly.
- IDs are UUID v7. Internal time is UTC. API times are RFC3339. Dates are `YYYY-MM-DD`.
- Lifecycle changes go through explicit operations. Do not accept arbitrary status mutation on generic PATCH.

## Database

- All schema changes are new migrations under `crates/forge-server/migrations/`.
- **Never edit an existing migration.**
- Enable foreign keys on every connection.
- Index FK columns and real query paths only. Do not over-index.
- Soft-archive; no hard deletes of historical records. `ON DELETE RESTRICT`.
- `visions` and `daily_executions` are leftover Phase 0 tables. Do not drop them in this phase.

## After each implementation step, report

1. Files created
2. Files modified
3. Architectural decisions
4. Commands executed
5. Test results
6. Known limitations
7. Recommended next step

Do not implement a later phase unless explicitly requested.
