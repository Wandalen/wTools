//! Trivial example demonstrating `include_md!` and `include_md_section!`.
//!
//! Run with:
//!
//! ```shell
//! cargo run --example include_md_trivial --features enabled
//! ```

fn main()
{
  // Include a complete markdown file at compile time.
  // Path resolves relative to this source file (same semantics as `include_str!`).
  let full = include_md::include_md!( "../readme.md" );
  println!( "readme.md ({} bytes):", full.len() );
  println!( "{}", &full[ ..full.len().min( 120 ) ] );
  println!( "...\n" );

  // Include a single named section from a markdown file at compile time.
  // Path resolves relative to CARGO_MANIFEST_DIR (the crate root).
  let section = include_md::include_md_section!( "readme.md", "## Quick Start" );
  println!( "## Quick Start section:" );
  println!( "{section}" );
}
