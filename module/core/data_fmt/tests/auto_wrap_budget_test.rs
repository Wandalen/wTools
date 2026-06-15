//! Budget allocation acceptance criteria and invariant tests for auto-wrap (Task 019)
//!
//! ## What This Tests
//!
//! Additional acceptance criteria (AC) and invariant (IN) tests verifying budget
//! allocation algorithm correctness, CSV/TSV bypass, and backward compatibility
//! guarantees not covered by the primary TDD test matrix (T01-T23 in `auto_wrap_test.rs`).
//!
//! See `docs/feature/005_auto_fit.md § Strategy 2 — Column Wrapping` and
//! `docs/algorithm/004_budget_allocation.md` for the behavioral specification.
//!
//! ## Test Matrix
//!
//! BA AC-6: CSV preset bypasses budget allocation regardless of content width
//! BA AC-7: remainder characters distributed to leftmost flex columns
//! BA AC-8: flex budget floored at minimum when terminal narrower than fixed columns
//! WC IN-3: bordered preset with `auto_wrap=false` produces byte-identical output

#![ cfg( feature = "enabled" ) ]
use data_fmt::{ RowBuilder, TableFormatter, TableConfig, ColumnFlex, Format, DecoratedText };

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
      .with_auto_wrap( true )
      .with_terminal_width( Some( 30 ) )
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
      .with_terminal_width( Some( 35 ) )
      .with_column_flex( vec![ ColumnFlex::Flex, ColumnFlex::Flex, ColumnFlex::Flex ] )
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
      .with_terminal_width( Some( 5 ) )
      .with_column_flex( vec![ ColumnFlex::Fixed, ColumnFlex::Flex ] )
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
    TableConfig::unicode_box().with_auto_wrap( false ).with_terminal_width( Some( 40 ) )
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
    TableConfig::markdown().with_auto_wrap( false ).with_terminal_width( Some( 40 ) )
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
    TableConfig::minimal().with_auto_wrap( false ).with_terminal_width( Some( 40 ) )
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
    TableConfig::bordered().with_auto_wrap( false ).with_terminal_width( Some( 40 ) )
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
    TableConfig::grid().with_auto_wrap( false ).with_terminal_width( Some( 40 ) )
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
    TableConfig::csv().with_auto_wrap( false ).with_terminal_width( Some( 40 ) )
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
    TableConfig::tsv().with_auto_wrap( false ).with_terminal_width( Some( 40 ) )
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
    TableConfig::compact().with_auto_wrap( false ).with_terminal_width( Some( 40 ) )
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
      .with_terminal_width( Some( 25 ) )
      .with_auto_wrap( true )
  );
  let output = formatter.format( &view ).expect( "must not fail" );

  // Column A (12-char Fixed) must appear untruncated — all 12 chars present
  assert!(
    output.contains( fixed_val ),
    "12-char column must be treated as Fixed and appear untruncated:\n{output:?}",
  );
}

// --- BA AC-11: 13-character threshold boundary — column at exactly 13 chars uses Flex ---

/// AC-11 — `004_budget_allocation`: column whose max cell content is exactly 13 visible
/// characters is classified as `Flex` by the auto-flex heuristic (threshold > 12).
///
/// With a narrow `terminal_width`, the Flex budget for this column is less than 13 chars,
/// causing content wrapping. This proves the threshold is `> 12` (not `>= 12`), complementing
/// AC-9 which proves 12-char content is Fixed.
// test_kind: standard
#[ test ]
fn thirteen_char_threshold_column_treated_as_flex_ac11()
{
  // Column A: exactly 13-char content → auto-heuristic → Flex (13 > 12)
  let flex_val = "abcdefghijklm"; // exactly 13 chars
  // Column B: 1-char anchor → auto-heuristic → Fixed (1 ≤ 12)
  let anchor_val = "X";

  let view = RowBuilder::new( vec![ "A".into(), "B".into() ] )
    .add_row( vec![ flex_val.into(), anchor_val.into() ] )
    .build_view();

  // terminal_width(10): narrower than the 13-char content.
  // If A is Flex: budget ≈ 10 − overhead(2) − fixed(1) = 7 → wrapping → "abcdefghijklm" split.
  // If A were Fixed: "abcdefghijklm" would appear verbatim.
  let formatter = TableFormatter::with_config(
    TableConfig::plain()
      .with_terminal_width( Some( 10 ) )
      .with_auto_wrap( true )
  );
  let output = formatter.format( &view ).expect( "must not fail for 13-char threshold test" );

  assert!( !output.is_empty(), "output must be non-empty:\n{output:?}" );
  // 13-char content must be wrapped (Flex budget < 13) — must not appear as unbroken string
  assert!(
    !output.contains( flex_val ),
    "13-char column must be treated as Flex and content wrapped; \
     '{flex_val}' must not appear unbroken at terminal_width=10:\n{output:?}",
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
      .with_terminal_width( Some( 25 ) )
      .with_column_flex( vec![ ColumnFlex::Fixed, ColumnFlex::Fixed, ColumnFlex::Flex ] )
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
