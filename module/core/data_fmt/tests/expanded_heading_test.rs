//! Tests for `ExpandedConfig::with_heading()` / `with_footer()` — the same `Heading`
//! titled-rule pattern as `table_heading_test.rs` / `table_footer_test.rs` /
//! `tree_heading_test.rs`, applied to `ExpandedFormatter`. Like Tree, Expanded output
//! has no fixed column width, so the rule fills to the widest rendered line's display
//! width instead of a precomputed `table_width`. FT-25..FT-30 continue the ID sequence
//! from `tests/docs/feature/007_table_heading.md` (FT-19..FT-24 were Tree's).

#![ cfg( feature = "enabled" ) ]

use data_fmt::{ RowBuilder, ExpandedFormatter, ExpandedConfig, Heading, Format };

fn simple_view() -> data_fmt::TableView
{
  RowBuilder::new( vec![ "Name".into(), "Age".into() ] )
    .add_row( vec![ "Alice".into(), "30".into() ] )
    .add_row( vec![ "Bob".into(), "25".into() ] )
    .build_view()
}

fn display_width( s : &str ) -> usize
{
  s.chars().count()
}

/// FT-25 — `feature/007`: title-only heading renders titled rule before expanded output.
// test_kind: standard
#[ test ]
fn title_only_heading_renders_before_expanded_output_ft25()
{
  let config = ExpandedConfig::new().with_heading( Heading::new( "Users" ) );
  let output = ExpandedFormatter::with_config( config ).format( &simple_view() ).unwrap_or_default();

  let lines : Vec< &str > = output.lines().collect();
  let first_line = *lines.first().unwrap_or( &"" );
  assert!(
    first_line.starts_with( "─── Users" ),
    "first line must be the heading; got: '{first_line}'",
  );
  assert!( output.contains( "Alice" ), "expanded body must still render; got:\n{output}" );
}

/// FT-26 — `feature/007`: title-only footer renders titled rule after expanded output.
// test_kind: standard
#[ test ]
fn title_only_footer_renders_after_expanded_output_ft26()
{
  let config = ExpandedConfig::new().with_footer( Heading::new( "2 records" ) );
  let output = ExpandedFormatter::with_config( config ).format( &simple_view() ).unwrap_or_default();

  let lines : Vec< &str > = output.lines().collect();
  let last_line = *lines.last().unwrap_or( &"" );
  assert!(
    last_line.starts_with( "─── 2 records" ),
    "last line must be the footer; got: '{last_line}'",
  );
  assert!( output.contains( "Bob" ), "expanded body must still render; got:\n{output}" );
}

/// FT-27 — `feature/007`: heading/footer fill to the widest rendered expanded line, not
/// a fixed table-style width — expanded lines are ragged (key/value width varies per
/// record), so the rule must track the actual body content rather than any
/// config-declared width.
// test_kind: standard
#[ test ]
fn heading_fills_to_widest_expanded_line_ft27()
{
  let config = ExpandedConfig::new().with_heading( Heading::new( "H" ) );
  let output = ExpandedFormatter::with_config( config ).format( &simple_view() ).unwrap_or_default();

  let lines : Vec< &str > = output.lines().collect();
  let body_lines = &lines[ 1.. ];
  let expected_width = body_lines.iter().map( | l | display_width( l ) ).max().unwrap_or( 0 );
  let heading_width = display_width( lines[ 0 ] );

  assert_eq!(
    heading_width, expected_width,
    "heading must fill to the widest body line ({expected_width}), got {heading_width}",
  );
}

/// FT-28 — `feature/007`: no heading/footer configured produces byte-identical output
/// to an `ExpandedFormatter` baseline (passthrough — Invariant 1 equivalent for Expanded).
// test_kind: standard
#[ test ]
fn no_heading_no_footer_output_unchanged_ft28()
{
  let plain = ExpandedFormatter::with_config( ExpandedConfig::new() ).format( &simple_view() ).unwrap_or_default();
  let baseline = ExpandedFormatter::new().format( &simple_view() ).unwrap_or_default();

  assert_eq!( plain, baseline, "ExpandedConfig::new() with no heading/footer must match ExpandedFormatter::new() baseline" );
  assert!( !plain.starts_with( '─' ), "no heading configured — first line must not be a rule line" );
}

/// FT-29 — `feature/007`: heading and footer coexist on expanded output without interfering.
// test_kind: standard
#[ test ]
fn heading_and_footer_coexist_on_expanded_output_ft29()
{
  let config = ExpandedConfig::new()
    .with_heading( Heading::new( "Top" ) )
    .with_footer( Heading::new( "End" ) );
  let output = ExpandedFormatter::with_config( config ).format( &simple_view() ).unwrap_or_default();

  let lines : Vec< &str > = output.lines().collect();
  let first_line = *lines.first().unwrap_or( &"" );
  let last_line = *lines.last().unwrap_or( &"" );

  assert!( first_line.starts_with( "─── Top" ), "first line must be heading; got: '{first_line}'" );
  assert!( last_line.starts_with( "─── End" ), "last line must be footer; got: '{last_line}'" );
  assert_eq!(
    display_width( first_line ), display_width( last_line ),
    "heading and footer must fill to the same width",
  );
  assert!( output.contains( "Alice" ) && output.contains( "Bob" ), "expanded body unaffected; got:\n{output}" );
}

/// FT-30 — `feature/007`: heading applies even on the zero-columns early-return branch
/// (`format_view()` returns immediately when `headers.is_empty()`) — proves the wrap
/// applies uniformly across `ExpandedFormatter`'s internal branches, not only the
/// populated-headers path. Mirrors Tree's FT-24 (leaf-only-root branch coverage).
// test_kind: standard
#[ test ]
fn heading_applies_to_empty_headers_view_ft30()
{
  let empty_view = RowBuilder::new( vec![] ).build_view();
  let config = ExpandedConfig::new().with_heading( Heading::new( "Empty" ) );
  let output = ExpandedFormatter::with_config( config ).format( &empty_view ).unwrap_or_default();

  assert_eq!( output.lines().count(), 1, "expected exactly one line (heading only, no body); got:\n{output}" );
  assert!( output.starts_with( "─── Empty" ), "first line must be the heading; got: '{output}'" );
}
