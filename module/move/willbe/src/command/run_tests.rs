/// Internal namespace.
mod private 
{
  use std::path::PathBuf;

  use crate::{ wtools, endpoint, path, bool::{BoolLike, ToBoolLike} };

	use anyhow::{ Ok, anyhow };
  use wca::{ Args, Props };
  use wtools::error::Result;

	/// run tests in specified crate
	pub fn run_tests( ( args, properties ) : ( Args, Props ) ) -> Result< () >
	{
    if args.get_owned::< PathBuf >( 0 ).is_none() 
    {
      return Err( anyhow!( "Directory path not specified" ) );
    }
		let path : PathBuf = args.get_owned( 0 ).unwrap();
    let path = path::canonicalize(path)?;
    let nightly = properties.get_owned( "nightly" ).map( | nightly : String | nightly.to_bool_like() ).unwrap_or_else( || BoolLike::False ).into();

    match endpoint::run_tests( &path, nightly )
    {
      core::result::Result::Ok( report ) =>
      {
        println!( "{report} ");
      }
      Err( e ) =>
      {
        return Err( e.context( "package test command" ) );
      }
    }

	 	Ok(())
	}
}

crate::mod_interface!
{
  /// run tests in specified crate
  prelude use run_tests;
}