//! Integration tests for `TextFormatter` general variants
//!
//! Covers Bullets, Numbered, `KeyValue`, Compact, and Sections variants
//! plus the builder pattern (indent/separator configuration).
//! `CliHelp` variant is tested separately in `text_cli_help.rs`.

#![ cfg( feature = "enabled" ) ]
#[ cfg( feature = "format_text" ) ]
mod text_tests
{
  use data_fmt::{ RowBuilder, TextFormatter, Format };

  #[ test ]
  fn text_formatter_bullets()
  {
    let view = RowBuilder::new( vec![ "Name".into(), "Age".into() ] )
      .add_row( vec![ "Alice".into(), "30".into() ] )
      .add_row( vec![ "Bob".into(), "25".into() ] )
      .build_view();

    let formatter = TextFormatter::bullets();
    let text = formatter.format( &view ).unwrap();

    assert!( text.contains( "•" ) );
    assert!( text.contains( "Alice 30" ) );
    assert!( text.contains( "Bob 25" ) );
  }

  #[ test ]
  fn text_formatter_numbered()
  {
    let view = RowBuilder::new( vec![ "Item".into() ] )
      .add_row( vec![ "First".into() ] )
      .add_row( vec![ "Second".into() ] )
      .build_view();

    let formatter = TextFormatter::numbered();
    let text = formatter.format( &view ).unwrap();

    assert!( text.contains( "1. First" ) );
    assert!( text.contains( "2. Second" ) );
  }

  #[ test ]
  fn text_formatter_key_value()
  {
    let view = RowBuilder::new( vec![ "Name".into(), "Age".into() ] )
      .add_row( vec![ "Alice".into(), "30".into() ] )
      .build_view();

    let formatter = TextFormatter::key_value();
    let text = formatter.format( &view ).unwrap();

    assert!( text.contains( "Name: Alice" ) );
    assert!( text.contains( "Age: 30" ) );
  }

  #[ test ]
  fn text_formatter_compact()
  {
    let view = RowBuilder::new( vec![ "Item".into() ] )
      .add_row( vec![ "A".into() ] )
      .add_row( vec![ "B".into() ] )
      .add_row( vec![ "C".into() ] )
      .build_view();

    let formatter = TextFormatter::compact();
    let text = formatter.format( &view ).unwrap();

    assert_eq!( text, "A, B, C" );
  }

  #[ test ]
  fn text_formatter_sections()
  {
    let view = RowBuilder::new( vec![ "Header1".into(), "Header2".into() ] )
      .add_row( vec![ "Value1".into(), "Value2".into() ] )
      .build_view();

    let formatter = TextFormatter::sections();
    let text = formatter.format( &view ).unwrap();

    assert!( text.contains( "Header1, Header2:" ) );
    assert!( text.contains( "Value1" ) );
  }

  #[ test ]
  fn text_formatter_builder_pattern()
  {
    let formatter = TextFormatter::bullets()
      .with_indent( 4 )
      .with_separator( ";\n".to_string() );

    assert_eq!( formatter.indent, 4 );
    assert_eq!( formatter.separator, ";\n" );
  }
}
