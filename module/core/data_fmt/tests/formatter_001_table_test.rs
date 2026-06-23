//! `TableFormatter` spec tests (FM-1..FM-7)
//!
//! Plain, bordered, Format trait dispatch, empty table, markdown, csv, config preset.

#![ cfg( feature = "enabled" ) ]

use data_fmt::{ RowBuilder, TableFormatter, TableConfig, Format };

/// FM-1: plain config produces unbordered output
// test_kind: spec_case(FM-1)
#[ test ]
fn formatter_001_fm_01_plain_config_produces_unbordered_output()
{
  let view = RowBuilder::new( vec![ "name".into(), "age".into() ] )
    .add_row( vec![ "Alice".into(), "30".into() ] )
    .build_view();

  let formatter = TableFormatter::with_config( TableConfig::plain() );
  let output = Format::format( &formatter, &view ).unwrap();

  assert!( output.contains( "name" ), "output should contain header 'name'" );
  assert!( output.contains( "Alice" ), "output should contain cell 'Alice'" );
  assert!( !output.contains( '|' ), "plain output should not contain '|'" );
  assert!( !output.contains( '+' ), "plain output should not contain '+'" );
  // No box-drawing characters
  assert!( !output.contains( '─' ), "plain output should not contain box-drawing chars" );
  assert!( output.ends_with( '\n' ), "output should end with trailing newline" );
}

/// FM-2: bordered config produces box-drawing characters
// test_kind: spec_case(FM-2)
#[ test ]
fn formatter_001_fm_02_bordered_config_produces_box_drawing_characters()
{
  let view = RowBuilder::new( vec![ "id".into(), "value".into() ] )
    .add_row( vec![ "1".into(), "hello".into() ] )
    .build_view();

  let formatter = TableFormatter::with_config( TableConfig::bordered() );
  let output = Format::format( &formatter, &view ).unwrap();

  assert!( output.contains( '|' ), "bordered output should contain '|' separators" );
  // Should have a horizontal rule between header and data
  let lines : Vec< &str > = output.lines().collect();
  assert!( lines.len() >= 3, "bordered output should have at least 3 lines (header, separator, data)" );
}

/// FM-3: Format trait dispatch returns well-formed string
// test_kind: spec_case(FM-3)
#[ test ]
fn formatter_001_fm_03_format_trait_dispatch_returns_well_formed_string()
{
  let view = RowBuilder::new( vec![ "x".into() ] )
    .add_row( vec![ "1".into() ] )
    .add_row( vec![ "2".into() ] )
    .build_view();

  let formatter = TableFormatter::new();
  let result = Format::format( &formatter, &view );

  assert!( result.is_ok(), "format should return Ok" );
  let output = result.unwrap();
  let data_lines : Vec< &str > = output.lines()
    .filter( | l | l.contains( '1' ) || l.contains( '2' ) )
    .collect();
  assert!( data_lines.len() >= 2, "should have at least 2 data lines" );
}

/// FM-4: empty table produces header-only output
// test_kind: spec_case(FM-4)
#[ test ]
fn formatter_001_fm_04_empty_table_produces_header_only_output()
{
  let view = RowBuilder::new( vec![ "col_a".into(), "col_b".into() ] )
    .build_view();

  let formatter = TableFormatter::with_config( TableConfig::plain() );
  let output = Format::format( &formatter, &view ).unwrap();

  assert!( output.contains( "col_a" ), "output should contain header 'col_a'" );
  assert!( !output.is_empty(), "output should not be empty" );
}

/// FM-5: markdown config produces pipe-delimited rows
// test_kind: spec_case(FM-5)
#[ test ]
fn formatter_001_fm_05_markdown_config_produces_pipe_delimited_rows()
{
  let view = RowBuilder::new( vec![ "key".into(), "val".into() ] )
    .add_row( vec![ "a".into(), "b".into() ] )
    .build_view();

  let formatter = TableFormatter::with_config( TableConfig::markdown() );
  let output = Format::format( &formatter, &view ).unwrap();

  let lines : Vec< &str > = output.lines().collect();
  // Header and data lines should start and end with |
  for line in &lines
  {
    let trimmed = line.trim();
    if !trimmed.is_empty()
    {
      assert!(
        trimmed.starts_with( '|' ) && trimmed.ends_with( '|' ),
        "markdown line should start and end with '|', got: '{trimmed}'",
      );
    }
  }
  // Should have a separator line with dashes
  let has_sep = lines.iter().any( | l | l.contains( "---" ) );
  assert!( has_sep, "markdown output should have a dash separator line" );
}

/// FM-6: csv config produces comma-separated values
// test_kind: spec_case(FM-6)
#[ test ]
fn formatter_001_fm_06_csv_config_produces_comma_separated_values()
{
  let view = RowBuilder::new( vec![ "name".into(), "city".into() ] )
    .add_row( vec![ "Alice".into(), "New York".into() ] )
    .add_row( vec![ "Bob".into(), "Paris".into() ] )
    .build_view();

  let formatter = TableFormatter::with_config( TableConfig::csv() );
  let output = Format::format( &formatter, &view ).unwrap();

  let lines : Vec< &str > = output.lines().collect();
  assert!( lines[ 0 ].contains( "name" ), "first line should be header" );
  assert!( lines[ 0 ].contains( ',' ), "header should be comma-separated" );
  // No box-drawing or alignment whitespace
  assert!( !output.contains( '|' ), "csv should not contain '|'" );
  assert!( !output.contains( '+' ), "csv should not contain '+'" );
}

/// FM-7: `with_config` applies the given `TableConfig` preset
// test_kind: spec_case(FM-7)
#[ test ]
fn formatter_001_fm_07_with_config_applies_the_given_table_config_preset()
{
  let view = RowBuilder::new( vec![ "a".into() ] )
    .add_row( vec![ "1".into() ] )
    .build_view();

  let plain = TableFormatter::with_config( TableConfig::plain() );
  let unicode = TableFormatter::with_config( TableConfig::unicode_box() );

  let plain_output = Format::format( &plain, &view ).unwrap();
  let unicode_output = Format::format( &unicode, &view ).unwrap();

  assert!( !plain_output.contains( '│' ), "plain should not have unicode box chars" );
  // Check for box-drawing chars in unicode output (U+2500..U+257F range)
  let has_box_drawing = unicode_output.chars().any( | c | ( '\u{2500}'..='\u{257F}' ).contains( &c ) );
  assert!( has_box_drawing, "unicode_box output should contain box-drawing characters" );
  assert_ne!( plain_output, unicode_output, "plain and unicode_box outputs should differ" );
}
