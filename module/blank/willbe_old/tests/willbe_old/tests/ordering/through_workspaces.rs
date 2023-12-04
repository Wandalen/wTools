use super::*;

#[ test ]
fn alphabetical()
{
  let workspace_asset = Asset::from( PathBuf::from( ASSET_PATH ).join( "workspaces/workspace1" ) ).copied();
  let workspace_path = workspace_asset.path_buf();
  let first_workspace = Workspace::try_from( workspace_path.to_owned() ).unwrap();

  let workspace_asset = Asset::from( PathBuf::from( ASSET_PATH ).join( "workspaces/workspace2" ) ).copied();
  let workspace_path = workspace_asset.path_buf();
  let second_workspace = Workspace::try_from( workspace_path.to_owned() ).unwrap();

  let source = 
  [
    &second_workspace, // module3, module4, module 5
    &first_workspace,   // module1, module2
  ];

  let expected =
  [
    "willbe_workspace1_module1",
    "willbe_workspace1_module2",

    "willbe_workspace2_module3",
    "willbe_workspace2_module4",
    "willbe_workspace2_module5"
  ];

  assert_eq!
  (
    expected.iter().map( | m | m.to_string() ).collect::< Vec< _ > >(),
    workspaces_packages_iterate( source.into_iter().cloned() )

    .ordered_iter( OrderStrategy::Alphabetical )

    .filter_map( | p | PackageMetadata::try_from( p ).ok() )
    .map( | p | p.name().to_owned() )
    .collect::< Vec< _ > >()
  );
}

#[ test ]
fn random()
{
  let workspace_asset = Asset::from( PathBuf::from( ASSET_PATH ).join( "workspaces/workspace1" ) ).copied();
  let workspace_path = workspace_asset.path_buf();
  let first_workspace = Workspace::try_from( workspace_path.to_owned() ).unwrap();

  let workspace_asset = Asset::from( PathBuf::from( ASSET_PATH ).join( "workspaces/workspace2" ) ).copied();
  let workspace_path = workspace_asset.path_buf();
  let second_workspace = Workspace::try_from( workspace_path.to_owned() ).unwrap();

  let source = 
  [
    &second_workspace, // module3, module4, module 5
    &first_workspace,   // module1, module2
  ];

  dbg!
  (
    workspaces_packages_iterate( source.into_iter().cloned() )

    .ordered( OrderStrategy::Random )

    .iter().cloned()
    .filter_map( | p | PackageMetadata::try_from( p ).ok() )
    .map( | p | p.name().to_owned() )
    .collect::< Vec< _ > >()
  );

  // TODO: make some check. Eg: source is not equal to ordered, but... It may be equal because random
  assert!( true );
}
