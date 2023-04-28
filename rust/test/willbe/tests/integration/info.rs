use super::*;

#[ test ]
fn package_info() -> Result< (), Box< dyn std::error::Error > >
{
  let mut cmd = Command::cargo_bin( MODULE_NAME )?;
  let package_asset = Asset::from( PathBuf::from( ASSET_PATH ).join( "package" ) ).copied();
  let package_path = package_asset.path_buf();

  cmd.current_dir( package_path );
  cmd.arg( ".crate.info" );

  cmd
  .assert()
  .success()
  .stdout
  (
    predicate::str::contains( "Name: \"willbe_verified_package\"" )
    .and
    (
      predicate::str::contains( "Version: \"0.1.0\"" )
    )
    .and
    (
      predicate::str::contains( "Description: \"Not found\"" )
    )
    .and
    (
      predicate::str::contains( "Documentation: \"Documentation text\"" )
    )
    .and
    (
      predicate::str::contains( "License: \"MIT\"" )
    )
    .and
    (
      predicate::str::contains( "Dependencies: []" )
    )
  );

  Ok( () )
}

#[ test ]
fn workspace_path_info() -> Result< (), Box< dyn std::error::Error > >
{
  let mut cmd = Command::cargo_bin( MODULE_NAME )?;
  let package_asset = Asset::from( PathBuf::from( ASSET_PATH ).join( "workspaces/workspace1" ) ).copied();
  let package_path = package_asset.path_buf();

  cmd.current_dir( package_path );
  cmd.arg( ".crate.info" );

  cmd
  .assert()
  .failure()
  .stderr( predicate::str::contains( "Package not found at current directory" ) );

  Ok( () )
}

#[ test ]
fn empty_path_info() -> Result< (), Box< dyn std::error::Error > > 
{
  let mut cmd = Command::cargo_bin( MODULE_NAME )?;
  let package_asset = Asset::from( PathBuf::from( ASSET_PATH ).join( "empty" ) ).copied();
  let package_path = package_asset.path_buf();

  cmd.current_dir( package_path );
  cmd.arg( ".crate.info" );

  cmd
  .assert()
  .failure()
  .stderr( predicate::str::contains( "Package not found at current directory" ) );

  Ok( () )
}
