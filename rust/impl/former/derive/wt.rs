// // #![ feature( type_name_of_val ) ]

// #[ macro_export ]
// macro_rules! inspect_type_of
// {
//   ( $src : expr ) =>
//   {{
//     let mut result = String::new();
//     let stringified = stringify!( $src );

//     result.push_str( &format!( "= {} at {}:{}", stringified, file!(), line!() ) );

//     let size = &std::mem::size_of_val( &$src ).to_string()[ .. ];
//     let name = std::any::type_name_of_val( &$src );
//     result.push_str( &format!( "\n  sizeof( {} ) = {}",name, size )[ .. ] );

//     let size = &std::mem::size_of_val( &&$src ).to_string()[ .. ];
//     let name = std::any::type_name_of_val( &&$src );
//     result.push_str( &format!( "\n  sizeof( {} ) = {}",name, size )[ .. ] );

//     result
//   }};
//   ( $( $src : expr ),+ $(,)? ) =>
//   {
//     ( $( $crate::dbg!( $src ) ),+ )
//   };
// }

// //

// #[ macro_export ]
// macro_rules! inspect_logging_type_of
// {
//   ( $src : expr ) =>
//   {{
//     let result = $crate::inspect_type_of!( $src );
//     println!( "{}", result );
//   }}
// }
