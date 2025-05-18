//! Defines the core instruction and argument structures for unilang.
use std::collections::HashMap;
use std::borrow::Cow;
use super::error::SourceLocation;

/// Represents a single argument to a command.
/// Values are stored as `Cow<'static, str>` because they are unescaped and thus potentially owned.
#[derive(Debug, PartialEq, Clone)]
pub struct Argument
{
  /// The name of the argument, if it's a named argument. Owned by the HashMap key in GenericInstruction.
  /// This field is Option<&str> if we want to point to the HashMap key, but that creates complex lifetimes.
  /// For simplicity now, it's not storing the name directly here if it's a named arg.
  /// The `name_location` can be used to find the name string if needed.
  pub name_slice : Option<&'static str>, // This is problematic if name is dynamic. Let's remove. Name is map key.
  /// The unescaped value of the argument. Now `'static` as it's typically owned after unescaping.
  pub value : Cow<'static, str>,
  /// The location of the argument's name, if applicable.
  pub name_location : Option<SourceLocation>,
  /// The location of the argument's value.
  pub value_location : SourceLocation,
}

/// Represents a generic instruction parsed from the input.
/// No longer generic over 'a as paths, arg names, and arg values become owned or 'static.
#[derive(Debug, PartialEq, Clone)]
pub struct GenericInstruction
{
  /// The sequence of strings forming the command path. (Owned)
  pub command_path_slices : Vec<String>,
  /// Named arguments, keyed by their name. (Owned key, Argument value is effectively 'static)
  pub named_arguments : HashMap<String, Argument>,
  /// Positional arguments, in the order they appeared. (Argument value is effectively 'static)
  pub positional_arguments : Vec<Argument>,
  /// Indicates if help was requested for this command (e.g., via a trailing '?').
  pub help_requested : bool,
  /// The overall location span of the entire instruction.
  pub overall_location : SourceLocation,
}