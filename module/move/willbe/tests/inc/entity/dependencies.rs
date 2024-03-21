use super::*;

use assert_fs::prelude::*;
use assert_fs::TempDir;
use the_module::Workspace;
use the_module::package::{ dependencies, DependenciesOptions, DependenciesSort };
use willbe::CrateDir;
use willbe::package::Package;
use willbe::_path::AbsolutePath;

//

fn arrange( asset_name : &str ) -> ( TempDir, Workspace )
{
  let path = CrateDir::try_from( AbsolutePath::try_from( std::path::Path::new( env!( "CARGO_MANIFEST_DIR" ) ) ).unwrap() ).unwrap();
  let mut metadata = Workspace::with_crate_dir( path ).unwrap();

  let root_path = metadata.load().unwrap().workspace_root().unwrap();
  let assets_relative_path = std::path::Path::new( ASSET_PATH );
  let assets_path = root_path.join( "module" ).join( "move" ).join( "willbe" ).join( assets_relative_path );
  let temp = TempDir::new().unwrap();
  temp.copy_from( assets_path.join( asset_name ), &[ "**" ] ).unwrap();

  let temp_crate_dir = CrateDir::try_from( AbsolutePath::try_from( temp.to_path_buf() ).unwrap() ).unwrap();
  let metadata = Workspace::with_crate_dir( temp_crate_dir ).unwrap();

  ( temp, metadata )
}

// a -> b -> c
#[ test ]
fn chain_of_three_packages()
{
  // Arrange
  let ( temp, mut metadata ) = arrange( "chain_of_packages" );

  let a = Package::try_from( AbsolutePath::try_from( temp.join( "a" ) ).unwrap() ).unwrap();
  let b = Package::try_from( AbsolutePath::try_from( temp.join( "b" ) ).unwrap() ).unwrap();
  let c = Package::try_from( AbsolutePath::try_from( temp.join( "c" ) ).unwrap() ).unwrap();

  // Act
  let output = dependencies( &mut metadata, &a, DependenciesOptions::default() ).unwrap();
  let output : Vec< _ > = output.iter().filter_map( | o | o.path.as_ref() ).map( | x | x.as_ref() ).collect();

  // Assert
  assert_eq!( 2, output.len() );
  assert!( ( c.crate_dir().as_ref() == output[ 0 ] && b.crate_dir().as_ref() == output[ 1 ] ) || ( c.crate_dir().as_ref() == output[ 1 ] && b.crate_dir().as_ref() == output[ 0 ] ) );

  let output = dependencies( &mut metadata, &b, DependenciesOptions::default() ).unwrap();
  let output : Vec< _ > = output.iter().filter_map( | o | o.path.as_ref() ).map( | x | x.as_ref() ).collect();
  assert_eq!( 1, output.len() );
  assert_eq!( c.crate_dir().as_ref(), output[ 0 ] );

  let output = dependencies( &mut metadata, &c, DependenciesOptions::default() ).unwrap();
  assert!( output.is_empty() );
}

// a -> b -> c
#[ test ]
fn chain_of_three_packages_topologically_sorted()
{
  // Arrange
  let ( temp, mut metadata ) = arrange( "chain_of_packages" );

  let a = Package::try_from( AbsolutePath::try_from( temp.join( "a" ) ).unwrap() ).unwrap();
  let b = Package::try_from( AbsolutePath::try_from( temp.join( "b" ) ).unwrap() ).unwrap();
  let c = Package::try_from( AbsolutePath::try_from( temp.join( "c" ) ).unwrap() ).unwrap();

  // Act
  let output = dependencies( &mut metadata, &a, DependenciesOptions { sort : DependenciesSort::Topological, ..Default::default() } ).unwrap();
  let output : Vec< _ > = output.iter().filter_map( | o | o.path.as_ref() ).map( | x | x.as_ref() ).collect();

  // Assert
   assert_eq!( &[ c.crate_dir().as_ref(), b.crate_dir().as_ref() ], output.as_slice() );

  let output = dependencies( &mut metadata, &b, DependenciesOptions { sort : DependenciesSort::Topological, ..Default::default() } ).unwrap();
  let output : Vec< _ > = output.iter().filter_map( | o | o.path.as_ref() ).map( | x | x.as_ref() ).collect();
   assert_eq!( &[ c.crate_dir().as_ref() ], output.as_slice() );

  let output = dependencies( &mut metadata, &c, DependenciesOptions { sort : DependenciesSort::Topological, ..Default::default() } ).unwrap();
  assert!( output.is_empty() );
}

// a -> ( remote, b )
#[ test ]
fn package_with_remote_dependency()
{
  // Arrange
  let ( temp, mut metadata ) = arrange( "package_with_remote_dependency" );

  let a = Package::try_from( AbsolutePath::try_from( temp.join( "a" ) ).unwrap() ).unwrap();
  let b = Package::try_from( AbsolutePath::try_from( temp.join( "b" ) ).unwrap() ).unwrap();

  // Act
  let output = dependencies( &mut metadata, &a, DependenciesOptions::default() ).unwrap();
  let output : Vec< _ > = output.iter().filter_map( | o | o.path.as_ref() ).map( | x | x.as_ref() ).collect();

  // Assert
  assert_eq!( 1, output.len() );
  assert_eq!( b.crate_dir().as_ref(), output[ 0 ] );
}

// a -> b -> a
#[ test ]
fn workspace_with_cyclic_dependency()
{
  // Arrange
  let ( temp, mut metadata ) = arrange( "workspace_with_cyclic_dependency" );

  let a = Package::try_from( AbsolutePath::try_from( temp.join( "a" ) ).unwrap() ).unwrap();
  let b = Package::try_from( AbsolutePath::try_from( temp.join( "b" ) ).unwrap() ).unwrap();

  // Act
  let output = dependencies( &mut metadata, &a, DependenciesOptions::default() ).unwrap();
  let output : Vec< _ > = output.iter().filter_map( | o | o.path.as_ref() ).map( | x | x.as_ref() ).collect();

  // Assert
  assert_eq!( 1, output.len() );
  assert!( b.crate_dir().as_ref() == output[ 0 ] );

  // Act
  let output = dependencies( &mut metadata, &b, DependenciesOptions::default() ).unwrap();
  let output : Vec< _ > = output.iter().filter_map( | o | o.path.as_ref() ).map( | x | x.as_ref() ).collect();

  // Assert
  assert_eq!( 1, output.len() );
  assert!( a.crate_dir().as_ref() == output[ 0 ] );
}