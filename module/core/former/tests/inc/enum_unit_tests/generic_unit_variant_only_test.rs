#[allow(clippy::used_underscore_binding, clippy::all, warnings)]
/// Test logic for unit variants in enums (temporarily non-generic).
use super::*;

#[ test ]
fn static_constructor()
{
  // Test the static constructor for unit variant
  assert_eq!(GenericOption::no_value(), GenericOption::NoValue);
}

#[ test ]
fn standalone_constructor()
{
  // Test the standalone constructor for unit variant  
  assert_eq!(no_value(), GenericOption::NoValue);
}