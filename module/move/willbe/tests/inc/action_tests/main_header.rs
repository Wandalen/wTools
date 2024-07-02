use crate::*;
use assert_fs::prelude::*;
use the_module::action;

use std::io::Read;
use willbe::path::AbsolutePath;


fn arrange( source : &str ) -> assert_fs::TempDir
{
  let root_path = std::path::Path::new( env!( "CARGO_MANIFEST_DIR" ) );
  let assets_relative_path = std::path::Path::new( ASSET_PATH );
  let assets_path = root_path.join( assets_relative_path );

  let temp = assert_fs::TempDir::new().unwrap();
  temp.copy_from( assets_path.join( source ), &[ "**" ] ).unwrap();

  temp
}

#[ test ]
fn tag_shout_stay()
{
  // Arrange
  let temp = arrange( "single_module" );

  // Act
  _ = action::readme_header_renew( AbsolutePath::try_from( temp.path() ).unwrap() ).unwrap();

  let mut file = std::fs::File::open( temp.path().join( "Readme.md" ) ).unwrap();

  let mut actual = String::new();

  _ = file.read_to_string( &mut actual ).unwrap();

  // Assert
  assert!( actual.contains( "<!--{ generate.main_header.start() }-->" ) );
  assert!( actual.contains( "<!--{ generate.main_header.end }-->" ) );
}

#[ test ]
fn branch_cell()
{
  // Arrange
  let temp = arrange( "single_module" );

  // Act
  _ = action::readme_header_renew( AbsolutePath::try_from( temp.path() ).unwrap() ).unwrap();

  let mut file = std::fs::File::open( temp.path().join( "Readme.md" ) ).unwrap();

  let mut actual = String::new();

  _ = file.read_to_string( &mut actual ).unwrap();

  // Assert
  assert!( actual.contains( "[![test_branch](https://img.shields.io/github/actions/workflow/status/Username/test/StandardRustScheduled.yml?branch=master&label=test_branch&logo=github)](https://github.com/Username/test/actions/workflows/StandardRustStatus.yml)" ) );
}

#[ test ]
fn discord_cell()
{
  // Arrange
  let temp = arrange( "single_module" );

  // Act
  _ = action::readme_header_renew( AbsolutePath::try_from( temp.path() ).unwrap() ).unwrap();

  let mut file = std::fs::File::open( temp.path().join( "Readme.md" ) ).unwrap();

  let mut actual = String::new();

  _ = file.read_to_string( &mut actual ).unwrap();

  // Assert
  assert!( actual.contains( "[![discord](https://img.shields.io/discord/872391416519737405?color=eee&logo=discord&logoColor=eee&label=ask)](https://discord.gg/m3YfbXpUUY)" ) );
}

#[ test ]
fn gitpod_cell()
{
  // Arrange
  let temp = arrange( "single_module" );

  // Act
  _ = action::readme_header_renew( AbsolutePath::try_from( temp.path() ).unwrap() ).unwrap();

  let mut file = std::fs::File::open( temp.path().join( "Readme.md" ) ).unwrap();

  let mut actual = String::new();

  _ = file.read_to_string( &mut actual ).unwrap();

  // Assert
  assert!( actual.contains( "[![Open in Gitpod](https://raster.shields.io/static/v1?label=try&message=online&color=eee&logo=gitpod&logoColor=eee)](https://gitpod.io/#RUN_PATH=.,SAMPLE_FILE=sample%2Frust%2Ftest_trivial%2Fsrc%2Fmain.rs,RUN_POSTFIX=--example%20test_trivial/https://github.com/Username/test)" ) );
}

#[ test ]
fn docs_cell()
{
  // Arrange
  let temp = arrange( "single_module" );

  // Act
  _ = action::readme_header_renew( AbsolutePath::try_from( temp.path() ).unwrap() ).unwrap();

  let mut file = std::fs::File::open( temp.path().join( "Readme.md" ) ).unwrap();

  let mut actual = String::new();

  _ = file.read_to_string( &mut actual ).unwrap();

  // Assert
  assert!( actual.contains( "[![docs.rs](https://raster.shields.io/static/v1?label=docs&message=online&color=eee&logo=docsdotrs&logoColor=eee)](https://docs.rs/test)" ) );
}

#[ test ]
fn without_fool_config()
{
  // Arrange
  let temp = arrange( "single_module_without_master_branch_and_discord" );

  // Act
  _ = action::readme_header_renew( AbsolutePath::try_from( temp.path() ).unwrap() ).unwrap();

  let mut file = std::fs::File::open( temp.path().join( "Readme.md" ) ).unwrap();

  let mut actual = String::new();

  _ = file.read_to_string( &mut actual ).unwrap();

  // Assert
  assert!( actual.contains( "[master]" ) );// master by default
  assert!( !actual.contains( "[discord]" ) );// without discord
}

#[ test ]
fn idempotency()
{
  // Arrange
  let temp = arrange( "single_module" );

  // Act
  _ = action::readme_header_renew( AbsolutePath::try_from( temp.path() ).unwrap() ).unwrap();
  let mut file = std::fs::File::open( temp.path().join( "Readme.md" ) ).unwrap();
  let mut actual1 = String::new();
  _ = file.read_to_string( &mut actual1 ).unwrap();
  drop( file );

  _ = action::readme_header_renew( AbsolutePath::try_from( temp.path() ).unwrap() ).unwrap();
  let mut file = std::fs::File::open( temp.path().join( "Readme.md" ) ).unwrap();
  let mut actual2 = String::new();
  _ = file.read_to_string( &mut actual2 ).unwrap();
  drop( file );

  // Assert
  assert_eq!( actual1, actual2 );
}

#[ test ]
#[ should_panic ]
fn without_needed_config()
{
  // Arrange
  let temp = arrange( "variadic_tag_configurations" );
  // Act
  _ = action::readme_header_renew( AbsolutePath::try_from( temp.path() ).unwrap() ).unwrap();
}