//! Example showing debug attribute functionality
//! 
//! This example demonstrates how to use the `debug` attribute 
//! with ComponentModel to see the generated code output.
//! 
//! Run with: `cargo run --example debug_macro_output`

use component_model::ComponentModel;

#[ derive( Default, ComponentModel ) ]
#[ debug ]
struct Config
{
  host : String,
  port : i32,  
  enabled : bool,
}

fn main() {
  let mut config = Config::default();
  
  // Use field-specific methods to avoid type ambiguity
  config.host_assign( "localhost".to_string() );
  config.port_assign( 8080i32 );
  config.enabled_assign( true );
  
  println!( "Config: host={}, port={}, enabled={}", config.host, config.port, config.enabled );
  
  // Fluent pattern also works
  let config2 = Config::default()
    .host_impute( "api.example.com".to_string() )
    .port_impute( 3000i32 )
    .enabled_impute( false );
    
  println!( "Config2: host={}, port={}, enabled={}", config2.host, config2.port, config2.enabled );
}