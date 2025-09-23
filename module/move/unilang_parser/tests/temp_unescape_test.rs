//! ## Test Matrix for `strs_tools` Unescaping
//!
//! This matrix details test cases for verifying the unescaping behavior of the `strs_tools` crate,
//! specifically for strings containing various escape sequences.
//!
//! **Test Factors: **
//! - Input String: Contains various escape sequences (backslash, double quote, single quote, newline, tab)
//! - Expected Unescaped String: The string after `strs_tools` unescaping.
//!
//! ---
//!
//! **Test Combinations: **
//!
//! | ID | Aspect Tested | Input String | Expected Unescaped String | Notes |
//! |---|---|---|---|---|
//! | T6.1 | Basic unescaping | `r#""a\\b\"c\'d\ne\tf""#` | `a\b"c'd\ne\tf` | Verifies handling of common escape sequences. |

// xxx: temporarily disabled due to missing string_split feature
// use strs_tools ::string ::split;

/// Tests basic unescaping of a string containing various escape sequences using `strs_tools`.
/// Test Combination: T6.1
#[ ignore = "temporarily disabled due to missing string_split feature" ]
#[ test ]
fn temp_strs_tools_unescaping() 
{
  // xxx: temporarily disabled due to missing string_split feature
}
