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

  /// Workspace' graph.
  layer workspace_graph;
  orphan use super::workspace_graph;

  /// Packages of workspace.
  layer workspace_package;
  orphan use super::workspace_package;

  /// Dependency of a package.
  layer dependency;
  orphan use super::dependency;

  /// To manipulate manifest data.
  layer manifest;
  orphan use super::manifest;

  /// Paths and files.
  layer files;
  orphan use super::files;

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

  /// Md's extension for workspace.
  layer workspace_md_extension;
  orphan use super::workspace_md_extension;

  /// Md's extension for workspace.
  layer package_md_extension;
  orphan use super::package_md_extension;

}
