use super::*;
const ASSETS_PATH : &str = "module/move/willbe/tests/assets";

use assert_fs::prelude::*;
use assert_fs::TempDir;
use TheModule::Workspace;
use TheModule::package::{ dependencies, DependenciesOptions, DependenciesSort };
use willbe::CrateDir;
use willbe::path::AbsolutePath;

//

fn arrange( asset_name : &str ) -> ( TempDir, Workspace )
{
  let mut metadata = Workspace::from_current_path().unwrap();

  let root_path = metadata.load().workspace_root();
  let assets_relative_path = std::path::Path::new( ASSETS_PATH );
  let assets_path = root_path.join( assets_relative_path );

  let temp = TempDir::new().unwrap();
  temp.copy_from( assets_path.join( asset_name ), &[ "**" ] ).unwrap();

  let temp_crate_dir = CrateDir::try_from( AbsolutePath::try_from( temp.to_path_buf() ).unwrap() ).unwrap();
  let metadata = Workspace::with_crate_dir( temp_crate_dir );

  ( temp, metadata )
}

// a -> b -> c
#[ test ]
fn chain_of_three_packages()
{
  // Arrange
  let ( temp, mut metadata ) = arrange( "chain_of_packages" );

  let a_path = temp.join( "a" );
  let b_path = temp.join( "b" );
  let c_path = temp.join( "c" );

  // Act
  let output = dependencies( &mut metadata, &a_path.join( "Cargo.toml" ), DependenciesOptions::default() ).unwrap();
  let output : Vec< _ > = output.into_iter().filter_map( | o | o.path ).collect();

  // Assert
  assert_eq!( 2, output.len() );
  assert!( ( c_path == output[ 0 ] && b_path == output[ 1 ] ) || ( c_path == output[ 1 ] && b_path == output[ 0 ] ) );

  let output = dependencies( &mut metadata, &b_path.join( "Cargo.toml" ), DependenciesOptions::default() ).unwrap();
  let output : Vec< _ > = output.into_iter().filter_map( | o | o.path ).collect();
  assert_eq!( 1, output.len() );
  assert_eq!( c_path, output[ 0 ] );

  let output = dependencies( &mut metadata, &c_path.join( "Cargo.toml" ), DependenciesOptions::default() ).unwrap();
  assert!( output.is_empty() );
}

// a -> b -> c
#[ test ]
fn chain_of_three_packages_topologically_sorted()
{
  // Arrange
  let ( temp, mut metadata ) = arrange( "chain_of_packages" );

  let a_path = temp.join( "a" );
  let b_path = temp.join( "b" );
  let c_path = temp.join( "c" );

  // Act
  let output = dependencies( &mut metadata, &a_path.join( "Cargo.toml" ), DependenciesOptions { sort : DependenciesSort::Topological, ..Default::default() } ).unwrap();
  let output : Vec< _ > = output.into_iter().filter_map( | o | o.path ).collect();

  // Assert
   assert_eq!( &[ c_path.clone(), b_path.clone() ], output.as_slice() );

  let output = dependencies( &mut metadata, &b_path.join( "Cargo.toml" ), DependenciesOptions { sort : DependenciesSort::Topological, ..Default::default() } ).unwrap();
  let output : Vec< _ > = output.into_iter().filter_map( | o | o.path ).collect();
   assert_eq!( &[ c_path.clone() ], output.as_slice() );

  let output = dependencies( &mut metadata, &c_path.join( "Cargo.toml" ), DependenciesOptions { sort : DependenciesSort::Topological, ..Default::default() } ).unwrap();
  assert!( output.is_empty() );
}

// a -> ( remote, b )
#[ test ]
fn package_with_remote_dependency()
{
  // Arrange
  let ( temp, mut metadata ) = arrange( "package_with_remote_dependency" );

  let a_path = temp.join( "a" );
  let b_path = temp.join( "b" );

  // Act
  let output = dependencies( &mut metadata, &a_path.join( "Cargo.toml" ), DependenciesOptions::default() ).unwrap();
  let output : Vec< _ > = output.into_iter().filter_map( | o | o.path ).collect();

  // Assert
  assert_eq!( 1, output.len() );
  assert_eq!( b_path, output[ 0 ] );
}

// a -> b -> a
#[ test ]
fn workspace_with_cyclic_dependency()
{
  // Arrange
  let ( temp, mut metadata ) = arrange( "workspace_with_cyclic_dependency" );

  let a_path = temp.join( "a" );
  let b_path = temp.join( "b" );

  // Act
  let output = dependencies( &mut metadata, &a_path.join( "Cargo.toml" ), DependenciesOptions::default() ).unwrap();
  let output : Vec< _ > = output.into_iter().filter_map( | o | o.path ).collect();

  // Assert
  assert_eq!( 1, output.len() );
  assert!( b_path == output[ 0 ] );

  // Act
  let output = dependencies( &mut metadata, &b_path.join( "Cargo.toml" ), DependenciesOptions::default() ).unwrap();
  let output : Vec< _ > = output.into_iter().filter_map( | o | o.path ).collect();

  // Assert
  assert_eq!( 1, output.len() );
  assert!( a_path == output[ 0 ] );
}
