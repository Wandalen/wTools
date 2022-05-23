/// Internal namespace.
mod internal
{
  use crate::exposed::*;

  // xxx : register graph_tools
  // xxx : no std
  // xxx : samples
  // qqq : paste generated code for each sample
  //
  // xxx : redo implements
  // xxx : add core::fmt to prelude
  // xxx : write article about the module

  ///
  /// Type constructor to define tuple wrapping a given type.
  ///
  /// Quite often you need to wrap a given type into new one.
  /// For example if orphan rule became and obstacle one should introduce a new type wrapping foreing one.
  /// Type constructr `types!` does exaclty that and auto-implement traits From, Into and Deref for the constructed type.
  ///
  /// ### Sample :: single line single.
  ///
  /// To define your own single use macro `types!`. Single-line definition looks like that.
  ///
  /// ```rust
  /// use type_constructor::prelude::*;
  /// types!( single MySingle : i32 );
  /// let x = MySingle( 13 );
  /// println!( "x : {}", x.0 );
  /// ```
  ///
  /// It gererates code:
  ///
  /// ```rust
  /// ```
  ///
  /// ### Sample :: single with derives and attributes.
  ///
  /// It's possible to define attributes as well as derives.
  ///
  /// ```rust
  /// use type_constructor::prelude::*;
  /// types!
  /// {
  ///   /// This is also attribute and macro understands it.
  ///   #[ derive( Debug ) ]
  ///   single MySingle : i32;
  /// }
  /// let x = MySingle( 13 );
  /// dbg!( x );
  /// ```
  ///
  /// It gererates code:
  ///
  /// ```rust
  /// ```
  ///
  /// ### Sample :: single with struct instead of macro.
  ///
  /// Sometimes it's sufficient to use common type instead of defining a brand new.
  /// You may use paramtetrized struct `Single< T >` instead of macro `types!` if that is the case.
  ///
  /// ```rust
  /// use type_constructor::prelude::*;
  /// let x = Single::< i32 >( 13 );
  /// dbg!( x );
  /// ```
  ///
  /// ### Sample :: single with parametrized element.
  ///
  /// Element of tuple could be parametrized.
  ///
  /// ```rust
  /// use type_constructor::prelude::*;
  /// types!
  /// {
  ///   #[ derive( Debug ) ]
  ///   single MySingle : std::sync::Arc< T : Copy >;
  /// }
  /// let x = MySingle( std::sync::Arc::new( 13 ) );
  /// dbg!( x );
  /// ```
  ///
  /// It gererates code:
  ///
  /// ```rust
  /// ```
  ///
  /// ### Sample :: single with parametrized tuple.
  ///
  /// Instead of parametrizing the element it's possible to define a parametrized tuple.
  ///
  ///
  /// ```rust
  /// use type_constructor::prelude::*;
  /// types!
  /// {
  ///   #[ derive( Debug ) ]
  ///   single MySingle : < T : Copy >;
  /// }
  /// let x = MySingle( 13 );
  /// dbg!( x );
  /// ```
  ///
  /// It gererates code:
  ///
  /// ```rust
  /// ```
  ///
  ///

  // #[ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/Readme.md" ) ) ]

  #[ macro_export ]
  macro_rules! types
  {

    // No more.

    (
    )
    =>
    {
    };

    // No more.

    (
      ;
    )
    =>
    {
    };

    // single

    (
      $( #[ $Meta : meta ] )*
      single
      $( $Rest : tt )*
    )
    =>
    {
      $crate::_single!
      {
        $( #[ $Meta ] )*
        single
        $( $Rest )*
      }
    };

    // pair

    (
      $( #[ $Meta : meta ] )*
      pair
      $( $Rest : tt )*
    )
    =>
    {
      $crate::_pair!
      {
        $( #[ $Meta ] )*
        pair
        $( $Rest )*
      }
    };

    // many

    (
      $( #[ $Meta : meta ] )*
      many
      $( $Rest : tt )*
    )
    =>
    {
      $crate::_many!
      {
        $( #[ $Meta ] )*
        many
        $( $Rest )*
      }
    };

    // bad syntax

    (
      $( $Rest : tt )*
    )
    =>
    {
      compile_error!
      (
        concat!
        (
          "Bad syntax.\n",
          "Expects : {kind} {name} : {type}.\n",
          "For example : `single MySingle : std::sync::Arc< T : Copy >`.\n",
          "But got:\n",
          stringify!
          (
            $( $Rest )*
          ),
        )
      );
    };

  }

  pub use types;
}

/// Protected namespace of the module.
pub mod protected
{
  pub use super::orphan::*;
}

pub use protected::*;

/// Orphan namespace of the module.
pub mod orphan
{
  pub use super::exposed::*;
}

/// Exposed namespace of the module.
pub mod exposed
{
  pub use super::prelude::*;
}

pub use exposed::*;

/// Prelude to use: `use wtools::prelude::*`.
pub mod prelude
{
  #[ doc( inline ) ]
  pub use super::internal::
  {
    types,
  };
}
