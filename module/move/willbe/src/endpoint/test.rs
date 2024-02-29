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
  use cargo_metadata::Package;

  use rayon::{ThreadPool, ThreadPoolBuilder};
  use former::Former;
	use wtools::
	{
		iter::Itertools,
		error::{ Result, for_app::{ format_err, Error } },
	};
	use process::CmdReport;
  use crate::cargo;
  use crate::path::AbsolutePath;
	
	/// Represents a report of test results.
  #[ derive( Debug, Default, Clone ) ]
  pub struct TestReport
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
      writeln!( f, "The tests will be executed using the following configurations:" )?;
      for ( channel, feature ) in self.tests.iter().flat_map( | ( c, f ) | f.iter().map ( |( f, _ )| ( *c, f ) ) )
      {
        writeln!( f, "channel: {channel} | feature(-s): [{}]", if feature.is_empty() { "no-features" } else { feature } )?;
      }
			writeln!( f, "\nPackage: [ {} ]:", self.package_name )?;
			if self.tests.is_empty()
			{
				writeln!( f, "unlucky" )?;
				return Ok( () );
			}

			for ( channel, features ) in &self.tests
			{
				for ( feature, result ) in features
				{
          if self.dry
          {
            let feature = if feature.is_empty() { "no-features" } else { feature };
            writeln!( f, "[{channel} | {feature}]: `{}`", result.command )?
          }
          else
          {
            // if tests failed or if build failed
            let failed = result.out.contains( "failures" ) || result.err.contains( "error" );
            if !failed
            {
              let feature = if feature.is_empty() { "no-features" } else { feature };
              writeln!( f, "  [ {} | {} ]: {}", channel, feature, if failed { "❌ failed" } else { "✅ successful" } )?;
            }
            else
            {
              let feature = if feature.is_empty() { "no-features" } else { feature };
              write!( f, "  Feature: [ {} | {} ]:\n  Tests status: {}\n{}\n{}", channel, feature, if failed { "❌ failed" } else { "✅ successful" }, result.out, result.err )?;
            }
          }
				}
			}

      Ok( () )
    }
  }

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
        writeln!(f, "The tests have not been run.")?;
        return Ok(());
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
	pub struct TestsArgs
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
	pub fn test( args : TestsArgs, dry : bool ) -> Result< TestsReport, ( TestsReport, Error ) >
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

		let exclude = args.exclude_features.iter().cloned().collect();
    let mut pool = ThreadPoolBuilder::new().use_current_thread();
    pool = if args.parallel { pool } else { pool.num_threads( 1 ) };
    let pool = pool.build().unwrap();

    for package in needed_packages( args.dir.clone() ).map_err( | e | ( reports.clone(), e ) )?
    {
      match run_tests( &args, dry, &exclude, package, &pool )
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
    if reports.failure_reports.is_empty()
    {
      Ok( reports )
    }
    else
    {
      Err(( reports, format_err!( "Some tests was failed" ) ))
    }
	}

  fn run_tests(args : &TestsArgs, dry : bool, exclude : &BTreeSet< String >, package : Package, pool : &ThreadPool ) -> Result< TestReport, ( TestReport, Error ) >
  {
    let mut report = TestReport::default();
    report.package_name = package.name.clone(); 
    let report = Arc::new( Mutex::new( report ) );
    
    let features_powerset = package
    .features
    .keys()
    .filter( | f | !args.exclude_features.contains( f ) && !args.include_features.contains( f ) )
    .cloned()
    .powerset()
    .map( BTreeSet::from_iter )
    .filter( | subset | subset.len() <= args.power as usize )
    .map
    ( 
      | mut subset | 
      {
        subset.extend( args.include_features.clone() );
        subset.difference( &exclude ).cloned().collect()
      }
    )
    .collect::< HashSet< BTreeSet< String > > >();
    print_temp_report( &package.name, &args.channels, &features_powerset );
    pool.scope
    (
      | s |
      {
        let dir = package.manifest_path.parent().unwrap();
        for channel in args.channels.clone()
        {
          for feature in &features_powerset
          {
            
            let r = report.clone();
            s.spawn
            ( 
              move | _ | 
              {
                let cmd_rep = cargo::test( dir, cargo::TestArgs::former().channel( channel ).with_default_features( false ).enable_features( feature.clone() ).form(), dry ).unwrap_or_else( | rep | rep.downcast().unwrap() );
                r.lock().unwrap().tests.entry( channel ).or_default().insert( feature.iter().join( "," ), cmd_rep );
              }
            );
          }
        }
      }
    );

    // unpack. all tasks must be completed until now
    let report = Mutex::into_inner( Arc::into_inner( report ).unwrap() ).unwrap();
    let at_least_one_failed = report.tests.iter().flat_map( | ( _, v ) | v.iter().map( | ( _, v ) | v ) ).any( | r | r.out.contains( "failures" ) || r.err.contains( "error" ) );
    if at_least_one_failed { Err( ( report, format_err!( "Some tests was failed" ) ) ) } else { Ok( report ) }
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
  
  fn print_temp_report(package_name : &str, channels : &HashSet< cargo::Channel >, features : &HashSet< BTreeSet< String > > )
  {
    println!( "Package : {}", package_name );
    for channel in channels
    {
      for feature in features
      {
        let feature = if feature.is_empty() { "no-features".to_string() } else { feature.iter().join( "," ) };
        println!( "[{channel} | {feature}]" );
      }
    }
  }
}

crate::mod_interface!
{
  /// run all tests in all crates
  exposed use test;
	protected use TestsArgs;
  protected use TestReport;
	protected use TestsReport;
}
