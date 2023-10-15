use super::*;

#[ test ]
fn many_workspaces_each_info() -> Result< (), Box< dyn std::error::Error > >
{
  let mut cmd = Command::cargo_bin( MODULE_NAME )?;
  let package_asset = Asset::from( PathBuf::from( ASSET_PATH ).join( "workspaces" ) ).copied();
  let package_path = package_asset.path_buf();

  cmd.current_dir( package_path );
  cmd.arg( ".each .crate.info" );

  cmd
  .assert()
  .success()
  .stdout
  (
    predicate::str::contains( "Name: \"willbe_workspace1_module1\"" ).count( 1 )
    .and
    (
      predicate::str::contains( "Name: \"willbe_workspace1_module2\"" ).count( 1 )
    )
    .and
    (
      predicate::str::contains( "Name: \"willbe_workspace2_module3\"" ).count( 1 )
    )
    .and
    (
      predicate::str::contains( "Name: \"willbe_workspace2_module4\"" ).count( 1 )
    )
    .and
    (
      predicate::str::contains( "Name: \"willbe_workspace2_module5\"" ).count( 1 )
    )
  );

  Ok( () )
}

#[ test ]
fn workspace_each_info() -> Result< (), Box< dyn std::error::Error > >
{
  let mut cmd = Command::cargo_bin( MODULE_NAME )?;
  let package_asset = Asset::from( PathBuf::from( ASSET_PATH ).join( "workspace_with_deps" ) ).copied();
  let package_path = package_asset.path_buf();

  cmd.current_dir( package_path );
  cmd.arg( ".each .crate.info" );

  cmd
  .assert()
  .success()
  .stdout
  (
    predicate::str::contains( "Name: \"willbe_with_deps_module1\"" ).count( 1 )
    .and
    (
      predicate::str::contains( "Name: \"willbe_with_deps_module2\"" ).count( 1 )
    )
    .and
    (
      predicate::str::contains( "Name: \"willbe_with_deps_module3\"" ).count( 1 )
    )
  );

  Ok( () )
}

#[ test ]
fn single_package_each_info() -> Result< (), Box< dyn std::error::Error > >
{
  let mut cmd = Command::cargo_bin( MODULE_NAME )?;
  let package_asset = Asset::from( PathBuf::from( ASSET_PATH ).join( "package" ) ).copied();
  let package_path = package_asset.path_buf();

  cmd.current_dir( package_path );
  cmd.arg( ".each .crate.info" );

  cmd
  .assert()
  .success()
  .stdout
  (
    predicate::str::contains( "Name: \"willbe_verified_package\"" ).count( 1 )
  );

  Ok( () )
}

#[ test ]
fn empty_path_each_info() -> Result< (), Box< dyn std::error::Error > >
{
  let mut cmd = Command::cargo_bin( MODULE_NAME )?;
  let package_asset = Asset::from( PathBuf::from( ASSET_PATH ).join( "empty" ) ).copied();
  let package_path = package_asset.path_buf();

  cmd.current_dir( package_path );
  cmd.arg( ".each .crate.info" );

  cmd
  .assert()
  .success()
  .stdout
  (
    predicate::str::contains( "Any package was found at current directory" )
  );

  Ok( () )
}

#[ test ]
fn another_command_after_each() -> Result< (), Box< dyn std::error::Error > >
{
  let mut cmd = Command::cargo_bin( MODULE_NAME )?;
  let package_asset = Asset::from( PathBuf::from( ASSET_PATH ).join( "package" ) ).copied();
  let package_path = package_asset.path_buf();

  cmd.current_dir( package_path );
  cmd.arg( ".each .crate.info .end .crate.info" );

  cmd
  .assert()
  .success()
  .stdout
  (
    predicate::str::contains( "Name: \"willbe_verified_package\"" ).count( 2 )
  );

  Ok( () )
}
