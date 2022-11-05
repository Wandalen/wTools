use super::*;
use utility::*;

const ASSET_PATH : &str = "rust/test/willbe/_asset";

#[ test ]
fn package_from_path()
{
  let asset = Asset::from( PathBuf::from( ASSET_PATH ).join( "package" ) );
  let path = asset.path_buf();

  let package = Package::try_from( path.to_owned() );

  assert!( package.is_ok() );
  assert_eq!( *path, *package.unwrap().path() );
}

#[ test ]
fn workspace_from_path()
{
  let package_asset = Asset::from( PathBuf::from( ASSET_PATH ).join( "package" ) );
  let package_path = package_asset.path_buf();
  assert!( Workspace::try_from( package_path.to_owned() ).is_err() );

  let workspace_asset = Asset::from( PathBuf::from( ASSET_PATH ).join( "workspaces/workspace1" ) );
  let workspace_path = workspace_asset.path_buf();
  assert!( Workspace::try_from( workspace_path.to_owned() ).is_ok() );
}

#[ test ]
fn workspace_iterator()
{
  let workspace_asset = Asset::from( PathBuf::from( ASSET_PATH ).join( "workspaces/workspace1" ) );
  let workspace_path = workspace_asset.path_buf();
  let workspace = Workspace::try_from( workspace_path.to_owned() ).unwrap();
  let packages = workspace.packages_iterate( OrderStrategy::Random ).collect::< Vec< _ > >();

  assert!( !packages.is_empty() );
}

#[ test ]
fn iterate_over_path_buf()
{
  let empty_asset = Asset::from( PathBuf::from( ASSET_PATH ).join( "empty" ) );
  let empty_path = empty_asset.path_buf();
  let dir_without_crates = packages_iterate( empty_path.to_owned(), OrderStrategy::Random ).collect::< Vec< _ > >();
  assert!( dir_without_crates.is_empty() );

  let package_asset = Asset::from( PathBuf::from( ASSET_PATH ).join( "package" ) );
  let package_path = package_asset.path_buf();
  let package = packages_iterate( package_path.to_owned(), OrderStrategy::Random ).collect::< Vec< _ > >();
  assert_eq!( 1, package.len() );

  let workspace_asset = Asset::from( PathBuf::from( ASSET_PATH ).join( "workspaces/workspace1" ) );
  let workspace_path = workspace_asset.path_buf();
  let workspace = packages_iterate( workspace_path.to_owned(), OrderStrategy::Random ).collect::< Vec< _ > >();
  assert!( workspace.len() >= 1 );

  let many_workspaces_asset = Asset::from( PathBuf::from( ASSET_PATH ).join( "workspaces" ) );
  let many_workspaces_path = many_workspaces_asset.path_buf();
  let many_workspaces = packages_iterate( many_workspaces_path.to_owned(), OrderStrategy::Random ).collect::< Vec< _ > >();
  assert!( many_workspaces.len() > workspace.len() );
}

#[ test ]
fn iterate_over_workspaces()
{
  let assets = vec!
  [
    Asset::from( PathBuf::from( ASSET_PATH ).join( "workspaces/workspace1" ) ),
    Asset::from( PathBuf::from( ASSET_PATH ).join( "workspaces/workspace2" ) ),
  ];
  let workspaces = assets.iter()
  .map( | asset | Workspace::try_from( asset.path_buf().to_owned() ) )
  .filter_map( Result::ok )
  .collect::< Vec< _ > >();

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
  let package_asset = Asset::from( PathBuf::from( ASSET_PATH ).join( "package" ) ).copied();
  let package_path = package_asset.path_buf();
  let package = Package::try_from( package_path.to_owned() ).unwrap();
  let info = package.info();

  assert!( !info.name.is_empty() );
  assert!( !info.version.to_string().is_empty() );
}

#[ test ]
fn verification()
{
  let asset = Asset::from( PathBuf::from( ASSET_PATH ).join( "package" ) ).copied();
  let path = asset.path_buf();

  let package = Package::try_from( path.to_owned() ).unwrap();

  assert!( package.has_license() );
  assert!( package.has_readme() );
  assert!( package.has_documentation() );
  assert!( package.is_tests_passed() );

  let asset = Asset::from( PathBuf::from( ASSET_PATH ).join( "package_no_verified" ) ).copied();
  let path = asset.path_buf();

  let package = Package::try_from( path.to_owned() ).unwrap();
  assert!( !package.has_license() );
  assert!( !package.has_readme() );
  assert!( !package.has_documentation() );
  assert!( !package.is_tests_passed() );
}
