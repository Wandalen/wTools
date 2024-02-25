pub( crate ) mod private
{
  use crate::*;
  use wtools::{ error::Result, err };

  /// State of a program runtime
  ///
  /// `RuntimeState` contains information about the current state of a running program.
  /// It is used to store information that can be modified during program execution.
  ///
  /// Can be used to change execution position at runtime.
  ///
  /// # Examples
  ///
  /// ```
  /// # use wca::RuntimeState;
  /// let mut state = RuntimeState::default();
  ///
  /// state.pos = 5; // modify current execution position
  ///
  /// assert_eq!( state.pos, 5 );
  /// ```
  #[ derive( Debug, Default, Clone ) ]
  pub struct RuntimeState
  {
    /// current execution position that can be changed by user
    pub pos : usize,
  }
  // qqq : for Bohdan : why? how is it useful? is it?

  /// Represents the state of the program's runtime, including the current context, execution position, and namespace of executable commands.
  ///
  /// Cloned Runtime will work with the same context.
  ///
  /// It performs callbacks to commands at the current execution position and, if necessary, provides context for them.
  ///
  /// ```
  /// # use wca::{ Runtime, Namespace, Context };
  /// let runtime = Runtime
  /// {
  ///   context : Context::default(),
  ///   pos : 0,
  ///   namespace : Namespace
  ///   {
  ///     commands: vec![]
  ///   }
  /// };
  ///
  /// assert!( runtime.is_finished() );
  /// ```
  #[ derive( Debug, Clone ) ]
  pub struct Runtime
  {
    /// context for current runtime
    pub context : Context,
    /// current execution position
    pub pos : usize,
    /// namespace which must be executed
    pub namespace : Namespace< ExecutableCommand_ >, // qqq : for Bohdan : use VerifiedCommand
  }
  // qqq : for Bohdan : why both Runtime and RuntimeState exist? probably one should removed
  // qqq : for Bohdan : why both Runtime and Context exist? What about incapsulating Context into Runtime maybe
  // qqq : for Bohdan : why both Runtime and Executor exist? rid off of Executor. Incapsulating Executor into Runtime.

  impl Runtime
  {
    /// returns true if execution position at the end
    pub fn is_finished( &self ) -> bool
    {
      self.namespace.commands.len() == self.pos
    }

    /// executes current command( command at current execution position )
    pub fn r#do( &mut self ) -> Result< () >
    {
      self.namespace.commands
      .get( self.pos )
      .ok_or_else( || err!( "No command here. Current execution pos was `{}`", self.pos ) )
      .and_then( | cmd |
      {
        _exec_command( cmd.clone(), self.context.clone() )
      })
    }
  }

  // qqq : for Bohdan : _exec_command probably should be method of Runtime.
  // qqq : for Bohdan : Accept reference instead of copy.
  /// executes a command
  pub fn _exec_command( command : ExecutableCommand_, ctx : Context ) -> Result< () >
  {
    match command.routine
    {
      Routine::WithoutContext( routine ) => routine( ( Args( command.subjects ), Props( command.properties ) )),
      Routine::WithContext( routine ) => routine( ( Args( command.subjects ), Props( command.properties ) ), ctx ),
    }
  }
}

//

crate::mod_interface!
{
  exposed use RuntimeState;
  exposed use Runtime;
  protected use _exec_command;
}
