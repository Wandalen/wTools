/// Internal namespace.
mod internal
{

  ///
  /// Mechanism to define test suite.
  /// This macro encourages refactoring the code of the test in the most readable way, gathering a list of all test routines at the end of the test file.
  ///
  /// Name of test routine should have postfix `*_test`. In the index of test routine the postfix should be ommited.
  ///
  /// ### Sample
  /// use wtest_basic::*;
  ///
  /// //
  ///
  /// fn pass1_test()
  /// {
  ///   assert_eq!( true, true );
  /// }
  ///
  /// //
  ///
  /// fn pass2_test()
  /// {
  ///   assert_eq!( 1, 1 );
  /// }
  ///
  /// //
  ///
  /// test_suite!
  /// {
  ///   pass1,
  ///   pass2,
  /// }
  ///

  #[ macro_export ]
  macro_rules! test_suite
  {

    () => { };

    (
      $( #[ $Meta : meta ] )*
      $Name : ident ,
      $( $Rest : tt )*
    )
    =>
    {
      $( #[ $Meta ] )*
      #[test]
      fn $Name()
      {
        $crate::paste::paste!([< $Name _test >])()
      }
      $crate::test_suite!( $( $Rest )* );
    };

  }

  // /// Pass only if callback fails either returning error or panicing.
  //
  // pub fn should_throw< R, F : FnOnce() -> anyhow::Result< R > >( f : F ) -> anyhow::Result< R >
  // {
  //   f()
  // }

  //

  // #[panic_handler]
  // fn panic( info : &core::panic::PanicInfo ) -> !
  // {
  //   println!( "{:?}", info );
  //   loop {}
  // }

  pub use test_suite;
  // pub use test_suite_internals;
  // pub use index;
}

/// Exposed namespace of the module.
pub mod exposed
{
  use super::internal as i;
  pub use super::prelude::*;
  pub use i::test_suite;
}

pub use exposed::*;

/// Prelude to use: `use wtools::prelude::*`.
pub mod prelude
{
  use super::internal as i;
  pub use i::test_suite;
}
