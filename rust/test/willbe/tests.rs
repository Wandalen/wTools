use super::*;

#[ test ]
fn package_from_path()
{
  let path = to_asset_path( PathBuf::from( "package" ) );
  let package = Package::try_from( path.to_owned() );

  assert!( package.is_ok() );
  assert_eq!( path, *package.unwrap().path() );
}

#[ test ]
fn workspace_from_path()
{
  assert!( Workspace::try_from( to_asset_path( PathBuf::from( "package" ) ) ).is_err() );
  assert!( Workspace::try_from( to_asset_path( PathBuf::from( "workspaces/workspace1" ) ) ).is_ok() );
}

#[ test ]
fn workspace_iterator()
{
  let workspace = Workspace::try_from( to_asset_path( PathBuf::from( "workspaces/workspace1" ) ) ).unwrap();
  let packages = workspace.packages_iterate( OrderStrategy::Random ).collect::< Vec< _ > >();

  assert!( !packages.is_empty() );
}

#[ test ]
fn iterate_over_path_buf()
{
  let dir_without_crates = packages_iterate( to_asset_path( PathBuf::from( "empty" ) ), OrderStrategy::Random ).collect::< Vec< _ > >();
  assert!( dir_without_crates.is_empty() );

  let package = packages_iterate( to_asset_path( PathBuf::from( "package" ) ), OrderStrategy::Random ).collect::< Vec< _ > >();
  assert_eq!( 1, package.len() );

  let workspace = packages_iterate( to_asset_path( PathBuf::from( "workspaces/workspace1" ) ) , OrderStrategy::Random ).collect::< Vec< _ > >();
  assert!( workspace.len() >= 1 );

  let many_workspaces = packages_iterate( to_asset_path( PathBuf::from( "workspaces" ) ) , OrderStrategy::Random ).collect::< Vec< _ > >();
  assert!( many_workspaces.len() > workspace.len() );
}

#[ test ]
fn iterate_over_workspaces()
{
  let workspaces = vec!
  [
    Workspace::try_from( to_asset_path( PathBuf::from( "workspaces/workspace1" ) ) ).unwrap(),
    Workspace::try_from( to_asset_path( PathBuf::from( "workspaces/workspace2" ) ) ).unwrap(),
  ];

  let packages_into_workspaces = workspaces_packages_iterate
  (
    workspaces.into_iter(),
    OrderStrategy::Random
  )
  .collect::< Vec< _ > >();

  assert!( !packages_into_workspaces.is_empty() );
}

#[ test ]
fn get_info()
{
  let package = Package::try_from( to_asset_path( PathBuf::from( "package" ) ) ).unwrap();
  let info = package.info();

  assert!( !info.name.is_empty() );
  assert!( !info.version.is_empty() );
}

#[ test ]
fn verification()
{
  let package = Package::try_from( to_asset_path( PathBuf::from( "package" ) ) ).unwrap();

  assert!( package.has_license() );
  assert!( package.has_readme() );
  assert!( package.has_documentation() );
  assert!( package.is_tests_passed() );
}
