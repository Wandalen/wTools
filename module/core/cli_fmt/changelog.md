# Changelog

## [v0.9.2 | 2026-06-23] Docs: formatting consistency fixes

**Docs:** Two formatting violations found by code_hyg_l1 round 4 and fixed.
- `task/completed/006_aspirational_test_surface.md` — tight list: removed 2 blank lines separating list items in the In Scope section (PRB-001; `l2_imp § Markdown Formatting : Tight Lists`)
- `docs/readme.md:3` — Scope heading: `## Scope` → `### Scope` (PRB-002; `l2_imp § Project Structure : Directory Readme Scope Section`)

## [v0.9.2 | 2026-06-23] Test hygiene: fix assertion gaps and doc placement

**Tests:** Resolved code_hyg_l1 audit findings in `tests/output.rs`; added AP-12 coverage and FT-41 passthrough feature (no API changes).
- `combined_streams_head_width` — added `assert!(result.width_truncated)` (FT-33 spec claim was unverified)
- `width_one_truncates` — added `assert!(!result.content.contains('→'))` (FT-17 suffix-absence claim was unverified)
- `width_exact_boundary` (BUG-005 reproducer) — moved 5-section `///` doc from module level to the function; module-level `//!` BUG-005 section removed
- `select_streams_both` — removed duplicate `// test_kind: bug_reproducer(BUG-006)` marker; `merge_streams_ordering` remains the canonical BUG-006 reproducer
- `output_config_with_width_zero_has_processing` — new AP-12 test: `with_width(0)` stores `Some(0)` not `None`; `has_processing() == true`, `is_default() == false`, but `width_truncated == false` at runtime (zero short-circuits the truncation stage)
- `output_passthrough` feature added (not in `default`/`full`) — new `tests/output_passthrough.rs` with `feature_flag_line_filtering_passthrough` (FT-41): verifies the `apply_line_filtering` passthrough branch when `string_split` is absent; run with `cargo nextest run --test output_passthrough --no-default-features --features output_passthrough`

## [v0.9.2 | 2026-06-23] Extend CliHelpData with grouped options, usage lines, and arguments

**Added:** `CliHelpData` extended with three new fields for richer help template rendering.
- `usage_lines: Vec<String>` — custom usage lines; renders each indented; falls back to `Usage: {binary}` when empty
- `arguments: Vec<OptionEntry>` — typed argument section with column-aligned padding; omitted when empty
- `option_groups: Vec<OptionGroup>` — named option group sections with per-group column padding; suppresses legacy `options` field when non-empty

**Changed:** `CliHelpData` marked `#[non_exhaustive]` — external struct literals rejected (E0639); callers use `CliHelpData::default()` + field assignment. `CliHelpData` now derives `Default` (all Vec fields empty, string fields `""`).

**Added:** `OptionGroup { name: String, entries: Vec<OptionEntry> }` — named option grouping type.

**Tests:** 27 new integration tests; 1 new compile-fail doc test (T-A08). Total (standard suite): 86 integration tests + 6 doc tests.
- `help.rs` (+18): T-A01..T-A07 (custom usage_lines, arguments, option_groups, CliHelpData::default), T-A09 (example construction pattern), T-B01..T-B10 (multi-entry usage, argument padding, empty command groups, empty data render, edge cases, suppression contracts, ordering).
- `output.rs` (+9): is_default_tail (FT-24), is_default_width (FT-25), stdout_filter_with_head (FT-36), head_tail_width_triple_combination (FT-37), width_empty_suffix_no_marker (FT-38), empty_stdout_stderr_with_head (FT-39), width_zero_with_head (FT-40), merge_streams_both_empty_infallible (AP-11), test_strs_tools_sole_runtime_dependency (IN-3).

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
