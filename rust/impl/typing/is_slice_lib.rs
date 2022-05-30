#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ warn( rust_2018_idioms ) ]
#![ warn( missing_debug_implementations ) ]
#![ warn( missing_docs ) ]

//!
//! Macro to answer the question: is it a slice?
//!

#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/Readme.md" ) ) ]

/// Internal namespace.
pub( crate ) mod private
{

  /// Macro to answer the question: is it a slice?
  ///
  /// ### Sample
  /// ```
  /// use is_slice::*;
  ///
  /// fn main()
  /// {
  ///   dbg!( is_slice!( Box::new( true ) ) );
  ///   // < is_slice!(Box :: new(true)) = false
  ///   dbg!( is_slice!( &[ 1, 2, 3 ] ) );
  ///   // < is_slice!(& [1, 2, 3]) = false
  ///   dbg!( is_slice!( &[ 1, 2, 3 ][ .. ] ) );
  ///   // < is_slice!(& [1, 2, 3] [..]) = true
  /// }
  /// ```

  #[ macro_export ]
  macro_rules! is_slice
  {
    ( $V : expr ) =>
    {{
      use ::core::marker::PhantomData;

      trait NotSlice
      {
        fn is_slice( self : &'_ Self ) -> bool { false }
      }

      impl< T > NotSlice
      for &'_ PhantomData< T >
      where T : ?Sized,
      {}

      trait Slice
      {
        fn is_slice( self : &'_ Self ) -> bool { true }
      }

      impl< 'a, T > Slice for PhantomData< &'a &[ T ] >
      {}

      fn does< T : Sized >( _ : &T ) -> PhantomData< &T >
      {
        PhantomData
      }

      ( &does( &$V ) ).is_slice()

    }}
  }

  pub use is_slice;
}

/// Protected namespace of the module.
pub mod protected
{
  pub use super::orphan::*;
}

#[ doc( inline ) ]
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

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
  #[ doc( inline ) ]
  pub use super::private::
  {
    is_slice,
  };
}
