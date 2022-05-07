#![ warn( rust_2018_idioms ) ]
#![ warn( missing_debug_implementations ) ]
#![ warn( missing_docs ) ]

// #![ allow( dead_code ) ]
// #![no_std]

//!
//! Tools for writing and running tests.
//!

#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/Readme.md" ) ) ]

// pub extern crate paste;
pub use ::paste;

///
/// Dependencies.
///

pub mod dependencies
{
  pub use paste;
  pub use trybuild;
  pub use anyhow;
  pub use rustversion;
}

///
/// Mechanism to define test suite. This macro encourages refactoring the code of the test in the most readable way, gathering a list of all test routines at the end of the test file.
///

#[ macro_export ]
macro_rules! test_suite
{
  () => { };
  (
    $Name : ident ,
    $( $Rest : tt )*
  )
  =>
  {
    #[test]
    fn $Name()
    {
      $crate::paste::paste!([< $Name _test >])()
    }
    $crate::test_suite!( $( $Rest )* );
  };
}

// #[ macro_export ]
// macro_rules! test_suite
// {
//   ( $( $Name : ident ),* $(,)? ) =>
//   {
//     $( #[test] fn $Name() { $crate::paste::paste!([< _ $Name >])() } )*
//     // $( #[test] fn $Name() { concat_idents!( _, $Name )() } )*
//   }
//   // ( $( $Name : ident ),* $(,)? ) =>
//   // {
//   //   // $( #[test] fn concat_idents!( $Name, _test )() { $Name() } )*
//   //   $( #[test] fn paste!([< $Name _test >])() { $Name() } )*
//   // }
// }

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
