//! Variant 028: Text Bullets spec tests (VT-1..VT-4)

#![ cfg( feature = "format_text" ) ]

use data_fmt::{ RowBuilder, TextFormatter, TextVariant, Format };

fn bullets_formatter() -> TextFormatter
{
  TextFormatter::new( TextVariant::Bullets )
}

/// VT-1: each row rendered with bullet prefix
// test_kind: spec_case(VT-1)
#[ test ]
fn variant_028_vt_01_bullet_prefix_per_row()
{
  let view = RowBuilder::new( vec![ "Name".into(), "Age".into() ] )
    .add_row( vec![ "Alice".into(), "30".into() ] )
    .add_row( vec![ "Bob".into(), "25".into() ] )
    .build_view();

  let out = Format::format( &bullets_formatter(), &view ).unwrap();

  // Count bullet markers (• or - at line start)
  let bullet_lines = out.lines()
    .filter( | l | {
      let trimmed = l.trim_start();
      trimmed.starts_with( '•' ) || trimmed.starts_with( '-' ) || trimmed.starts_with( '*' )
    })
    .count();
  assert!( bullet_lines >= 2, "at least 2 bullet items: found {bullet_lines}" );
}

/// VT-2: no border or alignment characters
// test_kind: spec_case(VT-2)
#[ test ]
fn variant_028_vt_02_no_borders_or_alignment()
{
  let view = RowBuilder::new( vec![ "A".into(), "B".into() ] )
    .add_row( vec![ "x".into(), "y".into() ] )
    .build_view();

  let out = Format::format( &bullets_formatter(), &view ).unwrap();

  assert!( !out.contains( '|' ), "no pipe borders" );
  assert!( !out.contains( '+' ), "no plus borders" );
  assert!( !out.contains( "---" ), "no dash borders" );
}

/// VT-3: multi-column rows formatted as key-value
// test_kind: spec_case(VT-3)
#[ test ]
fn variant_028_vt_03_multi_column_key_value()
{
  let view = RowBuilder::new( vec![ "Name".into(), "City".into() ] )
    .add_row( vec![ "Alice".into(), "NYC".into() ] )
    .build_view();

  let out = Format::format( &bullets_formatter(), &view ).unwrap();

  // Multi-column values joined on single bullet line
  assert!( out.contains( "Alice" ), "Alice value present" );
  assert!( out.contains( "NYC" ), "NYC value present" );
  // Both values on same bullet line
  let bullet_line = out.lines().find( | l | l.trim_start().starts_with( '•' ) ).unwrap();
  assert!( bullet_line.contains( "Alice" ) && bullet_line.contains( "NYC" ), "both columns on same bullet line" );
}

/// VT-4: empty table produces no bullets
// test_kind: spec_case(VT-4)
#[ test ]
fn variant_028_vt_04_empty_table_no_bullets()
{
  let view = RowBuilder::new( vec![ "Col".into() ] )
    .build_view();

  let out = Format::format( &bullets_formatter(), &view ).unwrap();

  let has_bullets = out.lines().any( | l | {
    let t = l.trim_start();
    t.starts_with( '•' ) || t.starts_with( '-' ) || t.starts_with( '*' )
  });
  assert!( !has_bullets, "no bullet items for empty table" );
}
