# Task Management

This document serves as the **single source of truth** for all project work.

## Tasks Index

**Execution Order**: Task 002 first (blocks all), then tasks 003-006 can run in parallel, then path decision determines 007-010.

| Order | ID | Advisability | Value | Easiness | Safety | Priority | Status | Task | Description |
|-------|-----|--------------|-------|----------|--------|----------|--------|------|-------------|
| 1 | [002](002_create_specification.md) | 1440 | 10 | 6 | 8 | 3 | ✅ (Completed) | Create time_tools Specification | Create comprehensive spec.md documenting current API, architecture, requirements (FOUNDATION - BLOCKS ALL) |
| 2 | [003](003_document_api.md) | 1200 | 10 | 5 | 8 | 3 | ✅ (Completed) | Document API in readme.md | Add API reference, examples, performance characteristics to readme (depends: 002) |
| 3 | [004](004_add_edge_case_tests.md) | 1120 | 10 | 4 | 7 | 4 | ✅ (Completed) | Add Edge Case Tests | Add comprehensive edge case tests for >90% coverage (depends: 002) |
| 4 | [005](005_improve_error_handling.md) | 960 | 10 | 8 | 8 | 1.5 | ✅ (Completed) | Document Panic Conditions | Document panic conditions, improve panic messages (depends: 002) |
| 5 | [006](006_infrastructure_cleanup.md) | 800 | 8 | 10 | 10 | 1 | ✅ (Completed) | Infrastructure Cleanup | Update .gitignore, version decision (depends: 002-005) |
| 6 | [007](007_add_chrono_dependency.md) | TBD | - | - | - | - | 🔄 (Deferred) | Add Chrono Dependency | Deferred - time_tools stays minimal |
| 7 | [008](008_implement_duration_formatting.md) | TBD | - | - | - | - | 🔄 (Deferred) | Duration Formatting | Deferred - formatting stays in consuming crates |
| 8 | [009](009_implement_timestamp_formatting.md) | TBD | - | - | - | - | 🔄 (Deferred) | Timestamp Formatting | Deferred - formatting stays in consuming crates |
| 9 | [010](010_integration_and_cleanup.md) | 800 | 8 | 10 | 10 | 1 | ✅ (Completed) | Cleanup (Defer Path) | Temp file cleanup, knowledge extraction |
| 10 | [001](001_extract_time_formatting_from_wplan.md) | 432 | 9 | 3 | 8 | 2 | 🔄 (Deferred) | Extract Time Formatting from wplan | Deferred - see spec.md Development History for lessons learned |

## Issues Index

| ID | Title | Related Task | Status |
|----|-------|--------------|--------|

## Issues

_(No issues yet)_
