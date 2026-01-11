#![allow(clippy::all)]
//! # Minimal Example - The RIGHT Way
//!
//! **✅ This is the RECOMMENDED approach** - compile-time registration with zero overhead.
//!
//! ## What You DON'T Need to Write
//!
//! **❌ NO build.rs** - unilang provides this automatically
//! **❌ NO `serde_yaml`** - already included via `yaml_parser` feature
//! **❌ NO `walkdir`** - already included via `multi_file` feature
//! **❌ NO `phf`** - already included via `static_registry` feature
//! **❌ NO manual YAML parsing** - happens at compile-time automatically
//!
//! ## What You DO Need
//!
//! 1. Add `unilang = "0.30"` to Cargo.toml
//! 2. Commands are defined in root `unilang.commands.yaml` (shared across all examples)
//! 3. Write this minimal example using the shared static command registry
//!
//! ## Important: Shared Command Registry
//!
//! All examples in unilang use the SAME static command registry generated from the root
//! `unilang.commands.yaml` file. Example-specific YAML files in `examples/` directory
//! are NOT discovered by the build script (intentionally excluded at build.rs:473 to
//! prevent test fixture pollution).
//!
//! Available commands: `.version`, `.help`, `.system.status`, `.system.info`,
//! `.performance.stats`, `.test.search`
//!
//! When you run `cargo build`, unilang's build script:
//! - Discovers `unilang.commands.yaml` at crate root
//! - Generates static command definitions in `OUT_DIR/static_commands.rs`
//! - Provides zero-overhead PHF-based command lookup
//!
//! Run with: `cargo run --example 00_minimal`

use unilang::prelude::*;

// Include compile-time generated commands (created automatically by unilang's build.rs)
include!( concat!( env!( "OUT_DIR" ), "/static_commands.rs" ) );

// Fix(issue-manifest-discovery): Use existing command from shared registry instead of non-existent .greet
// Root cause: Build script excludes examples/ directory from YAML discovery (build.rs:473)
// Pitfall: Always verify example code actually runs before documenting as "recommended approach"
fn main()
{
  // Zero-cost static registry (~80ns lookup vs ~4,000ns runtime)
  // Convert static commands to CommandRegistry for Pipeline API
  let registry = CommandRegistry::from_static_commands( &STATIC_COMMANDS );
  let pipeline = Pipeline::new( registry );

  // Execute command with O(1) lookup using shared command from root unilang.commands.yaml
  let result = pipeline.process_command_simple( ".help" );

  println!( "Success: {}", result.success );
  println!( "Error: {:?}", result.error );
  println!( "Outputs count: {}", result.outputs.len() );
  if !result.outputs.is_empty()
  {
    println!( "Output: {}", result.outputs[ 0 ].content );
  }
}
