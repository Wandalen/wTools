use super::*;
use assert_fs::prelude::*;

fn arrange( source : &str ) -> assert_fs::TempDir
{
  let root_path = std::path::Path::new( env!( "CARGO_MANIFEST_DIR" ) );
  let assets_relative_path = std::path::Path::new( ASSET_PATH );
  let assets_path = root_path.join( assets_relative_path );

  let temp = assert_fs::TempDir::new().unwrap();
  temp.copy_from( assets_path.join( source ), &[ "**" ] ).unwrap();

  temp
}

#[ test ]
fn package_no_features()
{
  // Arrange
  let temp = arrange( "three_packages/b" );
  // let x : PathBuf = temp.path().to_owned();
  let options = willbe::action::features::FeaturesOptions::former()
  .crate_dir( willbe::CrateDir::try_from( temp.path().to_owned() ).unwrap() )
  .form();

  // Act
  let report = willbe::action::features( options ).unwrap().to_string();

  // Assert
  assert!( report.contains(
"\
Package _three_packages_b:\
" ) );
}

#[ test ]
fn package_features()
{
  // Arrange
  let temp = arrange( "three_packages_with_features/b" );
  let options = willbe::action::features::FeaturesOptions::former()
  .crate_dir( willbe::CrateDir::try_from( temp.path().to_owned() ).unwrap() )
  .form();

  // Act
  let report = willbe::action::features( options ).unwrap().to_string();

  // Assert
  assert!( report.contains(
"\
Package _three_packages_with_features_b:
\t_three_packages_with_features_c
\tboo
\tdefault
\tenabled\
" ) );
}

#[ test ]
fn package_features_with_features_deps()
{
  let temp = arrange( "three_packages_with_features/b" );
  let options = willbe::action::features::FeaturesOptions::former()
  .crate_dir( willbe::CrateDir::try_from( temp.path().to_owned() ).unwrap() )
  .with_features_deps( true )
  .form();

  // Act
  let report = willbe::action::features( options ).unwrap().to_string();

  // Assert
  assert!( report.contains(
"\
Package _three_packages_with_features_b:
\t_three_packages_with_features_c: [dep:_three_packages_with_features_c]
\tboo: [_three_packages_with_features_c]
\tdefault: [boo]
\tenabled: []\
" ) );
}

#[ test ]
fn workspace_no_features()
{
  // Arrange
  let temp = arrange( "three_packages" );
  let options = willbe::action::features::FeaturesOptions::former()
  .crate_dir( willbe::CrateDir::try_from( temp.path().to_owned() ).unwrap() )
  .form();

  // Act
  let report = willbe::action::features( options ).unwrap().to_string();

  // Assert
  assert!( report.contains(
"\
Package _three_packages_b:\
" ) );

  assert!( report.contains(
"\
Package _three_packages_c:\
" ) );

  assert!( report.contains(
"\
Package _three_packages_d:\
" ) );
}

#[ test ]
fn workspace_features()
{
  // Arrange
  let temp = arrange( "three_packages_with_features" );
  let options = willbe::action::features::FeaturesOptions::former()
  .crate_dir( willbe::CrateDir::try_from( temp.path().to_owned() ).unwrap() )
  .form();

  // Act
  let report = willbe::action::features( options ).unwrap().to_string();

  // Assert
  assert!( report.contains(
"\
Package _three_packages_with_features_b:
\t_three_packages_with_features_c
\tboo
\tdefault
\tenabled\
" ) );

  assert!( report.contains(
"\
Package _three_packages_with_features_c:
\tdefault
\tenabled
\tfoo\
" ) );

  assert!( report.contains(
"\
Package _three_packages_with_features_d:
\tenabled\
" ) );
}

#[ test ]
fn workspace_features_with_features_deps()
{
  // Arrange
  let temp = arrange( "three_packages_with_features" );
  let options = willbe::action::features::FeaturesOptions::former()
  .crate_dir( willbe::CrateDir::try_from( temp.path().to_owned() ).unwrap() )
  .with_features_deps( true )
  .form();

  // Act
  let report = willbe::action::features( options ).unwrap().to_string();

  // Assert
  assert!( report.contains(
"\
Package _three_packages_with_features_b:
\t_three_packages_with_features_c: [dep:_three_packages_with_features_c]
\tboo: [_three_packages_with_features_c]
\tdefault: [boo]
\tenabled: []\
" ) );

  assert!( report.contains(
"\
Package _three_packages_with_features_c:
\tdefault: [foo]
\tenabled: []
\tfoo: []\
" ) );

  assert!( report.contains(
"\
Package _three_packages_with_features_d:
\tenabled: []\
" ) );
}
