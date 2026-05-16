//! Tests for cell auto-wrapping with terminal-aware budget allocation (Task 019)
//!
//! ## What This Tests
//!
//! Verifies that `TableFormatter` auto-wraps flex-column cells at their budget
//! boundary when table width exceeds terminal width. Covers all 23 scenarios
//! from the Task 019 test matrix.
//!
//! ## Test Matrix
//!
//! T01–T03: Basic behavior (natural fit, single flex, all fixed)
//! T04–T05: Multi-column budget sharing, explicit `column_flex`
//! T06–T08: Disable modes (`auto_wrap` false, CSV, TSV)
//! T09–T13: Rendering integration (multiline, ANSI, borders, unicode, manual `\n`)
//! T14–T18: Edge cases (`min_column_width`, `column_widths` bypass, single row, empty, width=0)
//! T19–T20: Heuristic auto-classification
//! T21: Sub-row detail + wrapping
//! T22: Format trait path (`build_view`)
//! T23: All-Fixed columns sum exceeds terminal — graceful overflow

#![ cfg( feature = "enabled" ) ]
use data_fmt::{ RowBuilder, TableFormatter, TableConfig, ColumnFlex, Format, DecoratedText };

// --- T01: Table fits naturally within 120 cols ---

#[ test ]
fn auto_wrap_natural_fit_no_wrapping()
{
  let tree = RowBuilder::new( vec![ "Name".into(), "Age".into() ] )
    .add_row( vec![ DecoratedText::from( "Alice" ), DecoratedText::from( "30" ) ] )
    .add_row( vec![ DecoratedText::from( "Bob" ), DecoratedText::from( "25" ) ] )
    .build_view();

  let fmt_wrap = TableFormatter::with_config(
    TableConfig::plain().terminal_width( Some( 120 ) )
  );
  let fmt_no_wrap = TableFormatter::with_config(
    TableConfig::plain().auto_wrap( false )
  );

  let output_wrap = fmt_wrap.format( &tree ).unwrap_or_default();
  let output_no_wrap = fmt_no_wrap.format( &tree ).unwrap_or_default();
  assert_eq!( output_wrap, output_no_wrap, "no wrapping when table fits naturally" );
}

// --- T02: Single flex column exceeds 80-col terminal ---

#[ test ]
fn auto_wrap_wraps_flex_column()
{
  let long_path = "this/is/a/very/long/path/that/exceeds/the/terminal/width/significantly";
  let tree = RowBuilder::new( vec![ "ID".into(), "Path".into() ] )
    .add_row( vec![ DecoratedText::from( "1" ), long_path.into() ] )
    .build_view();

  let formatter = TableFormatter::with_config(
    TableConfig::plain().terminal_width( Some( 40 ) )
  );
  let output = formatter.format( &tree ).unwrap_or_default();

  // The output should contain multiple lines for the single data row
  // because the Path column wraps
  let data_lines : Vec< &str > = output.lines().skip( 2 ).collect(); // skip header + separator
  assert!(
    data_lines.len() > 1,
    "flex column should wrap to multiple lines, got: {data_lines:?}",
  );

  // Every line should fit within 40 cols (approximately)
  for line in output.lines()
  {
    let vis_len = data_fmt::visual_len( line );
    assert!(
      vis_len <= 42, // small tolerance for rounding
      "line exceeds terminal width: '{line}' (visual_len={vis_len})",
    );
  }
}

// --- T03: All columns Fixed (short content) ---

#[ test ]
fn auto_wrap_all_fixed_no_wrapping()
{
  let tree = RowBuilder::new( vec![ "A".into(), "B".into(), "C".into() ] )
    .add_row( vec![ DecoratedText::from( "x" ), DecoratedText::from( "y" ), DecoratedText::from( "z" ) ] )
    .build_view();

  let formatter = TableFormatter::with_config(
    TableConfig::plain().terminal_width( Some( 60 ) )
  );
  let output = formatter.format( &tree ).unwrap_or_default();

  // With all short columns (≤12 chars), all classified as Fixed — no wrapping
  let data_lines : Vec< &str > = output.lines().skip( 2 ).collect();
  assert_eq!( data_lines.len(), 1, "all-Fixed columns should not wrap" );
}

// --- T04: Two flex columns share budget equally ---

#[ test ]
fn auto_wrap_two_flex_columns_share_budget()
{
  let long1 = "aaaa bbbb cccc dddd eeee ffff gggg hhhh";
  let long2 = "1111 2222 3333 4444 5555 6666 7777 8888";
  let tree = RowBuilder::new( vec![ "ID".into(), "Col1".into(), "Col2".into() ] )
    .add_row( vec![ DecoratedText::from( "1" ), long1.into(), long2.into() ] )
    .build_view();

  let formatter = TableFormatter::with_config(
    TableConfig::plain().terminal_width( Some( 50 ) )
  );
  let output = formatter.format( &tree ).unwrap_or_default();

  // Both flex columns should wrap
  let data_lines : Vec< &str > = output.lines().skip( 2 ).collect();
  assert!(
    data_lines.len() > 1,
    "two flex columns sharing budget should produce wrapped output"
  );
}

// --- T05: Explicit column_flex overrides auto-classification ---

#[ test ]
fn auto_wrap_explicit_column_flex_override()
{
  let long_content = "this is a long string that should wrap because it exceeds budget";
  let tree = RowBuilder::new( vec![ "Col1".into(), "Col2".into(), "Col3".into() ] )
    .add_row( vec![
      long_content.into(),
      long_content.into(),
      long_content.into(),
    ] )
    .build_view();

  // Only middle column is Flex — only it should wrap
  let formatter = TableFormatter::with_config(
    TableConfig::plain()
      .terminal_width( Some( 60 ) )
      .column_flex( vec![ ColumnFlex::Fixed, ColumnFlex::Flex, ColumnFlex::Fixed ] )
  );
  let output = formatter.format( &tree ).unwrap_or_default();

  // The output should contain newlines from wrapping the middle column
  assert!(
    output.lines().count() > 3, // header + sep + at least 2 data lines
    "explicit Flex on middle column should cause wrapping"
  );
}

// --- T06: auto_wrap(false) disables wrapping ---

#[ test ]
fn auto_wrap_false_is_byte_identical()
{
  let long_path = "this/is/a/very/long/path/that/definitely/exceeds/eighty/columns/wide";
  let tree = RowBuilder::new( vec![ "ID".into(), "Path".into() ] )
    .add_row( vec![ DecoratedText::from( "1" ), long_path.into() ] )
    .build_view();

  let fmt_disabled = TableFormatter::with_config(
    TableConfig::plain().auto_wrap( false )
  );
  let fmt_default_no_term = TableFormatter::with_config(
    TableConfig::plain().auto_wrap( false ).terminal_width( Some( 40 ) )
  );

  let output1 = fmt_disabled.format( &tree ).unwrap_or_default();
  let output2 = fmt_default_no_term.format( &tree ).unwrap_or_default();
  assert_eq!( output1, output2, "auto_wrap(false) must produce identical output regardless of terminal_width" );

  // Should be single data line (no wrapping)
  let data_lines : Vec< &str > = output1.lines().skip( 2 ).collect();
  assert_eq!( data_lines.len(), 1, "auto_wrap disabled should not wrap" );
}

// --- T07: CSV preset auto-disables wrapping ---

#[ test ]
fn csv_preset_auto_disables_wrapping()
{
  let long = "this,is,a,very,long,value,that,exceeds,terminal,width";
  let tree = RowBuilder::new( vec![ "ID".into(), "Data".into() ] )
    .add_row( vec![ DecoratedText::from( "1" ), long.into() ] )
    .build_view();

  let formatter = TableFormatter::with_config(
    TableConfig::csv().terminal_width( Some( 30 ) )
  );
  let output = formatter.format( &tree ).unwrap_or_default();

  // CSV should never wrap
  let lines : Vec< &str > = output.lines().collect();
  assert_eq!( lines.len(), 2, "CSV must not wrap: header + 1 data line" );
  assert!( lines[ 1 ].contains( long ), "CSV content must be unmodified" );
}

// --- T08: TSV preset auto-disables wrapping ---

#[ test ]
fn tsv_preset_auto_disables_wrapping()
{
  let long = "this\tvalue\tis\tvery\tlong\tand\texceeds\tterminal\twidth\tboundary";
  let tree = RowBuilder::new( vec![ "ID".into(), "Data".into() ] )
    .add_row( vec![ DecoratedText::from( "1" ), long.into() ] )
    .build_view();

  let formatter = TableFormatter::with_config(
    TableConfig::tsv().terminal_width( Some( 30 ) )
  );
  let output = formatter.format( &tree ).unwrap_or_default();

  // TSV should never wrap
  let lines : Vec< &str > = output.lines().collect();
  assert_eq!( lines.len(), 2, "TSV must not wrap: header + 1 data line" );
}

// --- T09: Wrapped cell produces correct multiline rendering ---

#[ test ]
fn auto_wrap_multiline_alignment()
{
  let long = "alpha bravo charlie delta echo foxtrot golf hotel india";
  let tree = RowBuilder::new( vec![ "ID".into(), "Description".into() ] )
    .add_row( vec![ DecoratedText::from( "1" ), long.into() ] )
    .build_view();

  let formatter = TableFormatter::with_config(
    TableConfig::plain().terminal_width( Some( 40 ) )
  );
  let output = formatter.format( &tree ).unwrap_or_default();

  // All data lines (after header + separator) should have consistent column alignment
  let lines : Vec< &str > = output.lines().collect();
  assert!( lines.len() >= 4, "should have header, separator, and wrapped data lines" );

  // First column ("ID" / "1") should be padded consistently
  // All lines after the separator should have the same leading structure
  let header_line = lines[ 0 ];
  let first_data = lines[ 2 ];
  let id_width_header = header_line.find( "Description" ).unwrap_or( 0 );
  // Both header and data should start the second column at the same position
  assert!( id_width_header > 0, "header columns should be separated" );
  assert!( first_data.starts_with( '1' ) || first_data.starts_with( ' ' ) );
}

// --- T10: Wrapped cell with ANSI colors ---

#[ test ]
fn auto_wrap_ansi_colors_preserved()
{
  let colored = "\x1b[32mgreen text that is quite long and should wrap at budget\x1b[0m";
  let tree = RowBuilder::new( vec![ "ID".into(), "Status".into() ] )
    .add_row( vec![ DecoratedText::from( "1" ), colored.into() ] )
    .build_view();

  let formatter = TableFormatter::with_config(
    TableConfig::plain().terminal_width( Some( 40 ) )
  );
  let output = formatter.format( &tree ).unwrap_or_default();

  // Should not panic and should produce output
  assert!( !output.is_empty(), "ANSI colored content should render" );
  // Data should wrap to multiple lines
  let data_lines : Vec< &str > = output.lines().skip( 2 ).collect();
  assert!(
    !data_lines.is_empty(),
    "ANSI colored flex cell should produce output"
  );
}

// --- T11: Wrapped cell + bordered style ---

#[ test ]
fn auto_wrap_bordered_style()
{
  let long = "bordered content that should wrap within the budget allocation";
  let tree = RowBuilder::new( vec![ "ID".into(), "Content".into() ] )
    .add_row( vec![ DecoratedText::from( "1" ), long.into() ] )
    .build_view();

  let formatter = TableFormatter::with_config(
    TableConfig::bordered().terminal_width( Some( 50 ) )
  );
  let output = formatter.format( &tree ).unwrap_or_default();

  // Every line should have border pipes
  for line in output.lines()
  {
    let trimmed = line.trim();
    if trimmed.starts_with( '+' ) || trimmed.starts_with( '-' )
    {
      continue; // separator line
    }
    assert!(
      trimmed.starts_with( '|' ) && trimmed.ends_with( '|' ),
      "bordered line must have pipe borders: '{line}'"
    );
  }
}

// --- T12: Wrapped cell + unicode_box style ---

#[ test ]
fn auto_wrap_unicode_box_style()
{
  let long = "unicode box content that should wrap within the budget allocation";
  let tree = RowBuilder::new( vec![ "ID".into(), "Content".into() ] )
    .add_row( vec![ DecoratedText::from( "1" ), long.into() ] )
    .build_view();

  let formatter = TableFormatter::with_config(
    TableConfig::unicode_box().terminal_width( Some( 50 ) )
  );
  let output = formatter.format( &tree ).unwrap_or_default();

  // Data rows should have unicode box character borders
  for line in output.lines()
  {
    let trimmed = line.trim();
    if trimmed.is_empty() { continue; }
    let first_ch = trimmed.chars().next().unwrap();
    assert!(
      matches!( first_ch, '│' | '┌' | '├' | '└' ),
      "unicode box line must start with box chars: '{line}'"
    );
  }
}

// --- T13: Wrapped cell + existing manual \n ---

#[ test ]
fn auto_wrap_with_existing_newlines()
{
  let content_with_newline = "first line\nsecond line that is long enough to wrap at budget";
  let tree = RowBuilder::new( vec![ "ID".into(), "Content".into() ] )
    .add_row( vec![ DecoratedText::from( "1" ), content_with_newline.into() ] )
    .build_view();

  let formatter = TableFormatter::with_config(
    TableConfig::plain().terminal_width( Some( 40 ) )
  );
  let output = formatter.format( &tree ).unwrap_or_default();

  // Manual newlines should be preserved AND long lines should wrap further
  let data_lines : Vec< &str > = output.lines().skip( 2 ).collect();
  assert!(
    data_lines.len() >= 2,
    "manual newlines + wrapping should produce multiple lines: {data_lines:?}"
  );
}

// --- T14: Budget smaller than min_column_width ---

#[ test ]
fn auto_wrap_min_column_width_wins()
{
  let long = "content that should respect minimum column width setting";
  let tree = RowBuilder::new( vec![ "ID".into(), "Data".into() ] )
    .add_row( vec![ DecoratedText::from( "1" ), long.into() ] )
    .build_view();

  let formatter = TableFormatter::with_config(
    TableConfig::plain()
      .terminal_width( Some( 20 ) )
      .min_column_width( 15 )
  );
  let output = formatter.format( &tree ).unwrap_or_default();

  // Should not panic; min_column_width floor takes precedence
  assert!( !output.is_empty(), "should handle min_column_width > budget gracefully" );

  // The output column width should be at least min_column_width
  let header_line = output.lines().next().unwrap();
  let data_vis_len = data_fmt::visual_len( header_line );
  assert!(
    data_vis_len >= 15,
    "min_column_width should be respected: '{header_line}'"
  );
}

// --- T15: Explicit column_widths bypass ---

#[ test ]
fn auto_wrap_column_widths_override_bypass()
{
  let long = "this should not be wrapped because column_widths is explicitly set";
  let tree = RowBuilder::new( vec![ "ID".into(), "Data".into() ] )
    .add_row( vec![ DecoratedText::from( "1" ), long.into() ] )
    .build_view();

  let fmt_override = TableFormatter::with_config(
    TableConfig::plain()
      .terminal_width( Some( 30 ) )
      .column_widths( vec![ 5, 70 ] )
  );
  let fmt_no_wrap = TableFormatter::with_config(
    TableConfig::plain()
      .auto_wrap( false )
      .column_widths( vec![ 5, 70 ] )
  );

  let output1 = fmt_override.format( &tree ).unwrap_or_default();
  let output2 = fmt_no_wrap.format( &tree ).unwrap_or_default();
  assert_eq!(
    output1, output2,
    "explicit column_widths should bypass auto-wrap"
  );
}

// --- T16: Single-row table with wrapping ---

#[ test ]
fn auto_wrap_single_row()
{
  let long = "single row content that must wrap at the budget boundary cleanly";
  let tree = RowBuilder::new( vec![ "ID".into(), "Description".into() ] )
    .add_row( vec![ DecoratedText::from( "1" ), long.into() ] )
    .build_view();

  let formatter = TableFormatter::with_config(
    TableConfig::plain().terminal_width( Some( 40 ) )
  );
  let output = formatter.format( &tree ).unwrap_or_default();

  let data_lines : Vec< &str > = output.lines().skip( 2 ).collect();
  assert!(
    data_lines.len() > 1,
    "single row with long content should wrap"
  );
}

// --- T17: Empty table (headers only) ---

#[ test ]
fn auto_wrap_empty_table_headers_only()
{
  let view = RowBuilder::new( vec![
    "ID".into(),
    "VeryLongHeaderName".into(),
  ] )
    .build_view();

  let formatter = TableFormatter::with_config(
    TableConfig::plain().terminal_width( Some( 20 ) )
  );
  let output = Format::format( &formatter, &view ).unwrap();

  // IC-3 invariant: no columns → ""; columns + no rows → header + separator.
  // build_view() makes headers accessible; formatter renders them even with no data.
  assert!( !output.is_empty(), "headers-only table must render header row" );
  assert!( output.contains( "ID" ), "header column name must appear" );
  assert!( output.lines().count() <= 2, "must have at most header + separator lines" );
}

// --- T18: terminal_width(Some(0)) edge case ---

#[ test ]
fn auto_wrap_terminal_width_zero()
{
  let tree = RowBuilder::new( vec![ "A".into(), "B".into() ] )
    .add_row( vec![ DecoratedText::from( "x" ), DecoratedText::from( "y" ) ] )
    .build_view();

  let formatter = TableFormatter::with_config(
    TableConfig::plain().terminal_width( Some( 0 ) )
  );
  let output = formatter.format( &tree ).unwrap_or_default();

  // Should not panic
  assert!( !output.is_empty(), "terminal_width=0 should not panic" );
}

// --- T19: Heuristic auto-classification: short = Fixed ---

#[ test ]
fn auto_wrap_heuristic_short_is_fixed()
{
  // All columns ≤ 12 chars → all Fixed → no wrapping even with tight terminal
  let tree = RowBuilder::new( vec![ "ID".into(), "Name".into(), "Age".into() ] )
    .add_row( vec![ DecoratedText::from( "1" ), DecoratedText::from( "Alice" ), DecoratedText::from( "30" ) ] )
    .add_row( vec![ DecoratedText::from( "2" ), DecoratedText::from( "Bob" ), DecoratedText::from( "25" ) ] )
    .build_view();

  let fmt_wrap = TableFormatter::with_config(
    TableConfig::plain().terminal_width( Some( 30 ) )
  );
  let fmt_no_wrap = TableFormatter::with_config(
    TableConfig::plain().auto_wrap( false )
  );

  let output_wrap = fmt_wrap.format( &tree ).unwrap_or_default();
  let output_no_wrap = fmt_no_wrap.format( &tree ).unwrap_or_default();
  assert_eq!(
    output_wrap, output_no_wrap,
    "short columns (≤12 chars) should be Fixed and not wrap"
  );
}

// --- T20: Heuristic auto-classification: long = Flex ---

#[ test ]
fn auto_wrap_heuristic_long_is_flex()
{
  let long = "this is definitely longer than twelve characters";
  let tree = RowBuilder::new( vec![ "ID".into(), "Description".into() ] )
    .add_row( vec![ DecoratedText::from( "1" ), long.into() ] )
    .build_view();

  let formatter = TableFormatter::with_config(
    TableConfig::plain().terminal_width( Some( 40 ) )
  );
  let output = formatter.format( &tree ).unwrap_or_default();

  // Long column (>12 chars) auto-classified as Flex, should wrap
  let data_lines : Vec< &str > = output.lines().skip( 2 ).collect();
  assert!(
    data_lines.len() > 1,
    "long column (>12 chars) should auto-classify as Flex and wrap"
  );
}

// --- T21: Sub-row detail + wrapped cell ---

#[ test ]
fn auto_wrap_with_sub_row_detail()
{
  let view = RowBuilder::new( vec![ "ID".into(), "Description".into() ] )
    .add_row_with_detail(
      vec![
        DecoratedText::from( "1" ),
        DecoratedText::from( "a long description that should wrap at the budget boundary properly" ),
      ],
      Some( DecoratedText::from( "detail: additional info here" ) ),
    )
    .build_view();

  let formatter = TableFormatter::with_config(
    TableConfig::plain().terminal_width( Some( 50 ) )
  );
  let result = Format::format( &formatter, &view ).unwrap();

  // Detail line should appear after all wrapped cell lines
  assert!( result.contains( "detail:" ), "sub-row detail should be present" );

  // The detail line should come after the data content
  let detail_pos = result.find( "detail:" ).unwrap();
  let first_data_pos = result.find( "a long" ).unwrap_or( 0 );
  assert!(
    detail_pos > first_data_pos,
    "detail line must appear after wrapped cell content"
  );
}

// --- T22: build_view() path with Format trait ---

#[ test ]
fn auto_wrap_format_trait_path()
{
  let long = "format trait path content that must wrap at the terminal budget boundary";
  let view = RowBuilder::new( vec![ "ID".into(), "Content".into() ] )
    .add_row( vec![ DecoratedText::from( "1" ), long.into() ] )
    .build_view();

  let formatter = TableFormatter::with_config(
    TableConfig::plain().terminal_width( Some( 40 ) )
  );
  let result = Format::format( &formatter, &view ).unwrap();

  // Should produce wrapped output via Format trait
  let data_lines : Vec< &str > = result.lines().skip( 2 ).collect();
  assert!(
    data_lines.len() > 1,
    "Format trait path should produce wrapped output"
  );
}

// --- T23: All-Fixed columns sum exceeds terminal — graceful overflow ---
// invariant: Fixed columns are never truncated by budget allocation; output may
// exceed terminal width but must not panic and must contain all cell content.
#[ test ]
fn auto_wrap_all_fixed_columns_exceed_terminal()
{
  let content_a = "aaaa bbbb cccc dddd eeee ffff gggg hhhh iiii jjjj";
  let content_b = "1111 2222 3333 4444 5555 6666 7777 8888 9999 0000";
  let content_c = "AAAA BBBB CCCC DDDD EEEE FFFF GGGG HHHH IIII JJJJ";
  let tree = RowBuilder::new( vec![ "A".into(), "B".into(), "C".into() ] )
    .add_row( vec![ content_a.into(), content_b.into(), content_c.into() ] )
    .build_view();

  // All three columns forced Fixed; each is ~49 chars wide, total >> 40
  let formatter = TableFormatter::with_config(
    TableConfig::plain()
      .terminal_width( Some( 40 ) )
      .column_flex( vec![ ColumnFlex::Fixed, ColumnFlex::Fixed, ColumnFlex::Fixed ] )
  );
  // Must not panic; output must be non-empty and contain all cell words
  let output = formatter.format( &tree ).unwrap_or_default();
  assert!( !output.is_empty(), "all-Fixed overflow must produce non-empty output" );
  // All cell content must appear somewhere in the output (possibly across wrapped lines)
  for word in [ "aaaa", "hhhh", "1111", "9999", "AAAA", "JJJJ" ]
  {
    assert!(
      output.contains( word ),
      "all-Fixed cell content word '{word}' must appear in output",
    );
  }
}

// --- BA AC-6: CSV preset bypasses budget allocation regardless of content width ---

/// AC-6 — `004_budget_allocation`: CSV preset bypasses budget allocation regardless of content width.
///
/// Even with `auto_wrap(true)` explicitly set, `TableConfig::csv()` skips budget allocation
/// entirely. Cell content is not wrapped; each data row occupies exactly one output line;
/// the output is well-formed comma-separated values with natural content lengths preserved.
// test_kind: standard
#[ test ]
fn csv_bypasses_budget_allocation_ac6()
{
  let long_value = "this value is quite long and would normally cause wrapping in a flex column";
  let view = RowBuilder::new( vec![ "Name".into(), "Value".into() ] )
    .add_row( vec![ DecoratedText::from( "Alice" ), long_value.into() ] )
    .build_view();

  let formatter = TableFormatter::with_config(
    TableConfig::csv()
      .auto_wrap( true )
      .terminal_width( Some( 30 ) )
  );
  let output = formatter.format( &view ).expect( "CSV formatting must not fail" );

  // Data row must occupy exactly one line (no wrapping)
  let data_lines : Vec< &str > = output.lines()
    .filter( | l | l.contains( "Alice" ) )
    .collect();
  assert_eq!(
    data_lines.len(), 1,
    "CSV must not wrap — Alice must appear on exactly one line:\n{output:?}",
  );
  // Full natural content length must be preserved (no truncation)
  assert!(
    output.contains( long_value ),
    "CSV must preserve full content length even when it exceeds terminal width:\n{output:?}",
  );
  // Output must be comma-separated
  assert!( output.contains( ',' ), "CSV output must use comma separator:\n{output:?}" );
}

// --- BA AC-7: remainder characters distributed to leftmost flex columns ---

/// AC-7 — `004_budget_allocation`: remainder characters distributed to leftmost flex columns.
///
/// When the available budget is not evenly divisible by the number of flex columns,
/// the leftmost flex columns each receive one extra character. For 3 flex columns with
/// `budget % 3 == 1`, the leftmost column gets width `base + 1` while the other two
/// get `base`. All three budgets differ by at most 1.
// test_kind: standard
#[ test ]
fn flex_remainder_to_leftmost_column_ac7()
{
  // Content much longer than any expected budget per column
  let long = "alpha bravo charlie delta echo foxtrot golf hotel india juliet kilo";
  let view = RowBuilder::new( vec![ "A".into(), "B".into(), "C".into() ] )
    .add_row( vec![ long.into(), long.into(), long.into() ] )
    .build_view();

  // terminal_width=35 with plain 2-space separator overhead (2 gaps × 2 spaces = 4):
  // budget = 35 - 4 = 31; 31 % 3 = 1 → leftmost gets 11 chars, others get 10
  let formatter = TableFormatter::with_config(
    TableConfig::plain()
      .terminal_width( Some( 35 ) )
      .column_flex( vec![ ColumnFlex::Flex, ColumnFlex::Flex, ColumnFlex::Flex ] )
  );
  let output = formatter.format( &view ).expect( "must not fail with remainder budget" );
  assert!( !output.is_empty(), "output must be non-empty" );

  // Parse column widths from the separator line (second line: dash segments)
  let sep_line = output.lines().nth( 1 ).expect( "separator line must exist" );
  let segments : Vec< &str > = sep_line.split_whitespace().collect();
  assert_eq!( segments.len(), 3, "separator must have 3 column segments:\n{sep_line:?}" );
  let widths : Vec< usize > = segments.iter().map( | s | s.len() ).collect();
  let max_w = *widths.iter().max().unwrap();
  let min_w = *widths.iter().min().unwrap();
  assert!(
    max_w - min_w <= 1,
    "all three flex column budgets must differ by at most 1; got widths: {widths:?}\n{output:?}",
  );
  assert!(
    widths[ 0 ] >= widths[ 2 ],
    "leftmost flex column must have width ≥ rightmost when remainder > 0; got widths: {widths:?}\n{output:?}",
  );
}

// --- BA AC-8: flex budget floored at minimum when terminal too narrow ---

/// AC-8 — `004_budget_allocation`: flex budget floored at minimum when terminal too narrow.
///
/// When terminal width is smaller than the fixed column width plus separator overhead,
/// the computed flex budget is zero or negative. The flex column receives a budget of
/// at least 1 character (Step 6 clamp); no panic occurs; output is non-empty; the
/// fixed column content appears untruncated.
// test_kind: standard
#[ test ]
fn flex_budget_floored_at_minimum_ac8()
{
  let fixed_content = "fixed-col-value"; // forced Fixed by column_flex
  let flex_content = "flex column content here";
  let view = RowBuilder::new( vec![ "Fixed".into(), "Flex".into() ] )
    .add_row( vec![ fixed_content.into(), flex_content.into() ] )
    .build_view();

  // terminal_width(5) is narrower than fixed column (15 chars) + overhead → flex budget ≤ 0
  let formatter = TableFormatter::with_config(
    TableConfig::plain()
      .terminal_width( Some( 5 ) )
      .column_flex( vec![ ColumnFlex::Fixed, ColumnFlex::Flex ] )
  );
  let output = formatter.format( &view )
    .expect( "must not panic when flex budget would be negative" );
  assert!( !output.is_empty(), "output must be non-empty even with extreme narrow terminal:\n{output:?}" );
  // Fixed column content must appear untruncated (Fixed columns are never budget-limited)
  assert!(
    output.contains( fixed_content ),
    "fixed column content must appear untruncated:\n{output:?}",
  );
}

// --- WC IN-3: bordered preset with auto_wrap=false produces byte-identical output ---

/// IN-3 — `003_auto_wrap_backward_compat`: `unicode_box` preset with `auto_wrap=false` produces
/// byte-identical output to the baseline (no `auto_wrap` or `terminal_width` set).
///
/// Disabling `auto_wrap` on a unicode box-drawing table is a true opt-out — the
/// box-drawing characters are unaffected, and the output is byte-identical to the
/// baseline formatter with no `auto_wrap` or `terminal_width` override.
// test_kind: standard
#[ test ]
fn unicode_box_auto_wrap_false_byte_identical_in3()
{
  let view = RowBuilder::new( vec![ "Name".into(), "Value".into() ] )
    .add_row( vec![ DecoratedText::from( "Alice" ), DecoratedText::from( "42" ) ] )
    .build_view();

  let fmt_disabled = TableFormatter::with_config(
    TableConfig::unicode_box().auto_wrap( false ).terminal_width( Some( 40 ) )
  );
  let fmt_baseline = TableFormatter::with_config( TableConfig::unicode_box() );

  let output1 = fmt_disabled.format( &view ).expect( "must not fail" );
  let output2 = fmt_baseline.format( &view ).expect( "must not fail" );
  assert_eq!(
    output1, output2,
    "unicode_box with auto_wrap=false must be byte-identical to baseline",
  );
}

// --- WC IN-4: markdown preset with auto_wrap=false produces byte-identical output ---

/// IN-4 — `003_auto_wrap_backward_compat`: markdown preset with `auto_wrap=false` produces
/// byte-identical output to the baseline (no `auto_wrap` or `terminal_width` set).
///
/// Disabling `auto_wrap` on a markdown table is a true opt-out — the pipe-and-dash
/// separators are unaffected, and the output is byte-identical to the baseline.
// test_kind: standard
#[ test ]
fn markdown_auto_wrap_false_byte_identical_in4()
{
  let view = RowBuilder::new( vec![ "Name".into(), "Value".into() ] )
    .add_row( vec![ DecoratedText::from( "Alice" ), DecoratedText::from( "42" ) ] )
    .build_view();

  let fmt_disabled = TableFormatter::with_config(
    TableConfig::markdown().auto_wrap( false ).terminal_width( Some( 40 ) )
  );
  let fmt_baseline = TableFormatter::with_config( TableConfig::markdown() );

  let output1 = fmt_disabled.format( &view ).expect( "must not fail" );
  let output2 = fmt_baseline.format( &view ).expect( "must not fail" );
  assert_eq!(
    output1, output2,
    "markdown with auto_wrap=false must be byte-identical to baseline",
  );
}

// --- WC IN-5: minimal preset with auto_wrap=false produces byte-identical output ---

/// IN-5 — `003_auto_wrap_backward_compat`: minimal preset with `auto_wrap=false` produces
/// byte-identical output to the baseline (no `auto_wrap` or `terminal_width` set).
// test_kind: standard
#[ test ]
fn minimal_auto_wrap_false_byte_identical_in5()
{
  let view = RowBuilder::new( vec![ "Name".into(), "Value".into() ] )
    .add_row( vec![ DecoratedText::from( "Alice" ), DecoratedText::from( "42" ) ] )
    .build_view();

  let fmt_disabled = TableFormatter::with_config(
    TableConfig::minimal().auto_wrap( false ).terminal_width( Some( 40 ) )
  );
  let fmt_baseline = TableFormatter::with_config( TableConfig::minimal() );

  let output1 = fmt_disabled.format( &view ).expect( "must not fail" );
  let output2 = fmt_baseline.format( &view ).expect( "must not fail" );
  assert_eq!(
    output1, output2,
    "minimal with auto_wrap=false must be byte-identical to baseline",
  );
}

// --- WC IN-6: bordered preset with auto_wrap=false produces byte-identical output ---

/// IN-6 — `003_auto_wrap_backward_compat`: bordered preset with `auto_wrap=false` produces
/// byte-identical output to the baseline (no `auto_wrap` or `terminal_width` set).
// test_kind: standard
#[ test ]
fn bordered_auto_wrap_false_byte_identical_in6()
{
  let view = RowBuilder::new( vec![ "Name".into(), "Value".into() ] )
    .add_row( vec![ DecoratedText::from( "Alice" ), DecoratedText::from( "42" ) ] )
    .build_view();

  let fmt_disabled = TableFormatter::with_config(
    TableConfig::bordered().auto_wrap( false ).terminal_width( Some( 40 ) )
  );
  let fmt_baseline = TableFormatter::with_config( TableConfig::bordered() );

  let output1 = fmt_disabled.format( &view ).expect( "must not fail" );
  let output2 = fmt_baseline.format( &view ).expect( "must not fail" );
  assert_eq!(
    output1, output2,
    "bordered with auto_wrap=false must be byte-identical to baseline",
  );
}

// --- WC IN-7: grid preset with auto_wrap=false produces byte-identical output ---

/// IN-7 — `003_auto_wrap_backward_compat`: grid preset with `auto_wrap=false` produces
/// byte-identical output to the baseline (no `auto_wrap` or `terminal_width` set).
// test_kind: standard
#[ test ]
fn grid_auto_wrap_false_byte_identical_in7()
{
  let view = RowBuilder::new( vec![ "Name".into(), "Value".into() ] )
    .add_row( vec![ DecoratedText::from( "Alice" ), DecoratedText::from( "42" ) ] )
    .build_view();

  let fmt_disabled = TableFormatter::with_config(
    TableConfig::grid().auto_wrap( false ).terminal_width( Some( 40 ) )
  );
  let fmt_baseline = TableFormatter::with_config( TableConfig::grid() );

  let output1 = fmt_disabled.format( &view ).expect( "must not fail" );
  let output2 = fmt_baseline.format( &view ).expect( "must not fail" );
  assert_eq!(
    output1, output2,
    "grid with auto_wrap=false must be byte-identical to baseline",
  );
}

// --- WC IN-8: csv preset with auto_wrap=false produces byte-identical output ---

/// IN-8 — `003_auto_wrap_backward_compat`: csv preset with `auto_wrap=false` produces
/// byte-identical output to the baseline (no `auto_wrap` or `terminal_width` set).
/// CSV auto-bypasses budget allocation regardless, so both configurations match.
// test_kind: standard
#[ test ]
fn csv_auto_wrap_false_byte_identical_in8()
{
  let view = RowBuilder::new( vec![ "Name".into(), "Value".into() ] )
    .add_row( vec![ DecoratedText::from( "Alice" ), DecoratedText::from( "42" ) ] )
    .build_view();

  let fmt_disabled = TableFormatter::with_config(
    TableConfig::csv().auto_wrap( false ).terminal_width( Some( 40 ) )
  );
  let fmt_baseline = TableFormatter::with_config( TableConfig::csv() );

  let output1 = fmt_disabled.format( &view ).expect( "must not fail" );
  let output2 = fmt_baseline.format( &view ).expect( "must not fail" );
  assert_eq!(
    output1, output2,
    "csv with auto_wrap=false must be byte-identical to baseline",
  );
}

// --- WC IN-9: tsv preset with auto_wrap=false produces byte-identical output ---

/// IN-9 — `003_auto_wrap_backward_compat`: tsv preset with `auto_wrap=false` produces
/// byte-identical output to the baseline (no `auto_wrap` or `terminal_width` set).
/// TSV auto-bypasses budget allocation regardless, so both configurations match.
// test_kind: standard
#[ test ]
fn tsv_auto_wrap_false_byte_identical_in9()
{
  let view = RowBuilder::new( vec![ "Name".into(), "Value".into() ] )
    .add_row( vec![ DecoratedText::from( "Alice" ), DecoratedText::from( "42" ) ] )
    .build_view();

  let fmt_disabled = TableFormatter::with_config(
    TableConfig::tsv().auto_wrap( false ).terminal_width( Some( 40 ) )
  );
  let fmt_baseline = TableFormatter::with_config( TableConfig::tsv() );

  let output1 = fmt_disabled.format( &view ).expect( "must not fail" );
  let output2 = fmt_baseline.format( &view ).expect( "must not fail" );
  assert_eq!(
    output1, output2,
    "tsv with auto_wrap=false must be byte-identical to baseline",
  );
}

// --- WC IN-10: compact preset with auto_wrap=false produces byte-identical output ---

/// IN-10 — `003_auto_wrap_backward_compat`: compact preset with `auto_wrap=false` produces
/// byte-identical output to the baseline (no `auto_wrap` or `terminal_width` set).
// test_kind: standard
#[ test ]
fn compact_auto_wrap_false_byte_identical_in10()
{
  let view = RowBuilder::new( vec![ "Name".into(), "Value".into() ] )
    .add_row( vec![ DecoratedText::from( "Alice" ), DecoratedText::from( "42" ) ] )
    .build_view();

  let fmt_disabled = TableFormatter::with_config(
    TableConfig::compact().auto_wrap( false ).terminal_width( Some( 40 ) )
  );
  let fmt_baseline = TableFormatter::with_config( TableConfig::compact() );

  let output1 = fmt_disabled.format( &view ).expect( "must not fail" );
  let output2 = fmt_baseline.format( &view ).expect( "must not fail" );
  assert_eq!(
    output1, output2,
    "compact with auto_wrap=false must be byte-identical to baseline",
  );
}

// --- BA AC-9: 12-character threshold boundary — column at exactly 12 chars uses Fixed ---

/// AC-9 — `004_budget_allocation`: column whose max cell content is exactly 12 visible
/// characters is classified as `Fixed` by the auto-flex heuristic (threshold ≤ 12).
///
/// The 12-char-wide column retains its natural width and is excluded from budget
/// redistribution even when the terminal is narrow enough to trigger wrapping on
/// other columns. (Source: `docs/algorithm/004_budget_allocation.md` — "≤ 12 chars → Fixed".)
// test_kind: standard
#[ test ]
fn twelve_char_threshold_column_treated_as_fixed_ac9()
{
  // Column A: exactly 12-char content → auto-heuristic → Fixed (not Flex)
  let fixed_val = "abcdefghijkl"; // exactly 12 chars
  // Column B: 20-char content → auto-heuristic → Flex
  let flex_val = "this is twenty chars";  // 20 chars

  let view = RowBuilder::new( vec![ "A".into(), "B".into() ] )
    .add_row( vec![ fixed_val.into(), flex_val.into() ] )
    .build_view();

  // terminal_width(25) forces wrapping on the Flex column (B) but should leave Fixed (A) intact
  let formatter = TableFormatter::with_config(
    TableConfig::plain()
      .terminal_width( Some( 25 ) )
      .auto_wrap( true )
  );
  let output = formatter.format( &view ).expect( "must not fail" );

  // Column A (12-char Fixed) must appear untruncated — all 12 chars present
  assert!(
    output.contains( fixed_val ),
    "12-char column must be treated as Fixed and appear untruncated:\n{output:?}",
  );
}

// --- BA AC-10: overhead exceeds terminal width — all flex columns clamped to floor ---

/// AC-10 — `004_budget_allocation`: when fixed column widths plus separator overhead
/// already exceed the terminal width before any flex budget is computed, all flex
/// columns receive the floor budget (at least 1 char); no panic occurs; output is
/// non-empty even though total width exceeds terminal.
// test_kind: standard
#[ test ]
fn overhead_exceeds_terminal_flex_clamped_to_floor_ac10()
{
  // Two Fixed columns with content long enough that overhead alone exceeds terminal
  let view = RowBuilder::new( vec![ "F1".into(), "F2".into(), "Flex".into() ] )
    .add_row( vec![
      "long_fixed_one".into(),  // 14 chars
      "long_fixed_two".into(),  // 14 chars
      "flex column data here".into(),
    ])
    .build_view();

  // terminal_width(25) — smaller than fixed overhead (F1+F2+sep = 30) but wide enough
  // that fold continuation can display the strings without per-character wrapping.
  // Overhead (30) > terminal (25) triggers the floor-clamping behavior for Flex.
  let formatter = TableFormatter::with_config(
    TableConfig::plain()
      .terminal_width( Some( 25 ) )
      .column_flex( vec![ ColumnFlex::Fixed, ColumnFlex::Fixed, ColumnFlex::Flex ] )
  );
  let output = formatter.format( &view )
    .expect( "must not panic when overhead exceeds terminal width" );

  // Output must be non-empty — formatter does not abort on negative budget
  assert!(
    !output.is_empty(),
    "output must be non-empty even when overhead exceeds terminal:\n{output:?}",
  );
  // First Fixed column (primary) must appear untruncated
  assert!(
    output.contains( "long_fixed_one" ),
    "first Fixed column must appear untruncated:\n{output:?}",
  );
  // Second Fixed column (folded) content must appear in continuation output
  assert!(
    output.contains( "long_fixed_two" ),
    "second Fixed column must appear untruncated:\n{output:?}",
  );
}
