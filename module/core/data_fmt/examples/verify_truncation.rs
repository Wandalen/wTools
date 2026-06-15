//! Visual Verification: Column Truncation Feature
//!
//! This example tests various column truncation scenarios for manual verification.
//! Run with: `cargo run --example verify_truncation`

#[ cfg( not( feature = "enabled" ) ) ]
fn main() {}

#[ cfg( feature = "enabled" ) ]
#[ allow( clippy::too_many_lines ) ]
fn main()
{
  use data_fmt::{ RowBuilder, TableFormatter, TableConfig, Format };
  println!( "╔═══════════════════════════════════════════════════════════════════╗" );
  println!( "║  MANUAL TEST 001: Column Truncation                               ║" );
  println!( "╚═══════════════════════════════════════════════════════════════════╝\n" );

  // TC-001: Basic Column Truncation
  println!( "┌─ TC-001: Basic Column Truncation (max_width=20) ─────────────────┐" );
  {
    let data = RowBuilder::new( vec![ "Description".into() ] )
      .add_row( vec![ "Short".into() ] )
      .add_row( vec![ "This is a very long description that exceeds twenty characters".into() ] )
      .add_row( vec![ "Exactly20Characters!".into() ] )
      .build_view();

    let config = TableConfig::plain()
      .with_max_column_width( Some( 20 ) );

    let formatter = TableFormatter::with_config( config );
    let output = formatter.format( &data ).unwrap_or_default();

    println!( "{output}" );
    println!( "Expected:" );
    println!( "  - 'Short' displayed fully (no truncation)" );
    println!( "  - Long text truncated to 17 chars + '...' = 20 total" );
    println!( "  - Exact 20 chars displayed fully (no truncation)" );
  }
  println!( "└───────────────────────────────────────────────────────────────────┘\n" );

  // TC-002: ANSI Code Preservation
  println!( "┌─ TC-002: ANSI Code Preservation in Truncation ───────────────────┐" );
  {
    let data = RowBuilder::new( vec![ "Colored Text".into() ] )
      .add_row( vec![ "\x1b[31mThis is red text that is very long and should be truncated\x1b[0m".into() ] )
      .add_row( vec![ "\x1b[32mShort green\x1b[0m".into() ] )
      .add_row( vec![ "\x1b[34mBlue text here is also quite long and needs truncation applied\x1b[0m".into() ] )
      .build_view();

    let config = TableConfig::plain()
      .with_max_column_width( Some( 25 ) );

    let formatter = TableFormatter::with_config( config );
    let output = formatter.format( &data ).unwrap_or_default();

    println!( "{output}" );
    println!( "Expected:" );
    println!( "  - Red text: truncated but color preserved" );
    println!( "  - Green text: displayed fully with color" );
    println!( "  - Blue text: truncated but color preserved" );
    println!( "Visual check: Colors should render correctly in terminal" );
  }
  println!( "└───────────────────────────────────────────────────────────────────┘\n" );

  // TC-003: Custom Truncation Marker
  println!( "┌─ TC-003: Custom Truncation Marker ───────────────────────────────┐" );
  {
    let data = RowBuilder::new( vec![ "Content".into() ] )
      .add_row( vec![ "Very long content that will be truncated with custom marker".into() ] )
      .build_view();

    let config = TableConfig::plain()
      .with_max_column_width( Some( 20 ) )
      .with_truncation_marker( "→".to_string() );

    let formatter = TableFormatter::with_config( config );
    let output = formatter.format( &data ).unwrap_or_default();

    println!( "{output}" );
    println!( "Expected:" );
    println!( "  - Content truncated to 19 chars + '→' = 20 total" );
    println!( "  - Arrow marker visible at end" );
  }
  println!( "└───────────────────────────────────────────────────────────────────┘\n" );

  // TC-004: Marker Longer Than Limit
  println!( "┌─ TC-004: Marker Longer Than Limit (edge case) ───────────────────┐" );
  {
    let data = RowBuilder::new( vec![ "Text".into() ] )
      .add_row( vec![ "Content".into() ] )
      .build_view();

    let config = TableConfig::plain()
      .with_max_column_width( Some( 5 ) )
      .with_truncation_marker( ".........".to_string() );

    let formatter = TableFormatter::with_config( config );
    let output = formatter.format( &data ).unwrap_or_default();

    println!( "{output}" );
    println!( "Expected:" );
    println!( "  - Graceful handling (saturating_sub)" );
    println!( "  - No panic or error" );
    println!( "  - Some reasonable output produced" );
  }
  println!( "└───────────────────────────────────────────────────────────────────┘\n" );

  // TC-005: Exact Fit
  println!( "┌─ TC-005: Exact Fit (no truncation at limit) ─────────────────────┐" );
  {
    let data = RowBuilder::new( vec![ "Column".into() ] )
      .add_row( vec![ "ExactlyTwentyChars!!".into() ] )      // 20 chars
      .add_row( vec![ "TwentyOneCharacters!!".into() ] )     // 21 chars (added one !)
      .add_row( vec![ "NineteenCharacter!!".into() ] )       // 19 chars
      .build_view();

    let config = TableConfig::plain()
      .with_max_column_width( Some( 20 ) );

    let formatter = TableFormatter::with_config( config );
    let output = formatter.format( &data ).unwrap_or_default();

    println!( "{output}" );
    println!( "Expected:" );
    println!( "  - 20 chars: displayed fully (no marker)" );
    println!( "  - 21 chars: truncated to 17 + '...' " );
    println!( "  - 19 chars: displayed fully (no marker)" );
  }
  println!( "└───────────────────────────────────────────────────────────────────┘\n" );

  // TC-013: Backward Compatibility
  println!( "┌─ TC-013: Backward Compatibility (no truncation config) ──────────┐" );
  {
    let data = RowBuilder::new( vec![ "Description".into() ] )
      .add_row( vec![ "This is a very long description that would normally be truncated but should display fully without config".into() ] )
      .build_view();

    let config = TableConfig::plain();
    let formatter = TableFormatter::with_config( config );
    let output = formatter.format( &data ).unwrap_or_default();

    println!( "{output}" );
    println!( "Expected:" );
    println!( "  - Full content displayed" );
    println!( "  - No truncation applied" );
    println!( "  - Identical to v0.4.0 behavior" );
  }
  println!( "└───────────────────────────────────────────────────────────────────┘\n" );

  // TC-014: Multiple Columns
  println!( "┌─ TC-014: Multiple Columns with Truncation ───────────────────────┐" );
  {
    let data = RowBuilder::new( vec![ "Name".into(), "Description".into(), "Status".into() ] )
      .add_row( vec![ "Alice".into(), "Short desc".into(), "Active".into() ] )
      .add_row( vec![ "Bob".into(), "Very long description that exceeds limit".into(), "Inactive today".into() ] )
      .add_row( vec![ "Charlie".into(), "Another long description here too".into(), "OK".into() ] )
      .build_view();

    let config = TableConfig::plain()
      .with_max_column_width( Some( 15 ) );

    let formatter = TableFormatter::with_config( config );
    let output = formatter.format( &data ).unwrap_or_default();

    println!( "{output}" );
    println!( "Expected:" );
    println!( "  - Name column: short names displayed fully" );
    println!( "  - Description column: long texts truncated independently" );
    println!( "  - Status column: mix of full and truncated" );
  }
  println!( "└───────────────────────────────────────────────────────────────────┘\n" );

  // TC-015: Headers with Truncation
  println!( "┌─ TC-015: Headers with Truncation ─────────────────────────────────┐" );
  {
    let data = RowBuilder::new( vec![ "VeryLongHeaderNameThatExceedsLimit".into(), "Short".into() ] )
      .add_row( vec![ "Data 1".into(), "Value 1".into() ] )
      .add_row( vec![ "Data 2".into(), "Value 2".into() ] )
      .build_view();

    let config = TableConfig::plain()
      .with_max_column_width( Some( 20 ) );

    let formatter = TableFormatter::with_config( config );
    let output = formatter.format( &data ).unwrap_or_default();

    println!( "{output}" );
    println!( "Expected:" );
    println!( "  - Long header truncated to 20 chars" );
    println!( "  - Short header displayed fully" );
    println!( "  - Data cells also respect limit" );
  }
  println!( "└───────────────────────────────────────────────────────────────────┘\n" );

  println!( "╔═══════════════════════════════════════════════════════════════════╗" );
  println!( "║  TEST COMPLETE - Review output above and mark results in         ║" );
  println!( "║  tests/manual/readme.md                                           ║" );
  println!( "╚═══════════════════════════════════════════════════════════════════╝" );
}
