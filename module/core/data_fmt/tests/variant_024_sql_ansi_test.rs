//! Variant 024: SQL ANSI spec tests (VT-1..VT-4)

#![ cfg( all( feature = "enabled", feature = "sql_ansi" ) ) ]

use data_fmt::{ RowBuilder, SqlFormatter, SqlVariant, Format };

fn sql_ansi_formatter() -> SqlFormatter
{
  SqlFormatter::with_variant( "test_table", SqlVariant::Ansi )
}

/// VT-1: output is valid ANSI SQL INSERT statement
// test_kind: spec_case(VT-1)
#[ test ]
fn variant_024_vt_01_valid_ansi_sql_insert()
{
  let view = RowBuilder::new( vec![ "Name".into(), "Age".into() ] )
    .add_row( vec![ "Alice".into(), "30".into() ] )
    .build_view();

  let out = Format::format( &sql_ansi_formatter(), &view ).unwrap();

  assert!( out.contains( "INSERT INTO" ), "INSERT INTO present" );
  assert!( out.contains( "VALUES" ), "VALUES clause present" );
}

/// VT-2: values are single-quote escaped
// test_kind: spec_case(VT-2)
#[ test ]
fn variant_024_vt_02_single_quote_escaped()
{
  let view = RowBuilder::new( vec![ "Name".into() ] )
    .add_row( vec![ "O'Brien".into() ] )
    .build_view();

  let out = Format::format( &sql_ansi_formatter(), &view ).unwrap();

  // Single quote doubled for SQL escaping
  assert!(
    out.contains( "O''Brien" ),
    "single quote escaped (doubled): {out}",
  );
}

/// VT-3: column names listed in INSERT
// test_kind: spec_case(VT-3)
#[ test ]
fn variant_024_vt_03_column_names_in_insert()
{
  let view = RowBuilder::new( vec![ "name".into(), "city".into() ] )
    .add_row( vec![ "Alice".into(), "NYC".into() ] )
    .build_view();

  let out = Format::format( &sql_ansi_formatter(), &view ).unwrap();

  assert!( out.contains( "name" ), "column 'name' listed" );
  assert!( out.contains( "city" ), "column 'city' listed" );
}

/// VT-4: empty table produces no INSERT statements
// test_kind: spec_case(VT-4)
#[ test ]
fn variant_024_vt_04_empty_table_no_inserts()
{
  let view = RowBuilder::new( vec![ "Col".into() ] )
    .build_view();

  let out = Format::format( &sql_ansi_formatter(), &view ).unwrap();

  // No value tuples after VALUES keyword
  let after_values = out.split( "VALUES" ).nth( 1 ).unwrap_or( "" );
  assert!( !after_values.contains( '(' ), "no value tuples for empty table: {out}" );
}
