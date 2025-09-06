//! Tests for compile-time and runtime assertions using `trybuild`.

/// These tests verify that compile-time assertions properly fail when conditions are not met.
/// The test cases are run without the `diagnostics_compiletime_assertions` feature
/// to ensure the assertions actually trigger compile errors.
fn main()
{
  // Skip trybuild tests if diagnostics_compiletime_assertions is enabled
  // since the assertions won't fail as expected
  #[cfg(not(feature = "diagnostics_compiletime_assertions"))]
  {
    let t = trybuild::TestCases::new();
    t.compile_fail( "tests/inc/snipet/cta_mem_same_size_fail.rs" );
    t.compile_fail( "tests/inc/snipet/cta_ptr_same_size_fail.rs" );
    t.compile_fail( "tests/inc/snipet/cta_true_fail.rs" );
    t.compile_fail( "tests/inc/snipet/cta_type_same_align_fail.rs" );
    t.compile_fail( "tests/inc/snipet/cta_type_same_size_fail.rs" );
  }
}
