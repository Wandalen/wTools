// #![ feature( type_name_of_val ) ]
#![ warn( missing_docs ) ]

//! Diagnostic-purpose tools to inspect type of a variable and its size.
//!
//! Sample
//! ```
//!#![ feature( type_name_of_val ) ]
//!
//! use inspect_type::*;
//!
//! fn main()
//! {
//!   inspect_type_of!( &[ 1, 2, 3 ][ .. ] );
//!   // < sizeof( &[1, 2, 3][..] : &[i32] ) = 16
//!   inspect_type_of!( &[ 1, 2, 3 ] );
//!   // < sizeof( &[1, 2, 3] : &[i32; 3] ) = 8
//! }
//!
//! ```

/// Macro to inspect type of a variable and its size exporting it as a string.

#[ macro_export ]
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

/// Macro to inspect type of a variable and its size printing into stdout and exporting it as a string.

#[ macro_export ]
macro_rules! inspect_type_of
{
  ( $src : expr ) =>
  {{
    let result = $crate::inspect_to_str_type_of!( $src );
    println!( "{}", result );
    result
  }}
}
