//!
//! ## Example: Using Trait Assign
//!
//! Demonstrates setting various components (fields) of a struct.
//!
//! The `former_types` crate provides a generic interface for setting components on an object. This example defines a `Person` struct
//! and implements the `Assign` trait for its fields. It shows how to use these implementations to set the fields of a `Person`
//! instance using different types that can be converted into the required types.
//!
//! ## Explanation
//!
//! - **Person Struct**: The `Person` struct has two fields: `age` (an integer) and `name` (a string). The `Default` and `PartialEq` traits are derived to facilitate default construction and comparison.
//!
//! - **Assign Implementations**: The `Assign` trait is implemented for the `age` and `name` fields of the `Person` struct.
//!   - For `age`: The trait is implemented for any type that can be converted into an `i32`.
//!   - For `name`: The trait is implemented for any type that can be converted into a `String`.
//!
//! - **Usage**: An instance of `Person` is created using the default constructor, and then the `assign` method is used to set the `age` and `name` fields.
//!   - `got.assign( 13 )`: Assigns the integer `13` to the `age` field.
//!   - `got.assign( "John" )`: Assigns the string `"John"` to the `name` field.
//!

#[ cfg( any( not( feature = "types_former" ), not( feature = "enabled" ) ) ) ]
fn main() {}

#[ cfg( all( feature = "types_former", feature = "enabled" ) ) ]
fn main()
{
  use former_types::Assign;

  #[ derive( Default, PartialEq, Debug ) ]
  struct Person
  {
    age : i32,
    name : String,
  }

  impl< IntoT > Assign< i32, IntoT > for Person
  where
    IntoT : Into< i32 >,
  {
    fn assign( &mut self, component : IntoT )
    {
      self.age = component.into();
    }
  }

  impl< IntoT > Assign< String, IntoT > for Person
  where
    IntoT : Into< String >,
  {
    fn assign( &mut self, component : IntoT )
    {
      self.name = component.into();
    }
  }

  let mut got : Person = Default::default();
  got.assign( 13 );
  got.assign( "John" );
  assert_eq!( got, Person { age : 13, name : "John".to_string() } );
  dbg!( got );
  // > Person {
  // >   age: 13,
  // >   name: "John",
  // > }

}
