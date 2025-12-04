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
//! 2. Create `00_minimal.commands.yaml` with command definitions
//! 3. Write this 15-line example
//!
//! When you run `cargo build`, you'll see unilang's build output showing:
//! - YAML files discovered
//! - Commands generated
//! - Confirmation you didn't need build.rs
//!
//! Run with: `cargo run --example 00_minimal`

use unilang::prelude::*;

// Include compile-time generated commands (created automatically by unilang's build.rs)
include!( concat!( env!( "OUT_DIR" ), "/static_commands.rs" ) );

fn main()
{
  // Zero-cost static registry (~80ns lookup vs ~4,000ns runtime)
  // Convert static commands to CommandRegistry for Pipeline API
  let registry = CommandRegistry::from_static_commands( &STATIC_COMMANDS );
  let pipeline = Pipeline::new( registry );

  // Execute command with O(1) lookup
  let result = pipeline.process_command_simple( ".greet name::Alice" );

  println!( "Success: {}", result.success );
  println!( "Error: {:?}", result.error );
  println!( "Outputs count: {}", result.outputs.len() );
  if !result.outputs.is_empty()
  {
    println!( "Output: {}", result.outputs[ 0 ].content );
  }
}
