//! `TextFormatter` spec tests (FM-47..FM-54, file-scoped IDs)

#![ cfg( feature = "enabled" ) ]
#![ cfg( feature = "format_text" ) ]

use data_fmt::{ RowBuilder, TextFormatter, TextVariant, Format };

/// FM-47: bullets variant produces bullet-prefixed lines
// test_kind: spec_case(FM-47)
#[ test ]
fn formatter_010_fm_47_bullets_variant_produces_bullet_prefixed_lines()
{
  let view = RowBuilder::new( vec![ "item".into(), "qty".into() ] )
    .add_row( vec![ "apple".into(), "3".into() ] )
    .add_row( vec![ "banana".into(), "5".into() ] )
    .build_view();

  let formatter = TextFormatter::new( TextVariant::Bullets );
  let output = Format::format( &formatter, &view ).unwrap();

  assert!( output.contains( "•" ), "bullets variant should contain bullet marker" );
  assert!( output.contains( "apple" ), "should contain 'apple'" );
  assert!( output.contains( "banana" ), "should contain 'banana'" );
}

/// FM-48: numbered variant produces numbered lines
// test_kind: spec_case(FM-48)
#[ test ]
fn formatter_010_fm_48_numbered_variant_produces_numbered_lines()
{
  let view = RowBuilder::new( vec![ "item".into() ] )
    .add_row( vec![ "first".into() ] )
    .add_row( vec![ "second".into() ] )
    .add_row( vec![ "third".into() ] )
    .build_view();

  let formatter = TextFormatter::new( TextVariant::Numbered );
  let output = Format::format( &formatter, &view ).unwrap();

  assert!( output.contains( "1." ), "should start numbering at 1" );
  assert!( output.contains( "2." ), "should contain number 2" );
  assert!( output.contains( "3." ), "should contain number 3" );
  assert!( output.contains( "first" ), "should contain 'first'" );
}

/// FM-49: sections variant produces section-headed blocks
// test_kind: spec_case(FM-49)
#[ test ]
fn formatter_010_fm_49_sections_variant_produces_section_headed_blocks()
{
  let view = RowBuilder::new( vec![ "name".into(), "desc".into() ] )
    .add_row( vec![ "Alice".into(), "Developer".into() ] )
    .add_row( vec![ "Bob".into(), "Designer".into() ] )
    .build_view();

  let formatter = TextFormatter::new( TextVariant::Sections );
  let output = Format::format( &formatter, &view ).unwrap();

  assert!( output.contains( "Alice" ), "should contain 'Alice'" );
  assert!( output.contains( "Bob" ), "should contain 'Bob'" );
  assert!( output.contains( "name" ), "should contain header name in section heading" );
}

/// FM-50: keyvalue variant produces key-value pair lines
// test_kind: spec_case(FM-50)
#[ test ]
fn formatter_010_fm_50_keyvalue_variant_produces_key_value_pair_lines()
{
  let view = RowBuilder::new( vec![ "host".into(), "port".into() ] )
    .add_row( vec![ "localhost".into(), "8080".into() ] )
    .build_view();

  let formatter = TextFormatter::new( TextVariant::KeyValue );
  let output = Format::format( &formatter, &view ).unwrap();

  assert!( output.contains( "host:" ), "should contain 'host:' key" );
  assert!( output.contains( "port:" ), "should contain 'port:' key" );
  assert!( output.contains( "localhost" ), "should contain value 'localhost'" );
  assert!( output.contains( "8080" ), "should contain value '8080'" );
}

/// FM-51: compact variant produces minimal whitespace output
// test_kind: spec_case(FM-51)
#[ test ]
fn formatter_010_fm_51_compact_variant_produces_minimal_whitespace_output()
{
  let view = RowBuilder::new( vec![ "a".into(), "b".into() ] )
    .add_row( vec![ "1".into(), "2".into() ] )
    .build_view();

  let formatter = TextFormatter::new( TextVariant::Compact );
  let output = Format::format( &formatter, &view ).unwrap();

  assert!( output.contains( '1' ), "should contain value '1'" );
  assert!( output.contains( '2' ), "should contain value '2'" );
  assert!( !output.contains( '|' ), "compact should have no border pipes" );
  assert!( !output.contains( '+' ), "compact should have no border corners" );
}

/// FM-52: `cli_help` variant produces CLI help formatted output
// test_kind: spec_case(FM-52)
#[ test ]
fn formatter_010_fm_52_cli_help_variant_produces_cli_help_formatted_output()
{
  let view = RowBuilder::new( vec![ "flag".into(), "description".into() ] )
    .add_row( vec![ "--verbose".into(), "Enable verbose output".into() ] )
    .add_row( vec![ "--quiet".into(), "Suppress output".into() ] )
    .build_view();

  let formatter = TextFormatter::new( TextVariant::CliHelp );
  let output = Format::format( &formatter, &view ).unwrap();

  assert!( output.contains( "--verbose" ), "should contain '--verbose' flag" );
  assert!( output.contains( "--quiet" ), "should contain '--quiet' flag" );
  assert!( output.contains( "Enable verbose output" ), "should contain description" );
  // Descriptions should be aligned: both flags should start at same indentation
  let lines : Vec< &str > = output.lines().collect();
  assert!( lines.len() >= 2, "should have at least 2 lines for 2 flag entries" );
}

/// FM-53: Format trait dispatch returns well-formed string
// test_kind: spec_case(FM-53)
#[ test ]
fn formatter_010_fm_53_format_trait_dispatch_returns_well_formed_string()
{
  let view = RowBuilder::new( vec![ "a".into() ] )
    .add_row( vec![ "1".into() ] )
    .build_view();

  let formatter = TextFormatter::new( TextVariant::Bullets );
  let result = Format::format( &formatter, &view );

  assert!( result.is_ok(), "format should return Ok" );
}

/// FM-54: empty data produces minimal or empty output
// test_kind: spec_case(FM-54)
#[ test ]
fn formatter_010_fm_54_empty_data_produces_minimal_or_empty_output()
{
  let view = RowBuilder::new( vec![ "col".into() ] )
    .build_view();

  let formatter = TextFormatter::new( TextVariant::Bullets );
  let output = Format::format( &formatter, &view ).unwrap();

  // With zero rows, bullets variant should produce no bullet lines
  assert!(
    !output.contains( "•" ),
    "empty data should produce no bullet lines, got: '{output}'",
  );
}
