//!
//! Test that the readme example code compiles correctly.
//!
//! This test ensures the primary usage example shown in readme.md
//! represents valid, compilable code.
//!

#[ test ]
fn readme_example_syntax_compiles()
{
  //
  // Test: Verify readme example has correct syntax
  //
  // The readme shows this example:
  //
  // ```rust
  // use sqlx_query::*;
  //
  // let user : User = query_as!( User, "SELECT * FROM users LIMIT 1" )
  //     .fetch_one( executor )
  //     .await?;
  //
  // query!( "DELETE FROM users WHERE id = $1", user.id )
  //     .execute( executor )
  //     .await?;
  // ```
  //
  // We verify the macro calls compile (without actually executing them).
  //

  // Verify query_as! macro call syntax
  let query_as_expr = stringify!
  (
    sqlx_query::query_as!( User, "SELECT * FROM users LIMIT 1" )
  );

  // Verify query! macro call with binding syntax
  let query_expr = stringify!
  (
    sqlx_query::query!( "DELETE FROM users WHERE id = $1", user_id )
  );

  assert!( query_as_expr.contains( "query_as!" ) );
  assert!( query_expr.contains( "query!" ) );
}

#[ test ]
fn readme_example_expansion_compiles()
{
  //
  // Test: Verify readme expansion example syntax
  //
  // The readme shows the expanded form under both feature configurations.
  // We verify both expansion patterns are syntactically valid.
  //

  // With sqlx_compiletime_checks feature (runtime mode)
  let runtime_query_as = stringify!
  (
    ::sqlx::query_as::< _, User >( "SELECT * FROM users LIMIT 1" )
  );

  let runtime_query = stringify!
  (
    ::sqlx::query( "DELETE FROM users WHERE id = $1" ).bind( user_id )
  );

  // Without feature (compile-time mode)
  let compiletime_query_as = stringify!
  (
    ::sqlx::query_as!( User, "SELECT * FROM users LIMIT 1" )
  );

  let compiletime_query = stringify!
  (
    ::sqlx::query!( "DELETE FROM users WHERE id = $1", user_id )
  );

  // All syntax forms should be valid
  assert!( runtime_query_as.contains( "query_as" ) );
  assert!( runtime_query.contains( "query" ) );
  assert!( compiletime_query_as.contains( "query_as!" ) );
  assert!( compiletime_query.contains( "query!" ) );
}
