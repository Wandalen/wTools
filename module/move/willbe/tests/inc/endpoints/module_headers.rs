const ASSETS_PATH : &str = "tests/assets";

use assert_fs::prelude::*;
use crate::TheModule::endpoint::{ self };

mod modules_headers_test
{
	use std::io::Read;
	use willbe::path::AbsolutePath;

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
  fn workspace_with_one_member()
	{
		// Arrange
		let temp = arrange( "single_module" );

		let expected = "<!--{ generate.module_header.start() }-->\n[![experimental](https://raster.shields.io/static/v1?label=&message=experimental&color=orange)](https://github.com/emersion/stability-badges#experimental) | [![rust-status](https://github.com/Wandalen/wTools/actions/workflows/ModuleTestModulePush.yml/badge.svg)](https://github.com/Wandalen/wTools/actions/workflows/ModuleTestModulePush.yml)[![docs.rs](https://img.shields.io/docsrs/test_module?color=e3e8f0&logo=docs.rs)](https://docs.rs/test_module)[![Open in Gitpod](https://raster.shields.io/static/v1?label=try&message=online&color=eee&logo=gitpod&logoColor=eee)](https://gitpod.io/#RUN_PATH=.,SAMPLE_FILE=sample%2Frust%2Ftest_module_trivial_sample%2Fsrc%2Fmain.rs,RUN_POSTFIX=--example%20test_module_trivial_sample/https://github.com/Wandalen/wTools)\n<!--{ generate.module_header.end }-->";

		// Act
		_ = endpoint::generate_modules_headers( AbsolutePath::try_from( temp.path() ).unwrap() ).unwrap();
		let mut file = std::fs::File::open( temp.path().join( "test_module" ).join( "Readme.md" ) ).unwrap();

		let mut actual = String::new();

		_ = file.read_to_string( &mut actual ).unwrap();

		// Assert
		assert_eq!( expected, actual );
	}

	#[ test ]
	fn idempotency()
	{
		// Arrange
		let temp = arrange( "single_module" );

		let expected = "<!--{ generate.module_header.start() }-->\n[![experimental](https://raster.shields.io/static/v1?label=&message=experimental&color=orange)](https://github.com/emersion/stability-badges#experimental) | [![rust-status](https://github.com/Wandalen/wTools/actions/workflows/ModuleTestModulePush.yml/badge.svg)](https://github.com/Wandalen/wTools/actions/workflows/ModuleTestModulePush.yml)[![docs.rs](https://img.shields.io/docsrs/test_module?color=e3e8f0&logo=docs.rs)](https://docs.rs/test_module)[![Open in Gitpod](https://raster.shields.io/static/v1?label=try&message=online&color=eee&logo=gitpod&logoColor=eee)](https://gitpod.io/#RUN_PATH=.,SAMPLE_FILE=sample%2Frust%2Ftest_module_trivial_sample%2Fsrc%2Fmain.rs,RUN_POSTFIX=--example%20test_module_trivial_sample/https://github.com/Wandalen/wTools)\n<!--{ generate.module_header.end }-->";

		// Act
		_ = endpoint::generate_modules_headers( AbsolutePath::try_from( temp.path() ).unwrap() ).unwrap();
		_ = endpoint::generate_modules_headers( AbsolutePath::try_from( temp.path() ).unwrap() ).unwrap();
		_ = endpoint::generate_modules_headers( AbsolutePath::try_from( temp.path() ).unwrap() ).unwrap();

		let mut file = std::fs::File::open( temp.path().join( "test_module" ).join( "Readme.md" ) ).unwrap();

		let mut actual = String::new();

		_ = file.read_to_string( &mut actual ).unwrap();

		// Assert
		assert_eq!( expected, actual );
	}
	
	#[ test ]
	fn with_many_members_and_varius_config()
	{
		let temp = arrange( "three_packages" );

		// without discord in module & stability
		let expected_a = "<!--{ generate.module_header.start() }-->\n[![experimental](https://raster.shields.io/static/v1?label=&message=experimental&color=orange)](https://github.com/emersion/stability-badges#experimental) | [![rust-status](https://github.com/Username/test/actions/workflows/ModuleChainOfPackagesAPush.yml/badge.svg)](https://github.com/Username/test/actions/workflows/ModuleChainOfPackagesAPush.yml)[![docs.rs](https://img.shields.io/docsrs/_chain_of_packages_a?color=e3e8f0&logo=docs.rs)](https://docs.rs/_chain_of_packages_a)[![Open in Gitpod](https://raster.shields.io/static/v1?label=try&message=online&color=eee&logo=gitpod&logoColor=eee)](https://gitpod.io/#RUN_PATH=.,SAMPLE_FILE=sample%2Frust%2F_chain_of_packages_a_trivial_sample%2Fsrc%2Fmain.rs,RUN_POSTFIX=--example%20_chain_of_packages_a_trivial_sample/https://github.com/Username/test)\n[![discord](https://img.shields.io/discord/872391416519737405?color=eee&logo=discord&logoColor=eee&label=ask)](https://discord.gg/123456789)\n<!--{ generate.module_header.end }-->";
		// without discord in module & stability = stable
		let expected_b = "<!--{ generate.module_header.start() }-->\n[![stability-stable](https://img.shields.io/badge/stability-stable-green.svg)](https://github.com/emersion/stability-badges#stable) | [![rust-status](https://github.com/Username/test/actions/workflows/ModuleChainOfPackagesBPush.yml/badge.svg)](https://github.com/Username/test/actions/workflows/ModuleChainOfPackagesBPush.yml)[![docs.rs](https://img.shields.io/docsrs/_chain_of_packages_b?color=e3e8f0&logo=docs.rs)](https://docs.rs/_chain_of_packages_b)[![Open in Gitpod](https://raster.shields.io/static/v1?label=try&message=online&color=eee&logo=gitpod&logoColor=eee)](https://gitpod.io/#RUN_PATH=.,SAMPLE_FILE=sample%2Frust%2F_chain_of_packages_b_trivial_sample%2Fsrc%2Fmain.rs,RUN_POSTFIX=--example%20_chain_of_packages_b_trivial_sample/https://github.com/Username/test)\n[![discord](https://img.shields.io/discord/872391416519737405?color=eee&logo=discord&logoColor=eee&label=ask)](https://discord.gg/123456789)\n<!--{ generate.module_header.end }-->";
		// with discord & stability = stable
		let expected_c = "<!--{ generate.module_header.start() }-->\n[![stability-stable](https://img.shields.io/badge/stability-stable-green.svg)](https://github.com/emersion/stability-badges#stable) | [![rust-status](https://github.com/Username/test/actions/workflows/ModuleChainOfPackagesCPush.yml/badge.svg)](https://github.com/Username/test/actions/workflows/ModuleChainOfPackagesCPush.yml)[![docs.rs](https://img.shields.io/docsrs/_chain_of_packages_c?color=e3e8f0&logo=docs.rs)](https://docs.rs/_chain_of_packages_c)[![Open in Gitpod](https://raster.shields.io/static/v1?label=try&message=online&color=eee&logo=gitpod&logoColor=eee)](https://gitpod.io/#RUN_PATH=.,SAMPLE_FILE=sample%2Frust%2F_chain_of_packages_c_trivial_sample%2Fsrc%2Fmain.rs,RUN_POSTFIX=--example%20_chain_of_packages_c_trivial_sample/https://github.com/Username/test)\n[![discord](https://img.shields.io/discord/872391416519737405?color=eee&logo=discord&logoColor=eee&label=ask)](https://discord.gg/m3YfbXpUUY)\n<!--{ generate.module_header.end }-->";
		// with discord in workspace
		let expected_d = "<!--{ generate.module_header.start() }-->\n[![stability-stable](https://img.shields.io/badge/stability-stable-green.svg)](https://github.com/emersion/stability-badges#stable) | [![rust-status](https://github.com/Username/test/actions/workflows/ModuleChainOfPackagesDPush.yml/badge.svg)](https://github.com/Username/test/actions/workflows/ModuleChainOfPackagesDPush.yml)[![docs.rs](https://img.shields.io/docsrs/_chain_of_packages_d?color=e3e8f0&logo=docs.rs)](https://docs.rs/_chain_of_packages_d)[![Open in Gitpod](https://raster.shields.io/static/v1?label=try&message=online&color=eee&logo=gitpod&logoColor=eee)](https://gitpod.io/#RUN_PATH=.,SAMPLE_FILE=sample%2Frust%2F_chain_of_packages_d_trivial_sample%2Fsrc%2Fmain.rs,RUN_POSTFIX=--example%20_chain_of_packages_d_trivial_sample/https://github.com/Username/test)\n[![discord](https://img.shields.io/discord/872391416519737405?color=eee&logo=discord&logoColor=eee&label=ask)](https://discord.gg/123456789)\n<!--{ generate.module_header.end }-->";

		_ = endpoint::generate_modules_headers( AbsolutePath::try_from( temp.path() ).unwrap() ).unwrap();

		let mut file_a = std::fs::File::open( temp.path().join( "a" ).join( "Readme.md" ) ).unwrap();
		let mut file_b = std::fs::File::open( temp.path().join( "b" ).join( "Readme.md" ) ).unwrap();
		let mut file_c = std::fs::File::open( temp.path().join( "c" ).join( "Readme.md" ) ).unwrap();
		let mut file_d = std::fs::File::open( temp.path().join( "d" ).join( "Readme.md" ) ).unwrap();

		let mut actual_a = String::new();
		let mut actual_b = String::new();
		let mut actual_c = String::new();
		let mut actual_d = String::new();

		_ = file_a.read_to_string( &mut actual_a ).unwrap();
		_ = file_b.read_to_string( &mut actual_b ).unwrap();
		_ = file_c.read_to_string( &mut actual_c ).unwrap();
		_ = file_d.read_to_string( &mut actual_d ).unwrap();

		assert_eq!( expected_a, actual_a );
		assert_eq!( expected_b, actual_b );
		assert_eq!( expected_c, actual_c );
		assert_eq!( expected_d, actual_d );
	}

	#[ test ]
	#[ should_panic ]
	fn without_needed_config()
	{
		// Arrange
		let temp = arrange( "variadic_tag_configurations" );
		
		// Act
		_ = endpoint::generate_modules_headers( AbsolutePath::try_from( temp.path() ).unwrap() ).unwrap();
	}
	
}