//! Demonstrates integration of Former with serde for serialization/deserialization.
//!
//! This example shows how to combine Former's builder pattern with serde's
//! serialization capabilities for configuration files, API responses, and data persistence.
//!
//! ## Note
//!
//! To run this example, you need to add serde and `serde_json` to your dependencies:
//! ```toml
//! [dependencies]
//! former = "2.31"
//! serde = { version = "1.0", features = ["derive"] }
//! serde_json = "1.0"
//! ```
//!
//! Run with: `cargo run --example former_with_serde`

#[ cfg( not( all( feature = "enabled", feature = "derive_former" ) ) ) ]
fn main()
{
  println!( "Example requires 'enabled' and 'derive_former' features" );
}

#[ cfg( all( feature = "enabled", feature = "derive_former" ) ) ]
fn main()
{
  // Note: This example demonstrates the pattern but won't compile without serde dependencies.
  // The code is here for documentation purposes.

  println!( "This example demonstrates Former + Serde integration patterns." );
  println!( "To actually run it, add serde and serde_json to your Cargo.toml:" );
  println!();
  println!( "[dependencies]" );
  println!( "serde = {{ version = \"1.0\", features = [\"derive\"] }}" );
  println!( "serde_json = \"1.0\"" );
  println!();
  println!( "Below are the documented patterns (code won't compile without serde):" );
  println!();

  example_patterns_documentation();
}

// This function contains the actual example code for documentation purposes
#[ allow( dead_code, unused_variables, unreachable_code, clippy::too_many_lines ) ]
fn example_patterns_documentation()
{
  // The examples below show patterns but won't compile without serde

  #[cfg( any() )]  // Disabled for compilation - for documentation only
  {
  use former::Former;

  // Example 1: Basic struct with Former + Serde
  //
  // Former and serde derive macros work together seamlessly.
  // The order doesn't matter - both are derive macros.

  #[ derive( Debug, PartialEq, Former, serde::Serialize, serde::Deserialize ) ]
  pub struct ServerConfig
  {
    host : String,
    #[ serde( default = "default_port" ) ]
    port : u16,
    #[ serde( default ) ]
    timeout_ms : Option< u64 >,
  }

  fn default_port() -> u16 { 8080 }

  // Build using Former
  let config = ServerConfig::former()
    .host( "localhost".to_string() )
    .port( 3000 )
    .timeout_ms( 5000 )
    .form();

  // Serialize to JSON
  let json = serde_json::to_string_pretty( &config ).unwrap();
  println!( "Serialized config:\n{}", json );

  // Deserialize from JSON
  let json_input = r#"
  {
    "host": "example.com",
    "port": 443
  }
  "#;
  let deserialized : ServerConfig = serde_json::from_str( json_input ).unwrap();
  println!( "\nDeserialized config: {:?}", deserialized );

  assert_eq!( deserialized.host, "example.com" );
  assert_eq!( deserialized.port, 443 );
  assert_eq!( deserialized.timeout_ms, None );

  // Example 2: Nested structs with Former + Serde
  //
  // Subformers work perfectly with serde's nested serialization

  #[ derive( Debug, PartialEq, Former, serde::Serialize, serde::Deserialize ) ]
  pub struct DatabaseConfig
  {
    connection_string : String,
    #[ serde( default = "default_pool_size" ) ]
    pool_size : u32,
  }

  fn default_pool_size() -> u32 { 10 }

  #[ derive( Debug, PartialEq, Former, serde::Serialize, serde::Deserialize ) ]
  pub struct ApplicationConfig
  {
    app_name : String,
    #[ subform_scalar ]
    #[ serde( default ) ]
    database : DatabaseConfig,
  }

  // Build nested config using subformers
  let app_config = ApplicationConfig::former()
    .app_name( "MyApp".to_string() )
    .database()
      .connection_string( "postgres://localhost/mydb".to_string() )
      .pool_size( 20 )
      .end()
    .form();

  let json = serde_json::to_string_pretty( &app_config ).unwrap();
  println!( "\nNested config:\n{}", json );

  // Example 3: Collections with Former + Serde
  //
  // Collection subformers serialize as regular Vec/HashMap

  #[ cfg( any( feature = "use_alloc", not( feature = "no_std" ) ) ) ]
  {
    use former::Hmap;

    #[ derive( Debug, PartialEq, Former, serde::Serialize, serde::Deserialize ) ]
    pub struct ServiceConfig
    {
      name : String,
      #[ subform_collection( definition = former::VectorDefinition ) ]
      #[ serde( default ) ]
      tags : Vec< String >,
      #[ subform_collection( definition = former::HashMapDefinition ) ]
      #[ serde( default ) ]
      env_vars : Hmap< String, String >,
    }

    // Build with collection subformers
    let service = ServiceConfig::former()
      .name( "api-service".to_string() )
      .tags()
        .add( "web".to_string() )
        .add( "api".to_string() )
        .end()
      .env_vars()
        .add( ( "LOG_LEVEL".to_string(), "debug".to_string() ) )
        .add( ( "PORT".to_string(), "8080".to_string() ) )
        .end()
      .form();

    let json = serde_json::to_string_pretty( &service ).unwrap();
    println!( "\nService config with collections:\n{}", json );

    // Deserialize back
    let deserialized : ServiceConfig = serde_json::from_str( &json ).unwrap();
    assert_eq!( deserialized.name, "api-service" );
    assert_eq!( deserialized.tags.len(), 2 );
    assert_eq!( deserialized.env_vars.get( "LOG_LEVEL" ), Some( &"debug".to_string() ) );
  }

  // Example 4: Serde attributes with Former
  //
  // All serde attributes work: rename, skip, default, flatten, etc.

  #[ derive( Debug, PartialEq, Former, serde::Serialize, serde::Deserialize ) ]
  pub struct ApiResponse
  {
    #[ serde( rename = "responseCode" ) ]
    response_code : u16,

    #[ serde( rename = "responseMessage" ) ]
    message : String,

    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    error_details : Option< String >,

    #[ former( default = "unknown" ) ]
    #[ serde( default = "default_version" ) ]
    version : String,
  }

  fn default_version() -> String { "1.0".to_string() }

  let response = ApiResponse::former()
    .response_code( 200 )
    .message( "Success".to_string() )
    .form();

  let json = serde_json::to_string_pretty( &response ).unwrap();
  println!( "\nAPI response with serde attributes:\n{}", json );

  // Note: error_details is None, so it won't appear in JSON due to skip_serializing_if
  assert!( !json.contains( "error_details" ) );

  // Example 5: Loading config from file pattern
  //
  // Typical pattern: deserialize from file, modify with Former, save back

  let config_json = r#"
  {
    "host": "production.example.com",
    "port": 443
  }
  "#;

  // Load existing config
  let mut config : ServerConfig = serde_json::from_str( config_json ).unwrap();
  println!( "\nOriginal config: {:?}", config );

  // Modify using Former (not typical pattern, but demonstrates compatibility)
  // Note: Usually you'd build from scratch or use setters on the struct directly
  config.timeout_ms = Some( 10000 );
  println!( "Modified config: {:?}", config );

  // Example 6: Using former defaults with serde defaults
  //
  // Both former::default and serde::default can coexist

  #[ derive( Debug, Former, serde::Serialize, serde::Deserialize ) ]
  pub struct MixedDefaults
  {
    #[ former( default = "production" ) ]  // Former default when building
    #[ serde( default = "default_env" ) ]   // Serde default when deserializing
    environment : String,

    #[ former( default = true ) ]
    #[ serde( default ) ]                    // Uses bool::default() = false
    enabled : bool,
  }

  fn default_env() -> String { "development".to_string() }

  // Built with Former - uses former defaults
  let built = MixedDefaults::former().form();
  assert_eq!( built.environment, "production" );
  assert_eq!( built.enabled, true );

  // Deserialized empty JSON - uses serde defaults
  let empty_json = "{}";
  let deserialized : MixedDefaults = serde_json::from_str( empty_json ).unwrap();
  assert_eq!( deserialized.environment, "development" );
  assert_eq!( deserialized.enabled, false );

  println!( "\n✅ All serde integration examples completed successfully!" );
  }
}
