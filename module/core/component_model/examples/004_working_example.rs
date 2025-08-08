//! # 004 - Real-World Usage Example
//! 
//! Shows practical usage of component model for configuration and data structures.

use component_model::Assign;

#[ derive( Default, Debug, PartialEq, Assign ) ]
struct AppConfig
{
  app_name : String,
  version : i32,
}

#[ derive( Default, Debug, PartialEq, Assign ) ]
struct ServerSettings
{
  bind_address : String,
  worker_count : i32,
}

fn main()
{
  println!( "=== Real-World Usage Example ===" );
  
  // Application configuration
  let mut app_config = AppConfig::default();
  app_config.assign( "MyWebApp" );
  app_config.assign( 1 );  // version 1
  println!( "App config: {app_config:?}" );
  
  // Server configuration with fluent style
  let server_config = ServerSettings::default()
    .impute( "127.0.0.1:8080" )
    .impute( 4 );  // 4 worker threads
  println!( "Server config: {server_config:?}" );
  
  // Configuration factory pattern
  fn create_dev_config() -> AppConfig {
    AppConfig::default()
      .impute( "MyWebApp-Dev" )
      .impute( 0 )  // development version
  }
  
  fn create_prod_config() -> AppConfig {
    AppConfig::default()
      .impute( "MyWebApp" )
      .impute( 2 )  // production version
  }
  
  let dev_config = create_dev_config();
  let prod_config = create_prod_config();
  
  println!( "Dev config: {dev_config:?}" );
  println!( "Prod config: {prod_config:?}" );
  
  // Environment-specific server settings
  let mut high_load_server = ServerSettings::default();
  high_load_server.assign( "0.0.0.0:80" );  // Bind to all interfaces
  high_load_server.assign( 16 );            // More workers for production
  
  println!( "High-load server: {high_load_server:?}" );
  
  // Verify configurations
  assert_eq!( app_config.app_name, "MyWebApp" );
  assert_eq!( app_config.version, 1 );
  assert_eq!( server_config.bind_address, "127.0.0.1:8080" );
  assert_eq!( server_config.worker_count, 4 );
  assert_eq!( dev_config.app_name, "MyWebApp-Dev" );
  assert_eq!( prod_config.version, 2 );
  
  println!( "✅ Real-world usage patterns complete!" );
}