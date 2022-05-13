/// Internal namespace.
pub mod internal
{
  ///
  /// Mechanism to expand and format test case.
  /// This macro encourages refactoring the code of the test in the most readable way, gathering a list of all test routines at the end of the test file.
  ///
  /// # Sample
  /// use wtest_basic::*;
  ///
  /// //
  ///
  /// fn pass1()
  /// {
  ///   test_case!{ "equal ints" =>
  ///   {
  ///     assert_eq!( 1, 1 );
  ///   }}
  /// }
  ///
  /// //
  ///
  /// test_suite!
  /// { simple =>
  ///   pass1,
  /// }
  ///

  #[ macro_export ]
  macro_rules! test_case_
  {
    ( $name : expr => { $( $tree : tt )* } ) =>
    {
      $({
        println!( "Test case::{}", std::stringify!( $name ) );
        $tree
      })*
    };
  }

  ///
  /// Mechanism to expand and format test routine.
  /// This macro encourages refactoring the code of the test in the most readable way, gathering a list of all test routines at the end of the test file.
  ///
  /// # Sample
  /// use wtest_basic::*;
  ///
  /// //
  ///
  /// test_routine!{ pass1 =>
  /// {
  ///   test_case!{ "equal ints" =>
  ///   {
  ///     assert_eq!( 1, 1 );
  ///   }}
  /// }}
  ///
  /// //
  ///
  /// test_suite!
  /// { simple =>
  ///   pass1,
  /// }
  ///

  #[ macro_export ]
  macro_rules! test_routine
  {
    ( $name : ident => { $( $tree : tt )* } ) =>
    {
      fn $name()
      {
        println!( "Test routine::{}", std::stringify!( $name ) );
        $( $tree )*
      }
    };
  }

  //

  pub use test_case_ as test_case;
  pub use test_routine;
  pub use test_suite::test_suite;
}

/// Exposed namespace of the module.
pub mod exposed
{
  use super::internal as i;
  pub use super::prelude::*;
  pub use i::test_case;
  pub use i::test_routine;
  pub use i::test_suite;
}

pub use exposed::*;

/// Prelude to use: `use wtools::prelude::*`.
pub mod prelude
{
  use super::internal as i;
  pub use i::test_case;
  pub use i::test_routine;
  pub use i::test_suite;
}
