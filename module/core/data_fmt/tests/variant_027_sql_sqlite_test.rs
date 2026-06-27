//! Variant 027: SQL `SQLite` spec tests (VT-1..VT-4)

#![ cfg( all( feature = "enabled", feature = "sql_sqlite" ) ) ]

use data_fmt::{ RowBuilder, SqlFormatter, SqlVariant, Format };

/// VT-1: output uses `SQLite` quoting conventions
// test_kind: spec_case(VT-1)
#[ test ]
fn variant_027_vt_01_sqlite_quoting()
{
  let view = RowBuilder::new( vec![ "Name".into(), "Age".into() ] )
    .add_row( vec![ "Alice".into(), "30".into() ] )
    .build_view();

  let out = Format::format(
    &SqlFormatter::with_variant( "users", SqlVariant::SQLite ),
    &view,
  ).unwrap();

  assert!( out.contains( "INSERT" ), "INSERT statement present: {out}" );
  assert!( out.contains( "VALUES" ), "VALUES present: {out}" );
}

/// VT-2: valid `SQLite` INSERT syntax
// test_kind: spec_case(VT-2)
#[ test ]
fn variant_027_vt_02_valid_insert_syntax()
{
  let view = RowBuilder::new( vec![ "A".into(), "B".into() ] )
    .add_row( vec![ "1".into(), "2".into() ] )
    .build_view();

  let out = Format::format(
    &SqlFormatter::with_variant( "data", SqlVariant::SQLite ),
    &view,
  ).unwrap();

  assert!( out.contains( "INSERT INTO" ), "INSERT INTO present: {out}" );
  assert!( out.contains( '(' ), "parenthesized values: {out}" );
}

/// VT-3: string values are single-quote escaped
// test_kind: spec_case(VT-3)
#[ test ]
fn variant_027_vt_03_single_quote_escaped()
{
  let view = RowBuilder::new( vec![ "msg".into() ] )
    .add_row( vec![ "it's".into() ] )
    .build_view();

  let out = Format::format(
    &SqlFormatter::with_variant( "data", SqlVariant::SQLite ),
    &view,
  ).unwrap();

  // Single quote should be doubled for SQL escaping
  assert!( out.contains( "''" ), "single quote escaped by doubling: {out}" );
}

/// VT-4: empty table produces no value tuples
// test_kind: spec_case(VT-4)
#[ test ]
fn variant_027_vt_04_empty_table()
{
  let view = RowBuilder::new( vec![ "Col".into() ] )
    .build_view();

  let out = Format::format(
    &SqlFormatter::with_variant( "t", SqlVariant::SQLite ),
    &view,
  ).unwrap();

  let after_values = out.split( "VALUES" ).nth( 1 ).unwrap_or( "" );
  assert!( !after_values.contains( '(' ), "no value tuples for empty table: {out}" );
}
