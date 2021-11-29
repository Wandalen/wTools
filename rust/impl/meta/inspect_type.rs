#![ warn( missing_docs ) ]

#[ macro_export ]
macro_rules! inspect_type_of
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

//

#[ macro_export ]
macro_rules! inspect_logging_type_of
{
  ( $src : expr ) =>
  {{
    let result = $crate::inspect_type_of!( $src );
    println!( "{}", result );
  }}
}

