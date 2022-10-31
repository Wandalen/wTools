use std::path::PathBuf;

use willbe::prelude::*;

#[ test ]
fn package_from_path()
{
  // current package is a package
  let path = PathBuf::from( "" );
  let package = Package::try_from( path.to_owned() );

  assert!( package.is_ok() );
  assert_eq!( path, *package.unwrap().path() );
}

#[ test ]
fn workspace_from_path()
{
  // current package is a package, not a workspace
  assert!( Workspace::try_from( PathBuf::from( "" ) ).is_err() );
  // main workspace is a workspace
  assert!( Workspace::try_from( PathBuf::from( "../../../" ) ).is_ok() );
}

#[ test ]
fn workspace_iterator()
{
  let workspace = Workspace::try_from( PathBuf::from( "../../../" ) ).unwrap();

  let packages = workspace.packages_iterate( OrderStrategy::Random ).collect::< Vec< _ > >();

  // current workspace must be not empty
  assert!( !packages.is_empty() );
}

#[ test ]
fn iterate_over_path_buf()
{
  // current package has only one package
  let current_package = packages_iterate( PathBuf::from( "" ), OrderStrategy::Random ).collect::< Vec< _ > >();
  assert_eq!( 1, current_package.len() );

  // current workspace has at least one package
  let current_workspace = packages_iterate( PathBuf::from( "../../../" ), OrderStrategy::Random ).collect::< Vec< _ > >();
  assert!( current_workspace.len() >= 1 );

  // module folder has no workspace nor package
  let dir_without_crates = packages_iterate( PathBuf::from( "../../" ), OrderStrategy::Random ).collect::< Vec< _ > >();
  assert!( dir_without_crates.is_empty() );
}

#[ test ]
fn iterate_over_workspaces()
{
  // list of workspaces to iterate
  // * must be more than one
  let workspaces = vec!
  [
    Workspace::try_from( PathBuf::from( "../../../" ) ).unwrap(),
  ];

  let packages_into_workspaces = workspaces_packages_iterate
  (
    workspaces.into_iter(),
    OrderStrategy::Random
  )
  .collect::< Vec< _ > >();

  assert!( !packages_into_workspaces.is_empty() );
}
