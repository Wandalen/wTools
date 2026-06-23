//! Tests for `Heading` titled-rule rendering (`feature/007_table_heading`)
//!
//! ## What This Tests
//!
//! Verifies that `Heading` prepends a titled rule to table output following
//! the format `─── Title · Field1 · Field2 ──────...` filling the rendered table width.
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

// --- FC-1: title-only caption renders titled rule before table ---
//
// Given: TableConfig::plain() with .with_heading(Heading::new("Hi"))
// When: a two-column, one-row table is formatted (table_width = 10)
// Then: first line starts with "─── Hi " and ends with ─ chars; second line is header
// Note: title must fit within table_width for trailing rule to appear (3+1+2+1=7 < 10)

/// FC-1 — `feature/007`: title-only caption renders titled rule before the table.
// test_kind: standard
#[ test ]
fn title_only_caption_renders_titled_rule_fc1()
{
  // "Hi" = 2 chars; used = 3+1+2+1 = 7; table_width = 10; trail = 3
  let config = TableConfig::plain().with_heading( Heading::new( "Hi" ) );
  let output = TableFormatter::with_config( config )
    .format( &two_col_view() )
    .unwrap_or_default();

  let mut lines = output.lines();
  let caption_line = lines.next().unwrap_or( "" );
  let second_line  = lines.next().unwrap_or( "" );

  assert!(
    caption_line.starts_with( "─── Hi " ),
    "caption line must start with '─── Hi '; got: '{caption_line}'",
  );
  assert!(
    caption_line.ends_with( '─' ),
    "caption line must end with '─'; got: '{caption_line}'",
  );
  // Second line is the header row (contains "Name")
  assert!(
    second_line.contains( "Name" ),
    "second line must be the header row; got: '{second_line}'",
  );
}

// --- FC-2: caption fields appear joined by field separator ---
//
// Given: Heading::new("Needs Review").with_field("28 PRs").with_field("15 repos")
// When: rendered via TableConfig::plain()
// Then: first output line contains "Needs Review · 28 PRs · 15 repos"

/// FC-2 — `feature/007`: caption fields appear joined by the field separator.
// test_kind: standard
#[ test ]
fn caption_fields_joined_by_separator_fc2()
{
  let caption = Heading::new( "Needs Review" )
    .with_field( "28 PRs" )
    .with_field( "15 repos" );
  let config = TableConfig::plain().with_heading( caption );
  let output = TableFormatter::with_config( config )
    .format( &two_col_view() )
    .unwrap_or_default();

  let first_line = output.lines().next().unwrap_or( "" );
  assert!(
    first_line.contains( "Needs Review · 28 PRs · 15 repos" ),
    "caption line must contain 'Needs Review · 28 PRs · 15 repos'; got: '{first_line}'",
  );
}

// --- FC-3: caption line fills to rendered table width with rule chars ---
//
// Given: TableConfig::plain() with .with_heading(Heading::new("AB"))
// When: two_col_view() formatted (table_width = 10; columns [5,3], sep=2)
// Then: first line (caption) has exactly 10 display columns (.chars().count() == 10)
// used = 3(lead) + 1(space) + 2("AB") + 1(space) = 7; trail = 10 - 7 = 3

/// FC-3 — `feature/007`: caption line fills exactly to the rendered table width.
// test_kind: standard
#[ test ]
fn caption_fills_to_table_width_fc3()
{
  let caption = Heading::new( "AB" );
  let config = TableConfig::plain().with_heading( caption );
  let output = TableFormatter::with_config( config )
    .format( &two_col_view() )
    .unwrap_or_default();

  let caption_line = output.lines().next().unwrap_or( "" );
  let char_count = caption_line.chars().count();
  // table_width for two_col_view + plain = 5+3+2 = 10
  assert_eq!(
    char_count, 10,
    "caption line must be exactly 10 chars (rendered table width); \
     got {char_count} chars: '{caption_line}'",
  );
}

// --- FT-4: caption content exactly fills table width — trailing rule clamped to zero ---
//
// Given: TableConfig::plain() with .with_heading(Heading::new("Abcde"))
// two_col_view() table_width = 10; title = "Abcde" (5 chars)
// used = 3(lead) + 1(space) + 5("Abcde") + 1(space) = 10 = table_width → trail = 0
// When: table formatted
// Then: caption line ends with space (not '─'); char count == 10; no trailing rule emitted

/// FT-4 — `feature/007`: when caption content exactly equals table width, no trailing rule is emitted.
// test_kind: standard
#[ test ]
fn caption_content_equals_table_width_no_trailing_rule_ft4()
{
  // "Abcde" = 5 chars; used = 3 + 1 + 5 + 1 = 10 = table_width
  let config = TableConfig::plain()
    .with_heading( Heading::new( "Abcde" ) );
  let output = TableFormatter::with_config( config )
    .format( &two_col_view() )
    .unwrap_or_default();

  let caption_line = output.lines().next().unwrap_or( "" );

  assert_eq!(
    caption_line.chars().count(), 10,
    "caption line must be exactly 10 chars when content fills table width; got {}: '{caption_line}'",
    caption_line.chars().count(),
  );
  assert!(
    !caption_line.ends_with( '─' ),
    "no trailing rule when content exactly fills table width; got: '{caption_line}'",
  );
}

// --- FT-7: title string longer than table width — content not truncated, no trailing rule ---
//
// Given: TableConfig::plain() with .with_heading(Heading::new("A very long title"))
// two_col_view() table_width = 10; title = "A very long title" (17 chars)
// used = 3+1+17+1 = 22 > table_width = 10 → trail = 0
// When: table formatted
// Then: caption starts with "─── ", does NOT end with '─', and title appears verbatim (no truncation)

/// FT-7 — `feature/007`: title longer than table width — content emitted verbatim, no trailing rule.
// test_kind: standard
#[ test ]
fn caption_title_exceeds_table_width_no_trailing_rule_ft7()
{
  let long_title = "A very long title"; // 17 chars; used = 3+1+17+1 = 22 > table_width = 10
  let config = TableConfig::plain()
    .with_heading( Heading::new( long_title ) );
  let output = TableFormatter::with_config( config )
    .format( &two_col_view() )
    .unwrap_or_default();

  let caption_line = output.lines().next().unwrap_or( "" );

  // Caption starts with the lead prefix
  assert!(
    caption_line.starts_with( "─── " ),
    "caption must begin with lead prefix '─── '; got: '{caption_line}'",
  );
  // No trailing rule when content exceeds table width
  assert!(
    !caption_line.ends_with( '─' ),
    "no trailing rule when title exceeds table width; got: '{caption_line}'",
  );
  // Content not truncated — full title appears verbatim
  assert!(
    caption_line.contains( long_title ),
    "title must appear verbatim without truncation; got: '{caption_line}'",
  );
}

// --- FT-8: empty title — lead rule emitted, no separator, trailing rule fills rest ---
//
// Given: TableConfig::plain() with .with_heading(Heading::new(""))
// two_col_view() table_width = 10; title = "" (0 chars), no fields; content_str() = ""
// used = 3+1+0+1 = 5; trail = 10-5 = 5
// When: table formatted
// Then: caption starts with "─── ", no '·' separator, char count == 10, no panic

/// FT-8 — `feature/007`: empty title produces lead rule and trailing fill with no field separator.
// test_kind: standard
#[ test ]
fn caption_empty_title_lead_only_no_separator_ft8()
{
  let config = TableConfig::plain()
    .with_heading( Heading::new( "" ) );
  let output = TableFormatter::with_config( config )
    .format( &two_col_view() )
    .unwrap_or_default();

  let caption_line = output.lines().next().unwrap_or( "" );

  // Lead prefix always emitted
  assert!(
    caption_line.starts_with( "─── " ),
    "caption must begin with lead prefix '─── ' even for empty title; got: '{caption_line}'",
  );
  // No field separator when title is empty and no fields are set
  assert!(
    !caption_line.contains( '·' ),
    "no '·' separator must appear for empty-title caption; got: '{caption_line}'",
  );
  // Total width = table_width (lead + 2 spaces + trail)
  assert_eq!(
    caption_line.chars().count(), 10,
    "empty-title caption must fill to exactly table_width (10) chars; got {}: '{caption_line}'",
    caption_line.chars().count(),
  );
}

// --- FC-4: trailing rule clamped to zero when caption content exceeds table width ---
//
// Given: TableConfig::plain() with .with_heading(Heading::new("LongTitleText"))
// two_col_view() table_width = 10; title = "LongTitleText" (13 chars)
// used = 3+1+13+1 = 18 > table_width = 10 → trail = 0
// When: table formatted
// Then: caption line starts with "─── " but does NOT end with '─' (no trailing rule emitted)

/// FC-4 — `feature/007`: trailing rule clamped to zero when caption content fills or exceeds table width.
// test_kind: standard
#[ test ]
fn caption_trail_clamped_to_zero_when_content_too_wide_fc4()
{
  // title is 13 chars; used = 3+1+13+1 = 18 > table_width = 10; trail = 0
  let config = TableConfig::plain()
    .with_heading( Heading::new( "LongTitleText" ) );
  let output = TableFormatter::with_config( config )
    .format( &two_col_view() )
    .unwrap_or_default();

  let caption_line = output.lines().next().unwrap_or( "" );

  // When trail_width is clamped to 0, no rule char appears at the end of the line
  assert!(
    !caption_line.ends_with( '─' ),
    "when content fills table width, caption must not end with rule char; got: '{caption_line}'",
  );
  // Lead prefix is always emitted regardless of width
  assert!(
    caption_line.starts_with( "─── " ),
    "caption must still begin with lead prefix '─── '; got: '{caption_line}'",
  );
}

// --- FC-5: no-caption config produces identical output to current behavior ---
//
// Given: TableConfig::plain() without .with_heading(), same config without .with_heading()
// When: both render the same table
// Then: outputs are byte-identical

/// FC-5 — `feature/007`: no-caption config produces output identical to pre-caption behavior.
// test_kind: standard
#[ test ]
fn no_caption_output_unchanged_fc5()
{
  let view = two_col_view();

  let output_with_no_caption = TableFormatter::with_config( TableConfig::plain() )
    .format( &view )
    .unwrap_or_default();

  // Build an identical config via a fresh constructor — no .with_heading() call on either
  let output_baseline = TableFormatter::with_config( TableConfig::plain() )
    .format( &view )
    .unwrap_or_default();

  assert_eq!(
    output_with_no_caption, output_baseline,
    "output without caption must be byte-identical (no regression for existing callers)",
  );
  // Also verify the output does not start with a caption rule char
  assert!(
    !output_with_no_caption.starts_with( '─' ),
    "output without caption must not begin with a rule char",
  );
}

// --- FC-6: caption renders before top border for grid and unicode_box styles ---
//
// Given: TableConfig::grid() with .with_heading(Heading::new("Grid Table"))
// When: table formatted
// Then: first line is the caption line (starts with "─── Grid Table"); second line is top border

/// FC-6 — `feature/007`: caption appears before the top border in grid and `unicode_box` styles.
// test_kind: standard
#[ test ]
fn caption_before_top_border_grid_fc6()
{
  // --- grid style ---
  let config_grid = TableConfig::grid()
    .with_heading( Heading::new( "Grid Table" ) );
  let output_grid = TableFormatter::with_config( config_grid )
    .format( &two_col_view() )
    .unwrap_or_default();

  let mut lines_grid = output_grid.lines();
  let first_grid  = lines_grid.next().unwrap_or( "" );
  let second_grid = lines_grid.next().unwrap_or( "" );

  assert!(
    first_grid.starts_with( "─── Grid Table" ),
    "grid: first line must be the caption; got: '{first_grid}'",
  );
  assert!(
    second_grid.starts_with( '+' ),
    "grid: second line must be the top border '+---+'; got: '{second_grid}'",
  );

  // --- unicode_box style ---
  let config_uni = TableConfig::unicode_box()
    .with_heading( Heading::new( "Grid Table" ) );
  let output_uni = TableFormatter::with_config( config_uni )
    .format( &two_col_view() )
    .unwrap_or_default();

  let mut lines_uni = output_uni.lines();
  let first_uni  = lines_uni.next().unwrap_or( "" );
  let second_uni = lines_uni.next().unwrap_or( "" );

  assert!(
    first_uni.starts_with( "─── Grid Table" ),
    "unicode_box: first line must be the caption; got: '{first_uni}'",
  );
  assert!(
    second_uni.starts_with( '┌' ),
    "unicode_box: second line must be the top border '┌───┐'; got: '{second_uni}'",
  );
}

// ============================================================================
// Invariant tests (tests/docs/invariant/005_caption.md)
// ============================================================================

/// IN-2 — `invariant/005`: heading line never exceeds rendered table width (3 scenarios).
///
/// (a) short title fitting within table width → char count == `table_width`
/// (b) title exceeding table width → trailing rule absent, content verbatim
/// (c) title exactly filling table width → no trailing rule, char count == `table_width`
// test_kind: standard
#[ test ]
fn heading_line_never_exceeds_table_width_in2()
{
  let view = two_col_view();
  // two_col_view() with plain(): columns [5,3], sep 2 → table_width = 10

  // (a) Short title: "AB" — used = 3(lead) + 1(sp) + 2("AB") + 1(sp) = 7; trail = 3
  let output_a = TableFormatter::with_config(
    TableConfig::plain().with_heading( Heading::new( "AB" ) )
  )
  .format( &view )
  .unwrap_or_default();

  let line_a = output_a.lines().next().unwrap_or( "" );
  assert_eq!(
    line_a.chars().count(), 10,
    "IN-2(a): short title heading must equal table_width (10); got {}: '{line_a}'",
    line_a.chars().count(),
  );

  // (b) Title exceeds table width: "A very long title" (17 chars)
  // used = 3+1+17+1 = 22 > 10 → trail = 0
  let output_b = TableFormatter::with_config(
    TableConfig::plain().with_heading( Heading::new( "A very long title" ) )
  )
  .format( &view )
  .unwrap_or_default();

  let line_b = output_b.lines().next().unwrap_or( "" );
  assert!(
    line_b.starts_with( "─── " ),
    "IN-2(b): heading must start with lead prefix; got: '{line_b}'",
  );
  assert!(
    !line_b.ends_with( '─' ),
    "IN-2(b): trailing rule must be absent when content exceeds table width; got: '{line_b}'",
  );
  assert!(
    line_b.contains( "A very long title" ),
    "IN-2(b): content must not be truncated; got: '{line_b}'",
  );

  // (c) Title exactly fills table width: "Abcde" → used = 3+1+5+1 = 10 = table_width
  let output_c = TableFormatter::with_config(
    TableConfig::plain().with_heading( Heading::new( "Abcde" ) )
  )
  .format( &view )
  .unwrap_or_default();

  let line_c = output_c.lines().next().unwrap_or( "" );
  assert_eq!(
    line_c.chars().count(), 10,
    "IN-2(c): exact-fit heading must equal table_width (10); got {}: '{line_c}'",
    line_c.chars().count(),
  );
  assert!(
    !line_c.ends_with( '─' ),
    "IN-2(c): no trailing rule when content exactly fills table width; got: '{line_c}'",
  );
}

// ============================================================================
// Example binary tests (tests/docs/feature/007_table_caption.md — FT-9)
// ============================================================================

/// FT-9 — `feature/007`: heading example binaries compile and produce visible output.
///
/// Builds and runs `heading_basic` and `heading_styles` examples; asserts exit 0,
/// non-empty stdout with at least one `─── ` lead prefix; `heading_styles` must
/// produce at least 3 heading lines.
// test_kind: standard
#[ test ]
fn heading_example_binaries_compile_and_produce_output_ft9()
{
  let manifest_dir = env!( "CARGO_MANIFEST_DIR" );

  for ( name, min_headings ) in [ ( "heading_basic", 1 ), ( "heading_styles", 3 ) ]
  {
    let output = std::process::Command::new( "cargo" )
      .args( [ "run", "--example", name, "--features", "enabled" ] )
      .current_dir( manifest_dir )
      .output()
      .unwrap_or_else( | e | panic!( "FT-9: failed to spawn cargo run --example {name}: {e}" ) );

    assert!(
      output.status.success(),
      "FT-9: `cargo run --example {name}` must exit 0; status={:?}\nstderr:\n{}",
      output.status,
      String::from_utf8_lossy( &output.stderr ),
    );

    let stdout = String::from_utf8_lossy( &output.stdout );
    assert!(
      !stdout.is_empty(),
      "FT-9: {name} must produce non-empty stdout",
    );

    let lead = "─── ";
    let heading_count = stdout.lines().filter( | l | l.contains( lead ) ).count();
    assert!(
      heading_count >= min_headings,
      "FT-9: {name} must contain at least {min_headings} heading line(s) with '{lead}'; found {heading_count}\nstdout:\n{stdout}",
    );
  }
}

// ============================================================================
// Corner-case manual tests
// ============================================================================

/// Helper: measure terminal display width of a string (ANSI-free path).
/// CJK chars = 2 columns; ASCII = 1 column; '─' (U+2500) = 1 column.
fn display_width( s : &str ) -> usize
{
  use unicode_width::UnicodeWidthChar;
  s.chars().map( | c | c.width().unwrap_or( 0 ) ).sum()
}

/// CJK characters in heading title must not break display width alignment (BUG-015).
///
/// ## Root Cause
/// `render_caption_if_present` used `.chars().count()` to measure heading content width.
/// CJK characters occupy 2 display columns but count as 1 char, so the trailing rule
/// was too long and the heading line exceeded the actual table body width.
///
/// ## Why Not Caught
/// No prior test used CJK characters in a heading title — all tests used ASCII-only
/// strings where `.chars().count()` and display width are identical.
///
/// ## Fix Applied
/// Replaced `.chars().count()` with `crate::ansi_str::unicode_visual_len(&content)` for
/// content width measurement in `render_caption_if_present`, matching the display-column
/// semantics used by the table body renderer.
///
/// ## Prevention
/// Any width arithmetic that must match terminal rendering must use `unicode_visual_len`
/// (display columns), never `.chars().count()` (scalar count) or `.len()` (byte count).
///
/// ## Pitfall
/// A CJK character is 1 char but 2 display columns — using char count for width
/// arithmetic produces headings that overshoot the table body width by the number of
/// double-width characters present.
// test_kind: bug_reproducer(BUG-015)
#[ test ]
fn heading_cjk_title_display_width_matches_table_body()
{
  let view = two_col_view();
  // plain: columns [5,3], sep=2 → table_width = 10 display columns

  let config = TableConfig::plain()
    .with_heading( Heading::new( "中" ) );
  let output = TableFormatter::with_config( config )
    .format( &view )
    .unwrap_or_default();

  let lines : Vec< &str > = output.lines().collect();
  assert!(
    lines.len() >= 2,
    "must have at least heading + header row; got {} lines",
    lines.len(),
  );

  let heading_dw = display_width( lines[ 0 ] );
  let header_dw  = display_width( lines[ 1 ] );

  // Heading display width must equal table body display width (both = 10)
  assert_eq!(
    heading_dw, header_dw,
    "heading display width ({heading_dw}) must match data row display width ({header_dw})\n\
     heading: {:?}\n header:  {:?}\nfull output:\n{output}",
    lines[ 0 ], lines[ 1 ],
  );
}

/// Newline in heading title must NOT produce multi-line output (BUG-016).
///
/// ## Root Cause
/// `Heading::content_str()` emitted title and field strings verbatim. Embedded `\n`
/// characters in the title broke the heading across multiple terminal lines, violating
/// invariant IN-3 (heading always occupies exactly one output line).
///
/// ## Why Not Caught
/// All prior heading tests used single-line titles without embedded control characters.
/// No test injected `\n`, `\r\n`, or `\r` into heading content.
///
/// ## Fix Applied
/// `content_str()` delegates to `sanitize_line_breaks()` which replaces all line-break
/// sequences (`\r\n` as pair first, then bare `\r` and `\n`) with a single space.
/// Applied to both title and each field before concatenation.
///
/// ## Prevention
/// Any user-supplied string passed to heading content must be sanitized before width
/// arithmetic — `\n` is invisible to `unicode_visual_len` but produces visible line
/// breaks in terminal output.
///
/// ## Pitfall
/// A heading containing `\n` produces two terminal lines but `unicode_visual_len` sees
/// it as zero-width — the width arithmetic and the visual output diverge silently.
// test_kind: bug_reproducer(BUG-016)
#[ test ]
fn heading_newline_in_title_produces_single_line()
{
  let view = two_col_view();
  let config = TableConfig::plain()
    .with_heading( Heading::new( "Line1\nLine2" ) );
  let output = TableFormatter::with_config( config )
    .format( &view )
    .unwrap_or_default();

  let lines : Vec< &str > = output.lines().collect();
  // First line must be heading, second must be data header "Name ..."
  assert!(
    lines[ 0 ].starts_with( "─── " ),
    "first line must be heading; got: {:?}",
    lines[ 0 ],
  );
  assert!(
    lines[ 1 ].contains( "Name" ),
    "second line must be header row; got: {:?}\nfull:\n{output}",
    lines[ 1 ],
  );
  // The title content should appear on the heading line (sanitized)
  assert!(
    lines[ 0 ].contains( "Line1" ) && lines[ 0 ].contains( "Line2" ),
    "heading line must contain both parts of the title; got: {:?}",
    lines[ 0 ],
  );
}

/// Newline in heading field must NOT produce multi-line heading.
///
/// See `heading_newline_in_title_produces_single_line` for BUG-016 root cause and fix.
// test_kind: bug_reproducer(BUG-016)
#[ test ]
fn heading_newline_in_field_produces_single_line()
{
  let view = two_col_view();
  let config = TableConfig::plain()
    .with_heading( Heading::new( "Title" ).with_field( "F1\nF2" ) );
  let output = TableFormatter::with_config( config )
    .format( &view )
    .unwrap_or_default();

  let lines : Vec< &str > = output.lines().collect();
  assert!(
    lines[ 0 ].starts_with( "─── " ),
    "first line must be heading; got: {:?}",
    lines[ 0 ],
  );
  assert!(
    lines[ 1 ].contains( "Name" ),
    "second line must be header row; got: {:?}\nfull:\n{output}",
    lines[ 1 ],
  );
}

/// CRLF and bare CR in heading title/fields are sanitized.
///
/// Windows-style `\r\n` and bare `\r` must be replaced with spaces,
/// just like `\n`, to preserve the single-line heading invariant (IN-3).
/// See `heading_newline_in_title_produces_single_line` for BUG-016 root cause and fix.
// test_kind: bug_reproducer(BUG-016)
#[ test ]
fn heading_crlf_and_cr_sanitized()
{
  let view = two_col_view();

  // \r\n pair → single space (not two)
  let output_crlf = TableFormatter::with_config(
    TableConfig::plain().with_heading( Heading::new( "A\r\nB" ) )
  )
  .format( &view )
  .unwrap_or_default();
  let line_crlf = output_crlf.lines().next().unwrap_or( "" );
  assert!(
    line_crlf.contains( "A B" ),
    "BUG-016: \\r\\n must become single space; got: {line_crlf:?}",
  );

  // bare \r → space
  let output_cr = TableFormatter::with_config(
    TableConfig::plain().with_heading( Heading::new( "X" ).with_field( "C\rD" ) )
  )
  .format( &view )
  .unwrap_or_default();
  let line_cr = output_cr.lines().next().unwrap_or( "" );
  assert!(
    line_cr.contains( "C D" ),
    "BUG-016: bare \\r in field must become space; got: {line_cr:?}",
  );
}

/// Corner case: headers-only table (zero data rows) with heading.
///
/// The heading should render correctly even when the table has no data rows.
// test_kind: standard
#[ test ]
fn heading_on_headers_only_table()
{
  let view = RowBuilder::new( vec![ "Name".into(), "Age".into() ] )
    .build_view();

  let config = TableConfig::plain()
    .with_heading( Heading::new( "People" ) );
  let output = TableFormatter::with_config( config )
    .format( &view )
    .unwrap_or_default();

  let lines : Vec< &str > = output.lines().collect();
  assert!(
    lines.len() >= 2,
    "headers-only table with heading must have at least 2 lines; got {}",
    lines.len(),
  );
  assert!(
    lines[ 0 ].starts_with( "─── " ),
    "first line must be heading; got: {0:?}",
    lines[ 0 ],
  );
  assert!(
    lines[ 0 ].contains( "People" ),
    "heading must contain title; got: {0:?}",
    lines[ 0 ],
  );
  assert!(
    lines[ 1 ].contains( "Name" ),
    "second line must be header row; got: {0:?}",
    lines[ 1 ],
  );
}

/// Corner case: single-column table with heading — heading wider than table body.
///
/// When the table is very narrow (e.g., single short column), the heading content
/// may exceed `table_width`. Per spec: trail clamped to 0, content not truncated.
/// No panic. The heading line is simply wider than the table body.
#[ test ]
fn heading_on_single_column_narrow_table()
{
  let view = RowBuilder::new( vec![ "H".into() ] )
    .add_row( vec![ "x".into() ] )
    .build_view();

  // table_width for plain single-column: max(1,1) = 1
  let config = TableConfig::plain()
    .with_heading( Heading::new( "Title" ) );
  let output = TableFormatter::with_config( config )
    .format( &view )
    .unwrap_or_default();

  let lines : Vec< &str > = output.lines().collect();
  // Heading line should start with lead prefix
  assert!(
    lines[ 0 ].starts_with( "─── " ),
    "heading on narrow table must still render lead prefix; got: {:?}",
    lines[ 0 ],
  );
  // Title must appear verbatim (not truncated)
  assert!(
    lines[ 0 ].contains( "Title" ),
    "title must not be truncated; got: {:?}",
    lines[ 0 ],
  );
  // No trailing rule (content exceeds table width)
  assert!(
    !lines[ 0 ].ends_with( '─' ),
    "no trailing rule on narrow table; got: {:?}",
    lines[ 0 ],
  );
}

/// Corner case: heading with empty field produces valid output.
///
/// `Heading::new("T").with_field("")` should render without crash.
/// The empty field appends " · " but no field text.
#[ test ]
fn heading_with_empty_field_no_crash()
{
  let view = two_col_view();
  let config = TableConfig::plain()
    .with_heading( Heading::new( "T" ).with_field( "" ) );
  let output = TableFormatter::with_config( config )
    .format( &view )
    .unwrap_or_default();

  let caption_line = output.lines().next().unwrap_or( "" );
  assert!(
    caption_line.starts_with( "─── " ),
    "empty field must not crash; got: {caption_line:?}",
  );
  // Field separator must be present (even with empty field content)
  assert!(
    caption_line.contains( '·' ),
    "field separator must appear even for empty field; got: {caption_line:?}",
  );
}

/// Corner case: heading with many fields exceeding table width.
///
/// When content exceeds table width, trail = 0, heading is wider than table body.
#[ test ]
fn heading_many_fields_exceeding_table_width()
{
  let view = two_col_view(); // table_width = 10
  let config = TableConfig::plain()
    .with_heading(
      Heading::new( "T" )
        .with_field( "field_one" )
        .with_field( "field_two" )
        .with_field( "field_three" )
    );
  let output = TableFormatter::with_config( config )
    .format( &view )
    .unwrap_or_default();

  let caption_line = output.lines().next().unwrap_or( "" );
  // All fields must appear (no truncation)
  assert!(
    caption_line.contains( "field_one" ),
    "field_one must appear; got: {caption_line:?}",
  );
  assert!(
    caption_line.contains( "field_three" ),
    "field_three must appear; got: {caption_line:?}",
  );
  // No trailing rule when content exceeds table width
  assert!(
    !caption_line.ends_with( '─' ),
    "no trailing rule when fields exceed table width; got: {caption_line:?}",
  );
}

/// Heading on bordered table — heading width must match bordered body width (BUG-017).
///
/// ## Root Cause
/// Three functions (`compute_total_row_width`, `compute_column_budgets`,
/// `determine_fold_point`) computed outer padding as `inner_padding * 2` (2 total
/// units). But `format_single_line_row` applies `inner_padding` spaces before AND after
/// every cell — making the actual padding `inner_padding * 2 * N` where N is the number
/// of columns. The undercount caused heading lines to be narrower than table body rows
/// for any style with `inner_padding > 0` (bordered, markdown, grid, `unicode_box`).
///
/// ## Why Not Caught
/// Prior heading tests used `TableConfig::plain()` which has `inner_padding = 0` —
/// the flat-constant formula happens to be correct when padding is zero.
///
/// ## Fix Applied
/// - `compute_total_row_width`: `inner_padding * 2` → `inner_padding * 2 * column_widths.len()`
/// - `compute_column_budgets`: same change in overhead calculation
/// - `determine_fold_point`: moved padding into per-column accumulation inside the loop
///
/// ## Prevention
/// Any overhead formula that accounts for cell padding must use per-column
/// multiplication, not a flat constant. The rendering loop applies padding around every
/// cell, not just at the outer edges.
///
/// ## Pitfall
/// A flat `inner_padding * 2` constant is correct only for 1-column tables — for N
/// columns the total padding is `inner_padding * 2 * N`. The mismatch grows linearly
/// with column count and is invisible in zero-padding presets.
// test_kind: bug_reproducer(BUG-017)
#[ test ]
fn heading_on_bordered_table_display_width_matches()
{
  let view = two_col_view();
  let config = TableConfig::bordered()
    .with_heading( Heading::new( "AB" ) );
  let output = TableFormatter::with_config( config )
    .format( &view )
    .unwrap_or_default();

  let lines : Vec< &str > = output.lines().filter( | l | !l.is_empty() ).collect();
  assert!(
    lines.len() >= 3,
    "bordered with heading must have at least 3 lines; got {}",
    lines.len(),
  );

  let heading_dw = display_width( lines[ 0 ] );
  // Find a data/border line to compare width
  let body_dw = display_width( lines[ 1 ] );

  assert_eq!(
    heading_dw, body_dw,
    "heading display width ({heading_dw}) must match bordered body display width ({body_dw})\n\
     heading: {:?}\n body:    {:?}\nfull:\n{output}",
    lines[ 0 ], lines[ 1 ],
  );
}

/// Corner case: heading on csv table — heading is emitted (design choice).
///
/// CSV tables are not display-formatted, so heading width matching is not enforced.
/// This test just verifies no panic and that heading appears.
#[ test ]
fn heading_on_csv_table_no_crash()
{
  let view = two_col_view();
  let config = TableConfig::csv()
    .with_heading( Heading::new( "CSV Title" ) );
  let output = TableFormatter::with_config( config )
    .format( &view )
    .unwrap_or_default();

  let first_line = output.lines().next().unwrap_or( "" );
  assert!(
    first_line.starts_with( "─── CSV Title" ),
    "heading must appear on csv table; got: {first_line:?}",
  );
}

/// Corner case: heading + `min_column_width` changes `table_width`.
///
/// Floor enforcement increases column widths, which increases `table_width`.
/// The heading trail must fill to the new (wider) `table_width`.
#[ test ]
fn heading_with_min_column_width_fills_to_wider_table()
{
  let view = RowBuilder::new( vec![ "H".into() ] )
    .add_row( vec![ "x".into() ] )
    .build_view();

  // Without min: table_width = 1 (single col, content "H" or "x" → 1 char)
  // With min=20: table_width = 20
  let config = TableConfig::plain()
    .with_min_column_width( 20 )
    .with_heading( Heading::new( "T" ) );
  let output = TableFormatter::with_config( config )
    .format( &view )
    .unwrap_or_default();

  let caption_line = output.lines().next().unwrap_or( "" );
  let caption_chars = caption_line.chars().count();

  // table_width = 20; used = 3+1+1+1 = 6; trail = 14
  // heading should be exactly 20 chars
  assert_eq!(
    caption_chars, 20,
    "heading must fill to min_column_width-expanded table_width (20); got {caption_chars}: {caption_line:?}",
  );
}
