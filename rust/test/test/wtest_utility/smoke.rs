
use super::*;

//

tests_impls!
{
  fn basic_no_args()
  {
    #[ cfg( debug_assertions ) ]
    let path = std::ffi::OsStr::new( "../../../target/debug/wtest" );
    #[ cfg( not( debug_assertions ) ) ]
    let path = std::ffi::OsStr::new( "../../../target/release/wtest" );
    let proc = std::process::Command::new( path ).output().unwrap();
    assert!( !proc.status.success() );
    let stderr = std::str::from_utf8( proc.stderr.as_slice() ).unwrap();
    assert!( stderr.contains( "Illformed command \"\"\n" ) );
    let stdout = std::str::from_utf8( proc.stdout.as_slice() ).unwrap();
    assert!( stdout.contains( ".smoke - Perform smoke testing on module." ) );
  }

  //

  fn basic_with_only_command()
  {
    #[ cfg( debug_assertions ) ]
    let path = std::ffi::OsStr::new( "./debug/wtest" );
    #[ cfg( not( debug_assertions ) ) ]
    let path = std::ffi::OsStr::new( "./release/wtest" );
    let mut current_dir = std::env::current_dir().unwrap();
    current_dir.push( "../../../target" );
    let proc = std::process::Command::new( path )
    .current_dir( current_dir )
    .arg( ".smoke " )
    .output()
    .unwrap();
    assert!( !proc.status.success() );
    let stdout = std::str::from_utf8( proc.stdout.as_slice() ).unwrap();
    assert_eq!( stdout, "Command \".smoke\"\n" );
    let stderr = std::str::from_utf8( proc.stderr.as_slice() ).unwrap();
    println!( "{}", stderr );
    assert!( stderr.contains( "has no file \"Cargo.toml\"" ) );
  }
}

//

tests_index!
{
  basic_no_args,
  basic_with_only_command,
}
