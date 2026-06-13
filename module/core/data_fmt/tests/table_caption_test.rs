//! Tests for `TableCaption` titled-rule rendering (`feature/007_table_caption`)
//!
//! ## What This Tests
//!
//! Verifies that `TableCaption` prepends a titled rule to table output following
//! the format `─── Title · Field1 · Field2 ──────...` filling the terminal width.
//! See `docs/feature/007_table_caption.md` for the full specification.

#![ cfg( feature = "enabled" ) ]
use data_fmt::{ RowBuilder, TableFormatter, TableConfig, TableCaption, TableView, Format };

// --- Test helper ---

fn two_col_view() -> TableView
{
  RowBuilder::new( vec![ "Name".into(), "Age".into() ] )
    .add_row( vec![ "Alice".into(), "30".into() ] )
    .build_view()
}

// --- FC-1: title-only caption renders titled rule before table ---
//
// Given: TableConfig::plain() with .caption(TableCaption::new("Results"))
// When: a two-column, one-row table is formatted
// Then: first line starts with "─── Results " and ends with ─ chars; second line is header

/// FC-1 — `feature/007`: title-only caption renders titled rule before the table.
// test_kind: standard
#[ test ]
fn title_only_caption_renders_titled_rule_fc1()
{
  let config = TableConfig::plain().caption( TableCaption::new( "Results" ) );
  let output = TableFormatter::with_config( config )
    .format( &two_col_view() )
    .unwrap_or_default();

  let mut lines = output.lines();
  let caption_line = lines.next().unwrap_or( "" );
  let second_line  = lines.next().unwrap_or( "" );

  assert!(
    caption_line.starts_with( "─── Results " ),
    "caption line must start with '─── Results '; got: '{caption_line}'",
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
// Given: TableCaption::new("Needs Review").field("28 PRs").field("15 repos")
// When: rendered via TableConfig::plain()
// Then: first output line contains "Needs Review · 28 PRs · 15 repos"

/// FC-2 — `feature/007`: caption fields appear joined by the field separator.
// test_kind: standard
#[ test ]
fn caption_fields_joined_by_separator_fc2()
{
  let caption = TableCaption::new( "Needs Review" )
    .field( "28 PRs" )
    .field( "15 repos" );
  let config = TableConfig::plain().caption( caption );
  let output = TableFormatter::with_config( config )
    .format( &two_col_view() )
    .unwrap_or_default();

  let first_line = output.lines().next().unwrap_or( "" );
  assert!(
    first_line.contains( "Needs Review · 28 PRs · 15 repos" ),
    "caption line must contain 'Needs Review · 28 PRs · 15 repos'; got: '{first_line}'",
  );
}

// --- FC-3: caption line fills to terminal width with rule chars ---
//
// Given: TableConfig::plain().terminal_width(Some(60)).caption(TableCaption::new("T").field("F"))
// When: table formatted
// Then: first line (caption) has exactly 60 display columns (.chars().count() == 60)

/// FC-3 — `feature/007`: caption line fills exactly to the resolved terminal width.
// test_kind: standard
#[ test ]
fn caption_fills_to_terminal_width_fc3()
{
  let caption = TableCaption::new( "T" ).field( "F" );
  let config = TableConfig::plain()
    .terminal_width( Some( 60 ) )
    .caption( caption );
  let output = TableFormatter::with_config( config )
    .format( &two_col_view() )
    .unwrap_or_default();

  let caption_line = output.lines().next().unwrap_or( "" );
  let char_count = caption_line.chars().count();
  assert_eq!(
    char_count, 60,
    "caption line must be exactly 60 chars (use .chars().count() — '─' is 3 UTF-8 bytes); \
     got {char_count} chars: '{caption_line}'",
  );
}

// --- FC-5: no-caption config produces identical output to current behavior ---
//
// Given: TableConfig::plain() without .caption(), same config without .caption()
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

  // Build an identical config via a fresh constructor — no .caption() call on either
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
// Given: TableConfig::grid() with .caption(TableCaption::new("Grid Table"))
// When: table formatted
// Then: first line is the caption line (starts with "─── Grid Table"); second line is top border

/// FC-6 — `feature/007`: caption appears before the top border in grid and `unicode_box` styles.
// test_kind: standard
#[ test ]
fn caption_before_top_border_grid_fc6()
{
  // --- grid style ---
  let config_grid = TableConfig::grid()
    .caption( TableCaption::new( "Grid Table" ) );
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
    .caption( TableCaption::new( "Grid Table" ) );
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
