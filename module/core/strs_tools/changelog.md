* [v0.39.0 | 2025-11-29] Moved cli_output to cli_tools crate
  - **BREAKING CHANGE**: Removed `cli_output` module → moved to `cli_tools` crate
  - **Migration**: Change `use strs_tools::cli_output::*` to `use cli_tools::cli_output::*`
  - **Rationale**: Separate CLI-specific helpers from general-purpose string utilities
  - **Removed**: `cli_output` feature flag (no longer needed)
  - **Preserved**: General-purpose ANSI functions remain in `ansi` module
    - `ansi::truncate_if_needed()` - Boundary-aware truncation
    - `ansi::truncate_lines()` - Multi-line truncation with tracking
  - **Files Deleted**: `src/cli_output.rs` (474 lines), `tests/cli_output.rs` (352 lines)
  - **See**: cli_tools v0.1.0 for replacement functionality

* [v0.38.0 | 2025-11-29] Enhanced ANSI truncation with boundary detection
  - **New Function**: `ansi::truncate_if_needed()` - Truncate only if text exceeds max_width
  - **New Function**: `ansi::truncate_lines()` - Multi-line truncation with tracking
  - **Bug Fix**: Prevents incorrect truncation of text that fits exactly within width limit
  - **Use Case**: General-purpose ANSI text truncation (terminals, logs, any width-constrained display)
  - **NOT CLI-specific**: Pure text transformation, reusable across applications
  - **Unicode Support**: Both char-based (Tier 1) and grapheme-based (Tier 2) versions
  - **Tests**: 6 new tests for boundary detection
  - **Files Modified**: `src/ansi/truncate.rs`, `src/ansi/mod.rs`, `spec.md`
  - **Preparation**: Extract general-purpose logic before moving cli_output to cli_tools crate

* [v0.37.0 | 2025-11-29] Added `cli_output` module for unified CLI output processing
  - **New Module**: `cli_output` - ANSI-aware CLI output processing with head/tail filtering, width truncation, and stream merging
  - **New Module**: `string::lines` - Line-based text operations (head, tail, head_and_tail)
  - **Feature**: `cli_output` feature flag (requires `enabled`, `ansi`, `string_split`)
  - **Migrated**: Replaces deprecated `unilang::output` module to eliminate code duplication (449 lines)
  - **API**: Builder pattern configuration (`OutputConfig::default().with_head(10)`)
  - **Improvements**: Configurable suffix, proper width boundary detection, two-tier Unicode support
  - **Tests**: 31 comprehensive tests for cli_output module
  - **Bug Fix**: Width truncation now correctly checks visible width before truncating (doesn't truncate text that fits exactly)
  - **Files Added**: `src/cli_output.rs` (474 lines), `src/string/lines.rs` (194 lines), `tests/cli_output.rs` (352 lines)
  - **Files Modified**: `Cargo.toml`, `src/lib.rs`, `src/string/mod.rs`
  - **Specification**: Added Section 2.7 documenting cli_output module

* [Increment 1 | 2025-07-08 09:58 UTC] Added a failing test case to `strs_tools` to reproduce the iterator compilation error.
* [Increment 2 | 2025-07-08 10:01 UTC] Corrected the `IntoIterator` implementation for `SplitOptions` and fixed the test case.
*   [Increment 2 | 2025-07-13 12:18 UTC] Implemented custom flag type for `SplitBehavior` and added tests.
*   [Increment 3 | 2025-07-13 12:34 UTC] Confirmed `bitflags` usage was already replaced by custom type in `split.rs` and verified compilation and tests.
*   [Increment 4 | 2025-07-13 12:35 UTC] Removed `bitflags` dependency from `Cargo.toml` and verified compilation and tests.
*   [Increment 5 | 2025-07-13 12:36 UTC] Finalized `bitflags` removal task, performed holistic review and verification.
* [Increment 5.1 | 2025-07-20 19:20 UTC] Fixed trailing whitespace handling in string splitting and resolved a compilation error.