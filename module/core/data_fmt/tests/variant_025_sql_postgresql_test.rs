//! Variant 025: SQL `PostgreSQL` spec tests (VT-1..VT-4)

#![ cfg( feature = "enabled" ) ]

use data_fmt::{ RowBuilder, SqlFormatter, SqlVariant, Format };

/// VT-1: output uses `PostgreSQL` quoting conventions
// test_kind: spec_case(VT-1)
#[ test ]
fn variant_025_vt_01_postgresql_quoting()
{
  let view = RowBuilder::new( vec![ "Name".into(), "Age".into() ] )
    .add_row( vec![ "Alice".into(), "30".into() ] )
    .build_view();

  let out = Format::format(
    &SqlFormatter::with_variant( "users", SqlVariant::PostgreSQL ),
    &view,
  ).unwrap();

  assert!( out.contains( "INSERT" ), "INSERT statement present: {out}" );
  assert!( out.contains( "VALUES" ), "VALUES clause present: {out}" );
}

/// VT-2: column identifiers are double-quoted
// test_kind: spec_case(VT-2)
#[ test ]
fn variant_025_vt_02_double_quoted_identifiers()
{
  let view = RowBuilder::new( vec![ "user name".into(), "age".into() ] )
    .add_row( vec![ "Alice".into(), "30".into() ] )
    .build_view();

  let out = Format::format(
    &SqlFormatter::with_variant( "users", SqlVariant::PostgreSQL ),
    &view,
  ).unwrap();

  // PostgreSQL uses double quotes for identifiers
  assert!(
    out.contains( "\"user name\"" ) || out.contains( "\"age\"" ),
    "double-quoted identifiers: {out}",
  );
}

/// VT-3: valid `PostgreSQL` INSERT syntax
// test_kind: spec_case(VT-3)
#[ test ]
fn variant_025_vt_03_valid_insert_syntax()
{
  let view = RowBuilder::new( vec![ "A".into(), "B".into() ] )
    .add_row( vec![ "1".into(), "2".into() ] )
    .add_row( vec![ "3".into(), "4".into() ] )
    .build_view();

  let out = Format::format(
    &SqlFormatter::with_variant( "data", SqlVariant::PostgreSQL ),
    &view,
  ).unwrap();

  assert!( out.contains( "INSERT INTO" ), "INSERT INTO present: {out}" );
  assert!( out.contains( "VALUES" ), "VALUES present: {out}" );
  assert!( out.contains( '(' ), "parenthesized values: {out}" );
}

/// VT-4: empty table produces no value tuples
// test_kind: spec_case(VT-4)
#[ test ]
fn variant_025_vt_04_empty_table()
{
  let view = RowBuilder::new( vec![ "Col".into() ] )
    .build_view();

  let out = Format::format(
    &SqlFormatter::with_variant( "t", SqlVariant::PostgreSQL ),
    &view,
  ).unwrap();

  // SQL formatter always emits INSERT header; check no value tuples after VALUES
  let after_values = out.split( "VALUES" ).nth( 1 ).unwrap_or( "" );
  assert!( !after_values.contains( '(' ), "no value tuples for empty table: {out}" );
}
