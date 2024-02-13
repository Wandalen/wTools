/// Internal namespace.
mod private 
{
	use crate::*;

	use core::fmt::Formatter;
  use std::
	{
		path::Path,
		collections::HashMap,
		sync::{ Arc, RwLock }
	};

	use rayon::prelude::*;
	use wtools::error::{ err, Result };
	use process::CmdReport;

	#[ derive( Debug, Default, Clone ) ]
  pub struct TestReport
  {
    tests : HashMap<String, CmdReport>,
		package_name: String,
		compilation_status: String,
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

			if self.tests.values().next().unwrap().err.contains( "toolchain 'nightly" ) 
			{
				f.write_fmt( format_args!( "unlucky, nightly not installed.\n For installation perform `rustup install nightly`" ) )?;
				return Ok( () );
			}
      
			for (feature, result) in &self.tests 
			{
				f.write_fmt( format_args!( "  Feature: [ {} ]:\n Tests status: {}\n", feature, result.out ) )?;
			}
			f.write_fmt( format_args!( "Compilation status:\n  {} ", self.compilation_status ) )?;
			
      Ok( () )
    }
  }

	/// The function runs tests with a different set of features in the selected crate (the path to the crate is specified in the dir variable). 
	/// Tests are run with each feature separately, with all features together, and without any features. 
	/// The tests are run in nightly and stable versions of Rust. 
	/// It is possible to enable and disable various features of the crate. 
	/// The function also has the ability to run tests in parallel using `Rayon` crate. 
	/// The result of the tests is written to the structure `TestReport` and returned as a result of the function execution.
	pub fn run_tests( 
		dir : &Path, 
		nightly : bool, 
		exclude_features : Vec< String >, 
		include_features : Vec< String >, 
		parallel : bool 
	) -> Result< TestReport >
	{
		let report = Arc::new( RwLock::new( TestReport::default() ) );

		let path = dir.join("Cargo.toml");

		let metadata = cargo_metadata::MetadataCommand::new()
		.manifest_path( &path )
		.features( cargo_metadata::CargoOpt::AllFeatures )
		.exec();

		if metadata.is_err() || metadata.as_ref().unwrap().packages.iter().find( |x| x.manifest_path == path ).is_none()
		{
			return Err( err!( "Directory path is not a crate" ) );
		}
		let metadata = metadata.unwrap();

		let toolchain = if nightly 
		{
			cargo::Channel::Nightly
		}
		else 
		{
			cargo::Channel::Stable
		};

		report.write().unwrap().package_name = metadata.packages.iter().find( |x| x.manifest_path == path ).unwrap().name.clone();
		
		let mut cmd_rep = cargo::test( dir, cargo::TestArgs::former().channel( toolchain ).form(), false )?;
		if cmd_rep.out.is_empty() 
		{
			cmd_rep.out = cmd_rep.err.clone();
			report.write().unwrap().compilation_status.push_str( "Error while compiling tests with feature [All features]\n" );
		}
		report.write().unwrap().tests.insert( "All features".to_string(), cmd_rep );
		
		let features = metadata.packages.iter().find( |x| x.manifest_path == path ).unwrap().features.clone();
		let mut features = features.keys().collect::< Vec< &String > >();

		if !include_features.is_empty() 
		{
			features = include_features.iter().map( | x | x ).collect();
		}

		if parallel 
		{
			features
			.par_iter()
			.for_each( |feature| 
				{
					if exclude_features.contains( &feature ) 
					{
						return;
					}
					let mut cmd_rep = cargo::test
					(
						dir,
						cargo::TestArgs::former()
						.channel( toolchain )
						.with_default_features( false )
						.enable_features([ ( *feature ).clone() ])
						.form(),
						false
					).unwrap();
					if cmd_rep.out.is_empty()
					{
						cmd_rep.out = cmd_rep.err.clone();
						report.write().unwrap().compilation_status.push_str( &format!( "Error while compiling tests with feature [{}]\n", feature ) );
					}
					report.write().unwrap().tests.insert( feature.to_string(), cmd_rep );
				}
			);
		}
		else 
		{
			for feature in features
			{
				if exclude_features.contains( &feature ) 
				{
					continue;
				}
				let mut cmd_rep = cargo::test
				(
					dir,
					cargo::TestArgs::former()
					.channel( toolchain )
					.with_default_features( false )
					.enable_features([ ( *feature ).clone() ])
					.form(),
					false
				).unwrap();
				if cmd_rep.out.is_empty()
				{
					cmd_rep.out = cmd_rep.err.clone();
					report.write().unwrap().compilation_status.push_str( &format!( "Error while compiling tests with feature [{}]\n", feature ) );
				}
				report.write().unwrap().tests.insert( feature.clone(), cmd_rep );
			}
		}
		
		let mut cmd_rep = cargo::test
		(
			dir,
			cargo::TestArgs::former()
			.channel( toolchain )
			.with_default_features( false )
			.form(),
			false
		).unwrap();
		if cmd_rep.out.is_empty()
		{
			cmd_rep.out = cmd_rep.err.clone();
			report.write().unwrap().compilation_status.push_str( "Error while compiling tests with feature [No features]\n" );
		}
		report.write().unwrap().tests.insert( "No features".to_string(), cmd_rep );
		if report.read().unwrap().compilation_status.is_empty() 
		{
			report.write().unwrap().compilation_status.push_str( "Compilation of all tests with each feature variant was successful\n" );
		}

		let report_lock = report.read().unwrap();
		let test_report: &TestReport = &*report_lock;
		Ok( test_report.clone() )
	}
}

crate::mod_interface!
{
  /// run all tests in all crates
  prelude use run_tests;
}