//! ## Test Matrix for `strs_tools` Unescaping
//!
//! This matrix details test cases for verifying the unescaping behavior of the `strs_tools` crate,
//! specifically for strings containing various escape sequences.
//!
//! **Test Factors:**
//! - Input String: Contains various escape sequences (backslash, double quote, single quote, newline, tab)
//! - Expected Unescaped String: The string after `strs_tools` unescaping.
//!
//! ---
//!
//! **Test Combinations:**
//!
//! | ID | Aspect Tested | Input String | Expected Unescaped String | Notes |
//! |---|---|---|---|---|
//! | T6.1 | Basic unescaping | `r#""a\\b\"c\'d\ne\tf""#` | `a\b"c'd\ne\tf` | Verifies handling of common escape sequences. |

use strs_tools::string::split;

/// Tests basic unescaping of a string containing various escape sequences using `strs_tools`.
/// Test Combination: T6.1
#[test]
fn temp_strs_tools_unescaping()
{
    let input = r#""a\\b\"c\'d\ne\tf""#; // Raw string literal to avoid Rust's unescaping
    let delimiters = vec![ " " ]; // Simple delimiter, not relevant for quoted string
    let split_iterator = split::SplitOptionsFormer::new(delimiters)
    .src( input )
    .preserving_delimeters( true )
    .quoting( true )
    .perform();

    let splits = split_iterator.collect::< Vec< _ > >();
    assert_eq!(splits.len(), 1);
    let s = &splits[0];
    assert_eq!(s.string, "a\\b\"c'd\ne\tf"); // Expected unescaped by strs_tools
}