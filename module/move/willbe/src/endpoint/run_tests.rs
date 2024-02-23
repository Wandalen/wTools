/// Internal namespace.
mod private 
{
	use crate::*;

	use core::fmt::Formatter;
  use std::
	{
		collections::{ BTreeMap, BTreeSet, HashSet },
		sync::{ Arc, Mutex },
	};

	use rayon::ThreadPoolBuilder;
	use former::Former;
	use wtools::
	{
		iter::Itertools,
		error::{ Result, for_app::format_err },
	};
	use process::CmdReport;

	/// Represents a report of test results.
  #[ derive( Debug, Default, Clone ) ]
  pub struct TestReport
  {
    /// A string containing the name of the package being tested.
		pub package_name : String,
    /// A `BTreeMap` where the keys are `cargo::Channel` enums representing the channels
    ///   for which the tests were run, and the values are nested `BTreeMap` where the keys are
    ///   feature names and the values are `CmdReport` structs representing the test results for
    ///   the specific feature and channel.
    pub tests : BTreeMap< cargo::Channel, BTreeMap< String, CmdReport > >,
  }

  impl std::fmt::Display for TestReport
  {
    fn fmt( &self, f : &mut Formatter< '_ > ) -> std::fmt::Result
    {
			f.write_fmt( format_args!( "Package: [ {} ]:\n", self.package_name ) )?;
			if self.tests.is_empty() 
			{
				f.write_fmt( format_args!( "unlucky" ) )?;
				return Ok( () );
			}

			for ( channel, features ) in &self.tests
			{
				for (feature, result) in features
				{
					// if tests failed or if build failed
					let failed = result.out.contains( "failures" ) || result.err.contains( "error" );
					if !failed
					{
						let feature = if feature.is_empty() { "no-features" } else { feature };
						f.write_fmt(format_args!("  [ {} | {} ]: {}\n", channel, feature, if failed { "❌ failed" } else { "✅ successful" } ) )?;
					}
					else
					{
						let feature = if feature.is_empty() { "no-features" } else { feature };
						f.write_fmt( format_args!( "  Feature: [ {} | {} ]:\n  Tests status: {}\n{}\n{}", channel, feature, if failed { "❌ failed" } else { "✅ successful" }, result.out, result.err ) )?;
					}
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
	pub struct TestsArgs
	{
		dir : CrateDir,
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
	pub fn run_tests( args : TestsArgs ) -> Result< TestReport >
	{
		// fail fast if some additional installations required
		let channels = cargo::available_channels( args.dir.as_ref() )?;
		let channels_diff = args.channels.difference( &channels ).collect::< Vec< _ > >();
		if !channels_diff.is_empty()
		{
			return Err( format_err!( "Missing toolchain(-s) that was required: [{}]. Try to install it with `rustup install {{toolchain name}}` command(-s)", channels_diff.into_iter().join( ", " ) ) )
		}

		let report = Arc::new( Mutex::new( TestReport::default() ) );

		let path = args.dir.absolute_path().join("Cargo.toml");
		let metadata = Workspace::with_crate_dir( args.dir.clone() )?;

		let package = metadata.packages_get()?.into_iter().find( |x| x.manifest_path == path.as_ref() ).ok_or( format_err!( "Package not found" ) )?;
		report.lock().unwrap().package_name = package.name.clone();

		let exclude = args.exclude_features.iter().cloned().collect();
		let features_powerset = package
		.features
		.keys()
		.filter( | f | !args.exclude_features.contains( f ) && !args.include_features.contains( f ) )
		.cloned()
		.powerset()
		.map( BTreeSet::from_iter )
		.filter( | subset | subset.len() <= args.power as usize )
		.map( | mut subset | { subset.extend( args.include_features.clone() ); subset.difference( &exclude ).cloned().collect() } )
		.collect::< HashSet< BTreeSet< String > > >();

		let mut pool = ThreadPoolBuilder::new().use_current_thread();
		pool = if args.parallel { pool } else { pool.num_threads( 1 ) };
		let pool = pool.build().unwrap();

		pool.scope( | s |
		{
			let dir = &args.dir;
			for channel in args.channels
			{
				for feature in &features_powerset
				{
					let r = report.clone();
					s.spawn( move | _ |
					{
						let cmd_rep = cargo::test( dir, cargo::TestArgs::former().channel( channel ).with_default_features( false ).enable_features( feature.clone() ).form(), false ).unwrap_or_else( | rep | rep.downcast().unwrap() );
						r.lock().unwrap().tests.entry( channel ).or_default().insert( feature.iter().join( "," ), cmd_rep );
					});
				}
			}
		});

		// unpack. all tasks must be completed until now
		let report = Mutex::into_inner( Arc::into_inner( report ).unwrap() ).unwrap();

		Ok( report )
	}
}

crate::mod_interface!
{
  /// run all tests in all crates
  prelude use run_tests;
	protected use TestsArgs;
  protected use TestReport;
}