#![allow(dead_code)]
// #![no_std]

//

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

//

#[macro_export]
macro_rules! debug_assert_eq
{
  ( $( $arg : tt )+ ) =>
  ({
    #[cfg(debug_assertions)]
    assert_eq!( $( $arg )+ );
  });
  // ( $left : expr, $right : expr $(,)? ) =>
  // ({
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
  // });
  // ( $left : expr, $right:expr, $( $arg : tt )+ ) =>
  // ({
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
  // });
}

//

#[macro_export]
macro_rules! debug_assert
{
  ( $( $arg : tt )+ ) =>
  ({
    #[cfg(debug_assertions)]
    assert!( $( $arg )+ );
  });
}
