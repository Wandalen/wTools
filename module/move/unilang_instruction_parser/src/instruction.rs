//! Defines the core instruction and argument structures for unilang.
use std::collections::HashMap;
use super::error::SourceLocation;

/// Represents a single argument to a command, either positional or named.
///
/// Values are stored as unescaped, owned `String`s. The original source location
/// of both the name (if applicable) and the value are preserved for error reporting
/// and potential tooling.
#[derive(Debug, PartialEq, Clone, Eq)] // Added Eq
pub struct Argument
{
  /// The name of the argument if it's a named argument (e.g., "name" in "name::value").
  /// This is `None` for positional arguments.
  pub name : Option<String>,
  /// The unescaped value of the argument.
  /// For quoted arguments, this is the content within the quotes after escape sequences
  /// have been processed. For unquoted arguments, this is the literal token string.
  pub value : String,
  /// The location (span) of the argument's name in the original input, if applicable.
  /// This points to the "name" part of a "name::value" pair.
  pub name_location : Option<SourceLocation>,
  /// The location (span) of the argument's raw value token in the original input.
  /// For quoted values, this refers to the span including the quotes.
  pub value_location : SourceLocation,
}

/// Represents a generic instruction parsed from the input string or slice.
///
/// An instruction consists of a command path (which can be multi-segment),
/// a collection of named arguments, a list of positional arguments, a flag indicating
/// if help was requested, and the overall location of the instruction in the source.
/// All string data (paths, argument names, argument values) is owned.
#[derive(Debug, PartialEq, Clone, Eq)] // Added Eq
pub struct GenericInstruction
{
  /// A vector of strings representing the segments of the command path.
  /// For example, `command.sub_command --arg` would result in `vec!["command", "sub_command"]`.
  /// If the input was `cmd arg1`, and `arg1` is consumed by greedy path parsing, this would be `vec!["cmd", "arg1"]`.
  pub command_path_slices : Vec<String>,
  /// A hash map of named arguments.
  /// The key is the argument name (e.g., "config" for `config::"path/to/file"`),
  /// and the value is an [`Argument`] struct containing the unescaped value and locations.
  pub named_arguments : HashMap<String, Argument>,
  /// A vector of positional arguments, stored as [`Argument`] structs.
  /// These are maintained in the order they appeared in the input.
  /// The `name` field within these `Argument` structs will be `None`.
  pub positional_arguments : Vec<Argument>,
  /// Indicates if help was requested for this command, typically via a trailing `?`
  /// immediately after the command path and before any arguments.
  pub help_requested : bool,
  /// The [`SourceLocation`] span covering the entire instruction from its first token
  /// to its last token in the original input.
  pub overall_location : SourceLocation,
}