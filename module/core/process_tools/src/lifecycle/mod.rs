/// Define a private namespace for all its items.
mod private {}

crate ::mod_interface!
{
  /// POSIX signal name/number bidirectional mapping.
  layer signal;

  /// Process existence detection via `kill(pid, 0)`.
  layer check;

  /// Unix process daemonization and PID file management.
  #[ cfg( unix ) ]
  layer daemon;
}
