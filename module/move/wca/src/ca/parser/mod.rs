crate::mod_interface!
{
  /// Parser configuration
  layer parser;
  /// Implementation for parsing command
  layer command;
  /// Implementation for parsing namespace
  layer namespace;
  /// Implementation for parsing program
  layer program;
  /// Entities representation to interact with
  layer entities;
}
