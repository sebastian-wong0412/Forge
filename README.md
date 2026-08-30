# Forge

> Personal execution, organized around outcomes.

[![version](https://img.shields.io/badge/version-v0.2.0-1f1b14)](https://github.com/sebastian-wong0412/Forge)
[![rust](https://img.shields.io/badge/rust-1.85+-dea584)](https://www.rust-lang.org/)
[![react](https://img.shields.io/badge/ui-react%2019-61dafb)](https://react.dev/)
[![tauri](https://img.shields.io/badge/desktop-tauri%202-24c8db)](https://tauri.app/)
[![sqlite](https://img.shields.io/badge/data-sqlite-003b57)](https://sqlite.org/)
[![license](https://img.shields.io/badge/license-MIT-3c3c3c)](./LICENSE)

Forge is a local-first personal execution system for turning a season of intent into a day of work.

Plan a cycle. Name the outcome. Break it into projects and tasks. Then open Today — only the work you chose for that day is there.

Data lives in SQLite on your machine. No account. No cloud. No second source of truth.

The desktop UI is Simplified Chinese. HTTP identifiers stay in English.

## Core Concepts

```
Cycle
 └── Objective
      ├── Key Result
      └── Project
           └── Task
                └── Today
```

**Cycle** is a bounded stretch of time — a quarter, a launch window, a personal sprint.

**Objective** is the result you want inside that cycle.

**Key Result** is how you know the objective is moving. It can be a number, a percentage, a milestone, or a written outcome.

**Project** is a bundle of work. **Task** is what you actually start and finish.

**Today** is a projection of tasks you scheduled. It is not another list you maintain by hand.

## Features

- Outcome-oriented planning, not a dump of every open item
- Flexible Key Results: numeric, percentage, milestone, qualitative
- Parent states advance when work actually starts
- Daily execution through scheduled tasks
- Append-only progress history
- Local-first architecture: SQLite is the source of truth

## Quick Start

There is no installer yet. Run from source.

Prerequisites: Rust stable, Node.js 22+, npm.

The desktop window does not start the backend. Use two terminals:

```bash
# Terminal 1 — API
cargo run -p forge-server

# Terminal 2 — desktop
npm install
npm run tauri dev
```

Or open the UI in a browser:

```bash
npm run dev
```

| Surface | Address |
|---|---|
| API | `http://127.0.0.1:8080` |
| Vite | `http://localhost:1420` |

## Architecture

```
React / TypeScript
        ↓
      Tauri
        ↓
   Rust HTTP API
        ↓
   Application
        ↓
     Domain
        ↓
     SQLite
```

The backend is authoritative. The desktop client is a view.

More detail: [docs/architecture.md](docs/architecture.md) · [docs/api.md](docs/api.md) · [docs/database.md](docs/database.md)

## License

[MIT](./LICENSE)

Release notes: [CHANGELOG](./CHANGELOG.md)
