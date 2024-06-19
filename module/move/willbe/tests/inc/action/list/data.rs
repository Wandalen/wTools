use super::*;

use assert_fs::prelude::*;
use the_module::action::{ self, list::* };
use willbe::CrateDir;
use willbe::path::AbsolutePath;


//

fn crate_dir( path : &std::path::Path ) -> CrateDir
{
  let absolut = AbsolutePath::try_from( path ).unwrap();
  CrateDir::try_from( absolut ).unwrap()
}

// a -> b -> c
mod chain_of_three_packages
{
  use super::*;

  fn arrange() -> assert_fs::TempDir
  {
    let root_path = std::path::Path::new( env!( "CARGO_MANIFEST_DIR" ) );
    let assets_relative_path = std::path::Path::new( ASSET_PATH );
    let assets_path = root_path.join( assets_relative_path );

    let temp = assert_fs::TempDir::new().unwrap();
    temp.copy_from( assets_path.join( "chain_of_packages" ), &[ "**" ] ).unwrap();

    temp
  }

  #[ test ]
  fn tree_format_for_single_package()
  {
    // Arrange
    let temp = arrange();
    let args = ListOptions::former()
    .path_to_manifest( crate_dir( &temp.join( "a" ) ) )
    .format( ListFormat::Tree )
    .dependency_sources([ DependencySource::Local ])
    .dependency_categories([ DependencyCategory::Primary ])
    .form();

    // Act
    let output = action::list( args ).unwrap();

    // Assert
    let ListReport::Tree( trees ) = &output else { panic!( "Expected `Tree` format, but found another" ) };

    assert_eq!( 1, trees.len() );
    let tree = &trees[ 0 ];
    assert_eq!( "_chain_of_packages_a", tree.info.name.as_str() );

    assert_eq!( 1, tree.info.normal_dependencies.len() );
    assert!( tree.info.dev_dependencies.is_empty() );
    assert!( tree.info.build_dependencies.is_empty() );

    let sub_tree = &tree.info.normal_dependencies[ 0 ];
    assert_eq!( "_chain_of_packages_b", sub_tree.name.as_str() );

    assert_eq!( 1, sub_tree.normal_dependencies.len() );
    assert!( sub_tree.dev_dependencies.is_empty() );
    assert!( sub_tree.build_dependencies.is_empty() );

    let mega_sub_tree = &sub_tree.normal_dependencies[ 0 ];
    assert_eq!( "_chain_of_packages_c", mega_sub_tree.name.as_str() );

    assert!( mega_sub_tree.normal_dependencies.is_empty() );
    assert!( mega_sub_tree.dev_dependencies.is_empty() );
    assert!( mega_sub_tree.build_dependencies.is_empty() );
  }

  #[ test ]
  fn list_format_for_single_package_1()
  {
    // Arrange
    let temp = arrange();
    let args = ListOptions::former()
    .path_to_manifest( crate_dir( &temp.join( "a" ) ) )
    .format( ListFormat::Topological )
    .dependency_sources([ DependencySource::Local ])
    .dependency_categories([ DependencyCategory::Primary ])
    .form();

    // Act
    let output = action::list( args ).unwrap();

    // Assert
    let ListReport::List( names ) = &output else { panic!("Expected `Topological` format, but found another") };

    assert_eq!( &[ "_chain_of_packages_c".to_string(), "_chain_of_packages_b".to_string(), "_chain_of_packages_a".to_string() ], names.as_slice() );
  }

  #[ test ]
  fn list_format_for_whole_workspace()
  {
    // Arrange
    let temp = arrange();
    let args = ListOptions::former()
      .path_to_manifest( crate_dir( &temp ) )
      .format( ListFormat::Topological )
      .dependency_sources([ DependencySource::Local ])
      .dependency_categories([ DependencyCategory::Primary ])
      .form();

    // Act
    let output = action::list( args ).unwrap();

    // Assert
    let ListReport::List( names ) = &output else { panic!( "Expected `Topological` format, but found another" ) };

    assert_eq!( &[ "_chain_of_packages_c".to_string(), "_chain_of_packages_b".to_string(), "_chain_of_packages_a".to_string() ], names.as_slice() );
  }
}

// a -> ( remote, b )
mod package_with_remote_dependency
{
  use super::*;

  fn arrange() -> assert_fs::TempDir
  {
    let root_path = std::path::Path::new( env!( "CARGO_MANIFEST_DIR" ) );
    let assets_relative_path = std::path::Path::new( ASSET_PATH );
    let assets_path = root_path.join( assets_relative_path );

    let temp = assert_fs::TempDir::new().unwrap();
    temp.copy_from( assets_path.join( "package_with_remote_dependency" ), &[ "**" ] ).unwrap();

    temp
  }

  #[ test ]
  fn tree_format_for_single_package()
  {
    // Arrange
    let temp = arrange();
    let args = ListOptions::former()
      .path_to_manifest( crate_dir( &temp.join( "a" ) ) )
      .format( ListFormat::Tree )
      .dependency_sources([ DependencySource::Local, DependencySource::Remote ])
      .dependency_categories([ DependencyCategory::Primary ])
      .form();

    // Act
    let output = action::list( args ).unwrap();

    // Assert
    let ListReport::Tree( trees ) = &output else { panic!( "Expected `Tree` format, but found another" ) };

    assert_eq!( 1, trees.len() );
    let tree = &trees[ 0 ];
    assert_eq!( "_package_with_remote_dep_a", tree.info.name.as_str() );

    assert_eq!( 2, tree.info.normal_dependencies.len() );
    assert!( tree.info.dev_dependencies.is_empty() );
    assert!( tree.info.build_dependencies.is_empty() );

    let [ sub_tree_1, sub_tree_2, .. ] = tree.info.normal_dependencies.as_slice() else { unreachable!() };
    assert_eq!( "_package_with_remote_dep_b", sub_tree_1.name.as_str() );
    assert!( sub_tree_1.normal_dependencies.is_empty() );
    assert!( sub_tree_1.dev_dependencies.is_empty() );
    assert!( sub_tree_1.build_dependencies.is_empty() );

    assert_eq!( "foo", sub_tree_2.name.as_str() );
    assert!( sub_tree_2.normal_dependencies.is_empty() );
    assert!( sub_tree_2.dev_dependencies.is_empty() );
    assert!( sub_tree_2.build_dependencies.is_empty() );
  }

  #[ test ]
  fn list_format_for_single_package_2()
  {
    // Arrange
    let temp = arrange();
    let args = ListOptions::former()
      .path_to_manifest( crate_dir( &temp.join( "a" ) ) )
      .format( ListFormat::Topological )
      .dependency_sources([ DependencySource::Local, DependencySource::Remote ])
      .dependency_categories([ DependencyCategory::Primary ])
      .form();

    // Act
    let output = action::list( args ).unwrap();

    // Assert
    let ListReport::List( names ) = &output else { panic!( "Expected `Topological` format, but found another" ) };

    assert_eq!( 3, names.len() );
    // `a` must be last
    assert_eq!( "_package_with_remote_dep_a", &names[ 2 ] );
    // can be in any order
    assert!( ( "_package_with_remote_dep_b" == &names[ 0 ] && "foo" == &names[ 1 ] ) || ( "_package_with_remote_dep_b" == &names[ 1 ] && "foo" == &names[ 0 ] ) );
  }

  #[ test ]
  fn only_local_dependency_filter()
  {
    // Arrange
    let temp = arrange();
    let args = ListOptions::former()
      .path_to_manifest( crate_dir( &temp.join( "a" ) ) )
      .format( ListFormat::Topological )
      .dependency_sources([ DependencySource::Local ])
      .dependency_categories([ DependencyCategory::Primary ])
      .form();

    // Act
    let output = action::list( args ).unwrap();

    // Assert
    let ListReport::List( names ) = &output else { panic!( "Expected `Topological` format, but found another" ) };

    assert_eq!( &[ "_package_with_remote_dep_b".to_string(), "_package_with_remote_dep_a".to_string() ], names.as_slice() );
  }
}

// a -> b -> a
mod workspace_with_cyclic_dependency
{
  use super::*;

  #[ test ]
  fn tree_format()
  {
    // Arrange
    let root_path = std::path::Path::new( env!( "CARGO_MANIFEST_DIR" ) );
    let assets_relative_path = std::path::Path::new( ASSET_PATH );
    let assets_path = root_path.join( assets_relative_path );

    let temp = assert_fs::TempDir::new().unwrap();
    temp.copy_from( assets_path.join( "workspace_with_cyclic_dependency" ), &[ "**" ] ).unwrap();

    let args = ListOptions::former()
      .path_to_manifest( crate_dir( &temp.join( "a" ) ) )
      .format( ListFormat::Tree )
      .info([ PackageAdditionalInfo::Version ])
      .dependency_sources([ DependencySource::Local, DependencySource::Remote ])
      .dependency_categories([ DependencyCategory::Primary, DependencyCategory::Dev ])
      .form();

    // Act
    let output = action::list( args ).unwrap();

    // Assert
    let ListReport::Tree( trees ) = &output else { panic!( "Expected `Tree` format, but found another" ) };
    dbg!( trees );

    assert_eq!( 1, trees.len() );
    let tree = &trees[ 0 ];
    assert_eq!( "_workspace_with_cyclic_dep_a", tree.info.name.as_str() );
    assert_eq!( "0.1.0", tree.info.version.as_ref().unwrap().as_str() );

    assert_eq!( 1, tree.info.normal_dependencies.len() );
    assert!( tree.info.dev_dependencies.is_empty() );
    assert!( tree.info.build_dependencies.is_empty() );

    let sub_tree = &tree.info.normal_dependencies[ 0 ];
    assert_eq!( "_workspace_with_cyclic_dep_b", sub_tree.name.as_str() );
    assert_eq!( "*", sub_tree.version.as_ref().unwrap().as_str() );

    assert_eq!( 1, sub_tree.normal_dependencies.len() );
    assert!( sub_tree.dev_dependencies.is_empty() );
    assert!( sub_tree.build_dependencies.is_empty() );

    let mega_sub_tree = &sub_tree.normal_dependencies[ 0 ];
    assert_eq!( "_workspace_with_cyclic_dep_a", mega_sub_tree.name.as_str() );
    assert_eq!( "*", mega_sub_tree.version.as_ref().unwrap().as_str() );

    assert_eq!( 1, mega_sub_tree.normal_dependencies.len() );
    assert!( mega_sub_tree.dev_dependencies.is_empty() );
    assert!( mega_sub_tree.build_dependencies.is_empty() );

    // (*) - means duplication
    let ultra_sub_tree = &mega_sub_tree.normal_dependencies[ 0 ];
    assert_eq!( "_workspace_with_cyclic_dep_b", ultra_sub_tree.name.as_str() );
    assert_eq!( "*", sub_tree.version.as_ref().unwrap().as_str() );
    assert!( ultra_sub_tree.duplicate );
    assert_eq!( "*", ultra_sub_tree.version.as_ref().unwrap().as_str() );

    assert!( ultra_sub_tree.normal_dependencies.is_empty() );
    assert!( ultra_sub_tree.dev_dependencies.is_empty() );
    assert!( ultra_sub_tree.build_dependencies.is_empty() );
  }

  #[ test ]
  fn can_not_show_list_with_cyclic_dependencies()
  {
    // Arrange
    let root_path = std::path::Path::new( env!( "CARGO_MANIFEST_DIR" ) );
    let assets_relative_path = std::path::Path::new( ASSET_PATH );
    let assets_path = root_path.join( assets_relative_path );

    let temp = assert_fs::TempDir::new().unwrap();
    temp.copy_from( assets_path.join( "workspace_with_cyclic_dependency" ), &[ "**" ] ).unwrap();

    let args = ListOptions::former()
      .path_to_manifest( crate_dir( &temp.join( "a" ) ) )
      .format( ListFormat::Topological )
      .dependency_sources([ DependencySource::Local, DependencySource::Remote ])
      .dependency_categories([ DependencyCategory::Primary, DependencyCategory::Dev ])
      .form();

    // Act
    let output = action::list( args );

    // Assert

    // can not process topological sorting for cyclic dependencies
    assert!( output.is_err() );
  }
}
