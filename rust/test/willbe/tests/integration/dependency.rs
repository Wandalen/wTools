use super::*;
use predicate::str::contains;

#[ test ]
fn add_dependency() -> Result< (), Box< dyn std::error::Error > >
{
  let mut cmd = Command::cargo_bin( MODULE_NAME )?;
  let package_asset = Asset::from( PathBuf::from( ASSET_PATH ).join( "package" ) ).copied();
  let package_path = package_asset.path_buf();

  cmd.current_dir( package_path );
  cmd.args([ ".dep add foo", ".crate.info" ]);

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
    .and( contains( "Dependencies:" ).and( contains( "foo" ) ) )
  );

  Ok( () )
}

#[ test ]
fn remove_dependency() -> Result< (), Box< dyn std::error::Error > >
{
  let mut cmd = Command::cargo_bin( MODULE_NAME )?;
  let package_asset = Asset::from( PathBuf::from( ASSET_PATH ).join( "package" ) ).copied();
  let package_path = package_asset.path_buf();

  cmd.current_dir( package_path );
  cmd.args([ ".dep add foo", ".dep remove foo", ".crate.info" ]);

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
