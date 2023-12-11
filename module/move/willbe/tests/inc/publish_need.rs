use super::*;

const TEST_MODULE_PATH : &str = "../../test/";
use assert_fs::prelude::*;
use TheModule::{ manifest, version, cargo };
use TheModule::package::protected::publish_need;
use TheModule::package::Package;
use TheModule::path::AbsolutePath;
use TheModule::process;

// published the same as local
#[ test ]
fn no_changes()
{
  // Arrange
  let root_path = std::path::Path::new( env!( "CARGO_MANIFEST_DIR" ) ).join( TEST_MODULE_PATH );
  let package_path = root_path.join( "c" );
  // qqq : for Bohdan : make helper function returning package_path. reuse it for all relevant tests

  _ = process::start_sync( "cargo package", &package_path ).expect( "Failed to package a package" );
  let absolute = AbsolutePath::try_from( package_path ).unwrap();
  let package = Package::try_from( absolute ).unwrap();

  // Act
  let publish_needed = publish_need( &package );

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

  let mut manifest = manifest::open( temp.as_ref() ).unwrap();
  version::bump( &mut manifest, false ).unwrap();

  _ = cargo::package( &temp, false ).expect( "Failed to package a package" );

  let absolute = AbsolutePath::try_from( temp.as_ref() ).unwrap();
  let package = Package::try_from( absolute ).unwrap();

  // Act
  let publish_needed = publish_need( &package );

  // Assert
  assert!( publish_needed );
}
