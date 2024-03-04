/// Internal namespace.
mod private
{
  use std::collections::HashSet;

  use cargo_metadata::Package;

  use former::Former;
  use wtools::
  {
    error::
    {
      for_app::
      {
        Error,
        format_err
      },
      Result
    },
    iter::Itertools,
  };

  use crate::*;
  use crate::path::AbsolutePath;
  use crate::test::*;
  
	/// Used to store arguments for running tests.
	///
	/// - The `dir` field represents the directory of the crate under test.
	/// - The `channels` field is a set of `Channel` enums representing the channels for which the tests should be run.
	/// - The `parallel` field determines whether the tests should be run in parallel or not.
	/// - The `exclude_features` field is a vector of strings representing the names of features to exclude when running tests.
	/// - The `include_features` field is a vector of strings representing the names of features to include when running tests.
	#[ derive( Debug, Former ) ]
	pub struct TestsCommandArgs
	{
		dir : AbsolutePath,
		channels : HashSet< cargo::Channel >,
		#[ default( true ) ]
		parallel : bool,
    #[ default( 1u32 ) ]
		power : u32,
		include_features : Vec< String >,
		exclude_features : Vec< String >,
	}
  
	/// The function runs tests with a different set of features in the selected crate (the path to the crate is specified in the dir variable).
	/// Tests are run with each feature separately, with all features together, and without any features.
	/// The tests are run in nightly and stable versions of Rust.
	/// It is possible to enable and disable various features of the crate.
	/// The function also has the ability to run tests in parallel using `Rayon` crate.
	/// The result of the tests is written to the structure `TestsReport` and returned as a result of the function execution.
	pub fn test( args : TestsCommandArgs, dry : bool ) -> Result< TestsReport, ( TestsReport, Error ) >
	{
    let mut reports = TestsReport::default();
		// fail fast if some additional installations required
		let channels = cargo::available_channels( args.dir.as_ref() ).map_err( | e | ( reports.clone(), e ) )?;
		let channels_diff = args.channels.difference( &channels ).collect::< Vec< _ > >();
		if !channels_diff.is_empty()
		{
			return Err(( reports, format_err!( "Missing toolchain(-s) that was required: [{}]. Try to install it with `rustup install {{toolchain name}}` command(-s)", channels_diff.into_iter().join( ", " ) ) ))
		}

		reports.dry = dry;
    let TestsCommandArgs
    { 
      dir : _ , 
      channels, 
      parallel, 
      power, 
      include_features, 
      exclude_features 
    } = args;
    
    let t_args = TestArgs
    {
      channels,
      parallel,
      power,
      include_features,
      exclude_features,
    };
    let packages = needed_packages( args.dir.clone() ).map_err( | e | ( reports.clone(), e ) )?;
    
    run_tests( &t_args, &packages, dry )
	}
  
  fn needed_packages( path : AbsolutePath ) -> Result< Vec< Package > >
	{
		let path = if path.as_ref().file_name() == Some( "Cargo.toml".as_ref() )
		{
			path.parent().unwrap()
		}
		else
		{
			path
		};
		let metadata = Workspace::with_crate_dir( CrateDir::try_from( path.clone() )? )?;
		
		let result = metadata
		.packages()?
		.into_iter()
		.cloned()
		.filter( move | x | x.manifest_path.starts_with( path.as_ref() ) )
		.collect();
		Ok( result )
	}
}

crate::mod_interface!
{
  /// run all tests in all crates
  exposed use test;
	protected use TestsCommandArgs;
}
