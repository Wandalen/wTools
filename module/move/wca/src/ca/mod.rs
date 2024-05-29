
crate::mod_interface!
{

  /// Performs validation and type casting on commands values
  layer grammar;
  /// This component is responsible for parsing the raw string into `ParsedCommand`
  layer parser;
  /// Verify parsed command and convert to an appropriate type.
  layer verifier;
  /// This component is responsible for performing
  layer executor;

  /// Provides functionality for working with input data, including asking user questions and converting various string representations into a uniform `Input` struct.
  layer input;
  
  // /// The missing batteries of WCA.
  // layer facade;
  /// Genera-purpose tools which might be moved out one day.
  layer tool;

  /// Responsible for aggregating all commands that the user defines, and for parsing and executing them
  layer aggregator;
  /// This module provides functionality for generating help content for commands.
  layer help;
  /// Responsible for generating Markdown formatted documentation for commands
  layer formatter;
  // aaa : for Bohdan : write concise documentations
  // aaa : Is this enough or is more needed?

}
