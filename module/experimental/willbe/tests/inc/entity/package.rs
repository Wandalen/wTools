use super :: *;
use assert_fs ::TempDir;
use the_module ::
{
  CrateDir,
  package :: { Package, publish_need },
};

#[ test ]
fn publish_need_true_when_remote_missing()
{
  // Arrange : a name × version that has never been published to crates.io.
  // static.crates.io answers such requests with a real, deterministic HTTP 403 —
  // this exercises `publish_need`'s `Err( ureq ::Error ::StatusCode( 403 ) )` branch
  // without mocking the network.
  let crate_name = "nonexistent_crate_xyz";
  let crate_version = "0.1.0";

  let temp = TempDir ::new().unwrap();
  std ::fs ::create_dir_all( temp.join( "src" ) ).unwrap();
  std ::fs ::write( temp.join( "src" ).join( "lib.rs" ), [] ).unwrap();
  std ::fs ::write
  (
    temp.join( "Cargo.toml" ),
    format!( "[package]\nname = \"{crate_name}\"\nversion = \"{crate_version}\"\nedition = \"2021\"\n" ),
  ).unwrap();

  // an empty local `.crate` file is enough : `CrateArchive::decode` special-cases empty input
  let package_dir = temp.join( "package" );
  std ::fs ::create_dir_all( &package_dir ).unwrap();
  std ::fs ::write( package_dir.join( format!( "{crate_name}-{crate_version}.crate" ) ), [] ).unwrap();

  let crate_dir = CrateDir ::try_from( temp.to_path_buf() ).unwrap();
  let package = Package ::try_from( crate_dir ).unwrap();

  // Act
  let need = publish_need( &package, Some( temp.to_path_buf() ), temp.path() ).unwrap();

  // Assert
  assert!( need );
}
