//!
//! Macro to implement `From` for each component (field) of a structure.
//! This macro simplifies the creation of `From` trait implementations for struct fields,
//! enabling easy conversion from a struct reference to its field types.
//!
//! # Features
//!
//! - Requires the `derive_component_from` feature to be enabled for use.
//! - The `ComponentFrom` derive macro can be applied to structs to automatically generate
//!   `From` implementations for each field.
//!
//! # Attributes
//!
//! - `debug` : Optional attribute to enable debug-level output during the macro expansion process.
//!

#[ cfg( not( all( feature = "enabled", feature = "derive_component_from" ) ) ) ]
fn main() {}

#[ cfg( all( feature = "enabled", feature = "derive_component_from" ) ) ]
fn main()
{

  #[ derive( former::ComponentFrom ) ]
  struct MyStruct
  {
    pub field1 : i32,
    pub field2 : String,
  }

  // Generated implementations allow for the following conversions :
  let my_struct = MyStruct { field1 : 10, field2 : "Hello".into() };
  let field1 : i32 = From::from( &my_struct );
  let field2 : String = From::from( &my_struct );
  dbg!( field1 );
  dbg!( field2 );
  // > field1 = 10
  // > field2 = "Hello"

}
