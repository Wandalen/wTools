
use super::*;
use std::path::PathBuf;

fn tmp_dir_get( prefix : impl AsRef<str> ) -> PathBuf
{
  let mut tmp_dir = std::env::temp_dir();
  tmp_dir.push( prefix.as_ref() );
  tmp_dir
}

fn asset_copy_to_tmp( asset_dir : impl AsRef<str>, prefix : impl AsRef<str> ) -> std::io::Result< () >
{
  let tmp_dir = tmp_dir_get( prefix.as_ref() );
  std::fs::create_dir( &tmp_dir )?;
  let module_path = std::env::var( "CARGO_MANIFEST_DIR" ).unwrap();
  let mut current_dir = PathBuf::from( module_path );
  current_dir.push( "rust" );
  current_dir.push( "test" );
  current_dir.push( "publisher" );

  let dir = PathBuf::from( asset_dir.as_ref() );
  let mut dir = current_dir.join( dir );
  dir.push( prefix.as_ref() );

  if dir.is_dir()
  {
    dir_traverse( &dir.to_str().unwrap(), &tmp_dir, &dir )?
  } else
  {
    panic!( "not expected assets directory" );
  }
  Ok( () )
}

fn dir_traverse( dir : impl AsRef< str >, tmp_dir : &PathBuf, strip : &PathBuf ) -> std::io::Result< () >
{
  for entry in std::fs::read_dir( dir.as_ref() )?
  {
    let entry = entry?;
    let path = entry.path();
    if path.is_dir()
    {
      std::fs::create_dir_all( tmp_dir.join( &path.strip_prefix( strip ).unwrap() ) )?;
      dir_traverse( &path.to_str().unwrap(), tmp_dir, strip )?
    } else {
      std::fs::copy( &path, tmp_dir.join( &path.strip_prefix( strip ).unwrap() ) )?;
    }
  }
  Ok( () )
}

fn asset_clean_tmp( prefix : impl AsRef<str> ) -> std::io::Result< () >
{
  let tmp_dir = tmp_dir_get( prefix );
  std::fs::remove_dir_all( tmp_dir )
}

//

tests_impls!
{
  fn basic_no_args()
  {
    #[ cfg( debug_assertions ) ]
    let path = std::ffi::OsStr::new( "../../../target/debug/wpublisher" );
    #[ cfg( not( debug_assertions ) ) ]
    let path = std::ffi::OsStr::new( "../../../target/release/wpublisher" );
    let proc = std::process::Command::new( path ).output().unwrap();
    assert!( !proc.status.success() );
    let stderr = std::str::from_utf8( proc.stderr.as_slice() ).unwrap();
    assert_eq!( stderr, "Ambiguity. Did you mean?\n" );
    let stdout = std::str::from_utf8( proc.stdout.as_slice() ).unwrap();
    assert!( stdout.contains( ".list - List packages." ) );
  }

  //

  fn basic_with_args()
  {
    #[ cfg( debug_assertions ) ]
    let path = std::ffi::OsStr::new( "../../../target/debug/wpublisher" );
    #[ cfg( not( debug_assertions ) ) ]
    let path = std::ffi::OsStr::new( "../../../target/release/wpublisher" );
    let proc = std::process::Command::new( path ).arg( ".list" ).output().unwrap();
    assert!( proc.status.success() );
    let stdout = std::str::from_utf8( proc.stdout.as_slice() ).unwrap();
    assert_eq!( stdout, "" );
    let stderr = std::str::from_utf8( proc.stderr.as_slice() ).unwrap();
    assert_eq!( stderr, "" );
  }

  //

  fn basic_workspace_publish()
  {
    let tmp_dir = tmp_dir_get( "workspace" );
    asset_copy_to_tmp( "_asset", "workspace" ).unwrap();

    let module_path = std::env::var( "CARGO_MANIFEST_DIR" ).unwrap();
    let mut path = PathBuf::from( module_path );
    #[ cfg( debug_assertions ) ]
    path.push( "../../../target/debug/wpublisher" );
    #[ cfg( not( debug_assertions ) ) ]
    path.push( "../../../target/release/wpublisher" );

    let path = std::ffi::OsStr::new( &path );
    let proc = std::process::Command::new( path )
    .current_dir( &tmp_dir )
    .env( "CARGO_TERM_COLOR", "never" )
    .args([ ".workspace.publish", "dry:1" ])
    .output()
    .unwrap();
    assert!( proc.status.success() );
    let stdout = std::str::from_utf8( proc.stdout.as_slice() ).unwrap();
    assert!( stdout.contains( "Saved manifest data to" ) );
    let stderr = std::str::from_utf8( proc.stderr.as_slice() ).unwrap();
    assert!( stderr.contains( "Uploading module1" ) );
    assert!( stderr.contains( "Uploading module2" ) );
    asset_clean_tmp( "workspace" ).unwrap();
  }
}

//

tests_index!
{
  basic_no_args,
  basic_with_args,
  basic_workspace_publish,
}
