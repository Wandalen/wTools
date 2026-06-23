//! Variant 004: Table Markdown spec tests (VT-1..VT-4)

#![ cfg( feature = "enabled" ) ]

use data_fmt::{ RowBuilder, TableFormatter, TableConfig, Format };

/// VT-1: every line starts and ends with pipe
// test_kind: spec_case(VT-1)
#[ test ]
fn variant_004_vt_01_pipe_delimited_lines()
{
  let view = RowBuilder::new( vec![ "Name".into(), "Age".into() ] )
    .add_row( vec![ "Alice".into(), "30".into() ] )
    .build_view();

  let out = Format::format(
    &TableFormatter::with_config( TableConfig::markdown() ),
    &view,
  ).unwrap();

  for line in out.lines().filter( | l | !l.trim().is_empty() )
  {
    let trimmed = line.trim();
    assert!( trimmed.starts_with( '|' ), "line starts with pipe: {line}" );
    assert!( trimmed.ends_with( '|' ), "line ends with pipe: {line}" );
  }
}

/// VT-2: Markdown separator line between header and data
// test_kind: spec_case(VT-2)
#[ test ]
fn variant_004_vt_02_markdown_separator_line()
{
  let view = RowBuilder::new( vec![ "A".into(), "B".into() ] )
    .add_row( vec![ "1".into(), "2".into() ] )
    .build_view();

  let out = Format::format(
    &TableFormatter::with_config( TableConfig::markdown() ),
    &view,
  ).unwrap();

  let has_sep = out.lines().any( | l |
  {
    let t = l.trim();
    t.contains( "---" ) && t.contains( '|' )
  });
  assert!( has_sep, "markdown separator line with --- and | present: {out}" );
}

/// VT-3: output is valid GFM table syntax (no + characters)
// test_kind: spec_case(VT-3)
#[ test ]
fn variant_004_vt_03_valid_gfm_no_plus()
{
  let view = RowBuilder::new( vec![ "key".into(), "val".into() ] )
    .add_row( vec![ "a".into(), "b".into() ] )
    .add_row( vec![ "c".into(), "d".into() ] )
    .build_view();

  let out = Format::format(
    &TableFormatter::with_config( TableConfig::markdown() ),
    &view,
  ).unwrap();

  assert!( !out.contains( '+' ), "no ASCII grid + characters in GFM output: {out}" );

  // All non-empty lines have same pipe count (consistent column count)
  let pipe_counts : Vec< usize > = out.lines()
    .filter( | l | !l.trim().is_empty() )
    .map( | l | l.matches( '|' ).count() )
    .collect();

  let first = pipe_counts[ 0 ];
  for ( i, count ) in pipe_counts.iter().enumerate()
  {
    assert_eq!( *count, first, "line {i} pipe count {count} != first {first}" );
  }
}

/// VT-4: empty table produces header-only Markdown
// test_kind: spec_case(VT-4)
#[ test ]
fn variant_004_vt_04_empty_table_header_only()
{
  let view = RowBuilder::new( vec![ "Col".into() ] )
    .build_view();

  let out = Format::format(
    &TableFormatter::with_config( TableConfig::markdown() ),
    &view,
  ).unwrap();

  let non_empty : Vec< &str > = out.lines().filter( | l | !l.trim().is_empty() ).collect();
  // Header + separator, no data rows
  assert!( non_empty.len() <= 3, "header + separator only for empty table: {out}" );
}
