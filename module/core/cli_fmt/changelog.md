# Changelog

## [v0.5.0 | 2026-05-17] Add cli_help_template module

**Added:** `cli_help_template` feature — typed, configurable CLI help text renderer.
- `CliHelpStyle` — 13 style parameters (indents, column widths, ANSI color codes) with opinionated defaults
- `CliHelpData` — typed sections: binary name, tagline, command groups, options, examples
- `CliHelpTemplate::render() -> String` — column-aligned, ANSI-colored, TTY-conditional output
- `ExampleEntry.desc: Option<String>` — optional inline annotation rendered as `  # {text}` suffix

**Fixed:** `ExampleEntry.desc` silently dropped — `emit_examples()` ignored the field entirely (issue-T09).

**Tests:** 9 integration tests (T01–T09); T09 is a bug reproducer for the desc silent-drop fix.

## [v0.4.0 | 2026-04-19] Dependency version bump

**Updated:** Dependency version alignment with workspace release.

## [v0.3.0 | 2025-12-19] Dependency version bump

**Updated:** Dependency version alignment with workspace release.

## [v0.2.0 | 2025-11-29] Renamed from cli_tools; stream ordering fix

**Renamed:** `cli_tools` → `cli_fmt` to reflect scope (CLI formatting utilities).

**Fixed:** Correct stream ordering in merged output — stderr now appears before stdout
as required by CLI convention.

## [v0.1.0 | 2025-11-29] Initial Release

**Created:** New crate for CLI-application-specific utilities.

**Rationale:** Separated from `strs_tools` to maintain clear architectural boundaries.
- `strs_tools`: General-purpose string/ANSI manipulation
- `cli_fmt`: CLI-specific application helpers

**Modules:**
- `output` - CLI output processing
  - Migrated from `strs_tools::output` (which replaced `unilang::output`)
  - Head/tail line filtering
  - ANSI-aware width truncation
  - Stream merging (stdout/stderr)
  - Builder pattern API

**Dependencies:**
- `strs_tools` - Uses general-purpose functions:
  - `ansi::truncate_lines()` - Multi-line ANSI truncation with boundary detection
  - `string::lines::{head, tail, head_and_tail}` - Line filtering

**Migration from strs_tools:**
- Change: `use strs_tools::output::*` → `use cli_fmt::output::*`
- API unchanged - drop-in replacement

**Tests:** 31 comprehensive tests
