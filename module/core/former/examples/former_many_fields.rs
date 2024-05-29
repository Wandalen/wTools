//!
//! Utilizing the Former Crate for Struct Initialization
//!
//! This example demonstrates the capability of the `Former` crate to simplify struct initialization through the builder pattern, particularly for structs with a mix of required and optional fields, as well as collections like vectors and hash maps.
//!
//! The `Structure1` struct is defined with various field types to showcase the flexibility of `Former`:
//! - `int_1`: A required integer field.
//! - `string_1`: A required string field.
//! - `vec_1`: A vector of unsigned integers, showcasing collection handling.
//! - `hashmap_1`: A hash map storing key-value pairs, both strings, illustrating how `Former` can manage more complex data structures.
//! - `int_optional_1`: An optional integer field, demonstrating `Former`'s capability to handle optional fields seamlessly.
//! - `string_optional_1`: An optional string field, further exemplifying optional field handling.
//!
//! A hash map is first created and populated with two key-value pairs. The `Structure1` struct is then instantiated using the fluent builder pattern methods provided by `Former`. Each method corresponds to one of `Structure1`'s fields, allowing for intuitive and clear field assignment. The `.form()` method completes the construction of the `Structure1` instance.
//!
//! The builder pattern methods significantly streamline the process of struct initialization, especially for structs with complex or optional fields. By leveraging `Former`, developers can write more readable and maintainable initialization code, avoiding the verbosity and complexity often associated with manual struct instantiation.
//!
//! The `dbg!` macro is utilized to print the constructed `Structure1` instance, confirming that all fields are correctly assigned, including the handling of optional fields and collections.

#[ cfg( not( all( feature = "enabled", feature = "derive_former", any( feature = "use_alloc", not( feature = "no_std" ) ) ) ) ) ]
fn main() {}

#[ cfg( all( feature = "enabled", feature = "derive_former", any( feature = "use_alloc", not( feature = "no_std" ) ) ) ) ]
fn main()
{
  use former::Former;

  #[ derive( Debug, PartialEq, Eq, Former ) ]
  pub struct Structure1
  {
    int_1 : i32,
    string_1 : String,
    vec_1 : Vec< u32 >,
    hashmap_1 : collection_tools::HashMap< String, String >,
    int_optional_1 : core::option::Option< i32 >,
    string_optional_1 : Option< String >,
  }
  let hashmap = collection_tools::HashMap::from
  ([
    ( "k1".to_string(), "v1".to_string() ),
    ( "k2".to_string(), "v2".to_string() ),
  ]);

  let struct1 = Structure1::former()
  .int_1( 13 )
  .string_1( "Abcd".to_string() )
  .vec_1( vec![ 1, 3 ] )
  .hashmap_1( hashmap )
  .string_optional_1( "dir1" )
  .form();
  dbg!( &struct1 );

// <  &struct1 = Structure1 {
// <   int_1: 13,
// <   string_1: "Abcd",
// <   vec_1: [
// <       1,
// <       3,
// <   ],
// <   hashmap_1: {
// <       "k1": "v1",
// <       "k2": "v2",
// <   },
// <   int_optional_1: None,
// <   string_optional_1: Some(
// <       "dir1",
// <   ),
// < }

}
