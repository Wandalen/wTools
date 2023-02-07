use super::*;

#[ test ]
fn add_dependency()
{
  let asset = Asset::from( PathBuf::from( ASSET_PATH ).join( "package" ) ).copied();
  let path = asset.path_buf();

  let package = Package::try_from( path.to_owned() );
  assert!( package.is_ok() );
  let package = package.unwrap();

  assert!( package.clone().dependency( "foo" ).add().is_ok() );

  let metadata = package.metadata();
  assert!( metadata.is_ok() );

  let metadata = metadata.unwrap();
  let deps = metadata.all().dependencies.iter().map( | dep | dep.name.as_str() ).collect::< Vec< &str > >();
  
  assert!( deps.contains( &"foo" ) );
}

#[ test ]
fn remove_dependency()
{
  let asset = Asset::from( PathBuf::from( ASSET_PATH ).join( "package" ) ).copied();
  let path = asset.path_buf();

  let package = Package::try_from( path.to_owned() );
  assert!( package.is_ok() );
  let package = package.unwrap();

  assert!( package.clone().dependency( "foo" ).add().is_ok() );

  assert!( package.clone().dependency( "foo" ).remove().is_ok() );

  let metadata = package.metadata();
  assert!( metadata.is_ok() );

  let metadata = metadata.unwrap();
  let deps = metadata.all().dependencies.iter().map( | dep | dep.name.as_str() ).collect::< Vec< &str > >();
  
  assert!( !deps.contains( &"foo" ) );
}
