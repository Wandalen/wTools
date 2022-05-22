/// Internal namespace.
mod internal
{
  use crate::exposed::*;

  // xxx : module type_constructors
  // xxx : samples
  // xxx : redo implements

  // xxx : implement clone_as_tuple()
  // xxx : implement clone_as_array()
  // xxx : implement as_tuple()
  // xxx : implement as_array()
  // xxx : implement as_slice()

  // xxx : from type for homopair
  // xxx : from array for homopair
  // xxx : from slice for homopair

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
  /// use fundamental_data_type::prelude::*;
  /// types!( single MySingle : i32 );
  /// let x = MySingle( 13 );
  /// println!( "x : {}", x.0 );
  /// ```
  ///
  /// It gererates code:
  ///
  /// ```rust
  /// use fundamental_data_type::prelude::*;
  ///
  /// pub struct MySingle( pub i32 );
  ///
  /// impl core::ops::Deref for MySingle
  /// {
  ///   type Target = i32;
  ///   fn deref( &self ) -> &Self::Target
  ///   {
  ///     &self.0
  ///   }
  /// }
  /// impl From< i32 > for MySingle
  /// {
  ///   fn from( src : i32 ) -> Self
  ///   {
  ///     Self( src )
  ///   }
  /// }
  /// impl From< MySingle > for i32
  /// {
  ///   fn from( src : MySingle ) -> Self
  ///   {
  ///     src.0
  ///   }
  /// }
  ///
  /// let x = MySingle( 13 );
  /// println!( "x : {}", x.0 );
  /// ```
  ///
  /// ### Sample :: single with derives and attributes.
  ///
  /// It's possible to define attributes as well as derives.
  ///
  /// ```rust
  /// use fundamental_data_type::prelude::*;
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
  /// use fundamental_data_type::prelude::*;
  ///
  /// /// This is also attribute and macro understands it.
  /// #[ derive( Debug ) ]
  /// pub struct MySingle( pub i32 );
  ///
  /// impl core::ops::Deref for MySingle
  /// {
  ///   type Target = i32;
  ///   fn deref( &self ) -> &Self::Target
  ///   {
  ///     &self.0
  ///   }
  /// }
  /// impl From< i32 > for MySingle
  /// {
  ///   fn from( src : i32 ) -> Self
  ///   {
  ///     Self( src )
  ///   }
  /// }
  /// impl From< MySingle > for i32
  /// {
  ///   fn from( src : MySingle ) -> Self
  ///   {
  ///     src.0
  ///   }
  /// }
  ///
  /// let x = MySingle( 13 );
  /// dbg!( x );
  /// ```
  ///
  /// ### Sample :: single with struct instead of macro.
  ///
  /// Sometimes it's sufficient to use common type instead of defining a brand new.
  /// You may use paramtetrized struct `Single< T >` instead of macro `types!` if that is the case.
  ///
  /// ```rust
  /// use fundamental_data_type::prelude::*;
  /// let x = Single::< i32 >( 13 );
  /// dbg!( x );
  /// ```
  ///
  /// ### Sample :: single with parametrized element.
  ///
  /// Element of tuple could be parametrized.
  ///
  /// ```rust
  /// use fundamental_data_type::prelude::*;
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
  /// use fundamental_data_type::*;
  ///
  /// #[ derive( Debug ) ]
  /// pub struct MySingle< T : Copy >( pub std::sync::Arc< T > );
  ///
  /// impl<T: Copy> core::ops::Deref for MySingle< T >
  /// {
  ///   type Target = std::sync::Arc< T >;
  ///   fn deref( &self ) -> &Self::Target
  ///   {
  ///     &self.0
  ///   }
  /// }
  /// impl< T : Copy > From< std::sync::Arc< T > > for MySingle< T >
  /// {
  ///   fn from( src : std::sync::Arc<T>) -> Self {
  ///     Self( src )
  ///   }
  /// }
  /// impl< T : Copy > From< MySingle< T > > for std::sync::Arc< T >
  /// {
  ///   fn from(src: MySingle<T>) -> Self
  ///   {
  ///     src.0
  ///   }
  /// }
  ///
  /// let x = MySingle( std::sync::Arc::new( 13 ) );
  /// ```
  ///
  /// ### Sample :: single with parametrized tuple.
  ///
  /// Instead of parametrizing the element it's possible to define a parametrized tuple.
  ///
  ///
  /// ```rust
  /// use fundamental_data_type::prelude::*;
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
  /// #[ derive( Debug ) ]
  /// pub struct MySingle< T : Copy >( pub T );
  ///
  /// impl< T : Copy > core::ops::Deref
  /// for MySingle< T >
  /// {
  ///   type Target = T;
  ///   fn deref( &self ) -> &Self::Target
  ///   {
  ///     &self.0
  ///   }
  /// }
  ///
  /// impl< T : Copy > From< T >
  /// for MySingle< T >
  /// {
  ///   fn from( src : T ) -> Self
  ///   {
  ///     Self( src )
  ///   }
  /// }
  ///
  /// let x = MySingle( 13 );
  /// dbg!( 13 );
  /// ```
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
