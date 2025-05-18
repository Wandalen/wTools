//! Defines the core instruction and argument structures for unilang.
use std::collections::HashMap;
use std::borrow::Cow;
use super::error::SourceLocation;

/// Represents a single argument to a command.
#[derive(Debug, PartialEq, Clone)]
pub struct Argument<'a>
{
  /// The raw slice of the argument's name, if it's a named argument.
  pub name_slice : Option<&'a str>,
  /// The unescaped value of the argument.
  pub value : Cow<'a, str>,
  /// The location of the argument's name, if applicable.
  pub name_location : Option<SourceLocation>,
  /// The location of the argument's value.
  pub value_location : SourceLocation,
}

/// Represents a generic instruction parsed from the input.
#[derive(Debug, PartialEq, Clone)]
pub struct GenericInstruction<'a>
{
  /// The sequence of slices forming the command path.
  pub command_path_slices : Vec<&'a str>,
  /// Named arguments, keyed by their raw name slice.
  pub named_arguments : HashMap<&'a str, Argument<'a>>,
  /// Positional arguments, in the order they appeared.
  pub positional_arguments : Vec<Argument<'a>>,
  /// Indicates if help was requested for this command (e.g., via a trailing '?').
  pub help_requested : bool,
  /// The overall location span of the entire instruction.
  pub overall_location : SourceLocation,
}