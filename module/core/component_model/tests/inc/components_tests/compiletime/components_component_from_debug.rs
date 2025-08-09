// Standalone trybuild test file for ComponentFrom functionality
// This file tests that ComponentFrom derive compiles correctly

use component_model::ComponentFrom;

#[ derive( Debug, Default, PartialEq, ComponentFrom ) ]
pub struct Options1
{
  field1 : i32,
  field2 : String,
  field3 : f32,
}

fn main()
{
  let options = Options1
  {
    field1: 42,
    field2: "test".to_string(),
    field3: 3.14,
  };

  // Test that ComponentFrom generates code without compilation errors
  println!( "ComponentFrom derive test: {:?}", options );
}
