//! Build script for genfile CLI
//!
//! Currently this is a placeholder for future Multi-YAML build-time generation.
//! Command definitions exist in commands/*.yaml as specifications, but are
//! implemented in Rust for now due to unilang API limitations for external consumers.

fn main()
{
  // Tell Cargo to rerun this build script if commands/ directory changes
  println!( "cargo:rerun-if-changed=commands/" );
}
