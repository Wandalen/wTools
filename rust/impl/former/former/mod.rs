#![ warn( missing_docs ) ]

//!
//! Former - variation of builder pattern.
//!
//! # Sample
//! ```
//! use former::Former;
//!
//! #[derive( Debug, PartialEq, Former )]
//! pub struct Command
//! {
//!   int_1 : i32,
//!   string_1 : String,
//!   vec_1 : Vec< String >,
//!   hashmap_strings_1 : std::collections::HashMap< String, String >,
//!   int_optional_1 : core::option::Option< i32 >,
//!   string_optional_1 : Option< String >,
//! }
//!
//! fn main()
//! {
//!
//!   let command = Command::former()
//!   .int_1( 13 )
//!   .string_1( "Abcd".to_string() )
//!   .vec_1().push( "ghi" ).push( "klm" ).end()
//!   .hashmap_strings_1().insert( "k1", "v1" ).insert( "k2", "v2" ).end()
//!   .string_optional_1( "dir1" )
//!   .form();
//!   dbg!( &command );
//!
//! }
//! ```

// #![ feature( type_name_of_val ) ]
// #![ feature( trace_macros ) ]

// pub extern crate former_derive;
// pub extern crate former_runtime;

pub use former_derive as derive;
pub use former_runtime as runtime;
pub use derive::Former;

// #[macro_use]
// pub extern crate maplit;
