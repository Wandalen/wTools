crate::mod_interface!
{

  /// Executor that is responsible for executing the program’s commands
  layer executor;
  /// Represents the state of the program's runtime
  layer runtime;
  /// Container for contexts values
  layer context;
  /// Command callback representation
  layer routine;

}
