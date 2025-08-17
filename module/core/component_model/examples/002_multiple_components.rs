//! # 002 - Component Assignment Patterns
//! 
//! Shows different ways to assign components: individual assignment,
//! fluent chaining, and mixing mutable/fluent styles.

use component_model::Assign;

#[ derive( Default, Debug, PartialEq, Assign ) ]
struct DatabaseConnection
{
  host : String,
  port : i32,
}

fn main()
{
  println!( "=== Component Assignment Patterns ===" );
  
  let mut db_config = DatabaseConnection::default();
  
  // Assign components individually (simpler than tuple assignment)
  db_config.assign( "postgres.example.com" );  // String -> host
  db_config.assign( 5432 );                    // i32 -> port  
  
  println!( "Individual assignment result: {db_config:?}" );
  
  // Verify all fields were set correctly
  assert_eq!( db_config.host, "postgres.example.com" );
  assert_eq!( db_config.port, 5432 );
  
  // You can also use fluent style
  let db_config2 = DatabaseConnection::default()
    .impute( "localhost" )
    .impute( 3306 );
  
  println!( "Fluent assignment: {db_config2:?}" );
  
  // Mix mutable and fluent styles
  let mut db_config3 = DatabaseConnection::default()
    .impute( "dev.example.com" );
  
  db_config3.assign( 5433 );
  
  println!( "Mixed style: {db_config3:?}" );
  
  println!( "✅ Component assignment patterns complete!" );
}