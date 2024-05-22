
use super::*;

//

#[ test ]
fn is_standard_standard()
{
  // Test a selection of attributes known to be standard
  assert!( attr::is_standard( "cfg" ), "Expected 'cfg' to be a standard attribute." );
  assert!( attr::is_standard( "derive" ), "Expected 'derive' to be a standard attribute." );
  assert!( attr::is_standard( "inline" ), "Expected 'inline' to be a standard attribute." );
  assert!( attr::is_standard( "test" ), "Expected 'test' to be a standard attribute." );
  assert!( attr::is_standard( "doc" ), "Expected 'doc' to be a standard attribute." );
}

#[ test ]
fn is_standard_non_standard()
{
  // Test some made-up attributes that should not be standard
  assert!( !attr::is_standard( "custom_attr" ), "Expected 'custom_attr' to not be a standard attribute." );
  assert!( !attr::is_standard( "my_attribute" ), "Expected 'my_attribute' to not be a standard attribute." );
  assert!( !attr::is_standard( "special_feature" ), "Expected 'special_feature' to not be a standard attribute." );
}

#[ test ]
fn is_standard_edge_cases()
{
  // Test edge cases like empty strings or unusual input
  assert!( !attr::is_standard( "" ), "Expected empty string to not be a standard attribute." );
  assert!( !attr::is_standard( " " ), "Expected a single space to not be a standard attribute." );
  assert!( !attr::is_standard( "cfg_attr_extra" ), "Expected 'cfg_attr_extra' to not be a standard attribute." );
}

// xxx
// #[ test ]
// fn attribute_component_from_meta()
// {
//   // Define a sample attribute
//   let got : Attribute = parse_quote!( #[ my_component = "value" ] );
//
//   // Attempt to construct MyComponent from the attribute
//   let result = MyComponent::from_meta( &got );
//
//   // Assert that the construction was successful
//   assert!( result.is_ok() );
// }
//
// #[ test ]
// fn attribute_component_attribute()
// {
//   // Define a sample invalid attribute
//   let got : Attribute = parse_quote!( #[ other_component = "value" ] );
//
//   // Attempt to construct MyComponent from the invalid attribute
//   let result = MyComponent::from_meta( &got );
//
//   // Assert that the construction failed
//   assert!( result.is_err() );
// }
