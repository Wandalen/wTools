pub( crate ) mod private
{
  /// A type alias for `miette::Result<T, E>`.
  pub type Result< T = (), E = miette::Report > = miette::Result< T, E >;

  /// Creates a command-line interface (CLI) builder with the given initial state.
  ///
  /// This function initializes a `CommandBuilder` with the provided `state` and
  /// returns it for further configuration of the CLI.
  pub fn cli< T >( state: T ) -> CommandBuilder< T, 0 > 
  {
    CommandBuilder::with_state( state )
  }
}

crate::mod_interface!
{
  /// Macro for parsing WCA arguments.
  layer macros;
  /// This is a module for defining traits.
  layer traits;
  /// The `ir` module provides functionality for intermediate representation.
  layer ir;
}
