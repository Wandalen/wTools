
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

  // /// This component is responsible for aggregating all commands
  // layer commands_aggregator;

  /// User input
  layer input;
  // /// The missing batteries of WCA.
  // layer facade;
  /// Genera-purpose tools which might be moved out one day.
  layer tool;

  /// Responsible for aggregating all commands that the user defines, and for parsing and executing them
  layer aggregator;
  /// Helper commands
  layer help;
  /// Responsible for generating Markdown formatted documentation for commands
  layer formatter;
  // qqq : for Bohdan : write concise documentations

}
