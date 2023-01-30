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

  impl std::ops::Deref for PackagesIterator
  {
    type Target = Box< dyn Iterator< Item = Package > >;

    fn deref( &self ) -> &Self::Target
    {
      &self.0
    }
  }

  impl std::ops::DerefMut for PackagesIterator
  {
    fn deref_mut( &mut self ) -> &mut Self::Target
    {
      &mut self.0
    }
  }

  ///
  /// Iterate over packages
  ///

  pub fn each( _ : Args< NoSubject, NoProperties >, mut ctx : Context ) -> Result< (), BasicError >
  {
    println!( "[LOG] Called each command" );

    // Already iterate
    if let Some( iter ) = ctx.get_mut::< PackagesIterator >()
    {
      // It isn't end of iterator
      let is_current_package_exists = ctx.get_ref::< Option< Package > >().and_then( | p | p.as_ref() ).is_some();
      let next_package = iter.next();
      if is_current_package_exists && next_package.is_some()
      {
        ctx.insert( next_package );
      }
      else
      {
        ctx.remove::< Option< Package > >();
        ctx.remove::< PackagesIterator >();
        // At the end of each - go to first endpoint
        // remove self from startpoints
        ctx.get_mut::< StartPointStack >().and_then( | sp | sp.pop() );
        // go to endpoint
        let prog_state = ctx.get_mut::< wca::ProgramState >()
        .ok_or_else( || BasicError::new( "Have no Program State" ) )?;

        ctx.get_mut::< EndPointStack >()
        .and_then( | ep | ep.pop() )
        .map( | point | prog_state.set_pos( point ) )
        //? What is better - panic or go to the end of the program when endpoints doesn't exists for any reason
        .unwrap_or_else( || prog_state.finish() );
      }
    }
    else
    {
      // Begin iteration
      let current_path = env::current_dir().unwrap();
      let mut packages_iter = packages_iterate( current_path );

      let package = packages_iter.next();

      // But anyway program must found the end of `.each`
      if package.is_none()
      {
        println!( "Any package was found at current directory" );
      }

      // Add current package and the iterator to context
      ctx.insert( package );
      ctx.insert( PackagesIterator( packages_iter ) );

      // Start point to previous instruction( back to current )
      let prog_state = ctx.get_ref::< wca::ProgramState >()
      .ok_or_else( || BasicError::new( "Have no Program State" ) )?;
      ctx.get_or_default::< StartPointStack >().push( prog_state.get_pos() - 1 );
    }

    Ok( () )
  }
}

//

crate::mod_interface!
{
  prelude use each;
}
