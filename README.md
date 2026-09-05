# Forge

**Forge · 铸行**

**Turn intentions into execution.**

把意图，铸成行动。

Forge is a local-first personal system for moving from what matters to what you do today.

它不是又一个待办清单，也不是一份停在纸面上的目标表。Forge 关心的是中间那段距离：你知道什么重要，和你今天是否真的在做。

[Download for Windows](https://github.com/sebastian-wong0412/Forge/releases/latest) · [Releases](https://github.com/sebastian-wong0412/Forge/releases) · [Source](https://github.com/sebastian-wong0412/Forge)

---

## What is Forge?

Forge gives long-term intent a structure you can act on.

You name where you want to go. You decide what to achieve in a season. You measure progress, organize the work, and schedule the next concrete step. Then Today shows only what belongs to this day.

Forge 是一个个人执行系统：把模糊的意图和长期目标，通过层层拆解，最终落成真实、持续的行动。

Data lives in SQLite on your machine. No account. No cloud. No second source of truth.

数据保存在本机 SQLite。没有账号，也没有云端副本。

---

## From intention to execution

This is the meaning Forge is built around:

```
Vision
  ↓
Objective
  ↓
Key Result
  ↓
Project
  ↓
Task
  ↓
Daily Execution
```

| | English | 中文 |
|---|---|---|
| **Vision** | Where you want to go | 你想走向哪里 |
| **Objective** | What you want to achieve | 你想实现什么 |
| **Key Result** | How you measure progress | 如何衡量进展 |
| **Project** | How you move toward it | 如何推进它 |
| **Task** | What needs to be done | 具体需要做什么 |
| **Daily Execution** | What you do today | 今天真正做什么 |

In the current app, **Cycle** is the planning root: a bounded season of work — a quarter, a launch window, a personal sprint. **Today** is daily execution. It is a projection of the tasks you scheduled, not a second list you maintain by hand.

当前产品以 **Cycle（周期）** 作为规划起点，以 **Today（今日）** 作为每天的执行面。Vision 还不是独立实体；它是 Forge 意图中更上一层的方向，而不是已经实现的功能。

```
Cycle
 └── Objective
      ├── Key Result
      └── Project
           └── Task
                └── Today
```

---

## Why Forge?

Most tools stop at one side of the problem.

A todo app collects work. An OKR doc names outcomes. The gap between them is where plans go quiet.

Forge keeps the hierarchy only so it can collapse back into a day. A cycle exists so an objective has a season. A key result exists so progress is visible. A project exists so work has a place. A task exists so something can be started. Today exists so the rest of the structure can stay out of the way.

目标不是把结构堆得更高，而是让结构最终落到今天的行动上。

---

## Features

What Forge already does:

- Plan work in time-bounded **cycles**, with objectives, projects, and tasks
- Track progress with **key results**: numeric, percentage, milestone, or written
- Record **append-only check-ins** — history is not edited away
- Start work and let the necessary parent cycle, objective, and project become active
- Open **Today** and see only what you scheduled for that date
- Review a cycle when the season ends
- Use the desktop app in **Simplified Chinese** or **English**, or follow Windows
- Follow the Windows theme, or switch to dark
- Keep everything **local**: SQLite on this machine
- Install on **Windows x64** without Node, Rust, or a terminal
- Check GitHub Releases for a newer installer — Forge does not overwrite itself

---

## Preview

Product screenshots are not in this repository yet.

Use the latest Windows installer if you want to see Forge as it actually runs.

---

## Getting started

### Windows

Download [`Forge_0.3.3_x64-setup.exe`](https://github.com/sebastian-wong0412/Forge/releases/tag/v0.3.3) from [Releases](https://github.com/sebastian-wong0412/Forge/releases).

Install it, then open Forge from the Start Menu.

You do not need Node.js, Rust, Cargo, or a terminal. The local backend starts with the app. Data lives in `%LOCALAPPDATA%\app.forge.desktop`.

The installer is currently unsigned. Windows SmartScreen may show a warning.

macOS and Linux are not available.

### From source

For development only. Prerequisites: Rust stable, Node.js 22+, npm.

```bash
npm install
npm run tauri dev
```

That opens the desktop window and the local API. The backend can also run on its own:

```bash
cargo run -p forge-server
```

| Surface | Address |
|---|---|
| Desktop API | `http://127.0.0.1:17340` |
| Developer CLI API | `http://127.0.0.1:8080` |
| Vite | `http://localhost:1420` |

---

## Updates

Forge is in active iteration.

New Windows builds are published on [GitHub Releases](https://github.com/sebastian-wong0412/Forge/releases). In Settings, you can check for a newer version and download the installer. Forge will not replace the current install for you.

Release notes: [CHANGELOG](./CHANGELOG.md)

---

## Roadmap

Direction that is already in view:

- **First-run experience** — planned for v0.4: a quieter onboarding path, and an optional example workspace so the hierarchy is easier to feel
- **Vision** — a longer-term layer above Cycle; not implemented yet
- Daily execution will continue to come from **tasks and Today**, not from expanding the frozen `DailyExecution` leftover

No other large roadmap is promised here.

---

## Development

Forge is a local-first desktop app:

```
React / TypeScript
        ↓
      Tauri 2
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

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
npm test
npm run build
```

More detail: [docs/architecture.md](docs/architecture.md) · [docs/api.md](docs/api.md) · [docs/database.md](docs/database.md)

---

## License

[MIT](./LICENSE)
