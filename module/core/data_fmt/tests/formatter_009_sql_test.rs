//! `SqlFormatter` spec tests (FM-39..FM-46, file-scoped IDs)

#![ cfg( feature = "enabled" ) ]
#![ cfg( feature = "format_sql" ) ]

use data_fmt::{ RowBuilder, SqlFormatter, SqlVariant, Format };

/// FM-39: ansi variant produces double-quoted identifiers
// test_kind: spec_case(FM-39)
#[ test ]
fn formatter_009_fm_39_ansi_variant_produces_double_quoted_identifiers()
{
  let view = RowBuilder::new( vec![ "name".into(), "age".into() ] )
    .add_row( vec![ "Alice".into(), "30".into() ] )
    .build_view();

  let formatter = SqlFormatter::with_variant( "data", SqlVariant::Ansi );
  let output = Format::format( &formatter, &view ).unwrap();

  assert!( output.contains( "INSERT INTO \"data\"" ), "should use double-quoted table name" );
  assert!( output.contains( "\"name\"" ), "column name should be double-quoted" );
  assert!( output.contains( "'Alice'" ), "string value should be single-quoted" );
}

/// FM-40: mysql variant produces backtick-quoted identifiers
// test_kind: spec_case(FM-40)
#[ test ]
fn formatter_009_fm_40_mysql_variant_produces_backtick_quoted_identifiers()
{
  let view = RowBuilder::new( vec![ "name".into() ] )
    .add_row( vec![ "Alice".into() ] )
    .build_view();

  let formatter = SqlFormatter::with_variant( "users", SqlVariant::MySQL );
  let output = Format::format( &formatter, &view ).unwrap();

  assert!( output.contains( "`users`" ), "MySQL should backtick-quote table name" );
  assert!( output.contains( "`name`" ), "MySQL should backtick-quote column name" );
  assert!( output.contains( "'Alice'" ), "values should be single-quoted" );
}

/// FM-41: postgresql variant produces double-quoted identifiers
// test_kind: spec_case(FM-41)
#[ test ]
fn formatter_009_fm_41_postgresql_variant_produces_double_quoted_identifiers()
{
  let view = RowBuilder::new( vec![ "name".into() ] )
    .add_row( vec![ "Alice".into() ] )
    .build_view();

  let formatter = SqlFormatter::with_variant( "users", SqlVariant::PostgreSQL );
  let output = Format::format( &formatter, &view ).unwrap();

  assert!( output.contains( "\"users\"" ), "PostgreSQL should double-quote table name" );
  assert!( output.contains( "\"name\"" ), "PostgreSQL should double-quote column name" );
}

/// FM-42: sqlite variant produces double-quoted identifiers
// test_kind: spec_case(FM-42)
#[ test ]
fn formatter_009_fm_42_sqlite_variant_produces_double_quoted_identifiers()
{
  let view = RowBuilder::new( vec![ "name".into() ] )
    .add_row( vec![ "Alice".into() ] )
    .build_view();

  let formatter = SqlFormatter::with_variant( "users", SqlVariant::SQLite );
  let output = Format::format( &formatter, &view ).unwrap();

  assert!( output.contains( "\"users\"" ), "SQLite should double-quote table name" );
  assert!( output.contains( "\"name\"" ), "SQLite should double-quote column name" );
}

/// FM-43: custom table name appears in INSERT statement
// test_kind: spec_case(FM-43)
#[ test ]
fn formatter_009_fm_43_custom_table_name_appears_in_insert_statement()
{
  let view = RowBuilder::new( vec![ "k".into() ] )
    .add_row( vec![ "v".into() ] )
    .build_view();

  let formatter = SqlFormatter::with_variant( "my_schema.my_table", SqlVariant::Ansi );
  let output = Format::format( &formatter, &view ).unwrap();

  assert!(
    output.contains( "INSERT INTO \"my_schema.my_table\"" ),
    "provided table name should appear verbatim inside quotes",
  );
}

/// FM-44: Format trait dispatch returns well-formed string
// test_kind: spec_case(FM-44)
#[ test ]
fn formatter_009_fm_44_format_trait_dispatch_returns_well_formed_string()
{
  let view = RowBuilder::new( vec![ "a".into() ] )
    .add_row( vec![ "1".into() ] )
    .build_view();

  let formatter = SqlFormatter::new( "t" );
  let result = Format::format( &formatter, &view );

  assert!( result.is_ok(), "format should return Ok" );
  let output = result.unwrap();
  assert!( output.contains( "INSERT INTO" ), "should contain INSERT INTO" );
  assert!( output.contains( "VALUES" ), "should contain VALUES" );
}

/// FM-45: empty data (zero rows) produces empty string
///
/// Fix(BUG-020): previously emitted `INSERT INTO ... VALUES;` which is invalid SQL.
/// Correct behavior: return empty string when no data rows exist.
// test_kind: spec_case(FM-45)
#[ test ]
fn formatter_009_fm_45_empty_data_produces_insert_with_no_value_rows()
{
  let view = RowBuilder::new( vec![ "col".into() ] )
    .build_view();

  let formatter = SqlFormatter::new( "t" );
  let output = Format::format( &formatter, &view ).unwrap();

  // Zero data rows → empty string (not invalid SQL like `VALUES;`)
  assert!(
    output.is_empty(),
    "zero-row SQL output must be empty string, got: '{output}'",
  );
}

/// FM-46: values with single quotes are escaped
// test_kind: spec_case(FM-46)
#[ test ]
fn formatter_009_fm_46_values_with_single_quotes_are_escaped()
{
  let view = RowBuilder::new( vec![ "text".into() ] )
    .add_row( vec![ "it's a test".into() ] )
    .build_view();

  let formatter = SqlFormatter::new( "t" );
  let output = Format::format( &formatter, &view ).unwrap();

  assert!(
    output.contains( "it''s a test" ),
    "single quote should be doubled for SQL escaping, got: '{output}'",
  );
}
