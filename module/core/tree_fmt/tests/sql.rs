//! Integration tests for `SqlFormatter`
//!
//! Tests SQL INSERT statement generation across different dialects.

#[ cfg( feature = "format_sql" ) ]
mod sql_tests
{
  use tree_fmt::{ RowBuilder, SqlFormatter, SqlVariant, Format };

  #[ test ]
  fn test_sql_basic_ansi_dialect()
  {
    let view = RowBuilder::new( vec![ "name".into(), "age".into(), "city".into() ] )
      .add_row( vec![ "Alice".into(), "30".into(), "NYC".into() ] )
      .add_row( vec![ "Bob".into(), "25".into(), "LA".into() ] )
      .build_view();

    let formatter = SqlFormatter::new( "users" );
    let sql = formatter.format( &view ).unwrap();

    assert!( sql.contains( "INSERT INTO \"users\"" ) );
    assert!( sql.contains( "(\"name\", \"age\", \"city\")" ) );
    assert!( sql.contains( "VALUES" ) );
    assert!( sql.contains( "('Alice', 30, 'NYC')" ) );
    assert!( sql.contains( "('Bob', 25, 'LA')" ) );
    assert!( sql.ends_with( ';' ) );
  }

  #[ test ]
  fn test_sql_postgresql_dialect()
  {
    let view = RowBuilder::new( vec![ "id".into(), "data".into() ] )
      .add_row( vec![ "1".into(), "test".into() ] )
      .build_view();

    let formatter = SqlFormatter::with_variant( "items", SqlVariant::PostgreSQL );
    let sql = formatter.format( &view ).unwrap();

    // PostgreSQL uses double quotes like ANSI
    assert!( sql.contains( "INSERT INTO \"items\"" ) );
    assert!( sql.contains( "(\"id\", \"data\")" ) );
  }

  #[ test ]
  fn test_sql_mysql_dialect()
  {
    let view = RowBuilder::new( vec![ "name".into(), "value".into() ] )
      .add_row( vec![ "test".into(), "data".into() ] )
      .build_view();

    let formatter = SqlFormatter::with_variant( "settings", SqlVariant::MySQL );
    let sql = formatter.format( &view ).unwrap();

    // MySQL uses backticks
    assert!( sql.contains( "INSERT INTO `settings`" ) );
    assert!( sql.contains( "(`name`, `value`)" ) );
  }

  #[ test ]
  fn test_sql_sqlite_dialect()
  {
    let view = RowBuilder::new( vec![ "col".into() ] )
      .add_row( vec![ "val".into() ] )
      .build_view();

    let formatter = SqlFormatter::with_variant( "table", SqlVariant::SQLite );
    let sql = formatter.format( &view ).unwrap();

    // SQLite uses double quotes like ANSI
    assert!( sql.contains( "INSERT INTO \"table\"" ) );
  }

  #[ test ]
  fn test_sql_quote_escaping()
  {
    let view = RowBuilder::new( vec![ "name".into(), "description".into() ] )
      .add_row( vec![ "O'Brien".into(), "It's working".into() ] )
      .add_row( vec![ "D'Angelo".into(), "Won't fail".into() ] )
      .build_view();

    let formatter = SqlFormatter::new( "people" );
    let sql = formatter.format( &view ).unwrap();

    // Single quotes should be doubled
    assert!( sql.contains( "('O''Brien', 'It''s working')" ) );
    assert!( sql.contains( "('D''Angelo', 'Won''t fail')" ) );
  }

  #[ test ]
  fn test_sql_numeric_detection()
  {
    let view = RowBuilder::new( vec![ "name".into(), "age".into(), "score".into(), "ratio".into() ] )
      .add_row( vec![ "Alice".into(), "30".into(), "95".into(), "0.95".into() ] )
      .add_row( vec![ "Bob".into(), "25".into(), "100".into(), "1.0".into() ] )
      .build_view();

    let formatter = SqlFormatter::new( "users" );
    let sql = formatter.format( &view ).unwrap();

    // Numeric values should not be quoted
    assert!( sql.contains( "('Alice', 30, 95, 0.95)" ) );
    assert!( sql.contains( "('Bob', 25, 100, 1.0)" ) );
  }

  #[ test ]
  fn test_sql_empty_as_null()
  {
    let view = RowBuilder::new( vec![ "name".into(), "email".into(), "phone".into() ] )
      .add_row( vec![ "Alice".into(), "alice@example.com".into(), String::new() ] )
      .add_row( vec![ "Bob".into(), String::new(), "555-1234".into() ] )
      .build_view();

    let formatter = SqlFormatter::new( "contacts" ).empty_as_null( true );
    let sql = formatter.format( &view ).unwrap();

    assert!( sql.contains( "('Alice', 'alice@example.com', NULL)" ) );
    assert!( sql.contains( "('Bob', NULL, '555-1234')" ) );
  }

  #[ test ]
  fn test_sql_empty_without_null_conversion()
  {
    let view = RowBuilder::new( vec![ "col".into() ] )
      .add_row( vec![ String::new() ] )
      .build_view();

    let formatter = SqlFormatter::new( "table" );
    let sql = formatter.format( &view ).unwrap();

    // Empty strings should be quoted, not NULL
    assert!( sql.contains( "('')" ) );
  }

  #[ test ]
  fn test_sql_mysql_backslash_escaping()
  {
    let view = RowBuilder::new( vec![ "path".into() ] )
      .add_row( vec![ r"C:\Users\John".into() ] )
      .add_row( vec![ r"\\server\share".into() ] )
      .build_view();

    let formatter = SqlFormatter::with_variant( "files", SqlVariant::MySQL );
    let sql = formatter.format( &view ).unwrap();

    // MySQL should escape backslashes
    assert!( sql.contains( r"('C:\\Users\\John')" ) );
    assert!( sql.contains( r"('\\\\server\\share')" ) );
  }

  #[ test ]
  fn test_sql_identifier_escaping()
  {
    let view = RowBuilder::new( vec![ "column with spaces".into(), "order".into() ] )
      .add_row( vec![ "val1".into(), "val2".into() ] )
      .build_view();

    let formatter = SqlFormatter::new( "my table" );
    let sql = formatter.format( &view ).unwrap();

    // Identifiers should be properly quoted
    assert!( sql.contains( "INSERT INTO \"my table\"" ) );
    assert!( sql.contains( "(\"column with spaces\", \"order\")" ) );
  }

  #[ test ]
  fn test_sql_many_rows()
  {
    let mut builder = RowBuilder::new( vec![ "id".into(), "name".into() ] );

    for i in 1..=100
    {
      builder = builder.add_row( vec![ i.to_string(), format!( "User{}", i ) ] );
    }

    let view = builder.build_view();
    let formatter = SqlFormatter::new( "users" );
    let sql = formatter.format( &view ).unwrap();

    // Check first, middle, and last rows
    assert!( sql.contains( "(1, 'User1')" ) );
    assert!( sql.contains( "(50, 'User50')" ) );
    assert!( sql.contains( "(100, 'User100')" ) );

    // Should have 99 commas between rows (100 rows)
    assert_eq!( sql.matches( "),\n  (" ).count(), 99 );
  }

  #[ test ]
  fn test_sql_unicode_content()
  {
    let view = RowBuilder::new( vec![ "name".into(), "text".into() ] )
      .add_row( vec![ "Japanese".into(), "こんにちは".into() ] )
      .add_row( vec![ "Arabic".into(), "مرحبا".into() ] )
      .add_row( vec![ "Emoji".into(), "🎉🚀".into() ] )
      .build_view();

    let formatter = SqlFormatter::new( "messages" );
    let sql = formatter.format( &view ).unwrap();

    assert!( sql.contains( "'こんにちは'" ) );
    assert!( sql.contains( "'مرحبا'" ) );
    assert!( sql.contains( "'🎉🚀'" ) );
  }

  #[ test ]
  fn test_sql_special_characters()
  {
    let view = RowBuilder::new( vec![ "data".into() ] )
      .add_row( vec![ "Line1\nLine2".into() ] )
      .add_row( vec![ "Tab\there".into() ] )
      .add_row( vec![ "Quote: \"test\"".into() ] )
      .build_view();

    let formatter = SqlFormatter::new( "content" );
    let sql = formatter.format( &view ).unwrap();

    // These should be preserved (no special escaping needed for SQL strings)
    assert!( sql.contains( "Line1\nLine2" ) );
    assert!( sql.contains( "Tab\there" ) );
    // Double quotes dont need escaping in SQL strings (only single quotes)
    assert!( sql.contains( "Quote: \"test\"" ) );
  }

  #[ test ]
  fn test_sql_zero_rows()
  {
    let view = RowBuilder::new( vec![ "col1".into(), "col2".into() ] ).build_view();

    let formatter = SqlFormatter::new( "empty_table" );
    let result = formatter.format( &view );

    // Should produce valid SQL even with no data rows
    let sql = result.unwrap();
    assert!( sql.contains( "INSERT INTO \"empty_table\"" ) );
    assert!( sql.contains( "(\"col1\", \"col2\")" ) );
    assert!( sql.contains( "VALUES" ) );
  }

  #[ test ]
  fn test_sql_negative_numbers()
  {
    let view = RowBuilder::new( vec![ "amount".into(), "balance".into() ] )
      .add_row( vec![ "-100".into(), "-50.5".into() ] )
      .add_row( vec![ "200".into(), "150.75".into() ] )
      .build_view();

    let formatter = SqlFormatter::new( "transactions" );
    let sql = formatter.format( &view ).unwrap();

    // Negative numbers should not be quoted
    assert!( sql.contains( "(-100, -50.5)" ) );
    assert!( sql.contains( "(200, 150.75)" ) );
  }

  #[ test ]
  fn test_sql_scientific_notation()
  {
    let view = RowBuilder::new( vec![ "value".into() ] )
      .add_row( vec![ "1.5e10".into() ] )
      .add_row( vec![ "3.14e-5".into() ] )
      .build_view();

    let formatter = SqlFormatter::new( "scientific" );
    let sql = formatter.format( &view ).unwrap();

    // Scientific notation should be recognized as numeric
    assert!( sql.contains( "(1.5e10)" ) );
    assert!( sql.contains( "(3.14e-5)" ) );
  }
}
