use super::*;

#[ test ]
fn from_empty_asset()
{
  let asset = Asset::from( PathBuf::from( ASSET_PATH ).join( "empty" ) );
  let path = asset.path_buf();

  let package = Package::try_from( path.to_owned() );
  assert!( package.is_err() );

  let workspace = Workspace::try_from( path.to_owned() );
  assert!( workspace.is_err() );
}

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
