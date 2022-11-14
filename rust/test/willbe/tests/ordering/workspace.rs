use super::*;

#[ test ]
fn alphabetical()
{
  let package_asset = Asset::from( PathBuf::from( ASSET_PATH ).join( "workspaces/workspace1/module/module1" ) ).copied();
  let package_path = package_asset.path_buf();
  let first_package = Package::try_from( package_path.to_owned() ).unwrap();

  let package_asset = Asset::from( PathBuf::from( ASSET_PATH ).join( "workspaces/workspace1/module/module2" ) ).copied();
  let package_path = package_asset.path_buf();
  let second_package = Package::try_from( package_path.to_owned() ).unwrap();

  let package_asset = Asset::from( PathBuf::from( ASSET_PATH ).join( "workspaces/workspace2/module/module3" ) ).copied();
  let package_path = package_asset.path_buf();
  let third_package = Package::try_from( package_path.to_owned() ).unwrap();

  let source =
  [
    &second_package,  // module2
    &first_package,   // module1
    &third_package,   // module3
  ];

  let expected = vec![ &first_package, &second_package, &third_package ];

  assert_eq!
  (
    expected.iter().map( | p | p.path().to_owned() ).collect::< Vec< _ > >(),
    source.into_iter().cloned()

    .ordered_iter( OrderStrategy::Alphabetical )

    .map( | p | p.path().to_owned() ).collect::< Vec< _ > >()
  );
}

#[ test ]
fn topological()
{
  let workspace_asset = Asset::from( PathBuf::from( ASSET_PATH ).join( "workspace_with_deps" ) ).copied();
  let workspace_path = workspace_asset.path_buf();
  let first_package = Package::try_from( workspace_path.to_owned().join( "module/module1" ) ).unwrap();
  let second_package = Package::try_from( workspace_path.to_owned().join( "module/module2" ) ).unwrap();
  let third_package = Package::try_from( workspace_path.to_owned().join( "module/module3" ) ).unwrap();

  let source =
  [
    &third_package,   // module3 dependent on module2
    &first_package,   // module1
    &second_package,  // module2 dependent on module1
  ];

  let expected = vec![ &first_package, &second_package, &third_package ];

  assert_eq!
  (
    expected.iter().map( | p | p.path().to_owned() ).collect::< Vec< _ > >(),
    source.into_iter().cloned()

    .ordered_iter( OrderStrategy::Topological )

    .map( | p | p.path().to_owned() ).collect::< Vec< _ > >()
  );
}

#[ test ]
fn random()
{
  let package_asset = Asset::from( PathBuf::from( ASSET_PATH ).join( "workspaces/workspace1/module/module1" ) ).copied();
  let package_path = package_asset.path_buf();
  let first_package = Package::try_from( package_path.to_owned() ).unwrap();

  let package_asset = Asset::from( PathBuf::from( ASSET_PATH ).join( "workspaces/workspace1/module/module2" ) ).copied();
  let package_path = package_asset.path_buf();
  let second_package = Package::try_from( package_path.to_owned() ).unwrap();

  let package_asset = Asset::from( PathBuf::from( ASSET_PATH ).join( "workspaces/workspace2/module/module3" ) ).copied();
  let package_path = package_asset.path_buf();
  let third_package = Package::try_from( package_path.to_owned() ).unwrap();

  let source =
  [
    &second_package,  // module2
    &first_package,   // module1
    &third_package,   // module3
  ];

  dbg!
  (
    source.into_iter().cloned()

    .ordered( OrderStrategy::Random )

    .iter().cloned()
    .filter_map( | p | PackageMetadata::try_from( p ).ok() )
    .map( | p | p.name().to_owned() )
    .collect::< Vec< _ > >()
  );

  // TODO: make some check. Eg: source is not equal to ordered, but... It may be equal because random
  assert!( true );
}
