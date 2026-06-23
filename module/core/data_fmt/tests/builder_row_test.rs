//! `RowBuilder` spec tests (BL-1..BL-8)
//!
//! Covers construction, empty tables, round-trip, fluent chaining,
//! mutable builder, detail annotations, named rows, and panic on mismatch.

#![ cfg( feature = "enabled" ) ]

use data_fmt::{ RowBuilder, DecoratedText };

/// BL-1: basic single-row construction
// test_kind: spec_case(BL-1)
#[ test ]
fn builder_001_bl_01_basic_single_row_construction()
{
  let view = RowBuilder::new( vec![ "Name".into(), "Age".into() ] )
    .add_row( vec![ "Alice".into(), "30".into() ] )
    .build_view();

  assert_eq!( view.rows.len(), 1, "expected exactly 1 row" );
  assert_eq!( view.rows[ 0 ][ 0 ].render(), "Alice", "first cell should be Alice" );
  assert_eq!( view.rows[ 0 ][ 1 ].render(), "30", "second cell should be 30" );
  assert_eq!( view.row_details[ 0 ], None, "detail should be None for plain add_row" );
}

/// BL-2: empty table with zero rows
// test_kind: spec_case(BL-2)
#[ test ]
fn builder_001_bl_02_empty_table_with_zero_rows()
{
  let view = RowBuilder::new( vec![ "X".into(), "Y".into() ] )
    .build_view();

  assert!( view.rows.is_empty(), "rows should be empty" );
  assert!( view.row_details.is_empty(), "row_details should be empty" );
  assert_eq!( view.metadata.column_names, vec![ "X", "Y" ], "headers should be preserved" );
}

/// BL-3: headers and multiple rows round-trip
// test_kind: spec_case(BL-3)
#[ test ]
fn builder_001_bl_03_headers_and_multiple_rows_round_trip()
{
  let view = RowBuilder::new( vec![ "A".into(), "B".into(), "C".into() ] )
    .add_row( vec![ "1".into(), "2".into(), "3".into() ] )
    .add_row( vec![ "4".into(), "5".into(), "6".into() ] )
    .add_row( vec![ "7".into(), "8".into(), "9".into() ] )
    .build_view();

  assert_eq!( view.rows.len(), 3, "expected 3 rows" );
  for row in &view.rows
  {
    assert_eq!( row.len(), 3, "each row should have 3 cells" );
  }
  assert_eq!( view.rows[ 0 ][ 0 ].render(), "1" );
  assert_eq!( view.rows[ 1 ][ 1 ].render(), "5" );
  assert_eq!( view.rows[ 2 ][ 2 ].render(), "9" );
  assert_eq!( view.row_details.len(), 3, "row_details should have 3 entries" );
  assert!( view.row_details.iter().all( Option::is_none ), "all details should be None" );
}

/// BL-4: fluent chaining preserves insertion order
// test_kind: spec_case(BL-4)
#[ test ]
fn builder_001_bl_04_fluent_chaining_preserves_insertion_order()
{
  let view = RowBuilder::new( vec![ "Col".into() ] )
    .add_row( vec![ "a".into() ] )
    .add_row( vec![ "b".into() ] )
    .add_row( vec![ "c".into() ] )
    .add_row( vec![ "d".into() ] )
    .add_row( vec![ "e".into() ] )
    .build_view();

  let values : Vec< String > = view.rows.iter()
    .map( | r | r[ 0 ].render() )
    .collect();
  assert_eq!( values, vec![ "a", "b", "c", "d", "e" ], "insertion order must be preserved" );
}

/// BL-5: `add_row_mut` in a loop
// test_kind: spec_case(BL-5)
#[ test ]
fn builder_001_bl_05_add_row_mut_in_a_loop()
{
  let mut builder = RowBuilder::new( vec![ "Value".into() ] );
  for i in 0..10
  {
    builder.add_row_mut( vec![ i.to_string().into() ] );
  }
  let view = builder.build_view();

  assert_eq!( view.rows.len(), 10, "expected 10 rows" );
  assert_eq!( view.rows[ 0 ][ 0 ].render(), "0", "first row value" );
  assert_eq!( view.rows[ 9 ][ 0 ].render(), "9", "last row value" );
}

/// BL-6: row with detail annotation
// test_kind: spec_case(BL-6)
#[ test ]
fn builder_001_bl_06_row_with_detail_annotation()
{
  let view = RowBuilder::new( vec![ "Name".into() ] )
    .add_row_with_detail( vec![ "Alice".into() ], Some( "extra info".into() ) )
    .add_row( vec![ "Bob".into() ] )
    .build_view();

  assert_eq!(
    view.row_details[ 0 ],
    Some( DecoratedText::from( "extra info" ) ),
    "first row should have detail annotation",
  );
  assert_eq!( view.row_details[ 1 ], None, "second row should have no detail" );
  assert_eq!( view.rows[ 0 ][ 0 ].render(), "Alice" );
  assert_eq!( view.rows[ 1 ][ 0 ].render(), "Bob" );
}

/// BL-7: named rows via `add_row_with_name`
// test_kind: spec_case(BL-7)
#[ test ]
fn builder_001_bl_07_named_rows_via_add_row_with_name()
{
  let view = RowBuilder::new( vec![ "Score".into() ] )
    .add_row_with_name( "Alice".into(), vec![ "95".into() ] )
    .add_row_with_name( "Bob".into(), vec![ "87".into() ] )
    .build_view();

  assert_eq!( view.rows.len(), 2, "expected 2 rows" );
  assert_eq!( view.rows[ 0 ][ 0 ].render(), "95" );
  assert_eq!( view.rows[ 1 ][ 0 ].render(), "87" );
}

/// BL-8: mismatched row length panics
// test_kind: spec_case(BL-8)
#[ test ]
#[ should_panic( expected = "row length 3 doesn't match headers length 2" ) ]
fn builder_001_bl_08_mismatched_row_length_panics()
{
  let _ = RowBuilder::new( vec![ "A".into(), "B".into() ] )
    .add_row( vec![ "1".into(), "2".into(), "3".into() ] );
}
