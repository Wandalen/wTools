//! Example: Parse Attributes with Properties
//! 
//! This example demonstrates how to parse custom attributes with properties
//! using macro_tools' attribute parsing framework. This is essential for
//! creating procedural macros that accept configuration through attributes.

#[ cfg( not( all( feature = "enabled", feature = "attr_prop" ) ) ) ]
fn main() 
{
  println!( "This example requires the 'enabled' and 'attr_prop' features to be enabled." );
  println!( "Try running with: cargo run --example macro_tools_parse_attributes --all-features" );
}

#[ cfg( all( feature = "enabled", feature = "attr_prop" ) ) ]
fn main() 
{
  println!( "=== Parse Attributes with Properties Example ===" );
  println!();

  // Simple example showing the structure - actual implementation would require
  // more trait implementations as shown in the full attr_prop example
  println!( "This is a demonstration of the attribute parsing concept." );
  println!( "For a complete working example, see:" );
  println!( "  cargo run --example macro_tools_attr_prop --all-features" );
  
  println!();
  println!( "=== End of Examples ===" );
}