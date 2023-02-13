pub( crate ) mod private
{
  use crate::{ Context, Args, Props, Routine, Namespace, ExecutableCommand };

  use wtools::{ Result, err };

  /// Program runtime
  /// 
  /// Cloned Runtime will work with the same context
  #[ derive( Debug, Clone ) ]
  pub struct Runtime
  {
    /// context for current runtime
    pub context : Context,
    /// current execution position
    pub pos : usize,
    /// namespace which must be executed
    pub namespace : Namespace< ExecutableCommand >,
  }

  impl Runtime
  {
    /// returns true if execution position at the end
    pub fn is_finished( &self ) -> bool
    {
      self.namespace.commands.len() == self.pos
    }

    /// executes current command and go to the next one
    pub fn r#do( &mut self ) -> Result< () >
    {
      self.namespace.commands
      .get( self.pos )
      .ok_or_else( || err!( "No command here. Current execution pos was `{}`", self.pos ) )
      .and_then( | cmd |
      {
        self.pos += 1;
        _exec_command( cmd.clone(), self.context.clone() )
      })
    }
  }

  /// executes a program
  pub fn _exec_command( command : ExecutableCommand, ctx : Context ) -> Result< () >
  {
    match command.routine
    {
      Routine::WithoutContext( routine ) => routine(( Args( command.subjects ), Props( command.properties ) )),
      Routine::WithContext( routine ) => routine( ( Args( command.subjects ), Props( command.properties ) ), ctx ),
    }
  }
}

//

crate::mod_interface!
{
  prelude use Runtime;
  protected use _exec_command;
}