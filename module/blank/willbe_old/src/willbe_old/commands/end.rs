/// Internal namespace.
pub( crate ) mod private
{
  use wca::
  {
    Args, Props,
    Context,
  };
  use error_tools::{ Result, BasicError };

  use crate::commands::{ StartPointStack, EndPointStack };

  // ! TODO: Remove this command somehow

  /// End command declaration
  pub fn end_command() -> wca::Command
  {
    wca::Command::former()
    .hint( "Command that is end of a block or a program" )
    .phrase( "end" )
    .form()
  }

  ///
  /// End of loop/program
  ///

  pub fn end( _ : ( Args, Props ), ctx : Context ) -> Result< () >
  {
    println!( "[LOG] end called" );

    if let Some( startpoints ) = ctx.get_ref::< StartPointStack >()
    {
      if let Some( point ) = startpoints.last()
      {
        let prog_state = ctx.get_mut::< wca::RuntimeState >()
        .ok_or_else( || BasicError::new( "Have no Program State" ) )?;

        let endpoints = ctx.get_or_default::< EndPointStack >();
        // if has no point at current instruction - push it
        if endpoints.last() != Some( &( prog_state.pos - 1 ) )
        {
          endpoints.push( prog_state.pos - 1 );
        }
         
        // Go to start point
        prog_state.pos = *point;
      }
    }

    Ok( () )
  }
}

//

crate::mod_interface!
{
  prelude use end_command;
  prelude use end;
}
