#![ cfg_attr( feature = "no_std", no_std ) ]
#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ doc( html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico" ) ]
#![ doc( html_root_url = "https://docs.rs/inspect_type/latest/inspect_type/" ) ]
#![ warn( rust_2018_idioms ) ]
#![ deny( missing_debug_implementations ) ]
#![ deny( missing_docs ) ]

//!
//! Diagnostic-purpose tools to inspect type of a variable and its size.
//!

#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/", "Readme.md" ) ) ]

#[ cfg( feature = "nightly" ) ]
#[ cfg( feature = "enabled" ) ]
mod nightly
{

  ///
  /// Macro to inspect type of a variable and its size exporting it as a string.
  ///

  #[ macro_export ]
  // #[ cfg_attr( feature = "nightly1", macro_export ) ]
  macro_rules! inspect_to_str_type_of
  {
    ( $src : expr ) =>
    {{
      let mut result = String::new();
      let stringified = stringify!( $src );

      let size = &std::mem::size_of_val( &$src ).to_string()[ .. ];
      let type_name = std::any::type_name_of_val( &$src );
      result.push_str( &format!( "sizeof( {} : {} ) = {}", stringified, type_name, size )[ .. ] );

      result
    }};
    ( $( $src : expr ),+ $(,)? ) =>
    {
      ( $( $crate::dbg!( $src ) ),+ )
    };
  }

  ///
  /// Macro to inspect type of a variable and its size printing into stdout and exporting it as a string.
  ///

  #[ macro_export ]
  // #[ cfg_attr( feature = "nightly1", macro_export ) ]
  macro_rules! inspect_type_of
  {
    ( $src : expr ) =>
    {{
      let result = $crate::inspect_to_str_type_of!( $src );
      println!( "{}", result );
      result
    }}
  }

  pub use inspect_to_str_type_of;
  pub use inspect_type_of;
}

// #[ cfg( feature = "nightly" ) ]
// #[ doc( inline ) ]
// pub use nightly::*;

/// Protected namespace of the module.
#[ cfg( feature = "enabled" ) ]
pub mod protected
{
  #[ doc( inline ) ]
  pub use super::orphan::*;
}

#[ doc( inline ) ]
#[ cfg( feature = "enabled" ) ]
pub use protected::*;

/// Orphan namespace of the module.
#[ cfg( feature = "enabled" ) ]
pub mod orphan
{
  #[ doc( inline ) ]
  pub use super::exposed::*;
}

/// Exposed namespace of the module.
#[ cfg( feature = "enabled" ) ]
pub mod exposed
{
  #[ doc( inline ) ]
  pub use super::prelude::*;
}

#[ cfg( feature = "enabled" ) ]
/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
  #[ doc( inline ) ]
  pub use super::private::
  {
    implements,
    instance_of,
  };

  #[ cfg( feature = "nightly" ) ]
  #[ doc( inline ) ]
  pub use super::nightly::*;

  #[ doc( inline ) ]
  pub use super::private::
  {
    is_slice,
  };
}
mod implements_impl;

#[ cfg( feature = "enabled" ) ]
pub( crate ) mod private
{
  ///
  /// Macro `implements` to answer the question: does it implement a trait?
  ///
  /// ### Basic use-case.
  /// ```
  /// use is_slice::*;
  ///
  /// dbg!( implements!( 13_i32 => Copy ) );
  /// // < implements!( 13_i32 => Copy ) : true
  /// dbg!( implements!( Box::new( 13_i32 ) => Copy ) );
  /// // < implements!( 13_i32 => Copy ) : false
  /// ```

  #[ macro_export ]
  macro_rules! implements
  {
    ( $( $arg : tt )+ ) =>
    {
      $crate::_implements!( $( $arg )+ );
    }
  }

  ///
  /// Macro `instance_of` to answer the question: does it implement a trait? Alias of the macro `implements`.
  ///
  /// ### Basic use-case.
  /// ```
  /// use is_slice::instance_of;
  ///
  /// dbg!( instance_of!( 13_i32 => Copy ) );
  /// // < instance_of!( 13_i32 => Copy ) : true
  /// dbg!( instance_of!( Box::new( 13_i32 ) => Copy ) );
  /// // < instance_of!( 13_i32 => Copy ) : false
  /// ```

  #[ macro_export ]
  macro_rules! instance_of
  {
    ( $( $arg : tt )+ ) =>
    {
      $crate::_implements!( $( $arg )+ );
    }
  }

  pub use implements;
  pub use instance_of;

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
