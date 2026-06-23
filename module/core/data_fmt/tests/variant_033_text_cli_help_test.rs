//! Variant 033: Text CLI Help spec tests (VT-1..VT-4)

#![ cfg( feature = "enabled" ) ]

use data_fmt::{ RowBuilder, TextFormatter, TextVariant, Format };

/// VT-1: output uses automatic alignment spacing
// test_kind: spec_case(VT-1)
#[ test ]
fn variant_033_vt_01_alignment_spacing()
{
  let view = RowBuilder::new( vec![ "Term".into(), "Description".into() ] )
    .add_row( vec![ "OPTIONS".into(), "".into() ] )
    .add_row( vec![ "--verbose".into(), "Enable verbose output".into() ] )
    .add_row( vec![ "--help".into(), "Show help".into() ] )
    .build_view();

  let out = Format::format(
    &TextFormatter::new( TextVariant::CliHelp ),
    &view,
  ).unwrap();

  // Descriptions should be aligned at the same column position
  let help_line = out.lines().find( | l | l.contains( "--help" ) ).expect( "help line" );
  let verbose_line = out.lines().find( | l | l.contains( "--verbose" ) ).expect( "verbose line" );

  let help_desc_pos = help_line.find( "Show" ).expect( "Show position" );
  let verbose_desc_pos = verbose_line.find( "Enable" ).expect( "Enable position" );
  assert_eq!( help_desc_pos, verbose_desc_pos, "descriptions aligned at same column" );
}

/// VT-2: sections separated by blank lines
// test_kind: spec_case(VT-2)
#[ test ]
fn variant_033_vt_02_blank_line_separators()
{
  let view = RowBuilder::new( vec![ "Term".into(), "Description".into() ] )
    .add_row( vec![ "USAGE".into(), "".into() ] )
    .add_row( vec![ "command [options]".into(), "".into() ] )
    .add_row( vec![ "OPTIONS".into(), "".into() ] )
    .add_row( vec![ "--verbose".into(), "Enable verbose".into() ] )
    .build_view();

  let out = Format::format(
    &TextFormatter::new( TextVariant::CliHelp ),
    &view,
  ).unwrap();

  assert!( out.contains( "\n\n" ), "blank line between sections: {out}" );
}

/// VT-3: multi-section option groups rendered
// test_kind: spec_case(VT-3)
#[ test ]
fn variant_033_vt_03_multi_section()
{
  let view = RowBuilder::new( vec![ "Term".into(), "Description".into() ] )
    .add_row( vec![ "GENERAL".into(), "".into() ] )
    .add_row( vec![ "--config".into(), "Config file".into() ] )
    .add_row( vec![ "OUTPUT".into(), "".into() ] )
    .add_row( vec![ "--json".into(), "JSON output".into() ] )
    .add_row( vec![ "DEBUG".into(), "".into() ] )
    .add_row( vec![ "--trace".into(), "Enable tracing".into() ] )
    .build_view();

  let out = Format::format(
    &TextFormatter::new( TextVariant::CliHelp ),
    &view,
  ).unwrap();

  assert!( out.contains( "GENERAL:" ), "general section: {out}" );
  assert!( out.contains( "OUTPUT:" ), "output section: {out}" );
  assert!( out.contains( "DEBUG:" ), "debug section: {out}" );
}

/// VT-4: empty table produces no CLI help output
// test_kind: spec_case(VT-4)
#[ test ]
fn variant_033_vt_04_empty_table()
{
  let view = RowBuilder::new( vec![ "Term".into(), "Description".into() ] )
    .build_view();

  let out = Format::format(
    &TextFormatter::new( TextVariant::CliHelp ),
    &view,
  ).unwrap();

  assert!( out.is_empty(), "empty CLI help output: {out}" );
}
