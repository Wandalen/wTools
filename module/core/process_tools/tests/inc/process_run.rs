use super::*;
use the_module::process;
use std::
{
  env::consts::EXE_EXTENSION,
  path::{ Path, PathBuf },
  process::Command,
};

// xxx : qqq : ?
pub fn path_to_exe( name : &Path, temp_path : &Path ) -> PathBuf
{

  _ = Command::new( "rustc" )
  .current_dir( temp_path )
  .arg( name )
  .status()
  .unwrap();

  PathBuf::from( temp_path )
  .join( name.file_name().unwrap() )
  .with_extension( EXE_EXTENSION )
}

#[ test ]
fn err_out_err()
{
  let temp = assert_fs::TempDir::new().unwrap();
  let crate_path = Path::new( env!( "CARGO_MANIFEST_DIR" ) );
  let assets_path = crate_path.join( Path::new( ASSET_PATH ) );
  // let args : [ OsString ; 0 ] = [];

  dbg!( path_to_exe( &assets_path.join( "err_out_test" ).join( "err_out_err.rs" ), temp.path() ) );

  let options = process::Run::former()
  .application( path_to_exe( &assets_path.join( "err_out_test" ).join( "err_out_err.rs" ), temp.path() ) )
  // .args( args.to_vec() )
  .path( temp.to_path_buf() )
  .joining_steams( true )
  .form();

  let report = process::run( options ).unwrap();

  println!( "{}", report );

  assert_eq!( "This is stderr text\nThis is stdout text\nThis is stderr text\n", report.out );
}

#[ test ]
fn out_err_out()
{
  let temp = assert_fs::TempDir::new().unwrap();
  let crate_path = Path::new( env!( "CARGO_MANIFEST_DIR" ) );
  let assets_path = crate_path.join( Path::new( ASSET_PATH ) );
  // let args : [ OsString ; 0 ] = [];

  let options = process::Run::former()
  .application( path_to_exe( &assets_path.join( "err_out_test" ).join( "out_err_out.rs" ), temp.path() ) )
  // .args( args.to_vec() )
  .path( temp.to_path_buf() )
  .joining_steams( true )
  .form();
  let report = process::run( options ).unwrap();

  assert_eq!( "This is stdout text\nThis is stderr text\nThis is stdout text\n", report.out );
}
