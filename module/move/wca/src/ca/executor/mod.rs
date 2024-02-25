crate::mod_interface!
{

  /// Executor that is responsible for executing the program’s commands
  layer executor;
  /// Represents the state of the program's runtime
  layer runtime;
  /// Converts from `VerifiedCommand` to `ExecutableCommand_`
  layer converter;
  /// Container for contexts values
  layer context;
  /// `ExecutableCommand_` representation
  layer command;
  /// Command callback representation
  layer routine;

}
