#![ allow( dead_code ) ]

//! Test reproducing the external user experience issue from `task/improve_former_derive_user_experience.md`
//!
//! This test verifies that the DESIRED import pattern works:
//! ```
//! use former::Former;  // Single, obvious import
//! ```
//!
//! NOT the current problematic pattern:
//! ```
//! use former_meta::Former;  // Requires knowledge of internal structure
//! use former_types::*;      // Requires knowledge of trait dependencies
//! ```

// TEST 1: Can we import Former from the root?
// This is the DESIRED user experience per the task
use former::Former;

/// Simple configuration struct to test basic Former usage.
///
/// TEST 2: Can we use it without additional imports?
/// The task states users currently need `use former_types::*;` but shouldn't.
#[ derive( Debug, PartialEq, Former ) ]
pub struct SimpleConfig
{
  host : String,
  port : u16,
}

/// Configuration struct with optional fields to test Former's Option handling.
///
/// TEST 3: Can we use it with Options?
#[ derive( Debug, PartialEq, Former ) ]
pub struct ConfigWithOptional
{
  host : String,
  port : u16,
  description : Option< String >,
}

// TEST 4: Can we use the builder?
#[ test ]
fn test_simple_former_usage()
{
  let config = SimpleConfig::former()
    .host( "localhost".to_string() )
    .port( 8080_u16 )
    .form();

  assert_eq!( config.host, "localhost" );
  assert_eq!( config.port, 8080 );
}

#[ test ]
fn test_former_with_optional()
{
  let config = ConfigWithOptional::former()
    .host( "localhost".to_string() )
    .port( 3000_u16 )
    .description( "Test server".to_string() )
    .form();

  assert_eq!( config.host, "localhost" );
  assert_eq!( config.port, 3000 );
  assert_eq!( config.description, Some( "Test server".to_string() ) );
}

/// Database configuration to test nested struct usage.
///
/// TEST 5: Test with nested struct (subformer)
#[ derive( Debug, PartialEq, Former ) ]
pub struct Database
{
  connection_string : String,
  timeout : u32,
}

/// Application configuration with nested Database struct.
///
/// Tests subformer functionality with `#[subform_scalar]` attribute.
#[ derive( Debug, PartialEq, Former ) ]
pub struct Application
{
  name : String,
  #[ subform_scalar ]
  database : Database,
}

#[ test ]
fn test_former_with_subform()
{
  let app = Application::former()
    .name( "MyApp".to_string() )
    .database()
      .connection_string( "postgres://localhost".to_string() )
      .timeout( 30_u32 )
      .form()
    .form();

  assert_eq!( app.name, "MyApp" );
  assert_eq!( app.database.connection_string, "postgres://localhost" );
  assert_eq!( app.database.timeout, 30 );
}
