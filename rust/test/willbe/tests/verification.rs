use super::*;

#[ test ]
fn verified()
{
  let asset = Asset::from( PathBuf::from( ASSET_PATH ).join( "package" ) ).copied();
  let path = asset.path_buf();

  let meta = PackageMetadata::try_from( path.to_owned() ).unwrap();

  assert!( meta.check_all() );
}

#[ test ]
fn no_verified()
{
  let asset = Asset::from( PathBuf::from( ASSET_PATH ).join( "package_no_verified" ) ).copied();
  let path = asset.path_buf();

  let meta = PackageMetadata::try_from( path.to_owned() ).unwrap();

  assert!( !meta.has_license() );
  assert!( !meta.has_readme() );
  assert!( !meta.has_documentation() );
  assert!( !meta.is_tests_passed() );
}
