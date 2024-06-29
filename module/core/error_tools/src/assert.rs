/// Internal namespace.
pub( crate ) mod private
{
  ///
  /// Macro asserts that two expressions are identical to each other. Unlike std::assert_eq it is removed from a release build.
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

  /// Macro asserts that two expressions are identical to each other. Unlike std::assert_eq it is removed from a release build. Alias of debug_assert_id.

  #[ macro_export ]
  macro_rules! debug_assert_identical
  {
    ( $( $arg : tt )+ ) =>
    {
      #[cfg(debug_assertions)]
      $crate::debug_assert_id!( $( $arg )+ );
    };
  }

  /// Macro asserts that two expressions are not identical to each other. Unlike std::assert_eq it is removed from a release build.

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

  /// Macro asserts that two expressions are not identical to each other. Unlike std::assert_eq it is removed from a release build.

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

  pub use debug_assert_id;
  pub use debug_assert_identical;
  pub use debug_assert_ni;
  pub use debug_assert_not_identical;
}

/// Protected namespace of the module.
pub mod protected
{
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::orphan::*;
}

#[ doc( inline ) ]
#[ allow( unused_imports ) ]
pub use protected::*;

/// Shared with parent namespace of the module
pub mod orphan
{
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::exposed::*;
}

/// Exposed namespace of the module.
#[ allow( unused_imports ) ]
pub mod exposed
{
  use super::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::prelude::*;
}

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
  pub use super::private::debug_assert_id;
  pub use super::private::debug_assert_identical;
  pub use super::private::debug_assert_ni;
  pub use super::private::debug_assert_not_identical;
}
