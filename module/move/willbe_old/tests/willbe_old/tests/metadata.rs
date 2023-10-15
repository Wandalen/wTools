use super::*;

#[ test ]
fn try_get_from_empty_asset()
{
  let empty_asset = Asset::from( PathBuf::from( ASSET_PATH ).join( "empty" ) ).copied();
  let empty_asset_path = empty_asset.path_buf();

  let meta = PackageMetadata::try_from( empty_asset_path.to_owned() );

  assert!( meta.is_err() );
}

#[ test ]
fn get_info()
{
  let package_asset = Asset::from( PathBuf::from( ASSET_PATH ).join( "package" ) ).copied();
  let package_path = package_asset.path_buf();

  let meta = PackageMetadata::try_from( package_path.to_owned() ).unwrap();

  assert!( !meta.name().is_empty() );
  assert!( !meta.version().is_empty() );
}
