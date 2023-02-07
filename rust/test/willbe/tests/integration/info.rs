use super::*;
use predicate::str::contains;

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
    contains( "Name: \"package\"" )
    .and( contains( "Version: \"0.1.0\"" ) )
    .and( contains( "Description: \"Not found\"" ) )
    .and( contains( "Documentation: \"Documentation text\"" ) )
    .and( contains( "License: \"MIT\"" ) )
    .and( contains( "Dependencies: []" ) )
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
  .stderr( contains( "Package not found at current directory" ) );

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
  .stderr( contains( "Package not found at current directory" ) );

  Ok( () )
}
