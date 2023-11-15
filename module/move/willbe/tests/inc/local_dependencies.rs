use super::*;
const ASSETS_PATH : &str = "module/move/willbe/tests/assets";

use cargo_metadata::MetadataCommand;
use assert_fs::prelude::*;
use TheModule::package::{ local_dependencies, LocalDependenciesOptions, LocalDependenciesSort };

//

tests_impls!
{
  // a -> b -> c
  fn chain_of_three_packages()
  {
    // Arrange
    let metadata = MetadataCommand::new().no_deps().exec().unwrap();

    let root_path = metadata.workspace_root.as_std_path();
    let assets_relative_path = std::path::Path::new( ASSETS_PATH );
    let assets_path = root_path.join( assets_relative_path );

    let temp = assert_fs::TempDir::new().unwrap();
    temp.copy_from( assets_path.join( "chain_of_packages" ), &[ "**" ] ).unwrap();

    let metadata = MetadataCommand::new().no_deps().current_dir( temp.as_ref() ).exec().unwrap();

    let a_path = temp.join( "a" );
    let b_path = temp.join( "b" );
    let c_path = temp.join( "c" );

    // Act
    let output = local_dependencies( &metadata, &a_path.join( "Cargo.toml" ), LocalDependenciesOptions::default() ).unwrap();

    // Assert
    assert_eq!( 2, output.len() );
    assert!( ( c_path == output[ 0 ] && b_path == output[ 1 ] ) || ( c_path == output[ 1 ] && b_path == output[ 0 ] ) );

    let output = local_dependencies( &metadata, &b_path.join( "Cargo.toml" ), LocalDependenciesOptions::default() ).unwrap();
    assert_eq!( 1, output.len() );
    assert_eq!( c_path, output[ 0 ] );

    let output = local_dependencies( &metadata, &c_path.join( "Cargo.toml" ), LocalDependenciesOptions::default() ).unwrap();
    assert!( output.is_empty() );
  }

  // a -> b -> c
  fn chain_of_three_packages_topologically_sorted()
  {
    // Arrange
    let metadata = MetadataCommand::new().no_deps().exec().unwrap();

    let root_path = metadata.workspace_root.as_std_path();
    let assets_relative_path = std::path::Path::new( ASSETS_PATH );
    let assets_path = root_path.join( assets_relative_path );

    let temp = assert_fs::TempDir::new().unwrap();
    temp.copy_from( assets_path.join( "chain_of_packages" ), &[ "**" ] ).unwrap();

    let metadata = MetadataCommand::new().no_deps().current_dir( temp.as_ref() ).exec().unwrap();

    let a_path = temp.join( "a" );
    let b_path = temp.join( "b" );
    let c_path = temp.join( "c" );

    // Act
    let output = local_dependencies( &metadata, &a_path.join( "Cargo.toml" ), LocalDependenciesOptions { sort : LocalDependenciesSort::Topological, ..Default::default() } ).unwrap();

    // Assert
     assert_eq!( &[ c_path.clone(), b_path.clone() ], output.as_slice() );

    let output = local_dependencies( &metadata, &b_path.join( "Cargo.toml" ), LocalDependenciesOptions { sort : LocalDependenciesSort::Topological, ..Default::default() } ).unwrap();
     assert_eq!( &[ c_path.clone() ], output.as_slice() );

    let output = local_dependencies( &metadata, &c_path.join( "Cargo.toml" ), LocalDependenciesOptions { sort : LocalDependenciesSort::Topological, ..Default::default() } ).unwrap();
    assert!( output.is_empty() );
  }

  // a -> ( remote, b )
  fn package_with_remote_dependency()
  {
    // Arrange
    let metadata = MetadataCommand::new().no_deps().exec().unwrap();

    let root_path = metadata.workspace_root.as_std_path();
    let assets_relative_path = std::path::Path::new( ASSETS_PATH );
    let assets_path = root_path.join( assets_relative_path );

    let temp = assert_fs::TempDir::new().unwrap();
    temp.copy_from( assets_path.join( "package_with_remote_dependency" ), &[ "**" ] ).unwrap();

    let metadata = MetadataCommand::new().no_deps().current_dir( temp.as_ref() ).exec().unwrap();

    let a_path = temp.join( "a" );
    let b_path = temp.join( "b" );

    // Act
    let output = local_dependencies( &metadata, &a_path.join( "Cargo.toml" ), LocalDependenciesOptions::default() ).unwrap();

    // Assert
    assert_eq!( 1, output.len() );
    assert_eq!( b_path, output[ 0 ] );
  }

  // a -> b -> a
  fn workspace_with_cyclic_dependency()
  {
    // Arrange
    let metadata = MetadataCommand::new().no_deps().exec().unwrap();

    let root_path = metadata.workspace_root.as_std_path();
    let assets_relative_path = std::path::Path::new( ASSETS_PATH );
    let assets_path = root_path.join( assets_relative_path );

    let temp = assert_fs::TempDir::new().unwrap();
    temp.copy_from( assets_path.join( "workspace_with_cyclic_dependency" ), &[ "**" ] ).unwrap();

    let metadata = MetadataCommand::new().no_deps().current_dir( temp.as_ref() ).exec().unwrap();

    let a_path = temp.join( "a" );
    let b_path = temp.join( "b" );

    // Act
    let output = local_dependencies( &metadata, &a_path.join( "Cargo.toml" ), LocalDependenciesOptions::default() ).unwrap();

    // Assert
    assert_eq!( 2, output.len() );
    assert!( ( a_path == output[ 0 ] && b_path == output[ 1 ] ) || ( a_path == output[ 1 ] && b_path == output[ 0 ] ) );
  }
}

//

tests_index!
{
  chain_of_three_packages,
  chain_of_three_packages_topologically_sorted,
  package_with_remote_dependency,
  workspace_with_cyclic_dependency,
}
