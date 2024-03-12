use super::*;

use std::
{
  io::Write,
  path::{ Path, PathBuf },
};

use assert_fs::prelude::*;
use TheModule::
{
  package::{ publish_need, Package },
  path::AbsolutePath,
  manifest,
  version,
  cargo
};

const TEST_MODULE_PATH : &str = "../../test/";

fn package_path< P : AsRef< Path > >( path : P ) -> PathBuf
{
  let root_path = Path::new( env!( "CARGO_MANIFEST_DIR" ) ).join( TEST_MODULE_PATH );
  root_path.join( path )
}

fn package< P : AsRef< Path > >( path : P ) -> Package
{
  let path = path.as_ref();
  _ = cargo::pack( cargo::PackOptions::former().path( path.to_path_buf() ).dry( false ).form() ).expect( "Failed to package a package" );
  let absolute = AbsolutePath::try_from( path ).unwrap();

  Package::try_from( absolute ).unwrap()
}

// published the same as local
#[ test ]
fn no_changes()
{
  // Arrange
  // qqq : for Bohdan : make helper function returning package_path. reuse it for all relevant tests
  // aaa : use `package_path` function
  let package_path = package_path( "c" );

  _ = cargo::pack( cargo::PackOptions::former().path( package_path.clone() ).dry( false ).form() ).expect( "Failed to package a package" );
  let absolute = AbsolutePath::try_from( package_path ).unwrap();
  let package = Package::try_from( absolute ).unwrap();

  // Act
  let publish_needed = publish_need( &package, None ).unwrap();

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

  _ = cargo::pack( cargo::PackOptions::former().path( temp.path().to_path_buf() ).dry( false ).form() ).expect( "Failed to package a package" );

  let absolute = AbsolutePath::try_from( temp.as_ref() ).unwrap();
  let package = Package::try_from( absolute ).unwrap();

  // Act
  let publish_needed = publish_need( &package, None ).unwrap();

  // Assert
  assert!( publish_needed );
}

// c(update) -> b(re-publish) -> a(re-publish)
#[ test ]
fn cascade_with_changes()
{
  let abc = [ "a", "b", "c" ].into_iter().map( package_path ).map( package ).collect::< Vec< _ > >();
  let [ a, b, c ] = abc.as_slice() else { unreachable!() };
  if ![ c, b, a ].into_iter().inspect( | x | { dbg!( x.name().unwrap() ); } ).map( | a | publish_need( a, None ) ).inspect( | x | { dbg!(x); } ).all( | p | !p.expect( "There was an error verifying whether the package needs publishing or not" ) )
  {
    panic!( "The packages must be up-to-dated" );
  }
  let temp = assert_fs::TempDir::new().unwrap();
  let temp_module = temp.child( "module" );
  std::fs::create_dir( &temp_module ).unwrap();
  temp_module.child( "a" ).copy_from( a.manifest_path().parent().unwrap(), &[ "**" ] ).unwrap();
  temp_module.child( "b" ).copy_from( b.manifest_path().parent().unwrap(), &[ "**" ] ).unwrap();
  temp_module.child( "c" ).copy_from( c.manifest_path().parent().unwrap(), &[ "**" ] ).unwrap();
  let a_temp_path = temp_module.join( "a" );
  let b_temp_path = temp_module.join( "b" );
  let c_temp_path = temp_module.join( "c" );

  let mut cargo_toml = std::fs::File::create( temp.join( "Cargo.toml" ) ).unwrap();
  write!( cargo_toml, r#"
[workspace]
resolver = "2"
members = [
    "module/*",
]
[workspace.dependencies.test_experimental_a]
version = "*"
path = "module/a"
default-features = true
[workspace.dependencies.test_experimental_b]
version = "*"
path = "module/b"
default-features = true
[workspace.dependencies.test_experimental_c]
version = "*"
path = "module/c"
default-features = true
"# ).unwrap();

  let absolute = AbsolutePath::try_from( c_temp_path.join( "Cargo.toml" ) ).unwrap();
  let mut manifest = manifest::open( absolute ).unwrap();
  version::bump( &mut manifest, false ).unwrap();

  let c_temp = package( c_temp_path );
  let b_temp = package( b_temp_path );
  let a_temp = package( a_temp_path );

  assert!( publish_need( &c_temp, None ).unwrap() );
  assert!( publish_need( &b_temp, None ).unwrap() );
  assert!( publish_need( &a_temp, None ).unwrap() );
}
