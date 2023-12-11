/// Internal namespace.
mod private 
{
  use std::path::PathBuf;

  use crate::{ wtools, endpoint, path, tools::bool_like::BoolLike };

	use anyhow::Ok;
  use wca::{ Args, Props };
  use wtools::error::Result;

	/// run tests in specified crate
	pub fn run_tests( ( args, properties ) : ( Args, Props ) ) -> Result< () >
	{
    let path : PathBuf = args.get_owned( 0 ).unwrap_or_else( || "./".into() );
    let path = path::canonicalize(path)?;
    let nightly = properties.get_owned( "nightly" ).map( | nightly : String | nightly.parse().map_or_else( | _ | BoolLike::False, | e | e ) ).unwrap_or_else( || BoolLike::False ).into();
    let exclude_features_list = properties.get_owned( "exclude" ).unwrap_or_else( || Vec::new() ).into();
    let include_features_list = properties.get_owned( "include" ).unwrap_or_else( || Vec::new() ).into();
    let parallel = properties.get_owned( "parallel" ).map( | parallel : String | parallel.parse().map_or_else( | _ | BoolLike::False, | e | e ) ).unwrap_or_else( || BoolLike::False ).into();

    match endpoint::run_tests( &path, nightly, exclude_features_list, include_features_list, parallel )
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