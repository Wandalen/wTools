//! Compile-fail tests covering spec cases where compilation must fail.
//!
//! Covered spec cases:
//! - `feature/002_into_constructors` FT-02: type annotation required for into map macros.
//!
//! Each `.rs` file in `tests/compile_fail/` must fail to compile. The companion `.stderr`
//! file records the expected compiler error and is generated automatically on the first run.

#[ cfg( feature = "collection_into_constructors" ) ]
#[ test ]
fn compile_fail()
{
  let t = trybuild ::TestCases ::new();
  t.compile_fail( "tests/compile_fail/*.rs" );
}
