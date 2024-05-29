use crate::*;

use std::path::{ Path, PathBuf };
use std::str::FromStr;
use std::io::Write;
use assert_fs::prelude::*;
use the_module::
{
  CrateDir,
  Manifest,
  version::Version,
  _path::AbsolutePath,
  package::Package,
  version::{ BumpOptions, version_bump, version_revert },
};

const TEST_MODULE_PATH : &str = "../../test/";

fn package_path< P : AsRef< Path > >( path : P ) -> PathBuf
{
  let root_path = Path::new( env!( "CARGO_MANIFEST_DIR" ) ).join( TEST_MODULE_PATH );
  root_path.join( path )
}

#[ test ]
fn patch()
{
  // Arrange
  let version = Version::from_str( "0.0.0" ).unwrap();

  // Act
  let new_version = version.bump();

  // Assert
  assert_eq!( "0.0.1", &new_version.to_string() );
}

#[ test ]
fn minor_without_patches()
{
  // Arrange
  let version = Version::from_str( "0.1.0" ).unwrap();

  // Act
  let new_version = version.bump();

  // Assert
  assert_eq!( "0.2.0", &new_version.to_string() );
}

#[ test ]
fn minor_with_patch()
{
  // Arrange
  let version = Version::from_str( "0.1.1" ).unwrap();

  // Act
  let new_version = version.bump();

  // Assert
  assert_eq!( "0.2.0", &new_version.to_string() );
}

#[ test ]
fn major_without_patches()
{
  // Arrange
  let version = Version::from_str( "1.0.0" ).unwrap();

  // Act
  let new_version = version.bump();

  // Assert
  assert_eq!( "1.1.0", &new_version.to_string() );
}

#[ test ]
fn major_with_minor()
{
  // Arrange
  let version = Version::from_str( "1.1.0" ).unwrap();

  // Act
  let new_version = version.bump();

  // Assert
  assert_eq!( "1.2.0", &new_version.to_string() );
}

#[ test ]
fn major_with_patches()
{
  // Arrange
  let version = Version::from_str( "1.1.1" ).unwrap();

  // Act
  let new_version = version.bump();

  // Assert
  assert_eq!( "1.2.0", &new_version.to_string() );
}

#[ test ]
fn package_version_bump()
{
  // Arrange
  let c = package_path( "c" );
  let temp = assert_fs::TempDir::new().unwrap();
  let temp_module = temp.child( "module" );
  std::fs::create_dir( &temp_module ).unwrap();
  temp_module.child( "c" ).copy_from( &c, &[ "**" ] ).unwrap();
  let c_temp_path = temp_module.join( "c" );
  let c_temp_absolute_path = AbsolutePath::try_from( c_temp_path ).unwrap();
  let c_temp_crate_dir = CrateDir::try_from( c_temp_absolute_path.clone() ).unwrap();
  let c_package = Package::try_from( c_temp_absolute_path.clone() ).unwrap();
  let version = c_package.version().unwrap();

  let root_manifest_path =  temp.join( "Cargo.toml" );
  let mut cargo_toml = std::fs::File::create( &root_manifest_path ).unwrap();
  let root_manifest_absolute_path = AbsolutePath::try_from( root_manifest_path.as_path() ).unwrap();
  write!( cargo_toml, r#"
[workspace]
resolver = "2"
members = [
    "module/*",
]
[workspace.dependencies.test_experimental_c]
version = "{version}"
path = "module/c"
default-features = true
"# ).unwrap();
  let version = Version::try_from( &version ).unwrap();
  let bumped_version = version.clone().bump();
  
  // Act
  let options = BumpOptions
  {
    crate_dir : c_temp_crate_dir,
    old_version : version.clone(),
    new_version : bumped_version.clone(),
    dependencies : vec![ CrateDir::try_from( root_manifest_absolute_path.parent().unwrap() ).unwrap() ],
    dry : false,
  };
  let bump_report = version_bump( options ).unwrap();

  // Assert
  assert_eq!( Some( version.to_string() ), bump_report.old_version );
  assert_eq!( Some( bumped_version.to_string() ), bump_report.new_version );
  assert_eq!
  (
    {
      let mut v = vec![ root_manifest_absolute_path.clone(), c_temp_absolute_path.join( "Cargo.toml" ) ];
      v.sort();
      v
    },
    {
      let mut v = bump_report.changed_files;
      v.sort();
      v
    }
  );
  let c_package = Package::try_from( c_temp_absolute_path.clone() ).unwrap();
  let name = c_package.name().unwrap();
  assert_eq!( bumped_version.to_string(), c_package.version().unwrap() );
  let mut root_manifest = Manifest::try_from( root_manifest_absolute_path ).unwrap();
  root_manifest.load().unwrap();
  let data = root_manifest.data();
  let current_version_item = data.get( "workspace" ).and_then( | w | w.get( "dependencies" ) ).and_then( | d | d.get( &name ) ).and_then( | p | p.get( "version" ) ).unwrap();
  let current_version = current_version_item.as_str().unwrap();
  assert_eq!( &bumped_version.to_string(), current_version );
}

#[ test ]
fn package_version_bump_revert()
{
  // Arrange
  let c = package_path( "c" );
  let temp = assert_fs::TempDir::new().unwrap();
  let temp_module = temp.child( "module" );
  std::fs::create_dir( &temp_module ).unwrap();
  temp_module.child( "c" ).copy_from( &c, &[ "**" ] ).unwrap();
  let c_temp_path = temp_module.join( "c" );
  let c_temp_absolute_path = AbsolutePath::try_from( c_temp_path ).unwrap();
  let c_temp_crate_dir = CrateDir::try_from( c_temp_absolute_path.clone() ).unwrap();
  let c_package = Package::try_from( c_temp_absolute_path.clone() ).unwrap();
  let version = c_package.version().unwrap();

  let root_manifest_path =  temp.join( "Cargo.toml" );
  let mut cargo_toml = std::fs::File::create( &root_manifest_path ).unwrap();
  let root_manifest_absolute_path = AbsolutePath::try_from( root_manifest_path.as_path() ).unwrap();
  write!( cargo_toml, r#"
[workspace]
resolver = "2"
members = [
    "module/*",
]
[workspace.dependencies.test_experimental_c]
version = "{version}"
path = "module/c"
default-features = true
"# ).unwrap();
  let version = Version::try_from( &version ).unwrap();
  let bumped_version = version.clone().bump();

  // Act
  let options = BumpOptions
  {
    crate_dir : c_temp_crate_dir,
    old_version : version.clone(),
    new_version : bumped_version.clone(),
    dependencies : vec![ CrateDir::try_from( root_manifest_absolute_path.parent().unwrap() ).unwrap() ],
    dry : false,
  };
  let bump_report = version_bump( options ).unwrap();
  version_revert( &bump_report ).unwrap();

  // Assert
  let c_package = Package::try_from( c_temp_absolute_path.clone() ).unwrap();
  let name = c_package.name().unwrap();
  assert_eq!( version.to_string(), c_package.version().unwrap() );
  let mut root_manifest = Manifest::try_from( root_manifest_absolute_path ).unwrap();
  root_manifest.load().unwrap();
  let data = root_manifest.data();
  let current_version_item = data.get( "workspace" ).and_then( | w | w.get( "dependencies" ) ).and_then( | d | d.get( &name ) ).and_then( | p | p.get( "version" ) ).unwrap();
  let current_version = current_version_item.as_str().unwrap();
  assert_eq!( &version.to_string(), current_version );
}
