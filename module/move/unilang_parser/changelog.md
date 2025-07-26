# Changelog

* [Increment 1 | 2025-07-05 10:34 UTC] Added failing test for incorrect command path parsing.
* [Increment 2 | 2025-07-05 10:58 UTC] Correctly parse command paths instead of treating them as arguments.
*   Investigated and documented the correct usage of `strs_tools::string::split::SplitOptionsFormer` with dynamic delimiters to resolve lifetime issues.
* [Increment 1 | 2025-07-06 06:42 UTC] Investigated `strs_tools` API issues and proposed switching to `regex` for string splitting.
- **Increment 1:** Refactored the parser engine to use official, unified data structures, establishing a consistent foundation.
* [2025-07-20 13:54 UTC] Refactor: Parser now uses `strs_tools` for robust tokenization and unescaping.
* [2025-07-20 13:55 UTC] Chore: Analyzed test coverage and created a detailed Test Matrix for spec adherence.
* [2025-07-20 13:58 UTC] Test: Implemented comprehensive spec adherence test suite and fixed uncovered bugs.
*   [2025-07-20 14:46 UTC] Reverted `parser_engine.rs` to a monolithic function and fixed the "Empty instruction" error for input ".".
* [Increment 1.2 | 2025-07-26 05:57:37 UTC] Fixed `unilang_parser::tests::path_parsing_test::test_parse_path_with_dots` by removing `.` from the delimiters in `strs_tools::split` configuration in `module/move/unilang_parser/src/parser_engine.rs`.
* [Increment 2 | 2025-07-26 05:58:17 UTC] Correctly parsed paths with dots by modifying `strs_tools::split` configuration in `module/move/unilang_parser/src/parser_engine.rs`. Confirmed fix with `unilang_parser` and `unilang` integration tests.