//! Tests for the `include_md!` proc-macro.
//!
//! ## Covered spec cases
//!
//! | Spec | Case | Assertion |
//! |------|------|-----------|
//! | `docs/feature/001_file_inclusion.md` | Valid file | Expands to full file contents |
//! | `docs/feature/001_file_inclusion.md` | Empty file | Returns empty string |
//! | `docs/invariant/002_compile_time_errors.md` | Missing file | Compile-time error |
//! | `docs/invariant/003_size_limit.md` | Oversized file | Compile-time error |
//! | `docs/invariant/002_compile_time_errors.md` | Invalid UTF-8 | Compile-time error |
//! | `docs/api/001_include_md.md` | No arguments | Compile-time error |
//! | `docs/api/001_include_md.md` | Two arguments | Compile-time error |

#![ cfg( feature = "enabled" ) ]

use include_md::include_md;
use std ::path ::PathBuf;

// ------------------------------------------------------------------ positive

/// Valid file: `include_md!` returns complete UTF-8 file contents as `&'static str`.
#[ test ]
fn valid_file_returns_full_contents()
{
  let content = include_md!( "fixture/sample.md" );
  let expected = "# Hello\n\nThis is a test fixture for include_md.\n";
  assert_eq!( content, expected, "include_md! must return complete file contents" );
}

/// Empty file: `include_md!` on a zero-byte file returns an empty string.
#[ test ]
fn empty_file_returns_empty_string()
{
  let content = include_md!( "fixture/empty.md" );
  assert_eq!( content, "", "include_md! on an empty file must return an empty string" );
}

// ------------------------------------------------------------------ compile-fail helpers

/// Spawn `cargo check` on `code` with `features` enabled on `include_md`.
/// Returns `true` when compilation fails — the expected outcome for compile-fail cases.
fn check_compile_fails( code : &str, features : &[ &str ] ) -> bool
{
  let manifest_dir = env!( "CARGO_MANIFEST_DIR" );
  let target_root = PathBuf ::from( manifest_dir ).join( "target" ).join( "compile_fail_tests_include_md" );
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
     include_md = {{ path = \"{manifest_dir}\", default-features = false, features = [{features_str}] }}\n",
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

// ------------------------------------------------------------------ compile-fail

/// Missing file is rejected at compile time.
#[ test ]
fn missing_file_is_compile_error()
{
  assert!(
    check_compile_fails
    (
      "fn main() { let _ = include_md::include_md!( \"fixture/does_not_exist.md\" ); }",
      &[ "enabled" ],
    ),
    "include_md! must not compile when the file does not exist",
  );
}

/// No arguments is rejected at compile time.
#[ test ]
fn no_args_is_compile_error()
{
  assert!(
    check_compile_fails
    (
      "fn main() { let _ = include_md::include_md!(); }",
      &[ "enabled" ],
    ),
    "include_md! must not compile with no arguments",
  );
}

/// Two arguments is rejected at compile time.
#[ test ]
fn two_args_is_compile_error()
{
  assert!(
    check_compile_fails
    (
      "fn main() { let _ = include_md::include_md!( \"a.md\", \"b\" ); }",
      &[ "enabled" ],
    ),
    "include_md! must not compile with two arguments",
  );
}

/// File exceeding 10 MB is rejected at compile time via the const assertion.
#[ test ]
fn oversized_file_is_compile_error()
{
  use std ::io ::Write;

  let tmp_dir = std ::env ::temp_dir().join
  (
    format!( "include_md_oversized_{}", std ::process ::id() )
  );
  std ::fs ::create_dir_all( &tmp_dir ).expect( "create temp dir" );
  let big_file = tmp_dir.join( "big.md" );

  {
    let mut f = std ::fs ::File ::create( &big_file ).expect( "create big file" );
    // Write just over 10 MB so the const assertion fires.
    let chunk = vec![ b'a'; 65_536 ];
    let mut written = 0_usize;
    while written < 10_000_001
    {
      let n = ( 10_000_001 - written ).min( chunk.len() );
      f.write_all( &chunk[ ..n ] ).expect( "write chunk" );
      written += n;
    }
  }

  // Use an absolute path so `include_bytes!` can locate the file from any source directory.
  let abs = big_file.to_string_lossy().into_owned();
  let code = format!( "fn main() {{ let _ = include_md::include_md!( \"{abs}\" ); }}" );
  let result = check_compile_fails( &code, &[ "enabled" ] );
  let _ = std ::fs ::remove_dir_all( &tmp_dir );
  assert!( result, "include_md! must not compile when the file exceeds 10 MB" );
}

/// File with invalid UTF-8 bytes is rejected at compile time via `include_str!`.
#[ test ]
fn invalid_utf8_is_compile_error()
{
  let tmp_dir = std ::env ::temp_dir().join
  (
    format!( "include_md_utf8_{}", std ::process ::id() )
  );
  std ::fs ::create_dir_all( &tmp_dir ).expect( "create temp dir" );
  let invalid_file = tmp_dir.join( "invalid.bin" );
  // 0xFF is never valid in UTF-8; `include_str!` will reject it at compile time.
  std ::fs ::write( &invalid_file, [ 0xFF_u8, 0xFE, 0x00 ] ).expect( "write invalid UTF-8" );

  let abs = invalid_file.to_string_lossy().into_owned();
  let code = format!( "fn main() {{ let _ = include_md::include_md!( \"{abs}\" ); }}" );
  let result = check_compile_fails( &code, &[ "enabled" ] );
  let _ = std ::fs ::remove_dir_all( &tmp_dir );
  assert!( result, "include_md! must not compile when the file contains invalid UTF-8" );
}
