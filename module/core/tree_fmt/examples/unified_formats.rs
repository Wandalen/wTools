//! Demonstrates the unified Format trait interface for all formatters.
//!
//! This example shows how the same data structure (`TableView`) can be formatted
//! in different ways using the Format trait, enabling format-agnostic code.

use tree_fmt::{ RowBuilder, Format };

fn main()
{
  // Build sample data once
  let view = RowBuilder::new( vec!
  [
    "Package".into(),
    "Version".into(),
    "Status".into(),
  ])
  .add_row( vec![ "serde".into(), "1.0.228".into(), "✅".into() ] )
  .add_row( vec![ "tokio".into(), "1.42.0".into(), "✅".into() ] )
  .add_row( vec![ "clap".into(), "4.5.23".into(), "✅".into() ] )
  .build_view();

  // Use the same data with different formatters through unified interface
  println!( "=== TABLE FORMAT (Plain Style) ===" );
  #[ cfg( feature = "format_table" ) ]
  {
    use tree_fmt::{ TableFormatter, TableConfig };
    let formatter = TableFormatter::with_config( TableConfig::plain() );
    println!( "{}", Format::format( &formatter, &view ).unwrap() );
  }
  #[ cfg( not( feature = "format_table" ) ) ]
  println!( "TableFormatter not enabled (requires 'format_table' feature)" );

  println!( "\n=== JSON FORMAT ===" );
  #[ cfg( feature = "format_json" ) ]
  {
    use tree_fmt::JsonFormatter;
    let formatter = JsonFormatter::new();
    println!( "{}", Format::format( &formatter, &view ).unwrap() );
  }
  #[ cfg( not( feature = "format_json" ) ) ]
  println!( "JsonFormatter not enabled (requires 'format_json' feature)" );

  println!( "\n=== YAML FORMAT ===" );
  #[ cfg( feature = "format_yaml" ) ]
  {
    use tree_fmt::YamlFormatter;
    let formatter = YamlFormatter::new();
    println!( "{}", Format::format( &formatter, &view ).unwrap() );
  }
  #[ cfg( not( feature = "format_yaml" ) ) ]
  println!( "YamlFormatter not enabled (requires 'format_yaml' feature)" );

  println!( "\n=== TEXT FORMAT (Bullets) ===" );
  #[ cfg( feature = "format_text" ) ]
  {
    use tree_fmt::TextFormatter;
    let formatter = TextFormatter::bullets();
    println!( "{}", Format::format( &formatter, &view ).unwrap() );
  }
  #[ cfg( not( feature = "format_text" ) ) ]
  println!( "TextFormatter not enabled (requires 'format_text' feature)" );

  println!( "\n=== TEXT FORMAT (Key-Value) ===" );
  #[ cfg( feature = "format_text" ) ]
  {
    use tree_fmt::TextFormatter;
    let formatter = TextFormatter::key_value();
    println!( "{}", Format::format( &formatter, &view ).unwrap() );
  }
  #[ cfg( not( feature = "format_text" ) ) ]
  println!( "TextFormatter not enabled (requires 'format_text' feature)" );
}
