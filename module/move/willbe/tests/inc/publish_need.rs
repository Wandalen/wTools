use super::*;

use std::path::{ Path, PathBuf };

use assert_fs::prelude::*;
use TheModule::{ manifest, version, cargo };
use TheModule::package::protected::publish_need;
use TheModule::package::Package;
use TheModule::path::AbsolutePath;

const TEST_MODULE_PATH : &str = "../../test/";

fn package_path< P : AsRef< Path > >( path : P ) -> PathBuf
{
  let root_path = Path::new( env!( "CARGO_MANIFEST_DIR" ) ).join( TEST_MODULE_PATH );
  root_path.join( path )
}

// published the same as local
#[ test ]
fn no_changes()
{
  // Arrange
  // qqq : for Bohdan : make helper function returning package_path. reuse it for all relevant tests
  // aaa : use `package_path` function
  let package_path = package_path( "c" );

  _ = cargo::package( &package_path, false ).expect( "Failed to package a package" );
  let absolute = AbsolutePath::try_from( package_path ).unwrap();
  let package = Package::try_from( absolute ).unwrap();

  // Act
  let publish_needed = publish_need( &package ).unwrap();

  // Assert
  assert!( !publish_needed );
}

// version bumped => publish required
#[ test ]
fn with_changes()
{
  // Arrange
  let package_path = package_path( "c" );

  let temp = assert_fs::TempDir::new().unwrap();
  temp.copy_from( &package_path, &[ "**" ] ).unwrap();

  let absolute = AbsolutePath::try_from( temp.as_ref() ).unwrap();
  let mut manifest = manifest::open( absolute ).unwrap();
  version::bump( &mut manifest, false ).unwrap();

  _ = cargo::package( &temp, false ).expect( "Failed to package a package" );

  let absolute = AbsolutePath::try_from( temp.as_ref() ).unwrap();
  let package = Package::try_from( absolute ).unwrap();

  // Act
  let publish_needed = publish_need( &package ).unwrap();

  // Assert
  assert!( publish_needed );
}
