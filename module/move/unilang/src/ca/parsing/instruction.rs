//! Generic instruction representation for the unilang parser.

/// Represents a parsed command instruction before validation against a command registry.
#[ derive( Debug, Clone, PartialEq, Eq ) ]
pub struct GenericInstruction< 'a >
{
  /// The raw command name string (e.g., ".namespace.command").
  pub command_name : &'a str,
  /// A list of raw named arguments (key-value string pairs).
  pub named_args : Vec< ( &'a str, &'a str ) >,
  /// A list of raw positional argument strings.
  pub positional_args : Vec< &'a str >,
  /// Flag indicating if a help request was made (e.g., via "?").
  pub help_requested : bool,
}