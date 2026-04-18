//! Sub-row detail lines: extra context below each table row
//!
//! Use when: Tabular data where rows need supplementary detail that
//! doesn't fit as a column — error messages, notes, log excerpts, etc.
//! Output: Table rows followed by indented detail line (optional per row)
//!
//! Run: `cargo run --example sub_row_detail`

use data_fmt::{ RowBuilder, TableFormatter, Format, DecoratedText };

fn main()
{
  println!( "=== Sub-row detail lines ===" );
  println!( "Best for: Per-row context that doesn't belong in a column\n" );

  let fmt = TableFormatter::new();

  // --- Basic: mix of rows with and without detail -------------------------

  println!( "-- Basic: mixed detail --" );

  let view = RowBuilder::new( vec![
    "Service".into(),
    "Status".into(),
    "Latency".into(),
  ])
  .add_row_with_detail(
    vec![ "auth".into(), "ERROR".into(), "n/a".into() ],
    Some( "connection refused: 10.0.1.5:5432".into() ),
  )
  .add_row(    vec![ "api-gw".into(),  "OK".into(), "12ms".into() ] )
  .add_row_with_detail(
    vec![ "worker".into(), "WARN".into(),  "340ms".into() ],
    Some( "queue depth 4 821 — consider scaling".into() ),
  )
  .add_row(    vec![ "cache".into(),   "OK".into(), "1ms".into() ] )
  .build_view();

  println!( "{}", Format::format( &fmt, &view ).unwrap() );

  // --- All rows annotated -------------------------------------------------

  println!( "-- All rows annotated --" );

  let view2 = RowBuilder::new( vec![ "File".into(), "Result".into() ] )
  .add_row_with_detail(
    vec![ "main.rs".into(),   "ok".into() ],
    Some( "compiled in 0.4s".into() ),
  )
  .add_row_with_detail(
    vec![ "parser.rs".into(), "FAIL".into() ],
    Some( "error[E0308] mismatched types at line 42".into() ),
  )
  .add_row_with_detail(
    vec![ "tests.rs".into(),  "ok".into() ],
    Some( "3 tests passed".into() ),
  )
  .build_view();

  println!( "{}", Format::format( &fmt, &view2 ).unwrap() );

  // --- No details (feature is transparent when unused) --------------------

  println!( "-- No details (normal table, feature transparent) --" );

  let view3 = RowBuilder::new( vec![ "Name".into(), "Score".into() ] )
  .add_row( vec![ "Alice".into(), "92".into() ] )
  .add_row( vec![ "Bob".into(),   "78".into() ] )
  .build_view();

  println!( "{}", Format::format( &fmt, &view3 ).unwrap() );

  // --- Colored details (ANSI highlight per row) --------------------------------

  println!( "-- Colored details (ANSI yellow/red per row) --" );

  let red    = "\x1b[31m";
  let yellow = "\x1b[33m";

  let view4 = RowBuilder::new( vec![ "Service".into(), "Status".into() ] )
  .add_row_with_detail(
    vec![ "auth".into(), "ERROR".into() ],
    Some( DecoratedText::from( "connection refused: 10.0.1.5:5432" ).with_color( red ) ),
  )
  .add_row(
    vec![ "api-gw".into(), "OK".into() ],
  )
  .add_row_with_detail(
    vec![ "worker".into(), "WARN".into() ],
    Some( DecoratedText::from( "queue depth 4821 — consider scaling" ).with_color( yellow ) ),
  )
  .build_view();

  println!( "{}", Format::format( &fmt, &view4 ).unwrap() );

  println!( "✓ Characteristics:" );
  println!( "  • Detail is per-row optional — rows without detail render normally" );
  println!( "  • Detail line is indented 2 spaces beneath the row" );
  println!( "  • Detail never gets column separators or borders" );
  println!( "  • Compatible with all TableFormatter configs (grid, plain, etc.)" );
}
