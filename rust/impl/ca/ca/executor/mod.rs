crate::mod_interface!
{
  /// Executor that is responsible for executing the program’s commands
  layer executor;
  /// All needed for `ExecutableCommand`
  layer execute;
  /// Represents the state of the program's runtime
  layer runtime;
  /// Converts from `GrammarCommand` to `ExecutableCommand`
  layer converter;
}
