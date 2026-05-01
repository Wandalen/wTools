//! Tests for compile-time and runtime assertions using `trybuild`.

/// These tests verify that compile-time assertions properly fail when conditions are not met.
/// The feature must be enabled so the macros are available and produce the expected compile errors.
fn main()
{
  // Run trybuild tests only when diagnostics_compiletime_assertions is enabled
  // so the macros are available and produce the expected compile errors
  #[ cfg(feature = "diagnostics_compiletime_assertions") ]
  {
  let t = trybuild ::TestCases ::new();
  t.compile_fail( "tests/inc/snipet/cta_mem_same_size_fail.rs" );
  t.compile_fail( "tests/inc/snipet/cta_ptr_same_size_fail.rs" );
  t.compile_fail( "tests/inc/snipet/cta_true_fail.rs" );
  t.compile_fail( "tests/inc/snipet/cta_type_same_align_fail.rs" );
  t.compile_fail( "tests/inc/snipet/cta_type_same_size_fail.rs" );
 }
}
