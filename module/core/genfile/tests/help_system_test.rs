//! Integration tests for the help system (FR9)
//!
//! ## Status: All cases deferred
//!
//! FR9 (Help System) is not yet implemented. The `.help`, `.`, and
//! `.command.help` commands do not exist in the current command registry.
//!
//! All FT- cases from `tests/docs/feature/009_help_system.md` are marked
//! `🔶 deferred` until FR9 is implemented. This file satisfies the anti-faking
//! check AF4 (`help_system_test.rs` must exist) without containing stub tests
//! that would silently pass without verifying real behavior.
//!
//! When FR9 is implemented, add test functions here covering:
//! - FT-01: `.help` command lists all available commands
//! - FT-02: `.command.help <name>` shows parameter documentation
//! - FT-03: Help output includes command descriptions
//! - FT-04: `.` (dot-only) shows abbreviated command list
