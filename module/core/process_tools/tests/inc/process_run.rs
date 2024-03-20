use super::*;
use the_module::process;
use std::
{
  env::consts::EXE_EXTENSION,
  path::{ Path, PathBuf },
  process::Command,
};

#[ path = "../tool/asset.rs" ]
mod asset;


// xxx : qqq : ?
// xxx2 : eliminate the function and use test_tools/process_tools instead
/// Poorly named function
pub fn path_to_exe( name : &Path, temp_path : &Path ) -> PathBuf
{

  // dbg!( name );

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
  let assets_path = asset::path().unwrap();

  // dbg!( path_to_exe( &assets_path.join( "err_out_test" ).join( "err_out_err.rs" ), temp.path() ) );

  let options = process::Run::former()
  .bin_path( path_to_exe( &assets_path.join( "err_out_test" ).join( "err_out_err.rs" ), temp.path() ) )
  .current_path( temp.to_path_buf() )
  .joining_streams( true )
  .form();

  let report = process::run( options ).unwrap();

  println!( "{}", report );

  assert_eq!( "This is stderr text\nThis is stdout text\nThis is stderr text\n", report.out );
}

#[ test ]
fn out_err_out()
{
  let temp = assert_fs::TempDir::new().unwrap();
  let assets_path = asset::path().unwrap();

  let options = process::Run::former()
  .bin_path( path_to_exe( &assets_path.join( "err_out_test" ).join( "out_err_out.rs" ), temp.path() ) )
  .current_path( temp.to_path_buf() )
  .joining_streams( true )
  .form();
  let report = process::run( options ).unwrap();

  assert_eq!( "This is stdout text\nThis is stderr text\nThis is stdout text\n", report.out );
}
