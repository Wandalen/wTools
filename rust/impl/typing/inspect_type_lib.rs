// #![ feature( type_name_of_val ) ]
#![ warn( rust_2018_idioms ) ]
#![ warn( missing_debug_implementations ) ]
#![ warn( missing_docs ) ]

//!
//! Diagnostic-purpose tools to inspect type of a variable and its size.
//!

#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/Readme.md" ) ) ]

#[ cfg( feature = "nightly" ) ]
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

}

#[ cfg( feature = "nightly" ) ]
#[ doc( inline ) ]
pub use nightly::*;
