//!
//! Core data structures for the Unilang framework.
//!

// use former::Former;

///
/// Defines a command, including its name, arguments, and other metadata.
///
/// This struct is the central piece of a command's definition, providing all
/// the necessary information for parsing, validation, and execution.
#[ derive( Debug, Clone/*, Former*/ ) ]
pub struct CommandDefinition
{
  /// The name of the command, used to invoke it from the command line.
  pub name : String,
  /// A brief, one-line description of what the command does.
  pub description : String,
  /// A list of arguments that the command accepts.
  // #[ former( default ) ]
  pub arguments : Vec< ArgumentDefinition >,
}

///
/// Defines an argument for a command.
///
/// Each argument has a name, a description, a data type, and can be
/// marked as optional.
#[ derive( Debug, Clone/*, Former*/ ) ]
pub struct ArgumentDefinition
{
  /// The name of the argument, used for identification.
  pub name : String,
  /// A brief description of the argument's purpose.
  pub description : String,
  /// The expected data type of the argument (e.g., "String", "Integer").
  pub kind : String,
  /// If `true`, the argument is not required for the command to execute.
  // #[ former( default ) ]
  pub optional : bool,
}

///
/// Represents a namespace for organizing commands.
///
/// Namespaces allow for grouping related commands under a common prefix,
/// improving discoverability and reducing naming conflicts.
#[ derive( Debug, Clone/*, Former*/ ) ]
pub struct Namespace
{
  /// The name of the namespace.
  pub name : String,
  /// A list of commands belonging to this namespace.
  // #[ former( default ) ]
  pub commands : Vec< CommandDefinition >,
}

///
/// Represents the successful output of a command execution.
///
/// This struct standardizes the way command results are returned, allowing
/// for consistent handling across different modalities.
#[ derive( Debug, Clone/*, Former*/ ) ]
pub struct OutputData
{
  /// The primary content of the output.
  pub content : String,
  /// The format of the content (e.g., "text", "json").
  pub format : String,
}

///
/// Represents an error that occurred during command execution.
///
/// This struct provides a standardized way to report errors, including a
/// unique, machine-readable code and a human-readable message.
#[ derive( Debug, Clone/*, Former*/ ) ]
pub struct ErrorData
{
  /// A unique, machine-readable code for the error (e.g., "COMMAND_NOT_FOUND").
  pub code : String,
  /// A human-readable message explaining the error.
  pub message : String,
}