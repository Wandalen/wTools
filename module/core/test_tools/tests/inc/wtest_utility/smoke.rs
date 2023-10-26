
use meta_tools::tests_impls;
use meta_tools::tests_index;

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
    assert!( stdout.contains( "smoke <properties> - Perform smoke testing on module." ) );
  }

  //

  fn basic_with_only_command()
  {
    #[ cfg( debug_assertions ) ]
    let dir = std::ffi::OsStr::new( "../../../target/debug/" );
    #[ cfg( not( debug_assertions ) ) ]
    let dir = std::ffi::OsStr::new( "../../../target/release/" );
    let path = "./wtest";
    #[ cfg( target_family="windows" ) ]
    let path = format!( "{}wtest", dir.to_str().unwrap() );
    let proc = std::process::Command::new( path )
    .arg( ".smoke " )
    .current_dir( dir )
    .output().unwrap();
    assert!( !proc.status.success() );
    let stderr = std::str::from_utf8( proc.stderr.as_slice() ).unwrap();
    assert!( stderr.contains( r#"has no file \"Cargo.toml\""# ) );
    let stdout = std::str::from_utf8( proc.stdout.as_slice() ).unwrap();
    assert!( stdout.contains( "Command \".smoke\"\n" ) );
  }
}

//

tests_index!
{
  basic_no_args,
  basic_with_only_command,
}
