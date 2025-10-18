#![allow(clippy::all)]
//! # Argument Types Demo
//!
//! This example demonstrates all the supported argument types in Unilang,
//! including basic types, collections, and complex validation.

use unilang::data::{ ArgumentAttributes, ArgumentDefinition, CommandDefinition, Kind, OutputData, ValidationRule };
use unilang::registry::CommandRegistry;
use unilang::types::Value;

#[allow(clippy::too_many_lines)]
fn main() -> Result< (), unilang::error::Error >
{
  println!( "=== Argument Types Demo ===\n" );

  #[allow(deprecated)]
  let mut registry = CommandRegistry::new();

  // Step 1: Command with various basic argument types
  let types_demo = CommandDefinition::former()
  .name( ".types_demo" )
  .namespace( String::new() )
  .description( "Demonstrates all supported argument types".to_string() )
  .hint( "Shows how different data types work" )
  .status( "stable" )
  .version( "1.0.0" )
  .aliases( vec![] )
  .tags( vec![ "demo".to_string(), "types".to_string() ] )
  .permissions( vec![] )
  .idempotent( true )
  .deprecation_message( String::new() )
  .http_method_hint( "POST".to_string() )
  .examples( vec!
  [
    ".types_demo text::hello number::42 flag::true".to_string(),
    ".types_demo url::https://example.com path::/tmp/file".to_string()
  ])
  .arguments( vec!
  [
    // String argument
    ArgumentDefinition {
      name: "text".to_string(),
      description: "A text string argument".to_string(),
      kind: Kind::String,
      hint: "Any text string".to_string(),
      attributes: ArgumentAttributes { optional: true, ..Default::default() },
      validation_rules: vec![ ValidationRule::MinLength(3) ],
      aliases: vec![ "t".to_string() ],
      tags: vec![ "string".to_string() ],
    },

    // Integer argument
    ArgumentDefinition {
      name: "number".to_string(),
      description: "An integer number".to_string(),
      kind: Kind::Integer,
      hint: "Whole number".to_string(),
      attributes: ArgumentAttributes { optional: true, ..Default::default() },
      validation_rules: vec![ ValidationRule::Min(0.0), ValidationRule::Max(100.0) ],
      aliases: vec![ "n".to_string() ],
      tags: vec![ "numeric".to_string() ],
    },

    // Float argument
    ArgumentDefinition {
      name: "decimal".to_string(),
      description: "A floating-point number".to_string(),
      kind: Kind::Float,
      hint: "Decimal number".to_string(),
      attributes: ArgumentAttributes { optional: true, ..Default::default() },
      validation_rules: vec![ ValidationRule::Min(0.0) ],
      aliases: vec![ "d".to_string() ],
      tags: vec![ "numeric".to_string() ],
    },

    // Boolean argument
    ArgumentDefinition {
      name: "flag".to_string(),
      description: "A boolean flag".to_string(),
      kind: Kind::Boolean,
      hint: "True or false value".to_string(),
      attributes: ArgumentAttributes { optional: true, ..Default::default() },
      validation_rules: vec![],
      aliases: vec![ "f".to_string() ],
      tags: vec![ "boolean".to_string() ],
    },

    // Path argument
    ArgumentDefinition {
      name: "path".to_string(),
      description: "A file system path".to_string(),
      kind: Kind::Path,
      hint: "File or directory path".to_string(),
      attributes: ArgumentAttributes { optional: true, ..Default::default() },
      validation_rules: vec![],
      aliases: vec![ "p".to_string() ],
      tags: vec![ "filesystem".to_string() ],
    },

    // URL argument
    ArgumentDefinition {
      name: "url".to_string(),
      description: "A web URL".to_string(),
      kind: Kind::Url,
      hint: "Valid HTTP/HTTPS URL".to_string(),
      attributes: ArgumentAttributes { optional: true, ..Default::default() },
      validation_rules: vec![ ValidationRule::Pattern("^https?://".to_string()) ],
      aliases: vec![ "u".to_string() ],
      tags: vec![ "web".to_string() ],
    },

    // DateTime argument
    ArgumentDefinition {
      name: "timestamp".to_string(),
      description: "A date and time".to_string(),
      kind: Kind::DateTime,
      hint: "ISO 8601 datetime".to_string(),
      attributes: ArgumentAttributes { optional: true, ..Default::default() },
      validation_rules: vec![],
      aliases: vec![ "ts".to_string() ],
      tags: vec![ "time".to_string() ],
    },

    // Enum argument
    ArgumentDefinition {
      name: "level".to_string(),
      description: "A predefined choice".to_string(),
      kind: Kind::Enum( vec![ "debug".to_string(), "info".to_string(), "warn".to_string(), "error".to_string() ] ),
      hint: "Log level choice".to_string(),
      attributes: ArgumentAttributes { optional: true, ..Default::default() },
      validation_rules: vec![],
      aliases: vec![ "l".to_string() ],
      tags: vec![ "choice".to_string() ],
    },

    // Pattern/Regex argument
    ArgumentDefinition {
      name: "pattern".to_string(),
      description: "A regular expression pattern".to_string(),
      kind: Kind::Pattern,
      hint: "Regex pattern string".to_string(),
      attributes: ArgumentAttributes { optional: true, ..Default::default() },
      validation_rules: vec![],
      aliases: vec![ "regex".to_string() ],
      tags: vec![ "pattern".to_string() ],
    },
  ])
  .end();

  let types_routine = Box::new( | cmd : unilang::semantic::VerifiedCommand, _ctx |
  {
    println!( "Processing arguments:" );

    for ( name, value ) in &cmd.arguments
    {
      match value
      {
        Value::String( s ) => println!( "  {name}: '{s}' (String)" ),
        Value::Integer( i ) => println!( "  {name}: {i} (Integer)" ),
        Value::Float( f ) => println!( "  {name}: {f} (Float)" ),
        Value::Boolean( b ) => println!( "  {name}: {b} (Boolean)" ),
        Value::Path( p ) => println!( "  {name}: {} (Path)", p.display() ),
        Value::File( f ) => println!( "  {name}: {} (File)", f.display() ),
        Value::Directory( d ) => println!( "  {name}: {} (Directory)", d.display() ),
        Value::Enum( e ) => println!( "  {name}: '{e}' (Enum)" ),
        Value::Url( u ) => println!( "  {name}: {u} (Url)" ),
        Value::DateTime( dt ) => println!( "  {name}: {dt} (DateTime)" ),
        Value::Pattern( p ) => println!( "  {name}: {p} (Pattern)" ),
        Value::List( items ) => println!( "  {name}: {items:?} (List)" ),
        Value::Map( map ) => println!( "  {name}: {map:?} (Map)" ),
        Value::JsonString( json ) => println!( "  {name}: {json} (JsonString)" ),
        Value::Object( obj ) => println!( "  {name}: {obj:?} (Object)" ),
      }
    }

    Ok( OutputData
    {
      content : "Arguments processed successfully".to_string(),
      format : "text".to_string(),
    })
  });

  #[allow(deprecated)]
  registry.command_add_runtime( &types_demo, types_routine )?;
  println!( "✓ Registered command with various argument types" );

  println!( "\n=== Supported Argument Types ===" );
  println!( "• String - Text data" );
  println!( "• Integer - Whole numbers" );
  println!( "• Float - Decimal numbers" );
  println!( "• Boolean - True/false values" );
  println!( "• Path - File system paths" );
  println!( "• File - File paths (validated)" );
  println!( "• Directory - Directory paths (validated)" );
  println!( "• Url - Web URLs" );
  println!( "• DateTime - Date/time values" );
  println!( "• Pattern - Regular expressions" );
  println!( "• Enum - Predefined choices" );
  println!( "• List - Collections of items" );
  println!( "• Map - Key-value pairs" );
  println!( "• JsonString - JSON text" );
  println!( "• Object - JSON objects" );

  println!( "\n=== Example Usage ===" );
  println!( "cargo run --bin unilang_cli types_demo \\" );
  println!( "  text::'Hello World' \\" );
  println!( "  number::42 \\" );
  println!( "  decimal::3.14 \\" );
  println!( "  flag::true \\" );
  println!( "  path::/tmp/test \\" );
  println!( "  url::https://example.com \\" );
  println!( "  level::info \\" );
  println!( "  pattern::'^[a-z]+$'" );

  Ok( () )
}