# Testing

3-tier test architecture for `claude_runner` CLI, organized by audience and concern.

## Architecture

```
testing/
├── readme.md              # This file — navigation and aggregate counts
├── param/                 # Tier 1: parameter edge cases (implementers)
│   ├── message.md
│   ├── dir.md
│   ├── continue.md
│   ├── max_tokens.md
│   ├── skip_permissions.md
│   ├── dry.md
│   ├── session_dir.md
│   └── model.md
├── param_group/           # Tier 2: group corner cases (testers)
│   ├── input.md
│   ├── environment.md
│   ├── behavior_flags.md
│   └── resource_control.md
└── command/               # Tier 3: command integration tests (users/QA)
    ├── run.md
    └── help.md
```

## Test Category Definitions

| ID Prefix | Category | Tier | Description |
|-----------|----------|------|-------------|
| EC | Edge Case | param/ | Single-parameter boundary conditions |
| CD | Co-Dependency | param_group/ | Direct parameter dependency rules |
| CC | Corner Case | param_group/ | Multi-parameter boundary intersections |
| IT | Integration Test | command/ | End-to-end command workflows |
| CSB | Command-Specific Behavior | command/ | Command vocabulary and semantic rules |
| RWS | Real-World Scenario | command/ | Common user workflow patterns |

## Audience Guide

| Audience | Start Here | Purpose |
|----------|------------|---------|
| Implementers | `param/` | Validate parameter parsing and validation logic |
| Testers | `param_group/` | Validate parameter interactions and corner cases |
| QA / Users | `command/` | Validate end-to-end behavior and acceptance |

## Test Priority Levels

| Priority | Criteria |
|----------|----------|
| P0 | Must pass; failure blocks release |
| P1 | Should pass; failure tracked as issue |
| P2 | Nice to have; failure is a minor concern |

## Aggregate Counts

| Tier | Files | Tests | Categories |
|------|-------|-------|------------|
| param/ | 8 | 58 | EC (58) |
| param_group/ | 4 | 16 | CC (15), CD (1) |
| command/ | 2 | 31 | IT (22), CSB (5), RWS (4) |
| **Total** | **14** | **105** | |

## Navigation

### param/ — Parameter Edge Cases

| File | Parameter | Type | EC |
|------|-----------|------|----|
| [message.md](param/message.md) | `message::` | MessageText | 8 |
| [dir.md](param/dir.md) | `dir::` | PathArg | 8 |
| [continue.md](param/continue.md) | `continue::` | bool | 6 |
| [max_tokens.md](param/max_tokens.md) | `max_tokens::` | TokenCount | 8 |
| [skip_permissions.md](param/skip_permissions.md) | `skip_permissions::` | bool | 6 |
| [dry.md](param/dry.md) | `dry::` | bool | 8 |
| [session_dir.md](param/session_dir.md) | `session_dir::` | PathArg | 7 |
| [model.md](param/model.md) | `model::` | ModelName | 7 |

### param_group/ — Group Corner Cases

| File | Group | Parameters | Tests |
|------|-------|------------|-------|
| [input.md](param_group/input.md) | Input | message | 3 |
| [environment.md](param_group/environment.md) | Environment | dir, session_dir | 4 |
| [behavior_flags.md](param_group/behavior_flags.md) | Behavior Flags | continue, skip_permissions, dry | 5 |
| [resource_control.md](param_group/resource_control.md) | Resource Control | max_tokens, model | 4 |

### command/ — Command Integration Tests

| File | Command | IT | CSB | RWS | Total |
|------|---------|----|-----|-----|-------|
| [run.md](command/run.md) | `.run` | 12 | 4 | 4 | 20 |
| [help.md](command/help.md) | `.help` | 10 | 1 | 0 | 11 |
