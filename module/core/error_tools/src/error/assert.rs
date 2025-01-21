/// Define a private namespace for all its items.
mod private
{
  ///
  /// Macro asserts that two expressions are identical to each other. Unlike `std::assert_eq` it is removed from a release build.
  ///

  #[ macro_export ]
  macro_rules! debug_assert_id
  {
    ( $( $arg : tt )+ ) =>
    {
      #[cfg(debug_assertions)]
      // $crate::assert_eq!( $( $arg )+ );
      std::assert_eq!( $( $arg )+ );
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

  /// Macro asserts that two expressions are identical to each other. Unlike `std::assert_eq` it is removed from a release build. Alias of `debug_assert_id`.

  #[ macro_export ]
  macro_rules! debug_assert_identical
  {
    ( $( $arg : tt )+ ) =>
    {
      #[cfg(debug_assertions)]
      $crate::debug_assert_id!( $( $arg )+ );
    };
  }

  /// Macro asserts that two expressions are not identical to each other. Unlike `std::assert_eq` it is removed from a release build.

  #[ macro_export ]
  macro_rules! debug_assert_ni
  {
    ( $( $arg : tt )+ ) =>
    {
      #[cfg(debug_assertions)]
      // $crate::assert_ne!( $( $arg )+ );
      std::assert_ne!( $( $arg )+ );
    };
  }

  /// Macro asserts that two expressions are not identical to each other. Unlike `std::assert_eq` it is removed from a release build.

  #[ macro_export ]
  macro_rules! debug_assert_not_identical
  {
    ( $( $arg : tt )+ ) =>
    {
      #[cfg(debug_assertions)]
      // $crate::assert_ne!( $( $arg )+ );
      $crate::debug_assert_ni!( $( $arg )+ );
    };
  }

  // /// Macro asserts that expression is ture. Unlike std::assert it is removed from a release build.
  //
  // #[ macro_export ]
  // macro_rules! debug_assert
  // {
  //   ( $( $arg : tt )+ ) =>
  //   {
  //     #[cfg(debug_assertions)]
  //     $crate::assert!( $( $arg )+ );
  //   };
  // }

  #[ allow( clippy::useless_attribute, clippy::pub_use ) ]
  pub use debug_assert_id;
  #[ allow( clippy::useless_attribute, clippy::pub_use ) ]
  pub use debug_assert_identical;
  #[ allow( clippy::useless_attribute, clippy::pub_use ) ]
  pub use debug_assert_ni;
  #[ allow( clippy::useless_attribute, clippy::pub_use ) ]
  pub use debug_assert_not_identical;
}

/// Own namespace of the module.
#[ allow( unused_imports ) ]
pub mod own
{
  #[ allow( clippy::wildcard_imports ) ]
  use super::*;
  #[ doc( inline ) ]
  #[ allow( clippy::useless_attribute, clippy::pub_use ) ]
  pub use orphan::*;
}

#[ doc( inline ) ]
#[ allow( unused_imports ) ]
#[ allow( clippy::pub_use ) ]
pub use own::*;

/// Shared with parent namespace of the module
#[ allow( unused_imports ) ]
pub mod orphan
{
  #[ allow( clippy::wildcard_imports ) ]
  use super::*;
  #[ doc( inline ) ]
  #[ allow( clippy::useless_attribute, clippy::pub_use ) ]
  pub use exposed::*;
}

/// Exposed namespace of the module.
#[ allow( unused_imports ) ]
pub mod exposed
{
  #[ allow( clippy::wildcard_imports ) ]
  use super::*;
  #[ doc( inline ) ]
  #[ allow( clippy::useless_attribute, clippy::pub_use ) ]
  pub use prelude::*;
}

/// Prelude to use essentials: `use my_module::prelude::*`.
#[ allow( unused_imports ) ]
pub mod prelude
{
  #[ allow( clippy::wildcard_imports ) ]
  use super::*;
  #[ allow( clippy::useless_attribute, clippy::pub_use ) ]
  pub use private::debug_assert_id;
  #[ allow( clippy::useless_attribute, clippy::pub_use ) ]
  pub use private::debug_assert_identical;
  #[ allow( clippy::useless_attribute, clippy::pub_use ) ]
  pub use private::debug_assert_ni;
  #[ allow( clippy::useless_attribute, clippy::pub_use ) ]
  pub use private::debug_assert_not_identical;
}
