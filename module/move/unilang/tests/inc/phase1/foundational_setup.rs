//!
//! Tests for the foundational setup of the crate.
//!

// The `super::*` import is not used in this file, but it is a common
// pattern in tests, so we keep it for consistency.
#[ allow( unused_imports ) ]
use super::*;

///
/// A compile-time test to ensure that the basic test case compiles.
///
#[ test ]
fn try_build()
{
  let t = test_tools::compiletime::TestCases::new();
  t.pass( "tests/inc/phase1/try_build.rs" );
}
