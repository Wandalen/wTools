//! Smoke testing of the package.

#[ test ]
fn smoke_test()
{
  // Verify crate imports and basic functionality
  use data_type as the_module;

  // Verify Either is accessible (when feature enabled)
  #[ cfg( feature = "either" ) ]
  {
    let _ = the_module ::Either ::Left::< i32, () >( 42 );
  }

  // Basic compilation smoke test - if this compiles and runs, the crate works
}
