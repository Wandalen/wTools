//! Tests for compile-time and runtime assertions using `trybuild`.
fn main()
{
  let t = trybuild::TestCases::new();
  t.compile_fail( "tests/inc/snipet/cta_mem_same_size_fail.rs" );
  t.compile_fail( "tests/inc/snipet/cta_ptr_same_size_fail.rs" );
  t.compile_fail( "tests/inc/snipet/cta_true_fail.rs" );
  t.compile_fail( "tests/inc/snipet/cta_type_same_align_fail.rs" );
  t.compile_fail( "tests/inc/snipet/cta_type_same_size_fail.rs" );
}