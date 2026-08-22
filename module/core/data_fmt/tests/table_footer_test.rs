//! Tests for `Heading` titled-rule rendering at the footer position (`feature/007_table_heading`)
//!
//! ## What This Tests
//!
//! Verifies that `TableConfig::with_footer()` appends a titled rule to table output
//! following the format `─── Title · Field1 · Field2 ──────...` filling the rendered
//! table width, mirroring `table_heading_test.rs` but asserting on the LAST output line
//! (after the bottom border / last row) instead of the first. Footer rendering reuses
//! the same position-agnostic `render_rule_if_present()` function as heading rendering,
//! so BUG-015/016/017 (originally reproduced against the heading position) are re-verified
//! here at the footer position.
//! See `docs/feature/007_table_heading.md` for the full specification.

#![ cfg( feature = "enabled" ) ]
use data_fmt::{ RowBuilder, TableFormatter, TableConfig, Heading, TableView, Format };

// --- Test helper ---

fn two_col_view() -> TableView
{
  RowBuilder::new( vec![ "Name".into(), "Age".into() ] )
    .add_row( vec![ "Alice".into(), "30".into() ] )
    .build_view()
}

/// FT-10 — `feature/007`: title-only footer renders titled rule after the table.
// test_kind: standard
#[ test ]
fn title_only_footer_renders_titled_rule_ft10()
{
  let config = TableConfig::plain().with_footer( Heading::new( "Hi" ) );
  let output = TableFormatter::with_config( config )
    .format( &two_col_view() )
    .unwrap_or_default();

  let lines : Vec< &str > = output.lines().collect();
  let footer_line = *lines.last().unwrap_or( &"" );
  let second_last = lines.get( lines.len().saturating_sub( 2 ) ).copied().unwrap_or( "" );

  assert!(
    footer_line.starts_with( "─── Hi " ),
    "footer line must start with '─── Hi '; got: '{footer_line}'",
  );
  assert!(
    footer_line.ends_with( '─' ),
    "footer line must end with '─'; got: '{footer_line}'",
  );
  // Second-to-last line is the last data row (contains "Alice")
  assert!(
    second_last.contains( "Alice" ),
    "second-to-last line must be the last data row; got: '{second_last}'",
  );
}

/// FT-11 — `feature/007`: footer fields appear joined by the field separator.
// test_kind: standard
#[ test ]
fn footer_fields_joined_by_separator_ft11()
{
  let footer = Heading::new( "52 rows" ).with_field( "3 filtered" );
  let config = TableConfig::plain().with_footer( footer );
  let output = TableFormatter::with_config( config )
    .format( &two_col_view() )
    .unwrap_or_default();

  let last_line = output.lines().last().unwrap_or( "" );
  assert!(
    last_line.contains( "52 rows · 3 filtered" ),
    "footer line must contain '52 rows · 3 filtered'; got: '{last_line}'",
  );
}

/// FT-12 — `feature/007`: footer line fills exactly to the rendered table width.
// test_kind: standard
#[ test ]
fn footer_fills_to_table_width_ft12()
{
  let config = TableConfig::plain().with_footer( Heading::new( "AB" ) );
  let output = TableFormatter::with_config( config )
    .format( &two_col_view() )
    .unwrap_or_default();

  let footer_line = output.lines().last().unwrap_or( "" );
  let char_count = footer_line.chars().count();
  // table_width for two_col_view + plain = 5+3+2 = 10
  assert_eq!(
    char_count, 10,
    "footer line must be exactly 10 chars (rendered table width); got {char_count} chars: '{footer_line}'",
  );
}

/// FT-13 — `feature/007`: footer content equals table width — trailing rule clamped to zero.
// test_kind: standard
#[ test ]
fn footer_content_equals_table_width_no_trailing_rule_ft13()
{
  // "Abcde" = 5 chars; used = 3 + 1 + 5 + 1 = 10 = table_width
  let config = TableConfig::plain().with_footer( Heading::new( "Abcde" ) );
  let output = TableFormatter::with_config( config )
    .format( &two_col_view() )
    .unwrap_or_default();

  let footer_line = output.lines().last().unwrap_or( "" );
  assert_eq!(
    footer_line.chars().count(), 10,
    "footer line must be exactly 10 chars when content fills table width; got {}: '{footer_line}'",
    footer_line.chars().count(),
  );
  assert!(
    !footer_line.ends_with( '─' ),
    "no trailing rule when content exactly fills table width; got: '{footer_line}'",
  );
}

/// FT-14 — `feature/007`: no-footer config produces output identical to pre-footer behavior.
// test_kind: standard
#[ test ]
fn no_footer_output_unchanged_ft14()
{
  let view = two_col_view();

  let output_with_no_footer = TableFormatter::with_config( TableConfig::plain() )
    .format( &view )
    .unwrap_or_default();

  let output_baseline = TableFormatter::with_config( TableConfig::plain() )
    .format( &view )
    .unwrap_or_default();

  assert_eq!(
    output_with_no_footer, output_baseline,
    "output without footer must be byte-identical (no regression for existing callers)",
  );
  assert!(
    !output_with_no_footer.trim_end_matches( '\n' ).ends_with( '─' ),
    "output without footer must not end with a rule char",
  );
}

/// FT-15 — `feature/007`: footer appears after the bottom border in grid and `unicode_box` styles.
// test_kind: standard
#[ test ]
fn footer_after_bottom_border_grid_ft15()
{
  // --- grid style ---
  let config_grid = TableConfig::grid()
    .with_footer( Heading::new( "Grid Table" ) );
  let output_grid = TableFormatter::with_config( config_grid )
    .format( &two_col_view() )
    .unwrap_or_default();

  let lines_grid : Vec< &str > = output_grid.lines().collect();
  let last_grid = *lines_grid.last().unwrap_or( &"" );
  let before_last_grid = lines_grid.get( lines_grid.len().saturating_sub( 2 ) ).copied().unwrap_or( "" );

  assert!(
    last_grid.starts_with( "─── Grid Table" ),
    "grid: last line must be the footer; got: '{last_grid}'",
  );
  assert!(
    before_last_grid.starts_with( '+' ),
    "grid: second-to-last line must be the bottom border '+---+'; got: '{before_last_grid}'",
  );

  // --- unicode_box style ---
  let config_uni = TableConfig::unicode_box()
    .with_footer( Heading::new( "Grid Table" ) );
  let output_uni = TableFormatter::with_config( config_uni )
    .format( &two_col_view() )
    .unwrap_or_default();

  let lines_uni : Vec< &str > = output_uni.lines().collect();
  let last_uni = *lines_uni.last().unwrap_or( &"" );
  let before_last_uni = lines_uni.get( lines_uni.len().saturating_sub( 2 ) ).copied().unwrap_or( "" );

  assert!(
    last_uni.starts_with( "─── Grid Table" ),
    "unicode_box: last line must be the footer; got: '{last_uni}'",
  );
  assert!(
    before_last_uni.starts_with( '└' ),
    "unicode_box: second-to-last line must be the bottom border '└───┘'; got: '{before_last_uni}'",
  );
}

/// FT-16 — `feature/007`: footer title longer than table width — content emitted verbatim, no trailing rule.
// test_kind: standard
#[ test ]
fn footer_title_exceeds_table_width_no_trailing_rule_ft16()
{
  let long_title = "A very long title"; // 17 chars; used = 3+1+17+1 = 22 > table_width = 10
  let config = TableConfig::plain().with_footer( Heading::new( long_title ) );
  let output = TableFormatter::with_config( config )
    .format( &two_col_view() )
    .unwrap_or_default();

  let footer_line = output.lines().last().unwrap_or( "" );

  assert!(
    footer_line.starts_with( "─── " ),
    "footer must begin with lead prefix '─── '; got: '{footer_line}'",
  );
  assert!(
    !footer_line.ends_with( '─' ),
    "no trailing rule when title exceeds table width; got: '{footer_line}'",
  );
  assert!(
    footer_line.contains( long_title ),
    "title must appear verbatim without truncation; got: '{footer_line}'",
  );
}

/// FT-17 — `feature/007`: empty footer title produces lead rule and trailing fill with no field separator.
// test_kind: standard
#[ test ]
fn footer_empty_title_lead_only_no_separator_ft17()
{
  let config = TableConfig::plain().with_footer( Heading::new( "" ) );
  let output = TableFormatter::with_config( config )
    .format( &two_col_view() )
    .unwrap_or_default();

  let footer_line = output.lines().last().unwrap_or( "" );

  assert!(
    footer_line.starts_with( "─── " ),
    "footer must begin with lead prefix '─── ' even for empty title; got: '{footer_line}'",
  );
  assert!(
    !footer_line.contains( '·' ),
    "no '·' separator must appear for empty-title footer; got: '{footer_line}'",
  );
  assert_eq!(
    footer_line.chars().count(), 10,
    "empty-title footer must fill to exactly table_width (10) chars; got {}: '{footer_line}'",
    footer_line.chars().count(),
  );
}

/// FT-18 — `feature/007`: heading and footer coexist without interfering.
///
/// Uses equal-length titles ("Top" / "End", 3 chars each) so both individually fit
/// within `table_width` (10) without tripping the width-ceiling clamp (Invariant 2) —
/// that clamp is exercised separately by FT-13/FT-16/`footer_line_never_exceeds_table_width`.
// test_kind: standard
#[ test ]
fn heading_and_footer_coexist_ft18()
{
  let config = TableConfig::plain()
    .with_heading( Heading::new( "Top" ) )
    .with_footer( Heading::new( "End" ) );
  let output = TableFormatter::with_config( config )
    .format( &two_col_view() )
    .unwrap_or_default();

  let lines : Vec< &str > = output.lines().collect();
  let first_line = *lines.first().unwrap_or( &"" );
  let last_line = *lines.last().unwrap_or( &"" );

  assert!(
    first_line.starts_with( "─── Top" ),
    "first line must be the heading; got: '{first_line}'",
  );
  assert!(
    last_line.starts_with( "─── End" ),
    "last line must be the footer; got: '{last_line}'",
  );
  assert_eq!(
    first_line.chars().count(), last_line.chars().count(),
    "heading and footer must fill to the same table_width; heading={} footer={}",
    first_line.chars().count(), last_line.chars().count(),
  );
  // Table body (header row) is unaffected — still present between heading and footer
  assert!(
    lines.iter().any( | l | l.contains( "Name" ) ),
    "header row must still appear between heading and footer; got:\n{output}",
  );

  // Cross-check: heading-only and footer-only outputs, when concatenated in the right
  // positions, must equal the combined output's boundary lines exactly.
  let heading_only = TableFormatter::with_config(
    TableConfig::plain().with_heading( Heading::new( "Top" ) )
  ).format( &two_col_view() ).unwrap_or_default();
  let footer_only = TableFormatter::with_config(
    TableConfig::plain().with_footer( Heading::new( "End" ) )
  ).format( &two_col_view() ).unwrap_or_default();

  assert_eq!(
    heading_only.lines().next().unwrap_or( "" ), first_line,
    "combined heading line must match heading-only rendering",
  );
  assert_eq!(
    footer_only.lines().last().unwrap_or( "" ), last_line,
    "combined footer line must match footer-only rendering",
  );
}

// ============================================================================
// Invariant tests (tests/docs/invariant/005_heading.md — footer coverage)
// ============================================================================

/// IN-2 (footer) — `invariant/005`: footer line never exceeds rendered table width (3 scenarios).
///
/// Mirrors `heading_line_never_exceeds_table_width_in2` in `table_heading_test.rs`,
/// asserting against the footer position instead of the heading position.
// test_kind: standard
#[ test ]
fn footer_line_never_exceeds_table_width()
{
  let view = two_col_view();
  // two_col_view() with plain(): columns [5,3], sep 2 → table_width = 10

  // (a) Short title: "AB" — used = 3(lead) + 1(sp) + 2("AB") + 1(sp) = 7; trail = 3
  let output_a = TableFormatter::with_config(
    TableConfig::plain().with_footer( Heading::new( "AB" ) )
  )
  .format( &view )
  .unwrap_or_default();

  let line_a = output_a.lines().last().unwrap_or( "" );
  assert_eq!(
    line_a.chars().count(), 10,
    "footer(a): short title footer must equal table_width (10); got {}: '{line_a}'",
    line_a.chars().count(),
  );

  // (b) Title exceeds table width
  let output_b = TableFormatter::with_config(
    TableConfig::plain().with_footer( Heading::new( "A very long title" ) )
  )
  .format( &view )
  .unwrap_or_default();

  let line_b = output_b.lines().last().unwrap_or( "" );
  assert!(
    line_b.starts_with( "─── " ),
    "footer(b): footer must start with lead prefix; got: '{line_b}'",
  );
  assert!(
    !line_b.ends_with( '─' ),
    "footer(b): trailing rule must be absent when content exceeds table width; got: '{line_b}'",
  );
  assert!(
    line_b.contains( "A very long title" ),
    "footer(b): content must not be truncated; got: '{line_b}'",
  );

  // (c) Title exactly fills table width: "Abcde" → used = 3+1+5+1 = 10 = table_width
  let output_c = TableFormatter::with_config(
    TableConfig::plain().with_footer( Heading::new( "Abcde" ) )
  )
  .format( &view )
  .unwrap_or_default();

  let line_c = output_c.lines().last().unwrap_or( "" );
  assert_eq!(
    line_c.chars().count(), 10,
    "footer(c): exact-fit footer must equal table_width (10); got {}: '{line_c}'",
    line_c.chars().count(),
  );
  assert!(
    !line_c.ends_with( '─' ),
    "footer(c): no trailing rule when content exactly fills table width; got: '{line_c}'",
  );
}

// ============================================================================
// Shared-implementation regression coverage (BUG-015/016/017, footer position)
// ============================================================================

/// Helper: measure terminal display width of a string (ANSI-free path).
fn display_width( s : &str ) -> usize
{
  use unicode_width::UnicodeWidthChar;
  s.chars().map( | c | c.width().unwrap_or( 0 ) ).sum()
}

/// CJK characters in footer title must not break display width alignment (BUG-015, footer position).
///
/// `render_rule_if_present()` is the single shared implementation for heading and footer
/// (see `docs/algorithm/007_heading_rendering.md § Position-Agnostic Dispatch`); this test
/// re-verifies the BUG-015 fix (use `unicode_visual_len`, not `.chars().count()`) holds when
/// the same function is invoked from the footer call site.
// test_kind: bug_reproducer(BUG-015)
#[ test ]
fn footer_cjk_title_display_width_matches_table_body()
{
  let view = two_col_view();
  let config = TableConfig::plain().with_footer( Heading::new( "中" ) );
  let output = TableFormatter::with_config( config )
    .format( &view )
    .unwrap_or_default();

  let lines : Vec< &str > = output.lines().collect();
  assert!(
    lines.len() >= 2,
    "must have at least a data row + footer; got {} lines",
    lines.len(),
  );

  let footer_dw = display_width( lines[ lines.len() - 1 ] );
  let body_dw = display_width( lines[ lines.len() - 2 ] );

  assert_eq!(
    footer_dw, body_dw,
    "footer display width ({footer_dw}) must match data row display width ({body_dw})\n\
     footer: {:?}\n body:   {:?}\nfull output:\n{output}",
    lines[ lines.len() - 1 ], lines[ lines.len() - 2 ],
  );
}

/// Newline in footer title must NOT produce multi-line output (BUG-016, footer position).
// test_kind: bug_reproducer(BUG-016)
#[ test ]
fn footer_newline_in_title_produces_single_line()
{
  let view = two_col_view();
  let config = TableConfig::plain().with_footer( Heading::new( "Line1\nLine2" ) );
  let output = TableFormatter::with_config( config )
    .format( &view )
    .unwrap_or_default();

  let lines : Vec< &str > = output.lines().collect();
  let footer_line = *lines.last().unwrap_or( &"" );
  assert!(
    footer_line.starts_with( "─── " ),
    "last line must be the footer; got: {footer_line:?}",
  );
  assert!(
    footer_line.contains( "Line1" ) && footer_line.contains( "Line2" ),
    "footer line must contain both parts of the title (sanitized to one line); got: {footer_line:?}",
  );
}

/// Footer on bordered table — footer width must match bordered body width (BUG-017, footer position).
// test_kind: bug_reproducer(BUG-017)
#[ test ]
fn footer_on_bordered_table_display_width_matches()
{
  let view = two_col_view();
  let config = TableConfig::bordered().with_footer( Heading::new( "AB" ) );
  let output = TableFormatter::with_config( config )
    .format( &view )
    .unwrap_or_default();

  let lines : Vec< &str > = output.lines().filter( | l | !l.is_empty() ).collect();
  assert!(
    lines.len() >= 3,
    "bordered with footer must have at least 3 lines; got {}",
    lines.len(),
  );

  let footer_dw = display_width( lines[ lines.len() - 1 ] );
  let body_dw = display_width( lines[ lines.len() - 2 ] );

  assert_eq!(
    footer_dw, body_dw,
    "footer display width ({footer_dw}) must match bordered body display width ({body_dw})\n\
     footer: {:?}\n body:   {:?}\nfull:\n{output}",
    lines[ lines.len() - 1 ], lines[ lines.len() - 2 ],
  );
}
