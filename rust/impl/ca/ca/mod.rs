
crate::mod_interface!
{
  /// This component is responsible for parsing the raw string into `RawCommand`
  layer parser;
  /// Performs validation and type casting on commands values
  layer grammar;
  /// This component is responsible for performing
  layer executor;
  /// This component is responsible for aggregating all commands
  layer commands_aggregator;
  /// User input
  layer input;
}
