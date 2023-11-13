/// Internal namespace.
mod private {
  use std::path::PathBuf;

  use crate::{ wtools, endpoint };

	use anyhow::Ok;
  use wca::{ Args, Props };
  use wtools::error::Result;

	/// run all tests in all crates
	pub fn run_tests( ( args, _ ) : ( Args, Props ) ) -> Result< () >
	{
		let mut patterns : Vec< PathBuf > = args.get_owned( 0 ).unwrap_or_default();
		if patterns.is_empty()
    {
      patterns.push( "./".into() );
    }

		for pattern in patterns
    {
      match endpoint::run_tests( &pattern )
      {
        core::result::Result::Ok( report ) =>
        {
          println!( "{report:#?} ");
        }
        Err( e ) =>
        {
          return Err( e.context( "package test command" ) );
        }
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