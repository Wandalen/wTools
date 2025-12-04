//! Manual Test 002: Multiline Cells Feature
//!
//! This example tests multiline cell rendering for manual verification.
//! Run with: `cargo run --example manual_test_002_multiline`

use tree_fmt::{ RowBuilder, TableFormatter, TableConfig };

#[ allow( clippy::too_many_lines ) ]
fn main()
{
  println!( "╔═══════════════════════════════════════════════════════════════════╗" );
  println!( "║  MANUAL TEST 002: Multiline Cells                                 ║" );
  println!( "╚═══════════════════════════════════════════════════════════════════╝\n" );

  // TC-006: Basic Multiline Cell
  println!( "┌─ TC-006: Basic Multiline Cell ───────────────────────────────────┐" );
  {
    let data = RowBuilder::new( vec![ "Address".into() ] )
      .add_row( vec![ "123 Main St\nApt 4B\nBoston, MA 02101".into() ] )
      .build();

    let config = TableConfig::plain();
    let formatter = TableFormatter::with_config( config );
    let output = formatter.format( &data );

    println!( "{output}" );
    println!( "Expected:" );
    println!( "  - Three visual lines in table" );
    println!( "  - Column separators maintained on each line" );
    println!( "  - Proper alignment" );
  }
  println!( "└───────────────────────────────────────────────────────────────────┘\n" );

  // TC-007: Mixed Heights
  println!( "┌─ TC-007: Multiline Row with Mixed Heights ───────────────────────┐" );
  {
    let data = RowBuilder::new( vec![ "Col1".into(), "Col2".into(), "Col3".into() ] )
      .add_row( vec![ "A".into(), "B\nC\nD".into(), "E".into() ] )
      .build();

    let config = TableConfig::plain();
    let formatter = TableFormatter::with_config( config );
    let output = formatter.format( &data );

    println!( "{output}" );
    println!( "Expected:" );
    println!( "  - Row height = 3 (max lines in any cell)" );
    println!( "  - 'A' and 'E' padded to 3 lines" );
    println!( "  - Vertical alignment of column separators" );
  }
  println!( "└───────────────────────────────────────────────────────────────────┘\n" );

  // TC-008: Multiline with ANSI
  println!( "┌─ TC-008: Multiline with ANSI Colors ─────────────────────────────┐" );
  {
    let data = RowBuilder::new( vec![ "Status".into() ] )
      .add_row( vec![ "\x1b[31mError\x1b[0m\n\x1b[33mWarning\x1b[0m\n\x1b[32mSuccess\x1b[0m".into() ] )
      .build();

    let config = TableConfig::plain();
    let formatter = TableFormatter::with_config( config );
    let output = formatter.format( &data );

    println!( "{output}" );
    println!( "Expected:" );
    println!( "  - Red 'Error' on line 1" );
    println!( "  - Yellow 'Warning' on line 2" );
    println!( "  - Green 'Success' on line 3" );
    println!( "Visual check: Colors should render correctly" );
  }
  println!( "└───────────────────────────────────────────────────────────────────┘\n" );

  // TC-009: Empty Lines
  println!( "┌─ TC-009: Empty Lines in Multiline Cell ──────────────────────────┐" );
  {
    let data = RowBuilder::new( vec![ "Content".into() ] )
      .add_row( vec![ "Line1\n\nLine3".into() ] )
      .build();

    let config = TableConfig::plain();
    let formatter = TableFormatter::with_config( config );
    let output = formatter.format( &data );

    println!( "{output}" );
    println!( "Expected:" );
    println!( "  - Three lines rendered" );
    println!( "  - Middle line empty but space preserved" );
    println!( "  - No collapsing of empty lines" );
  }
  println!( "└───────────────────────────────────────────────────────────────────┘\n" );

  // TC-012: CSV Format (Multiline Disabled)
  println!( "┌─ TC-012: Multiline in CSV Format (should be literal) ────────────┐" );
  {
    let data = RowBuilder::new( vec![ "Name".into(), "Address".into() ] )
      .add_row( vec![ "Alice".into(), "123 Main\nApt 4B".into() ] )
      .build();

    let config = TableConfig::csv();
    let formatter = TableFormatter::with_config( config );
    let output = formatter.format( &data );

    println!( "{output}" );
    println!( "Expected:" );
    println!( "  - Output contains literal \\n characters" );
    println!( "  - No actual line break in CSV" );
    println!( "  - CSV remains single-line per record" );
  }
  println!( "└───────────────────────────────────────────────────────────────────┘\n" );

  // Multiple rows with varying heights
  println!( "┌─ Additional: Multiple Rows with Varying Heights ─────────────────┐" );
  {
    let data = RowBuilder::new( vec![ "Name".into(), "Details".into(), "Status".into() ] )
      .add_row( vec![ "Item 1".into(), "Single line".into(), "OK".into() ] )
      .add_row( vec![ "Item 2".into(), "Line 1\nLine 2\nLine 3".into(), "Pending\nReview".into() ] )
      .add_row( vec![ "Item 3".into(), "Two lines\nHere".into(), "Done".into() ] )
      .build();

    let config = TableConfig::plain();
    let formatter = TableFormatter::with_config( config );
    let output = formatter.format( &data );

    println!( "{output}" );
    println!( "Expected:" );
    println!( "  - Row 1: height=1 (all single line)" );
    println!( "  - Row 2: height=3 (max from Details column)" );
    println!( "  - Row 3: height=2 (max from Details column)" );
    println!( "  - Each row independently sized" );
  }
  println!( "└───────────────────────────────────────────────────────────────────┘\n" );

  // Bordered style with multiline
  println!( "┌─ VI-003: Table Border Alignment (Visual Inspection) ─────────────┐" );
  {
    let data = RowBuilder::new( vec![ "Name".into(), "Info".into() ] )
      .add_row( vec![ "Alice".into(), "Line 1\nLine 2".into() ] )
      .add_row( vec![ "Bob".into(), "Single".into() ] )
      .build();

    let config = TableConfig::bordered();
    let formatter = TableFormatter::with_config( config );
    let output = formatter.format( &data );

    println!( "{output}" );
    println!( "Expected:" );
    println!( "  - Borders align correctly across all lines" );
    println!( "  - Corner characters position correctly" );
    println!( "  - No broken border segments" );
  }
  println!( "└───────────────────────────────────────────────────────────────────┘\n" );

  // Markdown style with multiline
  println!( "┌─ VI-004: Markdown Table Format (Visual Inspection) ──────────────┐" );
  {
    let data = RowBuilder::new( vec![ "Task".into(), "Description".into() ] )
      .add_row( vec![ "Task 1".into(), "First line\nSecond line".into() ] )
      .add_row( vec![ "Task 2".into(), "Single".into() ] )
      .build();

    let config = TableConfig::markdown();
    let formatter = TableFormatter::with_config( config );
    let output = formatter.format( &data );

    println!( "{output}" );
    println!( "Expected:" );
    println!( "  - Valid markdown syntax" );
    println!( "  - Pipe characters (|) align correctly" );
    println!( "  - Separator line (-) formatted correctly" );
  }
  println!( "└───────────────────────────────────────────────────────────────────┘\n" );

  println!( "╔═══════════════════════════════════════════════════════════════════╗" );
  println!( "║  TEST COMPLETE - Review output above and mark results in         ║" );
  println!( "║  tests/manual/readme.md                                           ║" );
  println!( "╚═══════════════════════════════════════════════════════════════════╝" );
}
