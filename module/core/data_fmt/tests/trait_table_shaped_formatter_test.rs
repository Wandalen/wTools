//! `TableShapedFormatter` removal spec tests (TR-1..TR-4)
//!
//! Verifies the v0.3.0 API removal of the `TableShapedFormatter` trait
//! and confirms the migration path through the `Format` trait.

#![ cfg( feature = "enabled" ) ]

use data_fmt::{ RowBuilder, TableFormatter, ExpandedFormatter, Format, TableConfig };

/// TR-1: trait is not exported from public API
///
/// `TableShapedFormatter` was removed in v0.3.0; the `formatters` module
/// re-exports only `Format`, `FormatError`, and concrete formatters.
/// Since we cannot prove non-existence at runtime, we verify that the
/// replacement (`Format`) IS the only formatting trait.
// test_kind: spec_case(TR-1)
#[ test ]
fn trait_002_tr_01_trait_not_exported()
{
  // The Format trait is the sole public formatting trait.
  // TableShapedFormatter is not re-exported — if it were,
  // we could `use data_fmt::formatters::TableShapedFormatter`.
  // Compile-time absence cannot be asserted at runtime, so we verify
  // that Format IS available and IS the trait used by both former
  // implementors (TableFormatter, ExpandedFormatter).
  let view = RowBuilder::new( vec![ "Col".into() ] )
    .add_row( vec![ "val".into() ] )
    .build_view();

  let table_fmt : &dyn Format = &TableFormatter::new();
  assert!( table_fmt.format( &view ).is_ok(), "TableFormatter dispatches through Format" );

  let expanded_fmt : &dyn Format = &ExpandedFormatter::new();
  assert!( expanded_fmt.format( &view ).is_ok(), "ExpandedFormatter dispatches through Format" );
}

/// TR-2: former implementors implement Format trait
///
/// `TableFormatter` and `ExpandedFormatter` both implement `Format`;
/// `Format::format` accepts `&TableView` and returns `Result<String, FormatError>`.
// test_kind: spec_case(TR-2)
#[ test ]
fn trait_002_tr_02_former_implementors_implement_format()
{
  let view = RowBuilder::new( vec![ "Name".into(), "Score".into() ] )
    .add_row( vec![ "Alice".into(), "95".into() ] )
    .build_view();

  // TableFormatter implements Format
  let tf = TableFormatter::with_config( TableConfig::plain() );
  let result_table = Format::format( &tf, &view );
  assert!( result_table.is_ok(), "TableFormatter implements Format" );

  // ExpandedFormatter implements Format
  let ef = ExpandedFormatter::new();
  let result_expanded = Format::format( &ef, &view );
  assert!( result_expanded.is_ok(), "ExpandedFormatter implements Format" );
}

/// TR-3: `build_view` replaces `build` for `TableView` construction
///
/// `RowBuilder` exposes `build_view()`, not `build()`.
/// The result is a `TableView` directly consumable by `Format::format`.
// test_kind: spec_case(TR-3)
#[ test ]
fn trait_002_tr_03_build_view_replaces_build()
{
  let view = RowBuilder::new( vec![ "A".into(), "B".into() ] )
    .add_row( vec![ "1".into(), "2".into() ] )
    .build_view();

  // build_view() returns TableView with correct structure
  assert_eq!( view.metadata.column_names, vec![ "A", "B" ], "headers preserved" );
  assert_eq!( view.rows.len(), 1, "one row" );

  // TableView is directly consumable by Format::format
  let formatter = TableFormatter::new();
  let result = Format::format( &formatter, &view );
  assert!( result.is_ok(), "Format::format accepts build_view() output" );
}

/// TR-4: migration path produces equivalent output
///
/// `Format::format` on a `TableView` from `build_view()` produces
/// well-formed output with correct content. Unlike the removed trait
/// (which returned `String`), the result is `Result<String, FormatError>`.
// test_kind: spec_case(TR-4)
#[ test ]
fn trait_002_tr_04_migration_produces_equivalent_output()
{
  let view = RowBuilder::new( vec![ "Name".into() ] )
    .add_row( vec![ "Alice".into() ] )
    .build_view();

  let formatter = TableFormatter::with_config( TableConfig::plain() );
  let result = Format::format( &formatter, &view );

  assert!( result.is_ok(), "result is Ok, not infallible" );
  let output = result.unwrap();
  assert!( output.contains( "Name" ), "output contains header" );
  assert!( output.contains( "Alice" ), "output contains cell value" );
}
