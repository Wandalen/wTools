#![allow(clippy::all)]
//! # Collection Types Demo
//!
//! This example demonstrates how to work with List and Map argument types,
//! including custom delimiters and nested structures.

use unilang::data::{ ArgumentAttributes, ArgumentDefinition, CommandDefinition, Kind, OutputData };
use unilang::registry::CommandRegistry;
use unilang::types::Value;

#[allow(clippy::too_many_lines)]
fn main() -> Result< (), unilang::error::Error >
{
  println!( "=== Collection Types Demo ===\n" );

  #[allow(deprecated)]
  let mut registry = CommandRegistry::new();

  // Step 1: Command demonstrating List types
  let list_demo = CommandDefinition::former()
  .name( ".list_demo" )
  .namespace( "collections".to_string() )
  .description( "Demonstrates List argument types with various delimiters".to_string() )
  .hint( "Shows how to work with lists" )
  .status( "stable" )
  .version( "1.0.0" )
  .aliases( vec![ ".lists".to_string() ] )
  .tags( vec![ "collections".to_string(), "demo".to_string() ] )
  .permissions( vec![] )
  .idempotent( true )
  .deprecation_message( String::new() )
  .http_method_hint( "POST".to_string() )
  .examples( vec!
  [
    "collections.list_demo numbers::1,2,3,4".to_string(),
    "collections.list_demo words::apple|banana|cherry".to_string(),
  ])
  .arguments( vec!
  [
    // List of integers with comma delimiter
    ArgumentDefinition {
      name: "numbers".to_string(),
      description: "A list of numbers separated by commas".to_string(),
      kind: Kind::List( Box::new( Kind::Integer ), Some( ',' ) ),
      hint: "Comma-separated integers".to_string(),
      attributes: ArgumentAttributes { optional: true, ..Default::default() },
      validation_rules: vec![],
      aliases: vec![ "nums".to_string() ],
      tags: vec![ "numeric".to_string(), "list".to_string() ],
    },

    // List of strings with pipe delimiter
    ArgumentDefinition {
      name: "words".to_string(),
      description: "A list of words separated by pipes".to_string(),
      kind: Kind::List( Box::new( Kind::String ), Some( '|' ) ),
      hint: "Pipe-separated strings".to_string(),
      attributes: ArgumentAttributes { optional: true, ..Default::default() },
      validation_rules: vec![],
      aliases: vec![ "w".to_string() ],
      tags: vec![ "text".to_string(), "list".to_string() ],
    },

    // List with default delimiter (space)
    ArgumentDefinition {
      name: "files".to_string(),
      description: "A list of file paths".to_string(),
      kind: Kind::List( Box::new( Kind::Path ), None ),
      hint: "Space-separated paths".to_string(),
      attributes: ArgumentAttributes { optional: true, multiple: true, ..Default::default() },
      validation_rules: vec![],
      aliases: vec![ "f".to_string() ],
      tags: vec![ "filesystem".to_string(), "list".to_string() ],
    },
  ])
  .end();

  let list_routine = Box::new( | cmd : unilang::semantic::VerifiedCommand, _ctx |
  {
    println!( "Processing list arguments:" );

    for ( name, value ) in &cmd.arguments
    {
      match value
      {
        Value::List( items ) =>
        {
          println!( "  {} (List with {} items):", name, items.len() );
          for ( i, item ) in items.iter().enumerate()
          {
            println!( "    [{i}]: {item:?}" );
          }
        },
        _ => println!( "  {name}: {value:?} (not a list)" ),
      }
    }

    Ok( OutputData
    {
      content : "List arguments processed".to_string(),
      format : "text".to_string(),
    })
  });

  #[allow(deprecated)]
  registry.command_add_runtime( &list_demo, list_routine )?;

  // Step 2: Command demonstrating Map types
  let map_demo = CommandDefinition::former()
  .name( ".map_demo" )
  .namespace( "collections".to_string() )
  .description( "Demonstrates Map argument types with custom delimiters".to_string() )
  .hint( "Shows how to work with key-value maps" )
  .status( "stable" )
  .version( "1.0.0" )
  .aliases( vec![ ".maps".to_string() ] )
  .tags( vec![ "collections".to_string(), "demo".to_string() ] )
  .permissions( vec![] )
  .idempotent( true )
  .deprecation_message( String::new() )
  .http_method_hint( "POST".to_string() )
  .examples( vec!
  [
    "collections.map_demo config::name=John,age=30,city=NYC".to_string(),
    "collections.map_demo scores::Alice:95|Bob:87|Carol:92".to_string(),
  ])
  .arguments( vec!
  [
    // Map with comma entry delimiter and equals key-value delimiter
    ArgumentDefinition {
      name: "config".to_string(),
      description: "Configuration key-value pairs".to_string(),
      kind: Kind::Map
      (
        Box::new( Kind::String ),
        Box::new( Kind::String ),
        Some( ',' ), // entry delimiter
        Some( '=' )  // key-value delimiter
      ),
      hint: "Format: key=value,key2=value2".to_string(),
      attributes: ArgumentAttributes { optional: true, ..Default::default() },
      validation_rules: vec![],
      aliases: vec![ "cfg".to_string() ],
      tags: vec![ "configuration".to_string(), "map".to_string() ],
    },

    // Map with pipe entry delimiter and colon key-value delimiter
    ArgumentDefinition {
      name: "scores".to_string(),
      description: "Student scores as name-value pairs".to_string(),
      kind: Kind::Map
      (
        Box::new( Kind::String ),
        Box::new( Kind::Integer ),
        Some( '|' ), // entry delimiter
        Some( ':' )  // key-value delimiter
      ),
      hint: "Format: name:score|name2:score2".to_string(),
      attributes: ArgumentAttributes { optional: true, ..Default::default() },
      validation_rules: vec![],
      aliases: vec![ "s".to_string() ],
      tags: vec![ "scoring".to_string(), "map".to_string() ],
    },

    // Map with default delimiters
    ArgumentDefinition {
      name: "metadata".to_string(),
      description: "Generic metadata pairs".to_string(),
      kind: Kind::Map
      (
        Box::new( Kind::String ),
        Box::new( Kind::String ),
        None, // default entry delimiter
        None  // default key-value delimiter
      ),
      hint: "Key-value metadata".to_string(),
      attributes: ArgumentAttributes { optional: true, ..Default::default() },
      validation_rules: vec![],
      aliases: vec![ "meta".to_string() ],
      tags: vec![ "metadata".to_string(), "map".to_string() ],
    },
  ])
  .end();

  let map_routine = Box::new( | cmd : unilang::semantic::VerifiedCommand, _ctx |
  {
    println!( "Processing map arguments:" );

    for ( name, value ) in &cmd.arguments
    {
      match value
      {
        Value::Map( map ) =>
        {
          println!( "  {} (Map with {} entries):", name, map.len() );
          for ( key, val ) in map
          {
            println!( "    '{key}' => {val:?}" );
          }
        },
        _ => println!( "  {name}: {value:?} (not a map)" ),
      }
    }

    Ok( OutputData
    {
      content : "Map arguments processed".to_string(),
      format : "text".to_string(),
    })
  });

  #[allow(deprecated)]
  registry.command_add_runtime( &map_demo, map_routine )?;

  println!( "✓ Registered collection type demonstration commands" );

  println!( "\n=== Collection Types Overview ===" );
  println!( "List Types:" );
  println!( "  • List(ItemType) - Default space-separated" );
  println!( "  • List(ItemType, delimiter) - Custom delimiter" );
  println!( "  • Multiple values can be handled positionally" );

  println!( "\nMap Types:" );
  println!( "  • Map(KeyType, ValueType) - Default delimiters" );
  println!( "  • Map(KeyType, ValueType, entry_delim, kv_delim) - Custom delimiters" );
  println!( "  • Supports nested types for keys and values" );

  println!( "\n=== Example Usage ===" );
  println!( "# List examples:" );
  println!( "cargo run --bin unilang_cli collections.list_demo numbers::1,2,3,4,5" );
  println!( "cargo run --bin unilang_cli collections.list_demo words::apple|banana|cherry" );
  println!( "cargo run --bin unilang_cli collections.list_demo files::'file1.txt file2.txt file3.txt'" );

  println!( "\n# Map examples:" );
  println!( "cargo run --bin unilang_cli collections.map_demo config::name=John,age=30,city=NYC" );
  println!( "cargo run --bin unilang_cli collections.map_demo scores::Alice:95|Bob:87|Carol:92" );

  Ok( () )
}