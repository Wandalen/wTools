/// Internal namespace.
mod private
{
  use core::fmt::Formatter;
  use std::collections::HashSet;

  use cargo_metadata::Package;
  use rayon::ThreadPoolBuilder;

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

  /// Represents a vector of reposts
	#[ derive( Debug, Default, Clone ) ]
  pub struct TestsReport 
  { 
    /// A boolean flag indicating whether or not the code is being run in dry mode.
    ///
    /// Dry mode is a mode in which the code performs a dry run, simulating the execution
    /// of certain tasks without actually making any changes. When the `dry` flag is set to
    /// `true`, the code will not perform any actual actions, but instead only output the
    /// results it would have produced.
    ///
    /// This flag can be useful for testing and debugging purposes, as well as for situations
    /// where it is important to verify the correctness of the actions being performed before
    /// actually executing them.
	  pub dry : bool,
	  /// Vector of succses reports.
	  pub succses_reports : Vec< TestReport >,
    /// Vector of failure reports.
    pub failure_reports : Vec< TestReport >,
	}
	
	impl std::fmt::Display for TestsReport
	{
		fn fmt( &self, f : &mut Formatter< '_ > ) -> std::fmt::Result
		{
      if self.succses_reports.is_empty() && self.failure_reports.is_empty()
      {
        writeln!( f, "The tests have not been run."  )?;
        return Ok( () );
      }
      if !self.succses_reports.is_empty()
      { 
        writeln!( f, "Successful:" )?;
        for report in &self.succses_reports 
        { 
          writeln!( f, "{}", report )?; 
        }
      }
      if !self.failure_reports.is_empty() 
      { 
        writeln!( f, "Failure:" )?;
        for report in &self.failure_reports 
        { 
          writeln!( f, "{}", report )?; 
        }
      }
      Ok( () )
		}
	}

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
	/// The result of the tests is written to the structure `TestReport` and returned as a result of the function execution.
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
    let TestsCommandArgs{ dir : _ , channels, parallel, power, include_features, exclude_features } = args;
    let t_args = TestsArgs
    {
      channels,
      parallel,
      power,
      include_features,
      exclude_features,
    };
    let packages = needed_packages( args.dir.clone() ).map_err( | e | ( reports.clone(), e ) )?;
    let mut pool = ThreadPoolBuilder::new().use_current_thread();
    pool = if args.parallel { pool } else { pool.num_threads( 1 ) };
    let pool = pool.build().unwrap();
    pool.scope
    ( 
      | _ |
      { 
        for package in packages 
        { 
          match run_tests( &t_args, package, dry ) 
          { 
            Ok( report ) => 
            { 
              reports.succses_reports.push( report ); 
            } 
            Err(( report, _ )) => 
            { 
              reports.failure_reports.push( report ); 
            } 
          } 
        }
      }
    );
    if reports.failure_reports.is_empty()
    {
      Ok( reports )
    }
    else
    {
      Err(( reports, format_err!( "Some tests was failed" ) ))
    }
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
	protected use TestsReport;
}
