
//!
//! Helpers for testing.
//!

// use super::*;

/// Internal namespace.
mod private
{

  // zzz : move here test tools

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

  // pub use test_suite;
  // pub use test_suite_internals;
  // pub use index;

  ///
  /// Required to convert integets to floats.
  ///

  #[ macro_export ]
  macro_rules! num
  {

    () =>
    {
    };

    ( $num : expr ) =>
    {
      num_traits::cast::< _, T >( $num ).unwrap()
    };

    ( $( $num : expr ),+ ) =>
    {(
      $( num_traits::cast::< _, T >( $num ).unwrap() ),+
    )};

  }

  ///
  /// Test a file with documentation.
  ///

  #[ macro_export ]
  macro_rules! doc_file_test
  {
    ( $file:expr ) =>
    {
      #[ allow( unused_doc_comments ) ]
      #[ cfg( doctest ) ]
      #[ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/", $file ) ) ]
      extern { }
    };
  }

  pub use num;
  pub use doc_file_test;
}

//

crate::mod_interface!
{
  // exposed use super;
  exposed use super::super::helper;

  prelude use
  {
    num,
    doc_file_test,
  };
}
