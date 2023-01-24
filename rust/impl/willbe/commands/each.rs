/// Internal namespace.
pub( crate ) mod private
{
  use std::env;
  use wtools::error::BasicError;
  use wca::
  {
    Args,
    NoSubject, NoProperties,
    Context,
  };

  use crate::protected::*;
  use crate::commands::{ StartPointStack, EndPointStack };

  struct PackagesIterator
  (
    Box< dyn Iterator< Item = Package > >
  );

  ///
  /// Iterate over packages
  ///

  pub fn each( _ : Args< NoSubject, NoProperties >, mut ctx : Context ) -> Result< (), BasicError >
  {
    println!( "[LOG] Called each command" );

    let mut is_end = false;
    // Already iterate
    if let Some( iter ) = ctx.get_mut::< PackagesIterator >()
    {
      if let Some( package ) = iter.0.next()
      {
        // Setup next package to context
        ctx.insert( package );
      }
      else
      {
        // Finish
        ctx.remove::< Package >();
        is_end = true;
      }
    }
    else
    {
      // Begin iteration
      let current_path = env::current_dir().unwrap();
      let mut packages_iter = packages_iterate( current_path )
      .into_iter();

      // If it has packages
      if let Some( package ) = packages_iter.next()
      {
      // Add current package and the iterator to context
        ctx.insert( package );
        ctx.insert( PackagesIterator( packages_iter ) );
      }
      else
      {
        println!( "Any package was found at current directory" );
        is_end = true;
      }
    }
    if is_end
    {
        let prog_state = ctx.get_mut::< wca::ProgramState >().ok_or_else( || BasicError::new( "Have no Program State" ) )?;
        // At the end of each - go to first endpoint or to the end of the program
        prog_state.current_pos = ctx
        .get_mut::< EndPointStack >()
        .and_then( | endpoints | endpoints.0.pop() )
        // TODO: WCA: prog_state - last_instruction_pos
        .unwrap_or( usize::MAX );
    }
    // Start point to previous instruction( back to current )
    // TODO: WCA: get_mut_or_insert()
    let startpoints = if let Some( startpoints ) = ctx.get_mut::< StartPointStack >()
    { startpoints }
    else
    {
      ctx.insert( StartPointStack::default() );
      ctx.get_mut::< StartPointStack >().unwrap()
    };
    let prog_state = ctx.get_ref::< wca::ProgramState >().ok_or_else( || BasicError::new( "Have no Program State" ) )?;
    startpoints.0.push( prog_state.current_pos - 1 );

    Ok( () )
  }
}

//

crate::mod_interface!
{
  prelude use each;
}
