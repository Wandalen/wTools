const ASSETS_PATH : &str = "tests/assets";

use assert_fs::prelude::*;
use crate::TheModule::endpoint::{ self };

mod modules_headers_test
{
	use std::io::Read;

	use super::*;

	fn arrange( source: &str ) -> assert_fs::TempDir
	{
		let root_path = std::path::Path::new( env!( "CARGO_MANIFEST_DIR" ) );
		let assets_relative_path = std::path::Path::new( ASSETS_PATH );
		let assets_path = root_path.join( assets_relative_path );

		let temp = assert_fs::TempDir::new().unwrap();
		temp.copy_from( assets_path.join( source ), &[ "**" ] ).unwrap();

		temp
	}

	#[ test ]
  fn default_case()
	{
		// Arrange
		let temp = arrange( "single_module" );

		let expected = "<!--{ generate.module_header }-->\n[![experimental](https://raster.shields.io/static/v1?label=&message=experimental&color=orange)](https://github.com/emersion/stability-badges#experimental) | [![rust-status](https://github.com/Wandalen/wTools/actions/workflows/ModuleTestModulePush.yml/badge.svg)](https://github.com/Wandalen/wTools/actions/workflows/ModuleTestModulePush.yml)[![docs.rs](https://img.shields.io/docsrs/test_module?color=e3e8f0&logo=docs.rs)](https://docs.rs/test_module)[![Open in Gitpod](https://raster.shields.io/static/v1?label=try&message=online&color=eee&logo=gitpod&logoColor=eee)](https://gitpod.io/#RUN_PATH=.,SAMPLE_FILE=sample%2Frust%2Ftest_module_trivial_sample%2Fsrc%2Fmain.rs,RUN_POSTFIX=--example%20test_module_trivial_sample/https://github.com/Wandalen/wTools)[![discord](https://img.shields.io/discord/872391416519737405?color=eee&logo=discord&logoColor=eee&label=ask)](https://discord.gg/m3YfbXpUUY)";

		// Act
		_ = endpoint::generate_modules_headers( &temp ).unwrap();
		let mut file = std::fs::File::open( temp.path().join( "test_module" ).join( "Readme.md" ) ).unwrap();

		let mut actual = String::new();

		_ = file.read_to_string( &mut actual ).unwrap();

		// Assert
		assert_eq!( expected, actual );
	}
}