crate::mod_interface!
{

  /// Compare two crate archives and create a difference report.
  layer diff;
  orphan use super::diff;

  /// Operation with features
  layer features;
  orphan use super::features;

  /// Handles operations related to packed Rust crates
  layer packed_crate;
  orphan use super::packed_crate;
  
  /// Facade for `preatytable` crate.
  layer table;
  orphan use super::table;

  /// Provides a set of functionalities for handling and manipulating packages.
  layer packages;
  orphan use super::packages;

  /// Offers capabilities for package management, facilitating the handling and organization of packages.
  layer package;
  orphan use super::package;

  /// It features the ability to interact with workspaces, manage their participants, and other functionalities.
  layer workspace;
  orphan use super::workspace;

  /// To manipulate manifest data.
  layer manifest;
  orphan use super::manifest;

  /// Provides an opportunity to work with versions.
  layer version;
  orphan use super::version;

  /// Operations with tests
  layer test;
  orphan use super::test;
  
  /// Rust toolchain channel: stable/nightly.
  layer channel;
  orphan use super::channel;
  
  /// Rust build optimization: debug/release
  layer optimization;
  orphan use super::optimization;
}
