//! Example demonstrating boolean assignment ambiguity solution
//!
//! This example shows how the boolean assignment type ambiguity issue
//! has been resolved with field-specific methods.
//!
//! Run with: `cargo run --example boolean_assignment_error`

use component_model ::ComponentModel;
use component_model_types ::Assign;

#[ derive( Default, ComponentModel ) ]
struct Config 
{
  host: String,
  port: i32,
  enabled: bool,
}

fn main() 
{
  let mut config = Config ::default();
  
  println!("Demonstrating boolean assignment ambiguity solution: ");
  
  // These work fine with generic assignment :
  config.assign( "localhost".to_string() );
  config.assign( 8080i32 );
  
  // OLD WAY: This would cause ambiguity error
  // config.assign( true );  // ERROR: type annotations needed
  
  // NEW WAY: Use field-specific method to avoid ambiguity
  config.enabled_set( true );  // ✅ Clear and unambiguous
  
  println!("✅ Config successfully set: ");
  println!("   host: {}", config.host);
  println!("   port: {}", config.port);  
  println!("   enabled: {}", config.enabled);
  
  // Alternative: Explicit type annotation still works
  let mut config2 = Config ::default();
  Assign :: < String, _ > ::assign( &mut config2, "api.example.com".to_string() );
  Assign :: < i32, _ > ::assign( &mut config2, 3000i32 );
  Assign :: < bool, _ > ::assign( &mut config2, false );
  
  println!("\n✅ Alternative with explicit types also works: ");
  println!("   host: {}", config2.host);
  println!("   port: {}", config2.port);
  println!("   enabled: {}", config2.enabled);
}