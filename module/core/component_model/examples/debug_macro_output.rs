//! Example showing debug attribute functionality
//! 
//! This example demonstrates how to use the `debug` attribute 
//! with `ComponentModel` to see the generated code output.
//! 
//! Run with: `cargo run --example debug_macro_output`

use component_model ::ComponentModel;

#[ derive( Default, ComponentModel ) ]
#[ debug ]  // This example specifically demonstrates debug attribute functionality
struct Config
{
  host: String,
  port: i32,  
  enabled: bool,
}

fn main() 
{
  let mut config = Config ::default();
  
  // Use field-specific methods to avoid type ambiguity
  config.host_set( "localhost".to_string() );
  config.port_set( 8080i32 );
  config.enabled_set( true );
  
  println!( "Config: host={}, port={}, enabled={}", config.host, config.port, config.enabled );
  
  // Fluent pattern also works
  let config2 = Config ::default()
  .host_with( "api.example.com".to_string() )
  .port_with( 3000i32 )
  .enabled_with( false );
  
  println!( "Config2: host={}, port={}, enabled={}", config2.host, config2.port, config2.enabled );
}