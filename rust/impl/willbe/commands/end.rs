/// Internal namespace.
pub( crate ) mod private
{
  use wtools::error::BasicError;
  use wca::
  {
    Args,
    NoSubject, NoProperties,
    Context,
  };

  use crate::commands::{ StartPointStack, EndPointStack };

  ///
  /// End of loop/program
  ///

  pub fn end( _ : Args< NoSubject, NoProperties >, ctx : Context ) -> Result< (), BasicError >
  {
    println!( "[LOG] end called" );

    if let Some( startpoints ) = ctx.get_ref::< StartPointStack >()
    {
      if let Some( point ) = startpoints.last()
      {
        let prog_state = ctx.get_mut::< wca::ProgramState >()
        .ok_or_else( || BasicError::new( "Have no Program State" ) )?;

        let endpoints = ctx.get_or_default::< EndPointStack >();
        // if has no point at current instruction - push it
        if endpoints.last() != Some( &( prog_state.get_pos() - 1 ) )
        {
          endpoints.push( prog_state.get_pos() - 1 );
        }
         
        // Go to start point
        prog_state.set_pos( *point );
      }
    }

    Ok( () )
  }
}

//

crate::mod_interface!
{
  prelude use end;
}
