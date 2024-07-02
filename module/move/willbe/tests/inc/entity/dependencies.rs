use super::*;

use assert_fs::prelude::*;
use assert_fs::TempDir;
use the_module::
{
  Workspace,
  dependency::{ self, DependenciesOptions, DependenciesSort },
  CrateDir,
  package::Package,
  path::AbsolutePath,
};

//

fn arrange( asset_name : &str ) -> ( TempDir, Workspace )
{
  let path = CrateDir::try_from( std::path::Path::new( env!( "CARGO_MANIFEST_DIR" ) ) ).unwrap();
  let workspace = Workspace::try_from( path ).unwrap();

  let root_path = workspace.workspace_root();
  let assets_relative_path = std::path::Path::new( ASSET_PATH );
  let assets_path = root_path.join( "module" ).join( "move" ).join( "willbe" ).join( assets_relative_path );
  let temp = TempDir::new().unwrap();
  temp.copy_from( assets_path.join( asset_name ), &[ "**" ] ).unwrap();

  let temp_crate_dir = CrateDir::try_from( AbsolutePath::try_from( temp.to_path_buf() ).unwrap() ).unwrap();
  let workspace = Workspace::try_from( temp_crate_dir ).unwrap();

  ( temp, workspace )
}

// a -> b -> c
#[ test ]
fn chain_of_three_packages()
{
  // Arrange
  let ( temp, mut workspace ) = arrange( "chain_of_packages" );

  let a = Package::try_from( willbe::CrateDir::try_from( temp.join( "a" ) ).unwrap() ).unwrap();
  let b = Package::try_from( willbe::CrateDir::try_from( temp.join( "b" ) ).unwrap() ).unwrap();
  let c = Package::try_from( willbe::CrateDir::try_from( temp.join( "c" ) ).unwrap() ).unwrap();

  // Act
  let output = dependency::list( &mut workspace, &a, DependenciesOptions::default() ).unwrap();
  let output : Vec< CrateDir > = output
  .into_iter()
  .filter_map( | p | p.crate_dir )
  .collect();

  // Assert
  assert_eq!( 2, output.len() );
  assert!
  (
    ( c.crate_dir() == output[ 0 ] && b.crate_dir() == output[ 1 ] ) ||
    ( c.crate_dir() == output[ 1 ] && b.crate_dir() == output[ 0 ] ),
  );

  let output = dependency::list( &mut workspace, &b, DependenciesOptions::default() ).unwrap();
  let output : Vec< CrateDir > = output
  .into_iter()
  .filter_map( | p | p.crate_dir )
  .collect();
  assert_eq!( 1, output.len() );
  assert_eq!( c.crate_dir(), output[ 0 ] );

  let output = dependency::list( &mut workspace, &c, DependenciesOptions::default() ).unwrap();
  assert!( output.is_empty() );
}

// a -> b -> c
#[ test ]
fn chain_of_three_packages_topologically_sorted()
{
  // Arrange
  let ( temp, mut workspace ) = arrange( "chain_of_packages" );

  let a = Package::try_from( willbe::CrateDir::try_from( temp.join( "a" ) ).unwrap() ).unwrap();
  let b = Package::try_from( willbe::CrateDir::try_from( temp.join( "b" ) ).unwrap() ).unwrap();
  let c = Package::try_from( willbe::CrateDir::try_from( temp.join( "c" ) ).unwrap() ).unwrap();

  // Act
  let output = dependency::list
  (
    &mut workspace,
    &a,
    DependenciesOptions { sort : DependenciesSort::Topological, ..Default::default() },
  ).unwrap();
  let output : Vec< CrateDir > = output
  .into_iter()
  .filter_map( | p | p.crate_dir )
  .collect();

  // Assert
   assert_eq!( &[ c.crate_dir(), b.crate_dir() ], output.as_slice() );

  let output = dependency::list( &mut workspace, &b, DependenciesOptions { sort : DependenciesSort::Topological, ..Default::default() } ).unwrap();
  let output : Vec< CrateDir > = output
  .into_iter()
  .filter_map( | p | p.crate_dir )
  .collect();
  assert_eq!( &[ c.crate_dir() ], output.as_slice() );

  let output = dependency::list( &mut workspace, &c, DependenciesOptions { sort : DependenciesSort::Topological, ..Default::default() } ).unwrap();
  assert!( output.is_empty() );
}

// a -> ( remote, b )
#[ test ]
fn package_with_remote_dependency()
{
  // Arrange
  let ( temp, mut workspace ) = arrange( "package_with_remote_dependency" );

  let a = Package::try_from( willbe::CrateDir::try_from( temp.join( "a" ) ).unwrap() ).unwrap();
  let b = Package::try_from( willbe::CrateDir::try_from( temp.join( "b" ) ).unwrap() ).unwrap();

  // Act
  let output = dependency::list( &mut workspace, &a, DependenciesOptions::default() ).unwrap();
  let output : Vec< CrateDir > = output
  .into_iter()
  .filter_map( | p | p.crate_dir )
  .collect();

  // Assert
  assert_eq!( 1, output.len() );
  assert_eq!( b.crate_dir(), output[ 0 ] );
}

// a -> b -> a
#[ test ]
fn workspace_with_cyclic_dependency()
{
  // Arrange
  let ( temp, mut workspace ) = arrange( "workspace_with_cyclic_dependency" );

  let a = Package::try_from( willbe::CrateDir::try_from( temp.join( "a" ) ).unwrap() ).unwrap();
  let b = Package::try_from( willbe::CrateDir::try_from( temp.join( "b" ) ).unwrap() ).unwrap();

  // Act
  let output = dependency::list( &mut workspace, &a, DependenciesOptions::default() ).unwrap();
  let output : Vec< CrateDir > = output
  .into_iter()
  .filter_map( | p | p.crate_dir )
  .collect();

  // Assert
  assert_eq!( 1, output.len() );
  assert!( b.crate_dir() == output[ 0 ] );

  // Act
  let output = dependency::list( &mut workspace, &b, DependenciesOptions::default() ).unwrap();
  let output : Vec< CrateDir > = output
  .into_iter()
  .filter_map( | p | p.crate_dir )
  .collect();

  // Assert
  assert_eq!( 1, output.len() );
  assert!( a.crate_dir() == output[ 0 ] );
}