//! Integration tests for `strs_tools_meta` procedural macros
//!
//! Verifies combined usage of `optimize_split` and `optimize_match` macros
//! within the same expansion context.
//!
//! Unit tests for each macro individually: see `optimize_split_tests.rs`
//! and `optimize_match_tests.rs`. Edge cases: see `corner_cases_test.rs`.

#[ cfg( all( feature = "optimize_split", feature = "optimize_match" ) ) ]
use strs_tools_meta ::{ optimize_split, optimize_match };

/// Both macros usable in the same scope without conflict.
#[ cfg( all( feature = "optimize_split", feature = "optimize_match" ) ) ]
#[ test ]
fn integration_both_macros_in_same_scope()
{
  let parts = optimize_split!( "http://example.com/path", "/" );
  let matched = optimize_match!( "http://example.com/path", "example" );
  assert_eq!( parts[ 0 ], "http:" );
  assert!( matched.is_some() );
}
