# Changelog

All notable changes to Forge are documented here.

## [Unreleased]

## [0.4.1] — 2026-09-06

### Product experience

- Refined Forge's product identity and bilingual positioning.
- Aligned the Chinese About Forge tagline with the official brand line.

### Visual identity

- Added the official Forge icon and refreshed application icon assets.
- Refreshed the README with new product visuals and bilingual product messaging.

## [0.4.0] — 2026-09-06

### Added

- First-run Welcome experience for new users
- Example Workspace demonstrating the complete Forge execution model
- Lightweight "How Forge works" concept tour
- Persistent onboarding / example state

### Improved

- Execution-focused Today experience
- Empty states with contextual next actions
- Project context displayed in Today task rows
- Clear distinction between Key Result Check-ins and Task completion
- Consistent navigation and Back behavior
- Form submission/loading states
- Keyboard focus visibility
- Bilingual product identity and About Forge content
- Consistent English / Chinese placeholders and terminology
- Cycles empty-state UX

### Fixed

- Duplicate Create Cycle action in the empty Cycles page
- Example exit navigation history
- Error states for onboarding / example operations
- Duplicate form submission opportunities
- Terminology ambiguity between progress and task completion

### Notes

- Windows installer remains unsigned. Windows SmartScreen may display a warning.

## [0.3.3] — 2026-09-05

### Added

- GitHub Actions 在推送 `vMAJOR.MINOR.PATCH` tag 后自动构建 Windows x64 NSIS installer，并创建正式 GitHub Release

### Fixed

- 在 `src-tauri` 的 clippy / test / Tauri build 之前生成 forge-server sidecar，避免全新 CI runner 因 sidecar 缺失失败

### Changed

- GitHub 官方 Actions 升级到 Node.js 24 runtime（`actions/checkout@v5`、`actions/setup-node@v5`、`actions/upload-artifact@v6`）

## [0.3.2] — 2026-09-05

### Fixed

- Today 在已有 Cycle 但当天没有任务时，改为从已有周期继续规划，而不再引导用户去创建 Cycle
- 日期输入恢复选择器，同时保持 `YYYY/MM/DD` 显示
- 统一各层级创建表单的示例 placeholder，并去掉产品自身相关示例

## [0.3.1] — 2026-09-05

### Fixed

- 修复左侧导航栏跟随主内容滚动的问题
- 增加各层级 detail 页面明确的返回导航
- 移除 Cycle 空状态中重复的创建按钮
- 为主要实体输入框增加示例 placeholder
- 统一日期显示格式为 `YYYY/MM/DD`

## [0.3.0] — 2026-08-31

### Added

- Settings page with language, theme, About, and update check
- Simplified Chinese and English UI, including Follow system
- Dark mode, including Follow Windows system theme
- Check for updates from GitHub Releases and download the Windows x64 installer

### Changed

- Existing UI copy now goes through a shared i18n catalog instead of hardcoded Chinese/English strings

### Technical

- Language and theme are stored in the Tauri app config directory, not in the SQLite business database
- Update check uses GitHub Releases; Forge never silently overwrites the current install

## [0.2.1] — 2026-08-30

### Added

- Windows NSIS installer for Forge
- Bundled local Forge backend
- Automatic backend startup when launching the desktop app
- Persistent local application data storage

### Changed

- Forge can now be installed and launched by non-technical Windows users without a terminal
- Desktop production backend uses a dedicated local port and application data directory

### Fixed

- Desktop startup no longer requires manually running `forge-server`

### Technical

- Added Tauri sidecar packaging for `forge-server`
- Added backend health-check before opening the desktop UI
- Added Windows Job Object lifecycle management for the backend process
- Switched Windows installer target from WiX/MSI to NSIS
- Existing v0.2.0 database migration and product functionality remain unchanged

## [0.2.0] — 2026-08-30

### Added

- 开始任务时，必要的上级周期、目标和项目会自动进入进行中
- 关键结果支持四种进展方式：数值、百分比、里程碑、定性
- 关键结果可以记录文字型进展说明，而不必勉强填一个数字
- Today / 任务执行流程继续保持简单直接
- MIT License

### Changed

- 草稿项目现在可以直接创建任务，不必先手动开始项目
- 界面里的「激活」统一改为更自然的「开始」
- Check-in 在中文界面统一为「更新进展」
- 更新进展的输入方式会随关键结果类型变化
- README 重写为简洁的产品介绍和快速上手

### Fixed

- 已经开始执行时，上级周期或目标仍停在规划中 / 未开始
- 关键结果只能用数值表达
- 部分中文术语是英文直译，读起来不自然

### Technical

- 新增数据库 migration `0004`
- 现有 numeric 关键结果与进展记录无损迁移
- HTTP 路由以及 Today / scheduling / DailyExecution 契约保持稳定
