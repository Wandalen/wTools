// use super::*;
// 
// use process_tools::process::*;
// 
// use std::env::consts::EXE_EXTENSION;
// use std::ffi::OsString;
// use std::path::{ Path, PathBuf };
// use std::process::Command;
// 
// pub fn path_to_exe( name : &Path, temp_path : &Path ) -> PathBuf
// {
//   _ = Command::new("rustc")
//   .current_dir( temp_path )
//   .arg( name )
//   .status()
//   .unwrap();
// 
//   PathBuf::from( temp_path )
//   .join( name.file_name().unwrap() )
//   .with_extension( EXE_EXTENSION )
// }
// 
// #[ test ]
// fn err_out_err()
// {
//   let temp = assert_fs::TempDir::new().unwrap();
//   let root_path = Path::new( env!( "CARGO_MANIFEST_DIR" ) );
//   let assets_relative_path = Path::new( ASSET_PATH );
//   let assets_path = root_path.join( assets_relative_path );
// 
//   let args : [ OsString ; 0 ] = [];
// 
//   let options = process::Run::former()
//   .bin_path( path_to_exe( &assets_path.join( "err_out_test" ).join( "err_out_err.rs" ), temp.path() ) )
//   .args( args.to_vec() )
//   .current_path( temp.to_path_buf() )
//   .joining_streams( true )
//   .form();
// 
//   let report = process::run( options ).unwrap().out;
// 
//   assert_eq!( "This is stderr text\nThis is stdout text\nThis is stderr text\n", report );
// }
// 
// #[ test ]
// fn out_err_out()
// {
//   let temp = assert_fs::TempDir::new().unwrap();
//   let root_path = Path::new( env!( "CARGO_MANIFEST_DIR" ) );
//   let assets_relative_path = Path::new( ASSET_PATH );
//   let assets_path = root_path.join( assets_relative_path );
// 
//   let args : [ OsString ; 0 ] = [];
// 
//   let options = process::Run::former()
//   .bin_path( path_to_exe( &assets_path.join( "err_out_test" ).join( "out_err_out.rs" ), temp.path() ) )
//   .args( args.to_vec() )
//   .current_path( temp.to_path_buf() )
//   .joining_streams( true )
//   .form();
//   let report = process::run( options ).unwrap().out;
// 
//   assert_eq!( "This is stdout text\nThis is stderr text\nThis is stdout text\n", report );
// }
// 
