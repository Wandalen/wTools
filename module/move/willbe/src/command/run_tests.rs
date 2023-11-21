/// Internal namespace.
mod private 
{
  use std::path::PathBuf;

  use crate::{ wtools, endpoint, path, bool::{BoolLike, ToBoolLike} };

	use anyhow::Ok;
  use wca::{ Args, Props };
  use wtools::error::Result;

	/// run all tests in all crates
	pub fn run_tests( ( args, properties ) : ( Args, Props ) ) -> Result< () >
	{
		let path : PathBuf = args.get_owned( 0 ).unwrap_or_else(|| "./".into() );
    let path = path::canonicalize(path)?;
    let nightly = properties.get_owned( "nightly" ).map( | nightly : String | nightly.to_bool_like() ).unwrap_or_else( || BoolLike::True ).into();

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
  /// run all tests in all crates
  prelude use run_tests;
}