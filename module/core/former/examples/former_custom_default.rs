//! The `Former` crate enhances struct initialization in Rust by allowing the specification of custom default values for fields through the `default` attribute.
//!
//! This feature not only provides a way to set initial values for struct fields without relying on the `Default` trait but also adds flexibility in handling cases where a field's type does not implement `Default`, or a non-standard default value is desired.
//! The above code snippet showcases the `Former` crate's ability to initialize struct fields with custom default values:
//! - The `number` field is initialized to `5`.
//! - The `greeting` field defaults to a greeting message, "Hello, Former!".
//! - The `numbers` field starts with a vector containing the integers `10`, `20`, and `30`.
//!
//! This approach significantly simplifies struct construction, particularly for complex types or where defaults beyond the `Default` trait's capability are required. By utilizing the `default` attribute, developers can ensure their structs are initialized safely and predictably, enhancing code clarity and maintainability.
//!

fn main()
{
  use former::Former;

  /// Structure with default attributes.
  #[ derive(  Debug, PartialEq, Former ) ]
  pub struct ExampleStruct
  {
    #[ default( 5 ) ]
    number : i32,
    #[ default( "Hello, Former!".to_string() ) ]
    greeting : String,
    #[ default( vec![ 10, 20, 30 ] ) ]
    numbers : Vec< i32 >,
  }

  //

  let instance = ExampleStruct::former().form();
  let expected = ExampleStruct
  {
    number : 5,
    greeting : "Hello, Former!".to_string(),
    numbers : vec![ 10, 20, 30 ],
  };
  assert_eq!( instance, expected );
  dbg!( &instance );
  // > &instance = ExampleStruct {
  // >    number: 5,
  // >    greeting: "Hello, Former!",
  // >    numbers: [
  // >        10,
  // >        20,
  // >        30,
  // >    ],
  // > }

}
