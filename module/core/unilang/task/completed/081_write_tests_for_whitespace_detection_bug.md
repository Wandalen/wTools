# Write tests for whitespace detection bug in parse_from_argv

## Description

Create comprehensive test suite that reproduces the whitespace detection bug in `unilang_parser/src/parser_engine.rs`. The current implementation only checks for spaces (`.contains(' ')`) when determining if values need quoting, but fails to detect tabs, newlines, and other whitespace characters. This undermines the entire purpose of the argv-based parser, which was designed to preserve token boundaries.

This task is part of the TDD bug-fix sequence. Related tasks: #082 (implement fix).

## Requirements

-   All work must strictly adhere to all applicable rulebooks
    (discover via `prompt .rulebooks.relevant`)

## Acceptance Criteria

-   Test suite created in `unilang_parser/tests/argv_multiword_bug_test.rs`
-   Tests verify tabs within values fail (currently marked as ignored)
-   Tests verify newlines within values fail (currently marked as ignored)
-   Tests verify all Unicode whitespace characters
-   Tests document expected behavior vs current broken behavior
-   All new tests fail consistently, demonstrating the bug
-   Test documentation includes root cause explanation
