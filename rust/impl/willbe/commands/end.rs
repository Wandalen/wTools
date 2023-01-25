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
        // Endpoint to next instruction
        // TODO: WCA: get_mut_or_insert()
        let endpoints = if let Some( endpoints ) = ctx.get_mut::< EndPointStack >()
        { endpoints }
        else
        {
          ctx.insert( EndPointStack::default() );
          ctx.get_mut::< EndPointStack >().unwrap()
        };

        let prog_state = ctx.get_mut::< wca::ProgramState >().ok_or_else( || BasicError::new( "Have no Program State" ) )?;
        endpoints.push( prog_state.current_pos - 1 );
         
        // Go to start point
        prog_state.current_pos = *point;
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
