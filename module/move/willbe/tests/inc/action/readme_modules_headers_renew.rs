const ASSETS_PATH : &str = "tests/assets";

use crate::*;
use assert_fs::prelude::*;
use TheModule::action;
use std::io::Read;
use willbe::path::AbsolutePath;

fn arrange( source : &str ) -> assert_fs::TempDir
{
  let root_path = std::path::Path::new( env!( "CARGO_MANIFEST_DIR" ) );
  let assets_relative_path = std::path::Path::new( ASSETS_PATH );
  let assets_path = root_path.join( assets_relative_path );

  let temp = assert_fs::TempDir::new().unwrap();
  temp.copy_from( assets_path.join( source ), &[ "**" ] ).unwrap();

  temp
}

// [![experimental](https://raster.shields.io/static/v1?label=&message=experimental&color=orange)](https://github.com/emersion/stability-badges#experimental)
// [![rust-status](https://github.com/Wandalen/wTools/actions/workflows/ModuleTestModulePush.yml/badge.svg)](https://github.com/Wandalen/wTools/actions/workflows/ModuleTestModulePush.yml)
// [![docs.rs](https://img.shields.io/docsrs/test_module?color=e3e8f0&logo=docs.rs)](https://docs.rs/test_module)
// [![Open in Gitpod](https://raster.shields.io/static/v1?label=try&message=online&color=eee&logo=gitpod&logoColor=eee)](https://gitpod.io/#RUN_PATH=.,SAMPLE_FILE=sample%2Frust%2Ftest_module_trivial_sample%2Fsrc%2Fmain.rs,RUN_POSTFIX=--example%20test_module_trivial_sample/https://github.com/Wandalen/wTools)
// [![discord](https://img.shields.io/discord/872391416519737405?color=eee&logo=discord&logoColor=eee&label=ask)](https://discord.gg/m3YfbXpUUY)
#[ test ]
fn tags_should_stay()
{
  // Arrange
  let temp = arrange( "single_module" );

  // Act
  _ = action::readme_modules_headers_renew( AbsolutePath::try_from( temp.path() ).unwrap() ).unwrap();
  let mut file = std::fs::File::open( temp.path().join( "test_module" ).join( "Readme.md" ) ).unwrap();

  let mut actual = String::new();

  _ = file.read_to_string( &mut actual ).unwrap();

  // Assert
  assert!( actual.contains( "<!--{ generate.module_header.start() }-->" ) );
  assert!( actual.contains( "<!--{ generate.module_header.end }-->" ) );
}

#[ test ]
fn default_stability()
{
  // Arrange
  let temp = arrange( "single_module" );

  // Act
  _ = action::readme_modules_headers_renew( AbsolutePath::try_from( temp.path() ).unwrap() ).unwrap();
  let mut file = std::fs::File::open( temp.path().join( "test_module" ).join( "Readme.md" ) ).unwrap();

  let mut actual = String::new();

  _ = file.read_to_string( &mut actual ).unwrap();

  // Assert
  assert!( actual.contains( "[![experimental](https://raster.shields.io/static/v1?label=&message=experimental&color=orange)](https://github.com/emersion/stability-badges#experimental)" ) );
}

#[ test ]
fn docs()
{
  // Arrange
  let temp = arrange( "single_module" );

  // Act
  _ = action::readme_modules_headers_renew( AbsolutePath::try_from( temp.path() ).unwrap() ).unwrap();
  let mut file = std::fs::File::open( temp.path().join( "test_module" ).join( "Readme.md" ) ).unwrap();

  let mut actual = String::new();

  _ = file.read_to_string( &mut actual ).unwrap();

  // Assert
  assert!( actual.contains( "[![docs.rs](https://img.shields.io/docsrs/test_module?color=e3e8f0&logo=docs.rs)](https://docs.rs/test_module)" ) );
}

#[ test ]
fn gitpod()
{
  // Arrange
  let temp = arrange( "single_module" );

  // Act
  _ = action::readme_modules_headers_renew( AbsolutePath::try_from( temp.path() ).unwrap() ).unwrap();
  let mut file = std::fs::File::open( temp.path().join( "test_module" ).join( "Readme.md" ) ).unwrap();

  let mut actual = String::new();

  _ = file.read_to_string( &mut actual ).unwrap();

  // Assert
  assert!( actual.contains( "[![Open in Gitpod](https://raster.shields.io/static/v1?label=try&message=online&color=eee&logo=gitpod&logoColor=eee)](https://gitpod.io/#RUN_PATH=.,SAMPLE_FILE=sample%2Frust%2Ftest_module_trivial_sample%2Fsrc%2Fmain.rs,RUN_POSTFIX=--example%20test_module_trivial_sample/https://github.com/Wandalen/wTools)" ) );
}

#[ test ]
fn discord()
{
  // Arrange
  let temp = arrange( "single_module" );

  // Act
  _ = action::readme_modules_headers_renew( AbsolutePath::try_from( temp.path() ).unwrap() ).unwrap();
  let mut file = std::fs::File::open( temp.path().join( "test_module" ).join( "Readme.md" ) ).unwrap();

  let mut actual = String::new();

  _ = file.read_to_string( &mut actual ).unwrap();

  // Assert
  assert!( actual.contains( "[![discord](https://img.shields.io/discord/872391416519737405?color=eee&logo=discord&logoColor=eee&label=ask)](https://discord.gg/m3YfbXpUUY)" ) );
}

#[ test ]
fn status()
{
  // Arrange
  let temp = arrange( "single_module" );

  // Act
  _ = action::readme_modules_headers_renew( AbsolutePath::try_from( temp.path() ).unwrap() ).unwrap();
  let mut file = std::fs::File::open( temp.path().join( "test_module" ).join( "Readme.md" ) ).unwrap();

  let mut actual = String::new();

  _ = file.read_to_string( &mut actual ).unwrap();

  // Assert
  assert!( actual.contains( "[![rust-status](https://github.com/Wandalen/wTools/actions/workflows/ModuleTestModulePush.yml/badge.svg)](https://github.com/Wandalen/wTools/actions/workflows/ModuleTestModulePush.yml)" ) );
}

#[ test ]
fn idempotency()
{
  // Arrange
  let temp = arrange( "single_module" );

  // Act
  _ = action::readme_modules_headers_renew( AbsolutePath::try_from( temp.path() ).unwrap() ).unwrap();
  let mut file = std::fs::File::open( temp.path().join( "test_module" ).join( "Readme.md" ) ).unwrap();
  let mut actual1 = String::new();
  _ = file.read_to_string( &mut actual1 ).unwrap();
  drop( file );

  _ = action::readme_modules_headers_renew( AbsolutePath::try_from( temp.path() ).unwrap() ).unwrap();
  let mut file = std::fs::File::open( temp.path().join( "test_module" ).join( "Readme.md" ) ).unwrap();
  let mut actual2 = String::new();
  _ = file.read_to_string( &mut actual2 ).unwrap();
  drop( file );

  // Assert
  assert_eq!( actual1, actual2 );
}

#[ test ]
fn with_many_members_and_varius_config()
{
  let temp = arrange( "three_packages" );

  _ = action::readme_modules_headers_renew( AbsolutePath::try_from( temp.path() ).unwrap() ).unwrap();

  let mut file_b = std::fs::File::open( temp.path().join( "b" ).join( "Readme.md" ) ).unwrap();
  let mut file_c = std::fs::File::open( temp.path().join( "c" ).join( "Readme.md" ) ).unwrap();
  let mut file_d = std::fs::File::open( temp.path().join( "d" ).join( "Readme.md" ) ).unwrap();

  let mut actual_b = String::new();
  let mut actual_c = String::new();
  let mut actual_d = String::new();

  _ = file_b.read_to_string( &mut actual_b ).unwrap();
  _ = file_c.read_to_string( &mut actual_c ).unwrap();
  _ = file_d.read_to_string( &mut actual_d ).unwrap();

  assert!( actual_b.contains( "[![stability-stable]" ) );
  assert!( actual_c.contains( "(https://discord.gg/m3YfbXpUUY)" ) );
  assert!( actual_d.contains( "(https://discord.gg/123456789)" ) );
}

#[ test ]
#[ should_panic ]
fn without_needed_config()
{
  // Arrange
  let temp = arrange( "variadic_tag_configurations" );

  // Act
  _ = action::readme_modules_headers_renew( AbsolutePath::try_from( temp.path() ).unwrap() ).unwrap();
}
