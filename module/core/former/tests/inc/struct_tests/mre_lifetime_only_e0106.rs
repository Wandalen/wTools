#![allow(clippy::used_underscore_binding, clippy::all, warnings, missing_docs)]
// MRE test for E0106 "missing lifetime specifier" error in lifetime-only structs
// This test ensures we don't regress on lifetime-only struct handling

use super::*;

// Minimal reproducible example of E0106 error
#[ derive( Debug, PartialEq, former::Former ) ]
pub struct LifetimeOnlyMRE<'a> {
  data: &'a str,
}

/// Reproduces the E0106 "missing lifetime specifier" error that occurred 
/// when deriving Former for structs containing only lifetime parameters.
/// This test ensures we don't regress on lifetime-only struct handling.
// test_kind: mre
#[ test ]
fn test_lifetime_only_mre_e0106()
{
  let input = "test";
  let instance = LifetimeOnlyMRE::former().data( input ).form();
  assert_eq!( instance.data, "test" );
}