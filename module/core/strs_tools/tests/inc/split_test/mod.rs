#![ cfg( feature = "string_split" ) ]

//! # Test Suite for `strs_tools::string::split`
//!
//! This module contains a comprehensive suite of tests for the string splitting
//! functionality provided by `strs_tools::string::split::SplitBuilder` and its
//! associated methods.
//!
//! ## Test Matrix
//!
//! The following matrix outlines the various factors and combinations tested.
//! This serves as a guide for ensuring comprehensive coverage.
//! (Note: This is an initial representative snippet. The full matrix will evolve
//! as tests are migrated and new specific cases are identified and covered.)
//!
//! **Factors:**
//! *   `F1: Input String`: Empty, Simple (no delimiters), Simple (with delimiters), Leading Delimiter, Trailing Delimiter, Consecutive Delimiters, All Delimiters, Contains Quotes.
//! *   `F2: Delimiter(s)`: Single Char, Multi-Char String, Multiple Strings, Empty String (if behavior defined), No Delimiter in String.
//! *   `F3: Preserving Empty Segments (PE)`: True, False (default).
//! *   `F4: Preserving Delimiters (PD)`: True, False (default).
//! *   `F5: Stripping Whitespace (S)`: True, False (default).
//! *   `F6: Quoting Enabled (Q)`: True, False (default).
//! *   `F7: Quote Character(s) (QC)`: Default (`"`, `'`), Custom (e.g., `|`). (Only if Q=True)
//! *   `F8: Preserving Quotes in Segments (PQ)`: True, False (default). (Only if Q=True)
//! *   `F9: Max Splits (N)`: None (default), 0, 1, `k` (where `1 < k < num_delimiters`), `num_delimiters`, `> num_delimiters`.
//! *   `F10: Indexing (Idx)`: None (default, all segments), `0` (first), `k` (positive), `-1` (last), `-k` (negative), Out-of-Bounds Positive, Out-of-Bounds Negative.
//!
//! **Test Matrix Snippet:**
//!
//! | Test_ID | Description        | Input      | Delimiters | PE  | PD  | S   | Q   | QC  | PQ  | N   | Idx | Expected Output                                  | Expected Index |
//! |---------|--------------------|------------|------------|-----|-----|-----|-----|-----|-----|-----|-----|--------------------------------------------------|----------------|
//! | M1.1    | Simple, default    | `a,b,c`    | `,`        | F   | F   | F   | F   | N/A | N/A | N/A | N/A | `["a", "b", "c"]` (kinds/indices omitted for brevity) | N/A            |
//! | M1.2    | Preserve empty     | `a,,c`     | `,`        | T   | F   | F   | F   | N/A | N/A | N/A | N/A | `["a", "", "c"]`                                 | N/A            |
//! | M1.3    | Strip, default     | ` a , b `  | `,`        | F   | F   | T   | F   | N/A | N/A | N/A | N/A | `["a", "b"]`                                     | N/A            |
//! | M1.4    | Quoting simple     | `"a,b",c`  | `,`        | F   | F   | F   | T   | def | F   | N/A | N/A | `["a,b", "c"]`                                   | N/A            |
//! | M1.5    | Indexing first     | `a,b,c`    | `,`        | F   | F   | F   | F   | N/A | N/A | N/A | 0   | `["a"]`                                          | Some(0)        |
//!

// Allow all lints for test modules.
#![allow(dead_code)]
#![allow(unused_imports)]

mod basic_split_tests;
mod preserving_options_tests;
mod stripping_options_tests;
mod quoting_options_tests;
mod indexing_options_tests;
mod combined_options_tests;
mod edge_case_tests;
mod quoting_and_unescaping_tests;
mod unescape_tests;