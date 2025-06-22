//!
//! Core data structures for the Unilang framework.
//!

// use former::Former;

///
/// Defines a command, including its name, arguments, and other metadata.
///
#[ derive( Debug, Clone ) ]
pub struct CommandDefinition
{
  /// The name of the command.
  pub name : String,
  /// A brief description of the command.
  pub description : String,
  /// A list of arguments the command accepts.
  // #[ former( default ) ]
  pub arguments : Vec< ArgumentDefinition >,
}

///
/// Defines an argument for a command.
///
#[ derive( Debug, Clone ) ]
pub struct ArgumentDefinition
{
  /// The name of the argument.
  pub name : String,
  /// A brief description of the argument.
  pub description : String,
  /// The data type of the argument (e.g., String, Integer).
  pub kind : String,
  /// Indicates if the argument is optional.
  // #[ former( default ) ]
  pub optional : bool,
}

///
/// Represents a namespace for organizing commands.
///
#[ derive( Debug ) ]
pub struct Namespace
{
  /// The name of the namespace.
  pub name : String,
  /// A list of commands within this namespace.
  // #[ former( default ) ]
  pub commands : Vec< CommandDefinition >,
}

///
/// Represents the successful output of a command execution.
///
#[ derive( Debug ) ]
pub struct OutputData
{
  /// The content of the output.
  pub content : String,
  /// The format of the output (e.g., text, json).
  pub format : String,
}

///
/// Represents an error that occurred during command execution.
///
#[ derive( Debug ) ]
pub struct ErrorData
{
  /// A unique code for the error.
  pub code : String,
  /// A human-readable error message.
  pub message : String,
}