use super::*;

#[ test ]
fn over_workspace()
{
  use std::collections::HashSet;

  let workspace_asset = Asset::from( PathBuf::from( ASSET_PATH ).join( "workspaces/workspace1" ) );
  let workspace_path = workspace_asset.path_buf();
  let workspace = Workspace::try_from( workspace_path.to_owned() ).unwrap();
  // `workspace.packages()` and `workspace.packages_iterate().collect::< Vec< _ > >()` is the same
  let packages = workspace.packages();

  let expected = HashSet::from([ "willbe_workspace1_module1".to_owned(), "willbe_workspace1_module2".to_owned() ]);

  assert_eq!( expected.len(), packages.len() );
  assert_eq!
  (
    expected,
    packages.iter().cloned()
    .filter_map( | p |
    {
      PackageMetadata::try_from( p ).ok()
    })
    .map( | meta | meta.name().to_owned() )
    .collect::< HashSet< _ > >()
  );
}

#[ test ]
fn over_workspaces_iterator()
{
  use std::collections::HashSet;

  let assets = vec!
  [
    Asset::from( PathBuf::from( ASSET_PATH ).join( "workspaces/workspace1" ) ),
    Asset::from( PathBuf::from( ASSET_PATH ).join( "workspaces/workspace2" ) ),
  ];
  let workspaces = assets.iter()
  .map( | asset | Workspace::try_from( asset.path_buf().to_owned() ) )
  .filter_map( Result::ok )
  .collect::< Vec< _ > >();

  let packages = workspaces_packages_iterate( workspaces.into_iter() )
  .collect::< Vec< _ > >();

  let expected = HashSet::from(
  [
    "willbe_workspace1_module1".to_owned(),
    "willbe_workspace1_module2".to_owned(),

    "willbe_workspace2_module3".to_owned(),
    "willbe_workspace2_module4".to_owned(),
    "willbe_workspace2_module5".to_owned(),
  ]);

  assert_eq!( expected.len(), packages.len() );
  assert_eq!
  (
    expected,
    packages.iter().cloned()
    .filter_map( | p |
    {
      PackageMetadata::try_from( p ).ok()
    })
    .map( | meta | meta.name().to_owned() )
    .collect::< HashSet< _ > >()
  );
}

#[ test ]
fn over_empty_path()
{
  let empty_asset = Asset::from( PathBuf::from( ASSET_PATH ).join( "empty" ) );
  let empty_path = empty_asset.path_buf();
  let packages = packages_iterate( empty_path.to_owned() ).collect::< Vec< _ > >();

  assert!( packages.is_empty() );
}

#[ test ]
fn over_single_package_path()
{
  let package_asset = Asset::from( PathBuf::from( ASSET_PATH ).join( "package" ) ).copied();
  let package_path = package_asset.path_buf();
  let package = packages_iterate( package_path.to_owned() ).collect::< Vec< _ > >();

  assert_eq!( 1, package.len() );
  assert_eq!( "willbe_verified_package", PackageMetadata::try_from( package[ 0 ].clone() ).unwrap().all().name.as_str() );
}

#[ test ]
fn over_single_workspace_path()
{
  use std::collections::HashSet;

  let workspace_asset = Asset::from( PathBuf::from( ASSET_PATH ).join( "workspaces/workspace1" ) ).copied();
  let workspace_path = workspace_asset.path_buf();
  let packages = packages_iterate( workspace_path.to_owned() ).collect::< Vec< _ > >();

  let expected = HashSet::from([ "willbe_workspace1_module1".to_owned(), "willbe_workspace1_module2".to_owned() ]);

  assert_eq!( expected.len(), packages.len() );
  assert_eq!
  (
    expected,
    packages.iter().cloned()
    .filter_map( | p |
    {
      PackageMetadata::try_from( p ).ok()
    })
    .map( | meta | meta.name().to_owned() )
    .collect::< HashSet< _ > >()
  );
}

#[ test ]
fn over_workspaces_root_path()
{
  use std::collections::HashSet;

  let many_workspaces_asset = Asset::from( PathBuf::from( ASSET_PATH ).join( "workspaces" ) ).copied();
  let many_workspaces_path = many_workspaces_asset.path_buf();
  let packages = packages_iterate( many_workspaces_path.to_owned() ).collect::< Vec< _ > >();

  let expected = HashSet::from(
  [
    "willbe_workspace1_module1".to_owned(),
    "willbe_workspace1_module2".to_owned(),

    "willbe_workspace2_module3".to_owned(),
    "willbe_workspace2_module4".to_owned(),
    "willbe_workspace2_module5".to_owned(),
  ]);

  assert_eq!( expected.len(), packages.len() );
  assert_eq!
  (
    expected,
    packages.iter().cloned()
    .filter_map( | p |
    {
      PackageMetadata::try_from( p ).ok()
    })
    .map( | meta | meta.name().to_owned() )
    .collect::< HashSet< _ > >()
  );
}
