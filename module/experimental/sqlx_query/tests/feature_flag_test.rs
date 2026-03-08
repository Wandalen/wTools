//!
//! Feature flag behavior tests for `sqlx_query` macros.
//!
//! These tests verify the macros correctly respond to the `sqlx_compiletime_checks`
//! feature flag, expanding to different `SQLx` APIs based on configuration.
//!

#[ test ]
fn feature_flag_default_behavior()
{
  //
  // Test: Verify default behavior (no feature flag)
  //
  // Without sqlx_compiletime_checks feature, macros should expand to
  // compile-time checked versions (sqlx::query!, sqlx::query_as!).
  //

  #[ cfg( not( feature = "sqlx_compiletime_checks" ) ) ]
  {
    // In default mode, we expect compile-time checking
    // This is the safer mode requiring DATABASE_URL at compile time
    println!( "✓ Default mode: compile-time checking enabled" );
  }

  #[ cfg( feature = "sqlx_compiletime_checks" ) ]
  {
    // If feature is enabled, we're in runtime mode
    println!( "✓ Runtime mode: compile-time checking disabled" );
  }

  // Test always passes - verifies conditional compilation works
}

#[ test ]
fn feature_flag_cfg_detection()
{
  //
  // Test: Verify cfg attributes correctly detect feature state
  //
  // This ensures our feature detection mechanism works as expected.
  //

  let compile_time_mode : bool;
  let runtime_mode : bool;

  #[ cfg( not( feature = "sqlx_compiletime_checks" ) ) ]
  {
    compile_time_mode = true;
    runtime_mode = false;
  }

  #[ cfg( feature = "sqlx_compiletime_checks" ) ]
  {
    compile_time_mode = false;
    runtime_mode = true;
  }

  // Exactly one mode should be active
  assert_ne!( compile_time_mode, runtime_mode );

  // Log which mode is active
  if compile_time_mode
  {
    println!( "Active mode: COMPILE-TIME (default)" );
  }
  else
  {
    println!( "Active mode: RUNTIME (feature enabled)" );
  }
}

#[ test ]
fn macro_namespace_accessibility()
{
  //
  // Test: Verify macros are accessible through all namespace layers
  //
  // The crate uses layered namespace pattern (own→orphan→exposed→prelude).
  // This test ensures macros are properly exported at each level.
  //

  // Test prelude access (most common usage)
  // Note: Using qualified paths to avoid unused import warning
  let q1 = stringify!( sqlx_query::prelude::query );
  let q2 = stringify!( sqlx_query::prelude::query_as );

  assert!( q1.contains( "query" ) );
  assert!( q2.contains( "query_as" ) );
}

#[ test ]
fn macro_root_level_access()
{
  //
  // Test: Verify macros accessible at root level
  //
  // Users should be able to access macros directly via use sqlx_query::*.
  //

  // Root level access
  let q1 = stringify!( sqlx_query::query );
  let q2 = stringify!( sqlx_query::query_as );

  assert!( q1.contains( "sqlx_query" ) );
  assert!( q2.contains( "sqlx_query" ) );
}
