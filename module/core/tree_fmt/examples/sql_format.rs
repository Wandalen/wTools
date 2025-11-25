//! Example demonstrating `SqlFormatter` usage
//!
//! Shows SQL INSERT statement generation across different dialects.
//!
//! Run with:
//! ```bash
//! cargo run --example sql_format --features format_sql
//! ```

#[ cfg( feature = "format_sql" ) ]
use tree_fmt::{ RowBuilder, SqlFormatter, SqlVariant, Format };

#[ cfg( not( feature = "format_sql" ) ) ]
fn main()
{
  println!( "This example requires the 'format_sql' feature." );
  println!( "Run with: cargo run --example sql_format --features format_sql" );
}

#[ cfg( feature = "format_sql" ) ]
fn main()
{
  println!( "=== SqlFormatter Examples ===\n" );

  // Sample data
  let view = RowBuilder::new( vec![ "name".into(), "age".into(), "city".into(), "salary".into() ] )
    .add_row( vec![ "Alice".into(), "30".into(), "NYC".into(), "75000".into() ] )
    .add_row( vec![ "Bob".into(), "25".into(), "LA".into(), "65000".into() ] )
    .add_row( vec![ "Carol".into(), "35".into(), "Chicago".into(), "80000".into() ] )
    .build_view();

  // Example 1: ANSI SQL (default)
  println!( "1. ANSI SQL Dialect (default):\n" );
  let formatter = SqlFormatter::new( "employees" );
  let sql = formatter.format( &view ).unwrap();
  println!( "{sql}\n" );

  // Example 2: PostgreSQL dialect
  println!( "2. PostgreSQL Dialect:\n" );
  let formatter = SqlFormatter::with_variant( "employees", SqlVariant::PostgreSQL );
  let sql = formatter.format( &view ).unwrap();
  println!( "{sql}\n" );

  // Example 3: MySQL dialect
  println!( "3. MySQL Dialect (backtick identifiers):\n" );
  let formatter = SqlFormatter::with_variant( "employees", SqlVariant::MySQL );
  let sql = formatter.format( &view ).unwrap();
  println!( "{sql}\n" );

  // Example 4: SQLite dialect
  println!( "4. SQLite Dialect:\n" );
  let formatter = SqlFormatter::with_variant( "employees", SqlVariant::SQLite );
  let sql = formatter.format( &view ).unwrap();
  println!( "{sql}\n" );

  // Example 5: Quote escaping
  println!( "5. SQL Quote Escaping:\n" );
  let quotes_view = RowBuilder::new( vec![ "name".into(), "description".into() ] )
    .add_row( vec![ "O'Brien".into(), "It's working".into() ] )
    .add_row( vec![ "D'Angelo".into(), "Won't fail".into() ] )
    .build_view();

  let formatter = SqlFormatter::new( "people" );
  let sql = formatter.format( &quotes_view ).unwrap();
  println!( "{sql}\n" );
  println!( "Note: Single quotes are doubled for proper SQL escaping.\n" );

  // Example 6: Numeric value detection
  println!( "6. Numeric Value Detection:\n" );
  let numeric_view = RowBuilder::new( vec![ "product".into(), "price".into(), "quantity".into(), "rating".into() ] )
    .add_row( vec![ "Laptop".into(), "999.99".into(), "5".into(), "4.5".into() ] )
    .add_row( vec![ "Mouse".into(), "29".into(), "150".into(), "4.8".into() ] )
    .build_view();

  let formatter = SqlFormatter::new( "products" );
  let sql = formatter.format( &numeric_view ).unwrap();
  println!( "{sql}\n" );
  println!( "Note: Numeric values are not quoted.\n" );

  // Example 7: Empty strings as NULL
  println!( "7. Empty Strings Converted to NULL:\n" );
  let null_view = RowBuilder::new( vec![ "name".into(), "email".into(), "phone".into() ] )
    .add_row( vec![ "Alice".into(), "alice@example.com".into(), String::new() ] )
    .add_row( vec![ "Bob".into(), String::new(), "555-1234".into() ] )
    .add_row( vec![ "Carol".into(), "carol@example.com".into(), "555-5678".into() ] )
    .build_view();

  let formatter = SqlFormatter::new( "contacts" ).empty_as_null( true );
  let sql = formatter.format( &null_view ).unwrap();
  println!( "{sql}\n" );
  println!( "Note: Empty strings become NULL when empty_as_null is enabled.\n" );

  // Example 8: MySQL backslash escaping
  println!( "8. MySQL Backslash Escaping:\n" );
  let path_view = RowBuilder::new( vec![ "path".into() ] )
    .add_row( vec![ r"C:\Users\John\Documents".into() ] )
    .add_row( vec![ r"\\server\share\folder".into() ] )
    .build_view();

  let formatter = SqlFormatter::with_variant( "file_paths", SqlVariant::MySQL );
  let sql = formatter.format( &path_view ).unwrap();
  println!( "{sql}\n" );
  println!( "Note: MySQL requires backslash escaping.\n" );

  // Example 9: Identifiers with special characters
  println!( "9. Identifiers with Special Characters:\n" );
  let special_view = RowBuilder::new( vec![ "column name".into(), "order".into() ] )
    .add_row( vec![ "value1".into(), "value2".into() ] )
    .build_view();

  let formatter = SqlFormatter::new( "table name" );
  let sql = formatter.format( &special_view ).unwrap();
  println!( "{sql}\n" );
  println!( "Note: Identifiers with spaces/keywords are properly quoted.\n" );

  // Example 10: Large dataset
  println!( "10. Large Dataset (100 rows):\n" );
  let mut builder = RowBuilder::new( vec![ "id".into(), "username".into(), "active".into() ] );

  for i in 1..=100
  {
    builder = builder.add_row( vec![
      i.to_string(),
      format!( "user{:03}", i ),
      if i % 2 == 0 { "1" } else { "0" }.into()
    ] );
  }

  let large_view = builder.build_view();
  let formatter = SqlFormatter::new( "users" );
  let sql = formatter.format( &large_view ).unwrap();

  // Just show first few lines
  let lines : Vec< &str > = sql.lines().take( 10 ).collect();
  println!( "{}", lines.join( "\n" ) );
  println!( "  ... ({} more rows) ...", 100 - 8 );
  println!( "  (100, 'user100', 0);\n" );

  println!( "=== Usage Tips ===" );
  println!( "- Choose dialect based on target database" );
  println!( "- ANSI dialect works for most databases" );
  println!( "- MySQL requires backticks, PostgreSQL/SQLite use double quotes" );
  println!( "- Numeric values detected automatically (no quotes)" );
  println!( "- Single quotes in strings are automatically escaped" );
  println!( "- Use empty_as_null(true) for optional fields" );
  println!( "- All SQL injection concerns handled via proper escaping" );
}
