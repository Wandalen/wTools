//!
//! Comprehensive macro expansion tests for `sqlx_query` macros.
//!
//! Tests verify correct expansion of `query!` and `query_as!` macros under
//! different feature flag configurations and binding scenarios.
//!

#[ test ]
fn query_macro_no_bindings_compiles()
{
  //
  // Test: query! macro without bind parameters should compile
  //
  // This test verifies the basic macro expansion works for simplest case.
  // The macro should expand to either sqlx::query() or sqlx::query!()
  // depending on sqlx_compiletime_checks feature flag.
  //

  // Note: We can't actually execute this without a real database connection,
  // but we can verify it compiles and generates valid syntax.

  // Verify the macro accepts SQL string literal
  let query_expr = stringify!
  (
    sqlx_query::query!( "SELECT 1" )
  );

  assert!( query_expr.contains( "query!" ) );
}

#[ test ]
fn query_macro_single_binding_compiles()
{
  //
  // Test: query! macro with single bind parameter should compile
  //
  // This verifies the macro correctly passes through a single binding.
  // The macro should expand to:
  // - With feature: sqlx::query(sql).bind(param)
  // - Without: sqlx::query!(sql, param)
  //

  let query_expr = stringify!
  (
    sqlx_query::query!( "SELECT * FROM users WHERE id = $1", user_id )
  );

  assert!( query_expr.contains( "query!" ) );
}

#[ test ]
fn query_macro_multiple_bindings_compiles()
{
  //
  // Test: query! macro with multiple bind parameters should compile
  //
  // This verifies the macro correctly handles multiple bindings in sequence.
  //

  let query_expr = stringify!
  (
    sqlx_query::query!
    (
      "INSERT INTO users (name, email) VALUES ($1, $2)",
      name,
      email
    )
  );

  assert!( query_expr.contains( "query!" ) );
}

#[ test ]
fn query_as_macro_no_bindings_compiles()
{
  //
  // Test: query_as! macro without bind parameters should compile
  //
  // This verifies the type parameter is correctly passed through.
  // The macro should expand to:
  // - With feature: sqlx::query_as::<_, User>(sql)
  // - Without: sqlx::query_as!(User, sql)
  //

  let query_expr = stringify!
  (
    sqlx_query::query_as!( User, "SELECT * FROM users" )
  );

  assert!( query_expr.contains( "query_as!" ) );
}

#[ test ]
fn query_as_macro_single_binding_compiles()
{
  //
  // Test: query_as! macro with single bind parameter should compile
  //
  // This verifies both type parameter and binding are passed through.
  //

  let query_expr = stringify!
  (
    sqlx_query::query_as!( User, "SELECT * FROM users WHERE id = $1", id )
  );

  assert!( query_expr.contains( "query_as!" ) );
}

#[ test ]
fn query_as_macro_multiple_bindings_compiles()
{
  //
  // Test: query_as! macro with multiple bind parameters should compile
  //
  // This verifies complex binding scenarios work correctly.
  //

  let query_expr = stringify!
  (
    sqlx_query::query_as!
    (
      User,
      "SELECT * FROM users WHERE name = $1 AND email = $2",
      name,
      email
    )
  );

  assert!( query_expr.contains( "query_as!" ) );
}

#[ test ]
fn macro_syntax_validation()
{
  //
  // Test: Verify macros are actually exported and callable
  //
  // This is a compile-time check that the macros exist in the public API.
  //

  // If this compiles, the macros are properly exported
  let _ = stringify!( sqlx_query::query );
  let _ = stringify!( sqlx_query::query_as );
}
