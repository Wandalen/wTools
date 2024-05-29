#![ cfg_attr( feature = "no_std", no_std ) ]
#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ doc( html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico" ) ]
#![ doc( html_root_url = "https://docs.rs/for_each/latest/for_each/" ) ]
#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/", "Readme.md" ) ) ]

/// Internal namespace.
#[ cfg( feature = "enabled" ) ]
pub( crate ) mod private
{

  #[ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/Readme.md" ) ) ]
  #[ macro_export ]
  macro_rules! for_each
  {

    // -- function-style

    (
      $Callback : path, $( $Each : tt ),* $(,)?
    ) =>
    {
      $(
        $crate::braces_unwrap!( $Callback, $Each );
      )*
    };

    // -- callback-less

    (
      @Prefix $Prefix : tt
      @Postfix $Postfix : tt
      @Each $( $Each : tt )*
    ) =>
    {
      $crate::for_each!
      {
        $crate::identity where
        @Prefix $Prefix
        @Postfix $Postfix
        @Each $( $Each )*
      }
    };

    (
      @Prefix $Prefix : tt
      @Each $( $Each : tt )*
    ) =>
    {
      $crate::for_each!
      {
        $crate::identity where
        @Prefix $Prefix
        @Each $( $Each )*
      }
    };

    (
      @Postfix $Postfix : tt
      @Each $( $Each : tt )*
    ) =>
    {
      $crate::for_each!
      {
        $crate::identity where
        @Postfix $Postfix
        @Each $( $Each )*
      }
    };

    // -- map-style

    (
      $Callback : path where
      @Each $( $Each : tt )*
    ) =>
    {
      $(
        $crate::braces_unwrap!( $Callback, $Each );
      )*
    };

    (
      $Callback : path
      where
        @Prefix $Prefix : tt
        @Postfix $Postfix : tt
        @Each $( $Each : tt )*
    ) =>
    {
      $(
        $crate::braces_unwrap!
        (
          $Callback where
          @Prefix{ $Prefix }
          @Postfix{ $Postfix }
          @SRC{ $Each }
        );
      )*
    };

    (
      $Callback : path where
      @Prefix $Prefix : tt
      @Each $( $Each : tt )*
    ) =>
    {
      $(
        $crate::braces_unwrap!
        (
          $Callback where
          @Prefix{ $Prefix }
          @SRC{ $Each }
        );
      )*
    };

    (
      $Callback : path where
      @Postfix $Postfix : tt
      @Each $( $Each : tt )*
    ) =>
    {
      $(
        $crate::braces_unwrap!
        (
          $Callback where
          @Postfix{ $Postfix }
          @SRC{ $Each }
        );
      )*
    };

  }

  ///
  /// Unwrap braces of token tree and pass its content to the passed callback. If token tree in not braced then it passed to callback as is.
  ///
  /// # Function-style sample
  /// ```rust
  /// use for_each::*;
  /// let ( a, b, c ) = ( 1, 2, 3 );
  /// braces_unwrap!( dbg, { a, b, c } );
  /// // generates :
  /// // dbg!( a, b, c );
  /// braces_unwrap!( dbg, a, b, c );
  /// // generates :
  /// // dbg!( a, b, c );
  /// ```
  ///
  /// # Map-style sample
  /// ```rust
  /// use for_each::*;
  /// let ( prefix, a, b, c, postfix ) = ( "prefix", 1, 2, 3, "postfix" );
  /// braces_unwrap!
  /// (
  ///   dbg where
  ///   @Prefix{ prefix, }
  ///   @Postfix{ postfix }
  ///   @SRC{ { a, b, c, } }
  /// );
  /// // generates :
  /// // dbg!( prefix, a, b, c, psotfix );
  /// braces_unwrap!
  /// (
  ///   dbg where
  ///   @Prefix{ prefix, }
  ///   @Postfix{ postfix }
  ///   @SRC{ a, b, c, }
  /// );
  /// // generates :
  /// // dbg!( prefix, a, b, c, psotfix );
  /// ```
  ///

  #[macro_export]
  macro_rules! braces_unwrap
  {

    // function-style

    ( $Callback : path, { $( $Src : tt )* } )
    =>
    {
      $Callback!
      (
        $( $Src )*
      );
    };
    ( $Callback : path, $( $Src : tt )* )
    =>
    {
      $Callback!
      (
        $( $Src )*
      );
    };

    // map-style

    (
      $Callback : path where
      @SRC{ { $( $Src : tt )* } }
    )
    =>
    {
      $Callback!
      (
        $( $Src )*
      );
    };
    (
      $Callback : path where
      @SRC{ $( $Src : tt )* }
    )
    =>
    {
      $Callback!
      (
        $( $Src )*
      );
    };

    // with prefix and psotfix

    /* 0 */
    (
      $Callback : path where
      @Prefix{ { $( $Prefix : tt )* } }
      @Postfix{ { $( $Postfix : tt )* } }
      @SRC{ { $( $Src : tt )* } }
    )
    =>
    {
      $Callback!
      (
        $( $Prefix )* $( $Src )* $( $Postfix )*
      );
    };
    /* 1 */
    (
      $Callback : path where
      @Prefix{ { $( $Prefix : tt )* } }
      @Postfix{ { $( $Postfix : tt )* } }
      @SRC{ $( $Src : tt )* }
    )
    =>
    {
      $Callback!
      (
        $( $Prefix )* $( $Src )* $( $Postfix )*
      );
    };
    /* 2 */
    (
      $Callback : path where
      @Prefix{ { $( $Prefix : tt )* } }
      @Postfix{ $( $Postfix : tt )* }
      @SRC{ { $( $Src : tt )* } }
    )
    =>
    {
      $Callback!
      (
        $( $Prefix )* $( $Src )* $( $Postfix )*
      );
    };
    /* 3 */
    (
      $Callback : path where
      @Prefix{ { $( $Prefix : tt )* } }
      @Postfix{ $( $Postfix : tt )* }
      @SRC{ $( $Src : tt )* }
    )
    =>
    {
      $Callback!
      (
        $( $Prefix )* $( $Src )* $( $Postfix )*
      );
    };
    /* 4 */
    (
      $Callback : path where
      @Prefix{ $( $Prefix : tt )* }
      @Postfix{ { $( $Postfix : tt )* } }
      @SRC{ { $( $Src : tt )* } }
    )
    =>
    {
      $Callback!
      (
        $( $Prefix )* $( $Src )* $( $Postfix )*
      );
    };
    /* 5 */
    (
      $Callback : path where
      @Prefix{ $( $Prefix : tt )* }
      @Postfix{ { $( $Postfix : tt )* } }
      @SRC{ $( $Src : tt )* }
    )
    =>
    {
      $Callback!
      (
        $( $Prefix )* $( $Src )* $( $Postfix )*
      );
    };
    /* 6 */
    (
      $Callback : path where
      @Prefix{ $( $Prefix : tt )* }
      @Postfix{ $( $Postfix : tt )* }
      @SRC{ { $( $Src : tt )* } }
    )
    =>
    {
      $Callback!
      (
        $( $Prefix )* $( $Src )* $( $Postfix )*
      );
    };
    /* 7 */
    (
      $Callback : path where
      @Prefix{ $( $Prefix : tt )* }
      @Postfix{ $( $Postfix : tt )* }
      @SRC{ $( $Src : tt )* }
    )
    =>
    {
      $Callback!
      (
        $( $Prefix )* $( $Src )* $( $Postfix )*
      );
    };

    // with prefix

    /* 0 */
    (
      $Callback : path where
      @Prefix{ { $( $Prefix : tt )* } }
      @SRC{ { $( $Src : tt )* } }
    )
    =>
    {
      $Callback!
      (
        $( $Prefix )* $( $Src )*
      );
    };
    /* 1 */
    (
      $Callback : path where
      @Prefix{ { $( $Prefix : tt )* } }
      @SRC{ $( $Src : tt )* }
    )
    =>
    {
      $Callback!
      (
        $( $Prefix )* $( $Src )*
      );
    };
    /* 2 */
    (
      $Callback : path where
      @Prefix{ $( $Prefix : tt )* }
      @SRC{ { $( $Src : tt )* } }
    )
    =>
    {
      $Callback!
      (
        $( $Prefix )* $( $Src )*
      );
    };
    /* 3 */
    (
      $Callback : path where
      @Prefix{ $( $Prefix : tt )* }
      @SRC{ $( $Src : tt )* }
    )
    =>
    {
      $Callback!
      (
        $( $Prefix )* $( $Src )*
      );
    };

    // with postfix

    /* 0 */
    (
      $Callback : path where
      @Postfix{ { $( $Postfix : tt )* } }
      @SRC{ { $( $Src : tt )* } }
    )
    =>
    {
      $Callback!
      (
        $( $Src )* $( $Postfix )*
      );
    };
    /* 1 */
    (
      $Callback : path where
      @Postfix{ { $( $Postfix : tt )* } }
      @SRC{ $( $Src : tt )* }
    )
    =>
    {
      $Callback!
      (
        $( $Src )* $( $Postfix )*
      );
    };
    /* 2 */
    (
      $Callback : path where
      @Postfix{ $( $Postfix : tt )* }
      @SRC{ { $( $Src : tt )* } }
    )
    =>
    {
      $Callback!
      (
        $( $Src )* $( $Postfix )*
      );
    };
    /* 3 */
    (
      $Callback : path where
      @Postfix{ $( $Postfix : tt )* }
      @SRC{ $( $Src : tt )* }
    )
    =>
    {
      $Callback!
      (
        $( $Src )* $( $Postfix )*
      );
    };

  }

  /// Macro which returns its input as is.
  #[macro_export]
  macro_rules! identity
  {
    (
      $( $Src : tt )*
    )
    =>
    {
      $( $Src )*
    };
  }

  //

  pub use for_each;
  pub use braces_unwrap;
  pub use identity;

}

#[ cfg( feature = "enabled" ) ]
#[ doc( inline ) ]
#[ allow( unused_imports ) ]
pub use protected::*;

/// Protected namespace of the module.
#[ cfg( feature = "enabled" ) ]
pub mod protected
{
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::orphan::*;
}

/// Orphan namespace of the module.
#[ cfg( feature = "enabled" ) ]
pub mod orphan
{
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::exposed::*;
}

/// Exposed namespace of the module.
#[ cfg( feature = "enabled" ) ]
pub mod exposed
{
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::prelude::*;
}

/// Prelude to use essentials: `use my_module::prelude::*`.
#[ cfg( feature = "enabled" ) ]
pub mod prelude
{
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::private::for_each;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::private::braces_unwrap;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::private::identity;
}
