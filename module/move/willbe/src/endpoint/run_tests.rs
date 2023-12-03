/// Internal namespace.
mod private 
{
  use std::{path::Path, collections::HashMap};

	use crate::{ wtools, process::{ self, CmdReport } };

	use wtools::error::Result;
	use anyhow::anyhow;
  use core::fmt::Formatter;

	#[ derive( Debug, Default, Clone ) ]
  pub struct TestReport
  {
    tests : HashMap<String, CmdReport>,
		package_name: String,
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
				f.write_fmt( format_args!( "\tFeature: [ {} ]:\n Tests status: {}\n", feature, result.out ) )?;
			}
			
      Ok( () )
    }
  }

	/// run all tests in all crates
	pub fn run_tests( dir : &Path, nightly : bool, exclude_features : Vec< String >, include_features : Vec< String > ) -> Result< TestReport >
	{
		let mut report = TestReport::default();

		let path = dir.join("Cargo.toml");

		let metadata = cargo_metadata::MetadataCommand::new()
		.manifest_path( &path )
		.features( cargo_metadata::CargoOpt::AllFeatures )
		.exec();

		if metadata.is_err() || metadata.as_ref().unwrap().packages.iter().find( |x| x.manifest_path == path ).is_none()
		{
			return Err( anyhow!( "Directory path is not a crate" ) );
		}
		let metadata = metadata.unwrap();

		let toolchain = if nightly 
		{
			"nightly"
		}
		else 
		{
			"stable"
		};

		report.package_name = metadata.packages.iter().find( |x| x.manifest_path == path ).unwrap().name.clone();
		
		let cmd_rep = process::start_sync( &format!( "cargo +{toolchain} test" ), dir )?;
		report.tests.insert( "All features".to_string(), cmd_rep );
		
		let features = metadata.packages.iter().find( |x| x.manifest_path == path ).unwrap().features.clone();
		let mut features = features.keys().collect::< Vec< &String > >();

		if !include_features.is_empty() 
		{
			features = include_features.iter().map( | x | x ).collect();
		}

		for feature in features 
		{
			if exclude_features.contains( &feature ) 
			{
				continue;
			}
			let cmd_rep = process::start_sync( &format!( "cargo +{toolchain} test --features {feature}" ), dir )?;
			report.tests.insert( feature.clone(), cmd_rep );
		}

		let cmd_rep = process::start_sync( &format!( "cargo +{toolchain} test --no-default-features" ), dir )?;
		report.tests.insert( "No features".to_string(), cmd_rep );

		Ok( report )
	}
}

crate::mod_interface!
{
  /// run all tests in all crates
  prelude use run_tests;
}