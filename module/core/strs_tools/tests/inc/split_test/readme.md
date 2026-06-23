# Split Test Suite

Comprehensive test matrix for `strs_tools::string::split` covering 10 factors.

| File | Responsibility |
|------|----------------|
| `basic_split_tests.rs` | Core splitting with default options |
| `combined_options_tests.rs` | Multiple options combined interaction tests |
| `edge_case_tests.rs` | Empty input, no delimiters, boundary conditions |
| `indexing_options_tests.rs` | Segment indexing with positive and negative indices |
| `preserving_options_tests.rs` | Empty segment and delimiter preservation tests |
| `quoting_and_unescaping_tests.rs` | Quote handling with escape sequence unescaping |
| `quoting_options_tests.rs` | Quote character and preservation option tests |
| `split_behavior_tests.rs` | Split behavior regression and comparison tests |
| `stripping_options_tests.rs` | Whitespace stripping option tests |
