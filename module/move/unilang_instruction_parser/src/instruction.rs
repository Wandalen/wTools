//! Defines the core instruction and argument structures for unilang.
use std::collections::HashMap;
// Cow is no longer needed here as we will use owned Strings for arguments
// use std::borrow::Cow;
use super::error::SourceLocation;

/// Represents a single argument to a command.
/// Values are stored as owned `String`s.
#[derive(Debug, PartialEq, Clone)]
pub struct Argument // Removed lifetime 'a
{
  /// The name of the argument, if it's a named argument. Owned.
  pub name : Option<String>, // Changed from name_slice: Option<Cow<'a, str>>
  /// The unescaped value of the argument. Owned.
  pub value : String, // Changed from Cow<'a, str>
  /// The location of the argument's name, if applicable.
  pub name_location : Option<SourceLocation>,
  /// The location of the argument's value.
  pub value_location : SourceLocation,
}

/// Represents a generic instruction parsed from the input.
/// Argument names and values are stored as owned `String`s.
#[derive(Debug, PartialEq, Clone)]
pub struct GenericInstruction // Removed lifetime 'a
{
  /// The sequence of strings forming the command path. (Owned)
  pub command_path_slices : Vec<String>,
  /// Named arguments, keyed by their name. (Owned key and Argument)
  pub named_arguments : HashMap<String, Argument>, // Use Argument
  /// Positional arguments, in the order they appeared. (Owned Argument)
  pub positional_arguments : Vec<Argument>,      // Use Argument
  /// Indicates if help was requested for this command (e.g., via a trailing '?').
  pub help_requested : bool,
  /// The overall location span of the entire instruction.
  pub overall_location : SourceLocation,
}