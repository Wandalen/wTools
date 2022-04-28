#![ warn( missing_docs ) ]
#![ warn( missing_debug_implementations ) ]
#![ allow( dead_code ) ]
// #![no_std]

//!
//! Tools for writing and running tests.
//!
//! # Sample
//! ``` rust
//! use wtest_basic::*;
//!
//! //
//!
//! fn _pass1()
//! {
//!   assert_eq!( true, true );
//! }
//!
//! //
//!
//! fn _pass2()
//! {
//!   assert_eq!( 1, 1 );
//! }
//!
//! //
//!
//! test_suite!
//! {
//!   pass1,
//!   pass2,
//! }
//!
//! ```

pub extern crate paste;

///
/// Dependencies.
///

pub mod dependencies
{
  pub use paste;
  // #[ cfg( test ) ]
  pub use trybuild;
  // #[ cfg( test ) ]
  pub use anyhow;
  // #[ cfg( test ) ]
  // #[ cfg( debug_assertions ) ]
  pub use rustversion;
}

/// Mechanism to define test suite. This macro encourages refactoring the code of the test in the most readable way, gathering a list of all test routines at the end of the test file.

#[ macro_export ]
macro_rules! test_suite
{
  ( $( $Name : ident ),* $(,)? ) =>
  {
    $( #[test] fn $Name() { $crate::paste::paste!([< _ $Name >])() } )*
    // $( #[test] fn $Name() { concat_idents!( _, $Name )() } )*
  }
  // ( $( $Name : ident ),* $(,)? ) =>
  // {
  //   // $( #[test] fn concat_idents!( $Name, _test )() { $Name() } )*
  //   $( #[test] fn paste!([< $Name _test >])() { $Name() } )*
  // }
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
