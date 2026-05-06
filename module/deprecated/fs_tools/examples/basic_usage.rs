//! Basic usage example for the `fs_tools` crate.
//!
//! Demonstrates the `file_tools` API re-exported through `fs_tools`:
//! path traversal (searching upward for files/directories) and
//! temporary directory construction.
//!
//! ## Running This Example
//!
//! ```bash
//! cargo run --example basic_usage --features full
//! ```

#[ cfg( feature = "enabled" ) ]
fn main()
{
  // --- Path traversal: search upward for a file ---
  let cargo_path = fs_tools::path::file_upward_find
  (
    std::path::Path::new( "." ),
    "Cargo.toml",
    10,
  );
  println!( "Cargo.toml found at: {cargo_path:?}" );

  // --- Path traversal: search upward for a directory ---
  let src_path = fs_tools::path::dir_upward_find
  (
    std::path::Path::new( "." ),
    "src",
    10,
  );
  println!( "src/ found at: {src_path:?}" );

  // --- TempDir: construct a temporary directory path ---
  let mut tmp = fs_tools::fs::TempDir::new();
  tmp.base_path = std::env::temp_dir();
  tmp.prefix_path = std::path::PathBuf::from( "fs_tools_example_" );
  println!( "TempDir path: {}", tmp.full_path().display() );
}

#[ cfg( not( feature = "enabled" ) ) ]
fn main()
{
  println!( "Example requires 'enabled' feature." );
  println!( "Run with: cargo run --example basic_usage --features enabled" );
}
