//! Defines the core instruction and argument structures for unilang.
use std::collections::HashMap;
use std::borrow::Cow;
use super::error::SourceLocation;

// RichItem is now in item_adapter.rs

/// Represents a single argument to a command.
#[derive(Debug, PartialEq, Clone)]
pub struct Argument<'a>
{
  /// The raw slice of the argument's name, if it's a named argument.
  /// This is kept as a slice for now, assuming names are typically short and from known set.
  /// If names also need to be owned by GenericInstruction, this could become String.
  pub name_slice : Option<&'a str>,
  /// The unescaped value of the argument.
  pub value : Cow<'a, str>,
  /// The location of the argument's name, if applicable.
  pub name_location : Option<SourceLocation>,
  /// The location of the argument's value.
  pub value_location : SourceLocation,
}

/// Represents a generic instruction parsed from the input.
/// Note: Lifetime 'a is primarily for Argument values. Paths and arg names are owned.
#[derive(Debug, PartialEq, Clone)]
pub struct GenericInstruction<'a> // Still 'a due to Argument<'a>
{
  /// The sequence of strings forming the command path. (Owned)
  pub command_path_slices : Vec<String>,
  /// Named arguments, keyed by their name. (Owned key)
  pub named_arguments : HashMap<String, Argument<'a>>,
  /// Positional arguments, in the order they appeared.
  pub positional_arguments : Vec<Argument<'a>>,
  /// Indicates if help was requested for this command (e.g., via a trailing '?').
  pub help_requested : bool,
  /// The overall location span of the entire instruction.
  pub overall_location : SourceLocation,
}