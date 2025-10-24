//! Comprehensive integration tests for unified Format trait
//!
//! Tests all formatters through the common Format trait interface,
//! ensuring consistent behavior across different output formats.

#![ allow( clippy::all, clippy::pedantic, clippy::nursery, warnings ) ]

use tree_fmt::{ RowBuilder, TableView, TableMetadata, Format, FormatError };

// =============================================================================
// Format Trait Consistency Tests
// =============================================================================

#[ test ]
fn test_row_builder_produces_table_view()
{
  let view = RowBuilder::new( vec![ "Name".into(), "Age".into() ] )
    .add_row( vec![ "Alice".into(), "30".into() ] )
    .add_row( vec![ "Bob".into(), "25".into() ] )
    .build_view();

  assert_eq!( view.metadata.column_names.len(), 2 );
  assert_eq!( view.rows.len(), 2 );
  assert_eq!( view.rows[ 0 ][ 0 ], "Alice" );
}

#[ test ]
fn test_table_view_direct_construction()
{
  let view = TableView::new(
    TableMetadata::new( vec![ "Col1".into(), "Col2".into() ] ),
    vec![
      vec![ "A".into(), "B".into() ],
      vec![ "C".into(), "D".into() ],
    ]
  );

  assert_eq!( view.metadata.column_names, vec![ "Col1", "Col2" ] );
  assert_eq!( view.rows.len(), 2 );
}

// =============================================================================
// TableFormatter with Format Trait
// =============================================================================

#[ cfg( feature = "format_table" ) ]
mod table_format_tests
{
  use super::*;
  use tree_fmt::{ TableFormatter, TableConfig };

  #[ test ]
  fn test_table_formatter_via_format_trait()
  {
    let view = RowBuilder::new( vec![ "Name".into() ] )
      .add_row( vec![ "Alice".into() ] )
      .build_view();

    let formatter = TableFormatter::with_config( TableConfig::plain() );
    let output = Format::format( &formatter, &view ).unwrap();

    assert!( output.contains( "Name" ) );
    assert!( output.contains( "Alice" ) );
    assert!( output.contains( "----" ) ); // Dash separator
  }

  #[ test ]
  fn test_table_formatter_all_presets()
  {
    let view = RowBuilder::new( vec![ "A".into(), "B".into() ] )
      .add_row( vec![ "1".into(), "2".into() ] )
      .build_view();

    let presets = vec![
      ("plain", TableConfig::plain()),
      ("minimal", TableConfig::minimal()),
      ("bordered", TableConfig::bordered()),
      ("markdown", TableConfig::markdown()),
      ("grid", TableConfig::grid()),
      ("unicode_box", TableConfig::unicode_box()),
      ("csv", TableConfig::csv()),
      ("tsv", TableConfig::tsv()),
      ("compact", TableConfig::compact()),
    ];

    for (name, config) in presets
    {
      let formatter = TableFormatter::with_config( config );
      let result = Format::format( &formatter, &view );
      assert!( result.is_ok(), "{} preset should format successfully", name );

      let output = result.unwrap();
      assert!( !output.is_empty(), "{} preset should produce output", name );
      assert!( output.contains( "A" ), "{} preset should contain data", name );
    }
  }

  #[ test ]
  fn test_table_formatter_empty_view()
  {
    let view = TableView::new(
      TableMetadata::new( vec![ "Col".into() ] ),
      vec![]
    );

    let formatter = TableFormatter::with_config( TableConfig::plain() );
    let output = Format::format( &formatter, &view ).unwrap();

    // Empty table should produce minimal output
    assert!( output.trim().is_empty() || output.len() < 10 );
  }
}

// =============================================================================
// JsonFormatter Tests
// =============================================================================

#[ cfg( feature = "format_json" ) ]
mod json_format_tests
{
  use super::*;
  use tree_fmt::JsonFormatter;

  #[ test ]
  fn test_json_formatter_via_format_trait()
  {
    let view = RowBuilder::new( vec![ "Name".into(), "Age".into() ] )
      .add_row( vec![ "Alice".into(), "30".into() ] )
      .build_view();

    let formatter = JsonFormatter::new();
    let output = Format::format( &formatter, &view ).unwrap();

    // Should be array format: [{"Name": "Alice", "Age": "30"}]
    assert!( output.starts_with( '[' ) );
    assert!( output.trim().ends_with( ']' ) );
    assert!( output.contains( "\"Name\"" ) );
    assert!( output.contains( "\"Alice\"" ) );
    assert!( output.contains( "\"Age\"" ) );
    assert!( output.contains( "\"30\"" ) );
  }

  #[ test ]
  fn test_json_formatter_pretty_vs_compact()
  {
    let view = RowBuilder::new( vec![ "X".into() ] )
      .add_row( vec![ "Y".into() ] )
      .build_view();

    let pretty = JsonFormatter::new().format( &view ).unwrap();
    let compact = JsonFormatter::compact().format( &view ).unwrap();

    // Pretty should have more newlines
    assert!( pretty.matches( '\n' ).count() > compact.matches( '\n' ).count() );

    // Both should have same data
    assert!( pretty.contains( "\"X\"" ) );
    assert!( compact.contains( "\"X\"" ) );
  }

  #[ test ]
  fn test_json_formatter_special_characters()
  {
    let view = RowBuilder::new( vec![ "Key".into() ] )
      .add_row( vec![ "Value with \"quotes\" and \\ backslash".into() ] )
      .build_view();

    let formatter = JsonFormatter::new();
    let output = Format::format( &formatter, &view ).unwrap();

    // JSON should properly escape special characters
    assert!( output.contains( "\\\"" ) ); // Escaped quotes
    assert!( output.contains( "\\\\" ) ); // Escaped backslash
  }

  #[ test ]
  fn test_json_formatter_unicode()
  {
    let view = RowBuilder::new( vec![ "Name".into() ] )
      .add_row( vec![ "Алиса".into() ] ) // Cyrillic
      .add_row( vec![ "爱丽丝".into() ] ) // Chinese
      .build_view();

    let formatter = JsonFormatter::new();
    let output = Format::format( &formatter, &view ).unwrap();

    assert!( output.contains( "Алиса" ) || output.contains( "\\u" ) );
    assert!( output.contains( "爱丽丝" ) || output.contains( "\\u" ) );
  }
}

// =============================================================================
// YamlFormatter Tests
// =============================================================================

#[ cfg( feature = "format_yaml" ) ]
mod yaml_format_tests
{
  use super::*;
  use tree_fmt::YamlFormatter;

  #[ test ]
  fn test_yaml_formatter_via_format_trait()
  {
    let view = RowBuilder::new( vec![ "Name".into(), "Age".into() ] )
      .add_row( vec![ "Alice".into(), "30".into() ] )
      .build_view();

    let formatter = YamlFormatter::new();
    let output = Format::format( &formatter, &view ).unwrap();

    // Should be YAML list format with keys
    assert!( output.contains( '-' ) );
    assert!( output.contains( "Name:" ) );
    assert!( output.contains( "Age:" ) );
    assert!( output.contains( "Alice" ) );
    assert!( output.contains( "30" ) || output.contains( "'30'" ) || output.contains( "\"30\"" ) );
  }

  #[ test ]
  fn test_yaml_formatter_no_json_syntax()
  {
    let view = RowBuilder::new( vec![ "A".into() ] )
      .add_row( vec![ "B".into() ] )
      .build_view();

    let formatter = YamlFormatter::new();
    let output = Format::format( &formatter, &view ).unwrap();

    // YAML should use dashes and colons, not JSON braces
    assert!( !output.contains( '{' ) );
    // Note: YAML may use [] for empty arrays, which is fine
    assert!( output.contains( '-' ) || output.contains( "[]" ) );
    assert!( output.contains( ':' ) );
  }
}

// =============================================================================
// TomlFormatter Tests
// =============================================================================

#[ cfg( feature = "format_toml" ) ]
mod toml_format_tests
{
  use super::*;
  use tree_fmt::TomlFormatter;

  #[ test ]
  fn test_toml_formatter_via_format_trait()
  {
    let view = RowBuilder::new( vec![ "Name".into(), "Age".into() ] )
      .add_row( vec![ "Alice".into(), "30".into() ] )
      .build_view();

    let formatter = TomlFormatter::new();
    let output = Format::format( &formatter, &view ).unwrap();

    // Should be TOML array of tables format: [[row]]
    assert!( output.contains( "[[row]]" ) );
    assert!( output.contains( "Name" ) );
    assert!( output.contains( "Age" ) );
    assert!( output.contains( "Alice" ) );
    assert!( output.contains( "30" ) );
  }
}

// =============================================================================
// TextFormatter Tests
// =============================================================================

#[ cfg( feature = "format_text" ) ]
mod text_format_tests
{
  use super::*;
  use tree_fmt::{ TextFormatter, TextVariant };

  #[ test ]
  fn test_text_formatter_bullets()
  {
    let view = RowBuilder::new( vec![ "Name".into(), "Age".into() ] )
      .add_row( vec![ "Alice".into(), "30".into() ] )
      .add_row( vec![ "Bob".into(), "25".into() ] )
      .build_view();

    let formatter = TextFormatter::bullets();
    let output = Format::format( &formatter, &view ).unwrap();

    assert!( output.contains( "•" ) );
    assert!( output.contains( "Alice 30" ) );
    assert!( output.contains( "Bob 25" ) );
  }

  #[ test ]
  fn test_text_formatter_numbered()
  {
    let view = RowBuilder::new( vec![ "Item".into() ] )
      .add_row( vec![ "First".into() ] )
      .add_row( vec![ "Second".into() ] )
      .add_row( vec![ "Third".into() ] )
      .build_view();

    let formatter = TextFormatter::numbered();
    let output = Format::format( &formatter, &view ).unwrap();

    assert!( output.contains( "1. First" ) );
    assert!( output.contains( "2. Second" ) );
    assert!( output.contains( "3. Third" ) );
  }

  #[ test ]
  fn test_text_formatter_key_value()
  {
    let view = RowBuilder::new( vec![ "Name".into(), "Age".into(), "City".into() ] )
      .add_row( vec![ "Alice".into(), "30".into(), "NYC".into() ] )
      .build_view();

    let formatter = TextFormatter::key_value();
    let output = Format::format( &formatter, &view ).unwrap();

    assert!( output.contains( "Name: Alice" ) );
    assert!( output.contains( "Age: 30" ) );
    assert!( output.contains( "City: NYC" ) );
  }

  #[ test ]
  fn test_text_formatter_compact()
  {
    let view = RowBuilder::new( vec![ "Item".into() ] )
      .add_row( vec![ "A".into() ] )
      .add_row( vec![ "B".into() ] )
      .add_row( vec![ "C".into() ] )
      .build_view();

    let formatter = TextFormatter::compact();
    let output = Format::format( &formatter, &view ).unwrap();

    assert_eq!( output, "A, B, C" );
  }

  #[ test ]
  fn test_text_formatter_all_styles()
  {
    let view = RowBuilder::new( vec![ "Col".into() ] )
      .add_row( vec![ "Data".into() ] )
      .build_view();

    let styles = vec![
      TextVariant::Bullets,
      TextVariant::Numbered,
      TextVariant::KeyValue,
      TextVariant::Sections,
      TextVariant::Compact,
    ];

    for style in styles
    {
      let formatter = TextFormatter::new( style );
      let result = Format::format( &formatter, &view );
      assert!( result.is_ok(), "{:?} style should format successfully", style );

      let output = result.unwrap();
      assert!( !output.is_empty(), "{:?} style should produce output", style );
    }
  }

  #[ test ]
  fn test_text_formatter_custom_indent()
  {
    let view = RowBuilder::new( vec![ "Item".into() ] )
      .add_row( vec![ "Data".into() ] )
      .build_view();

    let formatter = TextFormatter::bullets().with_indent( 4 );
    let output = Format::format( &formatter, &view ).unwrap();

    assert!( output.starts_with( "    •" ) ); // 4 spaces before bullet
  }
}

// =============================================================================
// Cross-Format Consistency Tests
// =============================================================================

#[ test ]
fn test_same_data_all_formats()
{
  let view = RowBuilder::new( vec![ "Name".into(), "Score".into() ] )
    .add_row( vec![ "Alice".into(), "95".into() ] )
    .add_row( vec![ "Bob".into(), "87".into() ] )
    .build_view();

  // All formats should contain the data
  #[ cfg( feature = "format_table" ) ]
  {
    use tree_fmt::{ TableFormatter, TableConfig };
    let formatter = TableFormatter::with_config( TableConfig::plain() );
    let output = Format::format( &formatter, &view ).unwrap();
    assert!( output.contains( "Alice" ) && output.contains( "95" ) );
  }

  #[ cfg( feature = "format_json" ) ]
  {
    use tree_fmt::JsonFormatter;
    let formatter = JsonFormatter::new();
    let output = Format::format( &formatter, &view ).unwrap();
    assert!( output.contains( "Alice" ) && output.contains( "95" ) );
  }

  #[ cfg( feature = "format_yaml" ) ]
  {
    use tree_fmt::YamlFormatter;
    let formatter = YamlFormatter::new();
    let output = Format::format( &formatter, &view ).unwrap();
    assert!( output.contains( "Alice" ) && output.contains( "95" ) );
  }

  #[ cfg( feature = "format_text" ) ]
  {
    use tree_fmt::TextFormatter;
    let formatter = TextFormatter::bullets();
    let output = Format::format( &formatter, &view ).unwrap();
    assert!( output.contains( "Alice" ) && output.contains( "95" ) );
  }
}

#[ test ]
fn test_empty_table_all_formats()
{
  let view = TableView::new(
    TableMetadata::new( vec![ "Col".into() ] ),
    vec![]
  );

  // All formats should handle empty tables gracefully
  #[ cfg( feature = "format_table" ) ]
  {
    use tree_fmt::{ TableFormatter, TableConfig, Format };
    let formatter = TableFormatter::with_config( TableConfig::plain() );
    assert!( Format::format( &formatter, &view ).is_ok() );
  }

  #[ cfg( feature = "format_json" ) ]
  {
    use tree_fmt::JsonFormatter;
    let formatter = JsonFormatter::new();
    let output = Format::format( &formatter, &view ).unwrap();
    assert!( output.contains( "[]" ) ); // Empty rows array
  }

  #[ cfg( feature = "format_yaml" ) ]
  {
    use tree_fmt::{ YamlFormatter, Format };
    let formatter = YamlFormatter::new();
    assert!( Format::format( &formatter, &view ).is_ok() );
  }

  #[ cfg( feature = "format_text" ) ]
  {
    use tree_fmt::TextFormatter;
    let formatter = TextFormatter::bullets();
    let output = Format::format( &formatter, &view ).unwrap();
    assert!( output.is_empty() || output.trim().is_empty() );
  }
}

// =============================================================================
// Error Handling Tests
// =============================================================================

#[ test ]
fn test_format_error_display()
{
  #[ cfg( feature = "serde_support" ) ]
  {
    let err = FormatError::Serialization( "test error".to_string() );
    let display = format!( "{}", err );
    assert!( display.contains( "Serialization error" ) );
    assert!( display.contains( "test error" ) );
  }

  let err = FormatError::InvalidData( "invalid".to_string() );
  let display = format!( "{}", err );
  assert!( display.contains( "Invalid data" ) );
}
