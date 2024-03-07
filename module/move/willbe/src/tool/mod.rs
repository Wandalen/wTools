
crate::mod_interface!
{

  /// Make sha-1 hash for data.
  layer sha;
  orphan use super::sha;

  /// Operate over files.
  layer files;
  orphan use super::files;

  /// Run external processes.
  layer process;
  orphan use super::process;

  /// Work with paths.
  layer path;
  orphan use super::path;

  /// Tools for working with dependencies graph.
  layer graph;
  orphan use super::graph;

  /// Traits and structs for templates.
  layer template;
  orphan use super::template;

  /// Git interaction module that enables seamless integration and management of version control workflows.
  layer git;
  orphan use super::git;

  /// Interaction module with the `cargo` utilities.
  layer cargo;
  orphan use super::cargo;

  /// Rust toolchain channel: stable/nightly.
  layer channel;
  orphan use super::channel;

}
