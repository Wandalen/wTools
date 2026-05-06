//! Feature-gate compile-fail tests.
//!
//! Verifies that `collection_constructors` and `collection_into_constructors` are
//! independently enforced: each macro set is absent when its governing feature is disabled.
//!
//! Covered spec cases:
//! - `feature/001_collection_constructors` FT-07: strict macros absent without feature.
//! - `feature/002_into_constructors` FT-06: into-macros absent without their feature.
//! - `api/001_collection_macros` AP-09: the two feature flags are orthogonal.
//!
//! Mechanism: each test spawns a `cargo check` subprocess with a minimal source file and
//! a specific subset of features, then asserts that compilation fails.

use std::path ::PathBuf;

/// Run `cargo check` on `code` with `features` enabled on `collection_tools`.
/// Returns `true` when compilation fails (the expected outcome for compile-fail cases).
fn check_compile_fails( code : &str, features : &[ &str ] ) -> bool
{
  let manifest_dir = env!( "CARGO_MANIFEST_DIR" );
  // Shared build cache under the crate's target dir; cargo resolves artifacts per feature set.
  let target_root = PathBuf::from( manifest_dir ).join( "target" ).join( "feature_gate_tests" );
  // Per-process source dir avoids conflicts between parallel nextest workers.
  let src_dir = target_root.join( format!( "src_{}", std::process::id() ) );
  std::fs::create_dir_all( src_dir.join( "src" ) ).expect( "create temp src dir" );

  let features_str = features
    .iter()
    .map( |f| format!( "\"{f}\"" ) )
    .collect ::< Vec< _ > >()
    .join( ", " );

  let cargo_toml = format!(
    "[package]\n\
     name = \"feature_gate_check\"\n\
     version = \"0.1.0\"\n\
     edition = \"2021\"\n\
     \n\
     [dependencies]\n\
     collection_tools = {{ path = \"{manifest_dir}\", default-features = false, features = [{features_str}] }}\n",
  );

  std::fs::write( src_dir.join( "Cargo.toml" ), &cargo_toml ).expect( "write temp Cargo.toml" );
  std::fs::write( src_dir.join( "src/main.rs" ), code ).expect( "write temp main.rs" );

  let cargo = std::env ::var( "CARGO" ).unwrap_or_else( |_| "cargo".to_owned() );
  let output = std::process ::Command ::new( &cargo )
    .args
    ( [
      "check",
      "--manifest-path",
      &src_dir.join( "Cargo.toml" ).to_string_lossy(),
    ] )
    // Use shared build dir so compilation artifacts are reused across runs.
    .env( "CARGO_TARGET_DIR", target_root.join( "build" ) )
    .output()
    .expect( "cargo check invocation failed" );

  // Remove temporary source tree; keep build artifacts for caching.
  let _ = std::fs ::remove_dir_all( &src_dir );

  !output.status.success()
}

/// FT-07: `collection_tools::vec!` fails to compile when `collection_constructors` is disabled.
#[ test ]
fn strict_macros_absent_without_collection_constructors()
{
  assert!(
    check_compile_fails(
      "fn main() { let _ = collection_tools ::vec![ 1, 2, 3 ]; }",
      &[ "enabled" ],
    ),
    "collection_tools::vec! must not compile without collection_constructors feature"
  );
}

/// into/FT-06: `collection_tools::vec!` fails to compile even when only
/// `collection_into_constructors` is enabled — the two feature flags are orthogonal.
#[ test ]
fn strict_macros_absent_with_only_into_feature()
{
  assert!(
    check_compile_fails(
      "fn main() { let _ = collection_tools ::vec![ 1, 2, 3 ]; }",
      &[ "enabled", "collection_into_constructors" ],
    ),
    "collection_tools::vec! must not compile without collection_constructors, even when collection_into_constructors is on"
  );
}

/// AP-09: `collection_tools::into_hmap!` fails to compile when only `collection_constructors`
/// is enabled — the two feature flags are orthogonal in the opposite direction.
#[ test ]
fn into_macros_absent_with_only_strict_feature()
{
  assert!(
    check_compile_fails(
      "fn main() { let _: std::collections::HashMap< &str, i32 > = collection_tools ::into_hmap!{ \"a\" => 1 }; }",
      &[ "enabled", "collection_constructors" ],
    ),
    "collection_tools::into_hmap! must not compile without collection_into_constructors, even when collection_constructors is on"
  );
}
