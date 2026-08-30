# Changelog

All notable changes to Forge are documented here.

## [Unreleased]

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
