# Fix whitespace detection bug in parse_from_argv

## Description

Fix the whitespace detection bug in `parse_from_argv` method at lines 1135 and 1148 of `unilang_parser/src/parser_engine.rs`. Change from checking only spaces (`.contains(' ')`) to checking all whitespace characters (`.chars().any(|c| c.is_whitespace())`).

This is a critical 2-line fix that enables the parser to properly quote values containing tabs, newlines, and other non-space whitespace, preserving argv token boundaries as designed.

This task implements the fix after tests are written in #081.

## Requirements

-   All work must strictly adhere to all applicable rulebooks
    (discover via `prompt .rulebooks.relevant`)
-   Tests from task #081 must be written and failing before implementing fix

## Acceptance Criteria

-   Line 1135 changed from `value.contains( ' ')` to `value.chars().any(|c| c.is_whitespace())`
-   Line 1148 changed from `arg.contains( ' ')` to `arg.chars().any(|c| c.is_whitespace())`
-   All tests from task #081 now pass
-   Previously ignored tests (`test_argv_tab_characters`, `test_argv_newline_characters`) now pass
-   No regressions in existing tests
-   Full test suite passes: `w3 .test l::3` on both unilang and unilang_parser crates
