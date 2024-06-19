use crate::*;

use the_module::*;
use std::path::{ Path, PathBuf };
use assert_fs::{ TempDir, prelude::* };
use crates_tools::CrateArchive;
use package::Package;
use diff::crate_diff;
use the_module::version::{ Version, BumpOptions, bump };

const TEST_MODULE_PATH : &str = "../../test/";

#[ test ]
fn no_changes()
{
  let tmp = &TempDir::new().unwrap();
  let package_path = package_path( "c" );

  let left = prepare( tmp, "left", &package_path );
  let left_crate = crate_file_path( &left );
  let left_archive = CrateArchive::read( &left_crate ).unwrap();

  let right = prepare( tmp, "right", &package_path );
  let right_crate = crate_file_path( &right );
  let right_archive = CrateArchive::read( &right_crate ).unwrap();

  let has_changes = crate_diff( &left_archive, &right_archive ).exclude( diff::PUBLISH_IGNORE_LIST ).has_changes();

  assert!( !has_changes );
}

#[ test ]
fn with_changes()
{
  let tmp = &TempDir::new().unwrap();
  let package_path = package_path( "c" );

  let left =
  {
    let left = prepare( tmp, "left", &package_path );
    let left_crate = crate_file_path( &left );
    CrateArchive::read( &left_crate ).unwrap()
  };

  let right =
  {
    let right = prepare( tmp, "right", &package_path );

    // let absolute = AbsolutePath::try_from( right.as_path() ).unwrap();
    let absolute = CrateDir::try_from( right.as_path() ).unwrap();
    let right_package = Package::try_from( absolute ).unwrap();
    let right_version = Version::try_from( &right_package.version().unwrap() ).unwrap();

    let bump_options = BumpOptions
    {
      crate_dir : CrateDir::try_from( right.clone() ).unwrap(),
      old_version : right_version.clone(),
      new_version : right_version.bump(),
      dependencies : vec![],
      dry : false,
    };
    bump( bump_options ).unwrap();

    let right_crate = crate_file_path( &right );
    CrateArchive::read( &right_crate ).unwrap()
  };

  let has_changes = crate_diff( &left, &right ).exclude( diff::PUBLISH_IGNORE_LIST ).has_changes();

  assert!( has_changes );
}

fn package_path< P : AsRef< Path > >( path : P ) -> PathBuf
{
  let root_path = Path::new( env!( "CARGO_MANIFEST_DIR" ) ).join( TEST_MODULE_PATH );
  root_path.join( path )
}

fn prepare( tmp : &TempDir, name : &str, manifest_dir_path : &Path ) -> PathBuf
{
  let dir = tmp.child( name );
  dir.create_dir_all().unwrap();
  dir.copy_from( manifest_dir_path, &[ "**" ] ).unwrap();

  dir.to_path_buf()
}

fn crate_file_path( manifest_dir_path : &Path ) -> PathBuf
{
  _ = cargo::pack( cargo::PackOptions::former().path( manifest_dir_path ).dry( false ).form() ).expect( "Failed to package a package" );

  let absolute = CrateDir::try_from( manifest_dir_path ).unwrap();
  let package = Package::try_from( absolute ).unwrap();
  manifest_dir_path
  .join( "target" )
  .join( "package" )
  .join( format!( "{}-{}.crate", package.name().unwrap(), package.version().unwrap() ) )

}
