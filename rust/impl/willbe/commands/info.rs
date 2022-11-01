/// Internal namespace.
pub( crate ) mod private
{
  use crate::protected::*;
  use std::env;
  use wtools::{ error::BasicError, err };

  ///
  /// Prints information about package
  /// 

  pub fn info( instruction : &crate::instruction::Instruction ) -> Result< (), BasicError >
  {
    dbg!( &instruction );

    let current_path = env::current_dir().unwrap();

    let package = Package::try_from( current_path )
    .or( Err( err!( "Package not found at current directory" ) ) )?;
    dbg!( package.info() );
    Ok( () )
  }
}

//

crate::mod_interface!
{
  prelude use info;
}
