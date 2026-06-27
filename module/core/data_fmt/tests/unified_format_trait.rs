//! Comprehensive integration tests for unified Format trait
//!
//! Tests all formatters through the common Format trait interface,
//! ensuring consistent behavior across different output formats.

#![ cfg( feature = "enabled" ) ]

#[ allow( unused_imports ) ]
use data_fmt::{ RowBuilder, TableView, TableMetadata, Format, FormatError };

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
  assert_eq!( view.rows[ 0 ][ 0 ].render(), "Alice" );
}

#[ test ]
fn test_table_view_direct_construction()
{
  let view = TableView::new(
    TableMetadata::new( vec![ "Col1".to_string(), "Col2".to_string() ] ),
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
  use data_fmt::{ TableFormatter, TableConfig };

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
      assert!( result.is_ok(), "{name} preset should format successfully" );

      let output = result.unwrap();
      assert!( !output.is_empty(), "{name} preset should produce output" );
      assert!( output.contains( 'A' ), "{name} preset should contain data" );
    }
  }

  #[ test ]
  fn test_table_formatter_empty_view()
  {
    let view = TableView::new(
      TableMetadata::new( vec![ "Col".to_string() ] ),
      vec![]
    );

    let formatter = TableFormatter::with_config( TableConfig::plain() );
    let output = Format::format( &formatter, &view ).unwrap();

    // Empty table should produce minimal output
    assert!( output.trim().is_empty() || output.len() < 10, "empty table should produce minimal output; got {} bytes:\n{output}", output.len() );
  }
}

// =============================================================================
// JsonFormatter Tests
// =============================================================================

#[ cfg( feature = "format_json" ) ]
mod json_format_tests
{
  use super::*;
  use data_fmt::JsonFormatter;

  #[ test ]
  fn test_json_formatter_via_format_trait()
  {
    let view = RowBuilder::new( vec![ "Name".into(), "Age".into() ] )
      .add_row( vec![ "Alice".into(), "30".into() ] )
      .build_view();

    let formatter = JsonFormatter::new();
    let output = Format::format( &formatter, &view ).unwrap();

    // Should be array format: [{"Name": "Alice", "Age": "30"}]
    assert!( output.starts_with( '[' ), "JSON output should start with '['; got:\n{output}" );
    assert!( output.trim().ends_with( ']' ), "JSON output should end with ']'; got:\n{output}" );
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
    assert!( pretty.matches( '\n' ).count() > compact.matches( '\n' ).count(), "pretty JSON should have more newlines than compact; pretty={}, compact={}", pretty.matches( '\n' ).count(), compact.matches( '\n' ).count() );

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
  use data_fmt::YamlFormatter;

  #[ test ]
  fn test_yaml_formatter_via_format_trait()
  {
    let view = RowBuilder::new( vec![ "Name".into(), "Age".into() ] )
      .add_row( vec![ "Alice".into(), "30".into() ] )
      .build_view();

    let formatter = YamlFormatter::new();
    let output = Format::format( &formatter, &view ).unwrap();

    // Should be YAML list format with keys
    assert!( output.contains( '-' ), "YAML output should contain list marker '-':\n{output}" );
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
    assert!( !output.contains( '{' ), "YAML output should not contain JSON-style '{{' braces:\n{output}" );
    // Note: YAML may use [] for empty arrays, which is fine
    assert!( output.contains( '-' ) || output.contains( "[]" ), "YAML output should contain '-' or '[]':\n{output}" );
    assert!( output.contains( ':' ), "YAML output should contain ':' key-value separator:\n{output}" );
  }
}

// =============================================================================
// TomlFormatter Tests
// =============================================================================

#[ cfg( feature = "format_toml" ) ]
mod toml_format_tests
{
  use super::*;
  use data_fmt::TomlFormatter;

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
  use data_fmt::{ TextFormatter, TextVariant };

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
      assert!( result.is_ok(), "{style:?} style should format successfully" );

      let output = result.unwrap();
      assert!( !output.is_empty(), "{style:?} style should produce output" );
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

#[ allow( unused_variables ) ]
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
    use data_fmt::{ TableFormatter, TableConfig };
    let formatter = TableFormatter::with_config( TableConfig::plain() );
    let output = Format::format( &formatter, &view ).unwrap();
    assert!( output.contains( "Alice" ) && output.contains( "95" ) );
  }

  #[ cfg( feature = "format_json" ) ]
  {
    use data_fmt::JsonFormatter;
    let formatter = JsonFormatter::new();
    let output = Format::format( &formatter, &view ).unwrap();
    assert!( output.contains( "Alice" ) && output.contains( "95" ) );
  }

  #[ cfg( feature = "format_yaml" ) ]
  {
    use data_fmt::YamlFormatter;
    let formatter = YamlFormatter::new();
    let output = Format::format( &formatter, &view ).unwrap();
    assert!( output.contains( "Alice" ) && output.contains( "95" ) );
  }

  #[ cfg( feature = "format_text" ) ]
  {
    use data_fmt::TextFormatter;
    let formatter = TextFormatter::bullets();
    let output = Format::format( &formatter, &view ).unwrap();
    assert!( output.contains( "Alice" ) && output.contains( "95" ) );
  }
}

#[ allow( unused_variables ) ]
#[ test ]
fn test_empty_table_all_formats()
{
  let view = TableView::new(
    TableMetadata::new( vec![ "Col".to_string() ] ),
    vec![]
  );

  // All formats should handle empty tables gracefully
  #[ cfg( feature = "format_table" ) ]
  {
    use data_fmt::{ TableFormatter, TableConfig, Format };
    let formatter = TableFormatter::with_config( TableConfig::plain() );
    assert!( Format::format( &formatter, &view ).is_ok(), "TableFormatter should handle empty table without error" );
  }

  #[ cfg( feature = "format_json" ) ]
  {
    use data_fmt::JsonFormatter;
    let formatter = JsonFormatter::new();
    let output = Format::format( &formatter, &view ).unwrap();
    assert!( output.contains( "[]" ) ); // Empty rows array
  }

  #[ cfg( feature = "format_yaml" ) ]
  {
    use data_fmt::{ YamlFormatter, Format };
    let formatter = YamlFormatter::new();
    assert!( Format::format( &formatter, &view ).is_ok(), "YamlFormatter should handle empty table without error" );
  }

  #[ cfg( feature = "format_text" ) ]
  {
    use data_fmt::TextFormatter;
    let formatter = TextFormatter::bullets();
    let output = Format::format( &formatter, &view ).unwrap();
    assert!( output.is_empty() || output.trim().is_empty(), "TextFormatter bullets on empty table should produce empty output; got:\n{output}" );
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
    let display = format!( "{err}" );
    assert!( display.contains( "Serialization error" ) );
    assert!( display.contains( "test error" ) );
  }

  let err = FormatError::InvalidData( "invalid".to_string() );
  let display = format!( "{err}" );
  assert!( display.contains( "Invalid data" ) );
}

/// FT-5 — `feature/003`: `TreeFormatter` dispatches directly (not via `Format` trait).
///
/// `TreeFormatter::format()` and `TreeFormatter::format_aligned()` are called directly.
/// Both return non-empty strings with node content. `TreeFormatter` does NOT implement
/// the `Format` trait — direct dispatch is the only invocation path.
// test_kind: standard
#[ test ]
fn tree_formatter_direct_dispatch_not_format_trait_ft5()
{
  use data_fmt::{ TreeNode, TreeFormatter, TreeConfig, ColumnData };

  // Build a simple tree: root with one child carrying generic data
  let mut root : TreeNode< u64 > = TreeNode::new( "root".to_string(), None );
  root.children.push( TreeNode::new( "child".to_string(), Some( 42u64 ) ) );

  // Direct dispatch via format() — NOT Format::format()
  let formatter = TreeFormatter::new();
  let output = formatter.format( &root, u64::to_string );

  assert!(
    !output.is_empty(),
    "TreeFormatter::format() direct dispatch must produce non-empty output:\n{output:?}",
  );
  assert!(
    output.contains( "42" ) || output.contains( "child" ),
    "direct dispatch output must contain child node data:\n{output:?}",
  );

  // Direct dispatch via format_aligned() with ColumnData
  let mut root_aligned : TreeNode< ColumnData > = TreeNode::new( "root".to_string(), None );
  root_aligned.children.push( TreeNode::new( "item".to_string(),
    Some( ColumnData::new( vec![ "val_a".to_string(), "val_b".to_string() ] ) )
  ) );

  let fmt_aligned = TreeFormatter::with_config( TreeConfig::new() );
  let out_aligned = fmt_aligned.format_aligned( &root_aligned );

  assert!(
    !out_aligned.is_empty(),
    "TreeFormatter::format_aligned() direct dispatch must produce non-empty output:\n{out_aligned:?}",
  );
  assert!(
    out_aligned.contains( "val_a" ),
    "format_aligned() output must contain column data:\n{out_aligned:?}",
  );

  // TreeFormatter does NOT implement the Format trait — this would not compile:
  // let _: &dyn Format = &formatter;  // compile error: Format not implemented
  // The absence of the impl is the architectural contract for direct-dispatch-only use.
}
