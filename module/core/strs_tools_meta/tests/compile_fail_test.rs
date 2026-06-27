//! Compile-fail tests for non-literal delimiter and pattern arguments.
//!
//! ## Covered spec cases
//!
//! | Spec | Case | Assertion |
//! |------|------|-----------|
//! | `tests/docs/api/001_optimize_split_api.md` | AP-3 | Non-literal delimiter → compile error |
//! | `tests/docs/api/002_optimize_match_api.md` | AP-3 | Non-literal pattern → compile error |
//!
//! ## Mechanism
//!
//! Each test spawns a `cargo check` subprocess with a minimal source file and the
//! required features enabled on `strs_tools_meta`, then asserts compilation fails.

#[ cfg( any( feature = "optimize_split", feature = "optimize_match" ) ) ]
use std ::path ::PathBuf;

/// Spawn `cargo check` on `code` with `features` enabled on `strs_tools_meta`.
/// Returns `true` when compilation fails — the expected outcome for compile-fail cases.
#[ cfg( any( feature = "optimize_split", feature = "optimize_match" ) ) ]
fn check_compile_fails( code : &str, features : &[ &str ] ) -> bool
{
  let manifest_dir = env!( "CARGO_MANIFEST_DIR" );
  let target_root = PathBuf ::from( manifest_dir ).join( "target" ).join( "compile_fail_tests" );
  let src_dir = target_root.join( format!( "src_{}", std ::process ::id() ) );
  std ::fs ::create_dir_all( src_dir.join( "src" ) ).expect( "create temp src dir" );

  let features_str = features
    .iter()
    .map( | f | format!( "\"{f}\"" ) )
    .collect ::< Vec< _ > >()
    .join( ", " );

  let cargo_toml = format!(
    "[package]\n\
     name = \"compile_fail_check\"\n\
     version = \"0.1.0\"\n\
     edition = \"2021\"\n\
     \n\
     [dependencies]\n\
     strs_tools_meta = {{ path = \"{manifest_dir}\", default-features = false, features = [{features_str}] }}\n",
  );

  std ::fs ::write( src_dir.join( "Cargo.toml" ), &cargo_toml ).expect( "write temp Cargo.toml" );
  std ::fs ::write( src_dir.join( "src/main.rs" ), code ).expect( "write temp main.rs" );

  let cargo = std ::env ::var( "CARGO" ).unwrap_or_else( | _ | "cargo".to_owned() );
  let output = std ::process ::Command ::new( &cargo )
    .args
    ( [
      "check",
      "--manifest-path",
      &src_dir.join( "Cargo.toml" ).to_string_lossy(),
    ] )
    .env( "CARGO_TARGET_DIR", target_root.join( "build" ) )
    .output()
    .expect( "cargo check invocation failed" );

  let _ = std ::fs ::remove_dir_all( &src_dir );
  !output.status.success()
}

/// AP-3 (api/001): `optimize_split!` rejects a non-literal delimiter at compile time.
#[ cfg( feature = "optimize_split" ) ]
#[ test ]
fn non_literal_delimiter_rejected_at_compile_time()
{
  assert!(
    check_compile_fails
    (
      "fn main() { let delim = \",\"; let _ = strs_tools_meta ::optimize_split!( \"a,b\", delim ); }",
      &[ "enabled", "optimize_split" ],
    ),
    "optimize_split! must not compile with a non-literal delimiter",
  );
}

/// AP-3 (api/002): `optimize_match!` rejects a non-literal pattern at compile time.
#[ cfg( feature = "optimize_match" ) ]
#[ test ]
fn non_literal_pattern_rejected_at_compile_time()
{
  assert!(
    check_compile_fails
    (
      "fn main() { let pat = \"test\"; let _ = strs_tools_meta ::optimize_match!( \"test_string\", pat ); }",
      &[ "enabled", "optimize_match" ],
    ),
    "optimize_match! must not compile with a non-literal pattern",
  );
}
