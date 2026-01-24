//! Verification test for readme.md example code
//!
//! This test ensures that the example code shown in readme.md actually compiles
//! and runs correctly. This prevents documentation drift.

#[cfg(test)]
mod readme_example_tests
{

  /// Test the basic example from readme.md
  #[ test ]
  fn test_readme_basic_example()
  {
    // This is the exact code from readme.md lines 78-106
    #[ cfg( feature = "enabled" ) ]
    #[ cfg( not( feature = "no_std" ) ) ]
    tests_impls!
    {
      fn pass1()
      {
        assert_eq!( true, true );
      }

      //

      fn pass2()
      {
        assert_eq!( 1, 1 );
      }
    }

    //
    #[ cfg( feature = "enabled" ) ]
    #[ cfg( not( feature = "no_std" ) ) ]
    tests_index!
    {
      pass1,
      pass2,
    }

    // If we got here, the macros work correctly
  }
}
