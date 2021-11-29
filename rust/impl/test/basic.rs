#![ warn( missing_docs ) ]
#![ allow( dead_code ) ]
// #![no_std]

//!
//! Tools for writing tests and runnint tests.
//!
//! # Sample
//! ``` rust
//! use wtest::test_suite;
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

/// Macro to define test suite. This macro encourages refactoring the code of the test in the most readable way, gathering a list of all test routines at the end of the test file.

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

/// Pass only if callback fails either returning error or panicing.

pub fn should_throw< R, F : FnOnce() -> anyhow::Result< R > >( f : F ) -> anyhow::Result< R >
{
  f()
}

//

// #[panic_handler]
// fn panic( info : &core::panic::PanicInfo ) -> !
// {
//   println!( "{:?}", info );
//   loop {}
// }

/// Macro asserts that two expressions are identical to each other. Unlike std::assert_eq it is removed from a release build.

#[macro_export]
macro_rules! debug_assert_id
{
  ( $( $arg : tt )+ ) =>
  {
    #[cfg(debug_assertions)]
    $crate::assert_eq!( $( $arg )+ );
  };
  // ( $left : expr, $right : expr $(,)? ) =>
  // {{
  //   match( &$left, &$right )
  //   {
  //     #[cfg(debug_assertions)]
  //     ( left_val, right_val ) =>
  //     {
  //       if !( *left_val == *right_val )
  //       {
  //         let kind = core::panicking::AssertKind::Eq;
  //         core::panicking::assert_failed
  //         (
  //           kind,
  //           &*left_val,
  //           &*right_val,
  //           core::option::Option::None,
  //         );
  //       }
  //     }
  //   }
  // }};
  // ( $left : expr, $right:expr, $( $arg : tt )+ ) =>
  // {{
  //   match( &$left, &$right )
  //   {
  //     #[cfg(debug_assertions)]
  //     ( left_val, right_val ) =>
  //     {
  //       if !(*left_val == *right_val)
  //       {
  //         let kind = core::panicking::AssertKind::Eq;
  //         core::panicking::assert_failed
  //         (
  //           kind,
  //           &*left_val,
  //           &*right_val,
  //           core::option::Option::Some( $crate::format_args!( $( $arg )+ ) ),
  //         );
  //       }
  //     }
  //   }
  // }};
}

/// Macro asserts that two expressions are identical to each other. Unlike std::assert_eq it is removed from a release build. Alias of debug_assert_id.

#[macro_export]
macro_rules! debug_assert_identical
{
  ( $( $arg : tt )+ ) =>
  {
    #[cfg(debug_assertions)]
    $crate::debug_assert_id!( $( $arg )+ );
  };
}

/// Macro asserts that two expressions are not identical to each other. Unlike std::assert_eq it is removed from a release build.

#[macro_export]
macro_rules! debug_assert_ni
{
  ( $( $arg : tt )+ ) =>
  {
    #[cfg(debug_assertions)]
    $crate::assert_ne!( $( $arg )+ );
  };
}

/// Macro asserts that two expressions are not identical to each other. Unlike std::assert_eq it is removed from a release build.

#[macro_export]
macro_rules! debug_assert_not_identical
{
  ( $( $arg : tt )+ ) =>
  {
    #[cfg(debug_assertions)]
    $crate::assert_ne!( $( $arg )+ );
  };
}

/// Macro asserts that expression is ture. Unlike std::assert it is removed from a release build.

#[macro_export]
macro_rules! debug_assert
{
  ( $( $arg : tt )+ ) =>
  {
    #[cfg(debug_assertions)]
    $crate::assert!( $( $arg )+ );
  };
}
