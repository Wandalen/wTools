# Changelog

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
**Documentation:** Full specification in spec.md
