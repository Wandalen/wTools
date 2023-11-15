const ASSETS_PATH : &str = "tests/assets";

use cargo_metadata::MetadataCommand;
use assert_fs::prelude::*;
use test_tools::{ tests_impls, tests_index };
use crate::TheModule::endpoint::{ self, list::* };

//

tests_impls!
{
  // a -> b -> c
  fn chain_of_three_packages_format()
  {
    // Arrange
    let root_path = std::path::Path::new( env!( "CARGO_MANIFEST_DIR" ) );
    let assets_relative_path = std::path::Path::new( ASSETS_PATH );
    let assets_path = root_path.join( assets_relative_path );

    let temp = assert_fs::TempDir::new().unwrap();
    temp.copy_from( assets_path.join( "chain_of_packages" ), &[ "**" ] ).unwrap();

    let metadata = MetadataCommand::new().no_deps().current_dir( temp.as_ref() ).exec().unwrap();

    // Tree format, without filters

    // Act
    let output = endpoint::list( temp.to_path_buf(), "a", ListFormat::Tree, ListFilter::Nothing ).unwrap();

    // Assert
    let ListReport::Tree { graph, names } = &output else { panic!( "Expected `Tree` format, but found another" ) };

    assert_eq!( 3, graph.node_count() );
    assert_eq!( 2, graph.edge_count() );
    assert_eq!( 1, names.len() );
    assert_eq!( Some( "a" ), graph.node_weight( names[ 0 ] ).map( | x | x.as_str() ) );

    // List format, without filters

    // Act
    let output = endpoint::list( temp.to_path_buf(), "a", ListFormat::Topological, ListFilter::Nothing ).unwrap();

    // Assert
    let ListReport::List( names ) = &output else { panic!( "Expected `Topological` format, but found another" ) };

    assert_eq!( &[ "c".to_string(), "b".to_string(), "a".to_string() ], names.as_slice() );
  }

  // a -> ( remote, b )
  fn package_with_remote_dependency_format()
  {
    // Arrange
    let root_path = std::path::Path::new( env!( "CARGO_MANIFEST_DIR" ) );
    let assets_relative_path = std::path::Path::new( ASSETS_PATH );
    let assets_path = root_path.join( assets_relative_path );

    let temp = assert_fs::TempDir::new().unwrap();
    temp.copy_from( assets_path.join( "package_with_remote_dependency" ), &[ "**" ] ).unwrap();

    let metadata = MetadataCommand::new().no_deps().current_dir( temp.as_ref() ).exec().unwrap();

    // Tree format, without filters

    // Act
    let output = endpoint::list( temp.to_path_buf(), "a", ListFormat::Tree, ListFilter::Nothing ).unwrap();

    // Assert
    let ListReport::Tree { graph, names } = &output else { panic!( "Expected `Tree` format, but found another" ) };

    assert_eq!( 3, graph.node_count() );
    assert_eq!( 2, graph.edge_count() );
    assert_eq!( 1, names.len() );
    assert_eq!( Some( "a" ), graph.node_weight( names[ 0 ] ).map( | x | x.as_str() ) );

    // List format, without filters

    // Act
    let output = endpoint::list( temp.to_path_buf(), "a", ListFormat::Topological, ListFilter::Nothing ).unwrap();

    // Assert
    let ListReport::List( names ) = &output else { panic!( "Expected `Topological` format, but found another" ) };

    assert_eq!( 3, names.len() );
    // `a` must be last
    assert_eq!( "a", &names[ 2 ] );
    // can be in any order
    assert!( ( "b" == &names[ 0 ] && "foo" == &names[ 1 ] ) || ( "b" == &names[ 1 ] && "foo" == &names[ 0 ] ) );
  }

  // a -> ( remote, b )
  fn package_with_remote_dependency_filter()
  {
    // Arrange
    let root_path = std::path::Path::new( env!( "CARGO_MANIFEST_DIR" ) );
    let assets_relative_path = std::path::Path::new( ASSETS_PATH );
    let assets_path = root_path.join( assets_relative_path );

    let temp = assert_fs::TempDir::new().unwrap();
    temp.copy_from( assets_path.join( "package_with_remote_dependency" ), &[ "**" ] ).unwrap();

    let metadata = MetadataCommand::new().no_deps().current_dir( temp.as_ref() ).exec().unwrap();

    // List format, with local only filter

    // Act
    let output = endpoint::list( temp.to_path_buf(), "a", ListFormat::Topological, ListFilter::Local ).unwrap();

    // Assert
    let ListReport::List( names ) = &output else { panic!( "Expected `Topological` format, but found another" ) };

    assert_eq!( &[ "b".to_string(), "a".to_string() ], names.as_slice() );
  }

  // a -> b -> a
  fn workspace_with_cyclic_dependency_format()
  {
    // Arrange
    let root_path = std::path::Path::new( env!( "CARGO_MANIFEST_DIR" ) );
    let assets_relative_path = std::path::Path::new( ASSETS_PATH );
    let assets_path = root_path.join( assets_relative_path );

    let temp = assert_fs::TempDir::new().unwrap();
    temp.copy_from( assets_path.join( "workspace_with_cyclic_dependency" ), &[ "**" ] ).unwrap();

    let metadata = MetadataCommand::new().no_deps().current_dir( temp.as_ref() ).exec().unwrap();

    // Act
    let output = endpoint::list( temp.to_path_buf(), "a", ListFormat::Tree, ListFilter::Nothing );

    // Assert

    // can not process topological sorting for cyclic dependencies
    assert!( output.is_err() );
  }
}

//

tests_index!
{
  chain_of_three_packages_format,
  package_with_remote_dependency_format,
  package_with_remote_dependency_filter,
  workspace_with_cyclic_dependency_format,
}
