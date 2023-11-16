const TEST_MODULE_PATH : &str = "../../test/";

use assert_fs::prelude::*;

use toml_edit::value;
use crate::TheModule::{ manifest, process, version };
use crate::TheModule::package::functions::protected::publish_need;

// published the same as local
#[ test ]
fn no_changes()
{
  // Arrange
  let root_path = std::path::Path::new( env!( "CARGO_MANIFEST_DIR" ) ).join( TEST_MODULE_PATH );
  let package_path = root_path.join( "c" );

  _ = process::start_sync( "cargo package", &package_path ).expect( "Failed to package a package" );
  let manifest = manifest::get( &package_path ).unwrap();

  // Act
  let publish_needed = publish_need( &manifest );

  // Assert
  assert!( !publish_needed );
}

// version bumped => publish required
#[ test ]
fn with_changes()
{
  // Arrange
  let root_path = std::path::Path::new( env!( "CARGO_MANIFEST_DIR" ) ).join( TEST_MODULE_PATH );
  let package_path = root_path.join( "c" );

  let temp = assert_fs::TempDir::new().unwrap();
  temp.copy_from( &package_path, &[ "**" ] ).unwrap();

  let mut manifest = manifest::get( temp.as_ref() ).unwrap();
  // REFACTOR: move this to a function
  let data = manifest.manifest_data.as_deref_mut().unwrap();
  let version = &data[ "package" ][ "version" ].clone();
  let version = version.as_str().expect( "Version should be valid UTF-8" );
  let new_version = version::bump( version ).unwrap();
  data[ "package" ][ "version" ] = value( &new_version );
  manifest.store().unwrap();

  _ = process::start_sync( "cargo package", temp.as_ref() ).expect( "Failed to package a package" );

  // Act
  let publish_needed = publish_need( &manifest );

  // Assert
  assert!( publish_needed );
}
