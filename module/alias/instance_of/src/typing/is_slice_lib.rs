#![ cfg_attr( feature = "no_std", no_std ) ]
#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ doc( html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico" ) ]
#![ doc( html_root_url = "https://docs.rs/is_slice/latest/is_slice/" ) ]
// #![ deny( rust_2018_idioms ) ]
// #![ deny( missing_debug_implementations ) ]
// #![ deny( missing_docs ) ]

//!
//! Macro to answer the question: is it a slice?
//!

#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/", "Readme.md" ) ) ]

/// Internal namespace.
pub( crate ) mod private
{

  /// Macro to answer the question: is it a slice?
  ///
  /// ### Basic use-case.
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

/// Own namespace of the module.
#[ allow( unused_imports ) ]
pub mod own
{
  use super::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use orphan::*;
}

#[ doc( inline ) ]
#[ allow( unused_imports ) ]
pub use own::*;

/// Orphan namespace of the module.
#[ allow( unused_imports ) ]
pub mod orphan
{
  use super::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use exposed::*;
}

/// Exposed namespace of the module.
#[ allow( unused_imports ) ]
pub mod exposed
{
  use super::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use prelude::*;
}

/// Prelude to use essentials: `use my_module::prelude::*`.
#[ allow( unused_imports ) ]
pub mod prelude
{
  use super::*;
  #[ doc( inline ) ]
  pub use private::
  {
    is_slice,
  };
}
