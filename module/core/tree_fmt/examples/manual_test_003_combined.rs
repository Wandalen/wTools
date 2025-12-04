//! Manual Test 003: Combined Features (Multiline + Truncation)
//!
//! This example tests the interaction between multiline and truncation features.
//! Run with: `cargo run --example manual_test_003_combined`

use tree_fmt::{ RowBuilder, TableFormatter, TableConfig };

#[ allow( clippy::too_many_lines ) ]
fn main()
{
  println!( "╔═══════════════════════════════════════════════════════════════════╗" );
  println!( "║  MANUAL TEST 003: Combined Features (Multiline + Truncation)      ║" );
  println!( "╚═══════════════════════════════════════════════════════════════════╝\n" );

  // TC-010: Basic Combination
  println!( "┌─ TC-010: Multiline + Truncation Combined ────────────────────────┐" );
  {
    let data = RowBuilder::new( vec![ "Content".into() ] )
      .add_row( vec![ "Very long first line that exceeds the limit\nShort second line".into() ] )
      .build();

    let config = TableConfig::plain()
      .max_column_width( Some( 25 ) );

    let formatter = TableFormatter::with_config( config );
    let output = formatter.format( &data );

    println!( "{output}" );
    println!( "Expected:" );
    println!( "  - First line truncated to 22 chars + '...' = 25" );
    println!( "  - Second line displayed fully (no truncation)" );
    println!( "  - Each line independently evaluated" );
  }
  println!( "└───────────────────────────────────────────────────────────────────┘\n" );

  // TC-011: All Three Features
  println!( "┌─ TC-011: Multiline + Truncation + ANSI ──────────────────────────┐" );
  {
    let data = RowBuilder::new( vec![ "Colored Content".into() ] )
      .add_row( vec![
        "\x1b[31mVery long red text that needs truncation applied here\x1b[0m\n\x1b[32mShort green line\x1b[0m\n\x1b[34mAnother long blue line that also exceeds the width limit\x1b[0m".into()
      ] )
      .build();

    let config = TableConfig::plain()
      .max_column_width( Some( 30 ) );

    let formatter = TableFormatter::with_config( config );
    let output = formatter.format( &data );

    println!( "{output}" );
    println!( "Expected:" );
    println!( "  - Line 1: Red color preserved, truncated with marker" );
    println!( "  - Line 2: Green color preserved, no truncation" );
    println!( "  - Line 3: Blue color preserved, truncated with marker" );
    println!( "Visual check: All colors render correctly" );
  }
  println!( "└───────────────────────────────────────────────────────────────────┘\n" );

  // Complex scenario: Multiple columns with multiline + truncation
  println!( "┌─ Complex: Multiple Columns, Multiline, Truncation ───────────────┐" );
  {
    let data = RowBuilder::new( vec![ "ID".into(), "Description".into(), "Status".into() ] )
      .add_row( vec![
        "1".into(),
        "Short description".into(),
        "OK".into()
      ] )
      .add_row( vec![
        "2".into(),
        "Very long description that spans multiple lines\nSecond line also very long\nThird line short".into(),
        "Pending\nReview".into()
      ] )
      .add_row( vec![
        "3".into(),
        "Another multiline description here\nWith a second line".into(),
        "Done".into()
      ] )
      .build();

    let config = TableConfig::plain()
      .max_column_width( Some( 25 ) );

    let formatter = TableFormatter::with_config( config );
    let output = formatter.format( &data );

    println!( "{output}" );
    println!( "Expected:" );
    println!( "  - ID column: short, no truncation" );
    println!( "  - Description column: multiline with per-line truncation" );
    println!( "  - Status column: multiline, no truncation needed" );
    println!( "  - Row heights vary (1, 3, 2)" );
  }
  println!( "└───────────────────────────────────────────────────────────────────┘\n" );

  // Edge case: Every cell multiline + truncated
  println!( "┌─ Edge Case: All Cells Multiline + Truncated ─────────────────────┐" );
  {
    let data = RowBuilder::new( vec![
      "Column One With Long Name".into(),
      "Column Two Also Long".into()
    ] )
      .add_row( vec![
        "First long line here\nSecond long line here\nThird".into(),
        "Another long line\nAnd another one\nShort".into()
      ] )
      .build();

    let config = TableConfig::plain()
      .max_column_width( Some( 20 ) );

    let formatter = TableFormatter::with_config( config );
    let output = formatter.format( &data );

    println!( "{output}" );
    println!( "Expected:" );
    println!( "  - Headers truncated" );
    println!( "  - Every data line independently truncated" );
    println!( "  - Row height = 3" );
    println!( "  - All alignment maintained" );
  }
  println!( "└───────────────────────────────────────────────────────────────────┘\n" );

  // Real-world scenario: Log messages with stack traces
  println!( "┌─ Real-World: Log Messages with Stack Traces ─────────────────────┐" );
  {
    let data = RowBuilder::new( vec![ "Time".into(), "Level".into(), "Message".into() ] )
      .add_row( vec![
        "10:30:01".into(),
        "\x1b[32mINFO\x1b[0m".into(),
        "Application started successfully".into()
      ] )
      .add_row( vec![
        "10:30:15".into(),
        "\x1b[31mERROR\x1b[0m".into(),
        "Failed to connect to database\nStack trace:\n  at Database.connect() line 42\n  at main() line 10".into()
      ] )
      .add_row( vec![
        "10:30:20".into(),
        "\x1b[33mWARN\x1b[0m".into(),
        "Retrying connection (attempt 1 of 3)".into()
      ] )
      .build();

    let config = TableConfig::plain()
      .max_column_width( Some( 40 ) );

    let formatter = TableFormatter::with_config( config );
    let output = formatter.format( &data );

    println!( "{output}" );
    println!( "Expected:" );
    println!( "  - Time column: no truncation" );
    println!( "  - Level column: colors preserved" );
    println!( "  - Message column: multiline for error with stack trace" );
    println!( "  - Long lines truncated appropriately" );
    println!( "  - Real-world usability demonstrated" );
  }
  println!( "└───────────────────────────────────────────────────────────────────┘\n" );

  // Bordered style with combined features
  println!( "┌─ Visual: Bordered Style with Both Features ──────────────────────┐" );
  {
    let data = RowBuilder::new( vec![ "Name".into(), "Address".into() ] )
      .add_row( vec![
        "Alice Johnson".into(),
        "123 Very Long Street Name\nApartment 4B\nBoston, MA 02101".into()
      ] )
      .add_row( vec![
        "Bob".into(),
        "456 Oak St".into()
      ] )
      .build();

    let config = TableConfig::bordered()
      .max_column_width( Some( 20 ) );

    let formatter = TableFormatter::with_config( config );
    let output = formatter.format( &data );

    println!( "{output}" );
    println!( "Expected:" );
    println!( "  - Borders align correctly with multiline" );
    println!( "  - Truncated lines maintain border alignment" );
    println!( "  - Mixed heights handled gracefully" );
  }
  println!( "└───────────────────────────────────────────────────────────────────┘\n" );

  println!( "╔═══════════════════════════════════════════════════════════════════╗" );
  println!( "║  TEST COMPLETE - Review output above and mark results in         ║" );
  println!( "║  tests/manual/readme.md                                           ║" );
  println!( "╚═══════════════════════════════════════════════════════════════════╝" );
}
