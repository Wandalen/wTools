/// Internal namespace.
pub( crate ) mod private
{
  use std::{ env, rc::Rc, cell::RefCell, };
  use wca::
  {
    Args, Props,
    Context,
  };
  use error_tools::{ Result, BasicError };

  use crate::protected::*;
  use crate::commands::{ StartPointStack, EndPointStack };

  #[ derive( Clone ) ]
  struct PackagesIterator
  (
    Rc< RefCell< dyn Iterator< Item = Package > > >
  );

  impl< I > From< I > for PackagesIterator
  where
    I : Iterator< Item = Package > + 'static
  {
    fn from( iter : I ) -> Self
    {
      Self( Rc::new( RefCell::new( iter ) ) )
    }
  }

  impl PackagesIterator
  {
    fn next( &self ) -> Option< Package >
    {
      self.0.borrow_mut().next()
    }
  }

  /// Each command declaration
  pub fn each_command() -> wca::Command
  {
    wca::Command::former()
    .hint( "Iterate over packages" )
    .long_hint( "Iterates over all packages from current directory" )
    .phrase( "each" )
    .form()
  }

  ///
  /// Iterate over packages
  ///

  pub fn each( _ : ( Args, Props ), mut ctx : Context ) -> Result< () >
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
        let prog_state = ctx.get_mut::< wca::RuntimeState >()
        .ok_or_else( || BasicError::new( "Have no Program State" ) )?;

        ctx.get_mut::< EndPointStack >()
        .and_then( | ep | ep.pop() )
        .map( | point | prog_state.pos = point )
        //? What is better - panic or go to the end of the program when endpoints doesn't exists for any reason
        .unwrap_or_else( || prog_state.pos = usize::MAX );
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
      ctx.insert::< PackagesIterator >( packages_iter.into() );

      // Start point to previous instruction( back to current )
      let prog_state = ctx.get_ref::< wca::RuntimeState >()
      .ok_or_else( || BasicError::new( "Have no Program State" ) )?;
      ctx.get_or_default::< StartPointStack >().push( prog_state.pos - 1 );
    }

    Ok( () )
  }
}

//

crate::mod_interface!
{
  prelude use each_command;
  prelude use each;
}
