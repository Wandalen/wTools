# Changelog

## [v0.6.0 | 2026-06-06] Comprehensive test surface coverage

**Tests:** 17 new integration tests across `output.rs` and `help.rs`; total 58 integration tests + 4 doc tests.
- `output.rs`: 12 new tests covering unicode_aware, stderr trailing-newline, boundary values (head/tail/width=0/1), overlapping head+tail windows, Both mode with empty streams, custom suffix, combined streams+head+width, is_default() discriminants for tail and width (FT-13 through FT-25, FT-26 through FT-33).
- `help.rs`: 5 new tests covering color field defaults (T10), empty-groups edge case (T11), opt_name_width minimum-padding (T12), tty_detect=true non-TTY ANSI suppression (T13), data_fmt dependency absence regression guard (T14).

**Docs:** Full test surface documentation added in `tests/docs/` — 5 spec files (feature/001, feature/002, api/001, api/002, invariant/001) covering all FT, AP, and IN spec cases; all spec cases implemented and passing.

## [v0.5.0 | 2026-05-17] Add cli_help_template module

**Added:** `cli_help_template` feature — typed, configurable CLI help text renderer.
- `CliHelpStyle` — 13 style parameters (indents, column widths, ANSI color codes) with opinionated defaults
- `CliHelpData` — typed sections: binary name, tagline, command groups, options, examples
- `CliHelpTemplate::render() -> String` — column-aligned, ANSI-colored, TTY-conditional output
- `ExampleEntry.desc: Option<String>` — optional inline annotation rendered as `  # {text}` suffix

**Fixed:** `ExampleEntry.desc` silently dropped — `emit_examples()` ignored the field entirely (BUG-007).

**Tests:** 9 integration tests for help module (T01–T09); T09 is a bug reproducer for the desc silent-drop fix. 2 new output tests added to fill gaps identified in test surface audit: `width_exact_boundary` (FT-11 — exact-boundary non-truncation) and `process_output_head_lines_omitted` (FT-12 — accurate lines_omitted under head filtering). Total: 42 integration tests.

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
