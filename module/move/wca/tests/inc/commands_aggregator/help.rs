use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};
use assert_fs::fixture::PathCopy;

const ASSET_PATH : &str = concat!( env!("CARGO_MANIFEST_DIR"), "/tests/assets/" );


fn arrange( source: &str ) -> assert_fs::TempDir
{
  let root_path = Path::new( env!( "CARGO_MANIFEST_DIR" ) );
  let assets_relative_path = Path::new( ASSET_PATH );
  let assets_path = root_path.join( assets_relative_path );

  let temp = assert_fs::TempDir::new().unwrap();
  temp.copy_from( assets_path.join( source ), &[ "**" ] ).unwrap();

  temp
}
pub fn start_sync< AP, Args, Arg, P >
(
  application : AP,
  args: Args,
  path : P,
) -> String where AP : AsRef< Path >, Args : IntoIterator< Item = Arg >, Arg : AsRef< std::ffi::OsStr >, P : AsRef< Path >,
{
  let ( application, path ) = ( application.as_ref(), path.as_ref() );
  let args = args.into_iter().map( | a | a.as_ref().into() ).collect::< Vec< std::ffi::OsString > >();
  let child = Command::new( application ).args( &args ).stdout( Stdio::piped() ).stderr( Stdio::piped() ).current_dir( path ).spawn().unwrap();
  let output = child.wait_with_output().unwrap();
  dbg!( &output );
  
  String::from_utf8( output.stdout ).unwrap()
}

#[ test ]
fn help_command_with_optional_params()
{
  let toml = format!
  (
    r#"[package]
name = "wca_hello_test"
version = "0.1.0"
edition = "2021"
[dependencies]
wca = {{path = "{}"}}"#,
    env!( "CARGO_MANIFEST_DIR" ).replace( "\\", "/" )
  ) ;

  let temp = arrange( "wca_hello_test" );
  let mut file = File::create( temp.path().join( "Cargo.toml" ) ).unwrap();
  file.write_all( toml.as_bytes() ).unwrap();
  let result = start_sync( "cargo", [ "r", ".help", "echo" ], temp.path() );

  assert_eq!
  (
    "Help command\n\n.echo < subjects > < properties > - prints all subjects and properties\n\nSubjects:\n\t- Subject [?String]\nProperties:\n\tproperty - simple property [?String]\n",
    result
  );
}

/// `wca_help_test_nature_order/src/main.rs` :
/// ```rust
/// fn main()
/// {
///   use wca::{ Type, VerifiedCommand };
/// 
///   let ca = wca::CommandsAggregator::former()
///   .command( "c" )
///     .hint( "c" )
///     .property( "c-property" ).kind( Type::String ).optional( true ).end()
///     .property( "b-property" ).kind( Type::String ).optional( true ).end()
///     .property( "a-property" ).kind( Type::String ).optional( true ).end()
///     .routine( | o : VerifiedCommand | { println!("c") } )
///     .end()
///   .command( "b" )
///     .hint( "b" )
///     .property( "b-property" ).kind( Type::String ).optional( true ).end()
///     .routine( | o : VerifiedCommand | { println!("b") } )
///     .end()
///   .command( "a" )
///     .hint( "a" )
///     .property( "a-property" ).kind( Type::String ).optional( true ).end()
///     .routine( | o : VerifiedCommand | { println!("a") } )
///     .end()
///   .with_nature_sort( true )
///   .perform();
/// 
///   let args = std::env::args().skip( 1 ).collect::< Vec< String > >();
///   ca.perform( args ).unwrap();
/// }
/// ```
#[ test ]
fn help_command_with_nature_order()
{
  let toml = format!
  (
    r#"[package]
name = "wca_hello_test"
version = "0.1.0"
edition = "2021"
[dependencies]
wca = {{path = "{}"}}"#,
    env!( "CARGO_MANIFEST_DIR" ).replace( "\\", "/" )
  ) ;

  let temp = arrange( "wca_help_test_nature_order" );
  let mut file = File::create( temp.path().join( "Cargo.toml" ) ).unwrap();
  file.write_all( toml.as_bytes() ).unwrap();
  let result = start_sync( "cargo", [ "r", ".help" ], temp.path() );

  // dbg!(&result);
  assert_eq!
  (
    "Help command\n\n.c  - c\n.b  - b\n.a  - a\n",
    result
  );

  let result = start_sync( "cargo", [ "r", ".help", "c" ], temp.path() );

  println!("{result}");
  
  assert_eq!
  (
    "Help command\n\n.c  - c\n\nProperties:\n\tc-property -  [?String]\n\tb-property -  [?String]\n\ta-property -  [?String]\n",
    result
  );
}

/// `wca_help_test_lexicography_order/src/main.rs` :
/// ```rust
/// fn main()
/// {
///   use wca::{ Type, VerifiedCommand };
/// 
///   let ca = wca::CommandsAggregator::former()
///   .command( "c" )
///     .hint( "c" )
///     .property( "c-property" ).kind( Type::String ).optional( true ).end()
///     .property( "b-property" ).kind( Type::String ).optional( true ).end()
///     .property( "a-property" ).kind( Type::String ).optional( true ).end()
///     .routine( | o : VerifiedCommand | { println!("c") } )
///     .end()
///   .command( "b" )
///     .hint( "b" )
///     .property( "b-property" ).kind( Type::String ).optional( true ).end()
///     .routine( | o : VerifiedCommand | { println!("b") } )
///     .end()
///   .command( "a" )
///     .hint( "a" )
///     .property( "a-property" ).kind( Type::String ).optional( true ).end()
///     .routine( | o : VerifiedCommand | { println!("a") } )
///     .end()
///     .with_nature_sort( false )
///   .perform();
/// 
///   let args = std::env::args().skip( 1 ).collect::< Vec< String > >();
///   ca.perform( args ).unwrap();
/// }
/// ```
#[ test ]
fn help_command_with_lexicography_order()
{
  let toml = format!
  (
    r#"[package]
name = "wca_hello_test"
version = "0.1.0"
edition = "2021"
[dependencies]
wca = {{path = "{}"}}"#,
    env!( "CARGO_MANIFEST_DIR" ).replace( "\\", "/" )
  ) ;

  let temp = arrange( "wca_help_test_lexicography_order" );
  let mut file = File::create( temp.path().join( "Cargo.toml" ) ).unwrap();
  file.write_all( toml.as_bytes() ).unwrap();
  let result = start_sync( "cargo", [ "r", ".help" ], temp.path() );

  // dbg!(&result);
  assert_eq!
  (
    "Help command\n\n.a  - a\n.b  - b\n.c  - c\n",
    result
  );

  let result = start_sync( "cargo", [ "r", ".help", "c" ], temp.path() );

  dbg!(&result);
  assert_eq!
  (
    "Help command\n\n.c  - c\n\nProperties:\n\ta-property -  [?String]\n\tb-property -  [?String]\n\tc-property -  [?String]\n",
    result
  );
}
