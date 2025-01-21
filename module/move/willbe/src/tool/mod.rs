mod private {}

crate::mod_interface!
{

  /// Interaction module with the `cargo` utilities.
  layer cargo;
  orphan use super::cargo;

  /// Function and structures to work with collections.
  // layer collection;
  // orphan use super::collection;
  use ::collection_tools;
  // own use ::collection_tools::own::*;

  /// Errors handling.
  // layer error;
  // orphan use super::error;
  use ::error_tools;

  /// Operate over files.
  layer files;
  orphan use super::files;

  /// Http requests.
  layer http;
  orphan use super::http;

  /// Iterating things.
  layer iter;
  orphan use super::iter;

  /// Work with paths.
  layer macros;
  orphan use super::macros;

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

  /// The parse function parses an input string into a HashMap where the keys are String and the values are of type Value.
  layer query;
  orphan use super::query;

  /// Tools for parsing and extracting information from url.
  layer url;
  orphan use super::url;

  /// Tools for printing a tree
  layer tree;
  orphan use super::tree;

  /// Repository tools.
  layer repository;
  orphan use super::repository;

  exposed use ::former::
  {
    Former,
    Assign,
  };

}