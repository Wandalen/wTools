//! Data structures and builders for defining and organizing Rust program source files.
//!
//! Provides three-level hierarchy for program configuration:
//! - `Source`: Single source file (`file_path` + `data`)
//! - `Program`: Collection of source files
//! - `Plan`: Top-level execution configuration
//!
//! Uses Former pattern for ergonomic builder API. Currently implements data structures only;
//! compilation and execution features are planned for future releases.

#[ cfg( feature = "enabled" ) ]
mod private
{
}

#[ cfg( feature = "enabled" ) ]
mod_interface::mod_interface!
{
  /// Builder types for program representation.
  layer program;
}
