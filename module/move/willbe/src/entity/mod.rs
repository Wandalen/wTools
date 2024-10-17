mod private {}

crate::mod_interface!
{
  /// Rust toolchain channel: stable/nightly.
  layer channel;
  orphan use super::channel;

  /// Source code.
  layer code;
  orphan use super::code;

  /// Dependency of a package.
  layer dependency;
  orphan use super::dependency;

  /// Compare two crate archives and create a difference report.
  layer diff;
  orphan use super::diff;

  /// Operation with features
  layer features;
  orphan use super::features;

  /// Paths and files.
  layer files;
  orphan use super::files;

  /// Git.
  layer git;
  orphan use super::git;

  /// To manipulate manifest data.
  layer manifest;
  orphan use super::manifest;

  /// Rust build optimization: debug/release
  layer optimization;
  orphan use super::optimization;

  /// Offers capabilities for package management, facilitating the handling and organization of packages.
  layer package;
  orphan use super::package;

  /// Md's extension for workspace.
  layer package_md_extension;
  orphan use super::package_md_extension;

  /// Provides a set of functionalities for handling and manipulating packages.
  layer packages;
  orphan use super::packages;

  /// Handles operations related to packed Rust crates
  layer packed_crate;
  orphan use super::packed_crate;

  /// Progress bar staff.
  layer progress_bar;
  orphan use super::progress_bar;

  /// Publish.
  layer publish;
  orphan use super::publish;

  /// Facade for `preatytable` crate.
  layer table;
  orphan use super::table;

  /// Operations with tests
  layer test;
  orphan use super::test;

  /// Provides an opportunity to work with versions.
  layer version;
  orphan use super::version;

  /// It features the ability to interact with workspaces, manage their participants, and other functionalities.
  layer workspace;
  orphan use super::workspace;

  /// Workspace' graph.
  layer workspace_graph;
  orphan use super::workspace_graph;

  /// Md's extension for workspace.
  layer workspace_md_extension;
  orphan use super::workspace_md_extension;

  /// Packages of workspace.
  layer workspace_package;
  orphan use super::workspace_package;
}
