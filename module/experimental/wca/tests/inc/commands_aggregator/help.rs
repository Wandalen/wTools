use std ::
{
  io ::Write,
  path ::{ Path, PathBuf },
  fs :: { DirBuilder, File, create_dir_all },
  process :: { Command, Stdio },
};

pub fn start_sync< AP, Args, Arg, P >( application: AP, args: Args, path: P, target_dir: Option< &Path > ) -> String
where
  AP: AsRef< Path >,
  Args: IntoIterator< Item = Arg >,
  Arg: AsRef< std ::ffi ::OsStr >,
  P: AsRef< Path >,
{
  let ( application, path ) = ( application.as_ref(), path.as_ref() );
  let args: Vec< std ::ffi ::OsString > = args.into_iter().map( |a| a.as_ref().into() ).collect();
  let mut cmd = Command ::new( application );
  cmd
  .args( &args )
  .stdout( Stdio ::piped() )
  .stderr( Stdio ::piped() )
  .current_dir( path );
  if let Some( dir ) = target_dir
  {
  cmd.env( "CARGO_TARGET_DIR", dir );
 }
  let child = cmd.spawn().unwrap();
  let output = child.wait_with_output().unwrap();

  if !output.status.success()
  {
  println!( "{}", String ::from_utf8( output.stderr ).unwrap() );
 }

  String ::from_utf8( output.stdout ).unwrap()
}

// Shared stable target dir for all help integration tests.
// All 4 tests build the same wca dep set, so cargo fingerprinting lets them
// share artifacts. Cargo's file lock serializes concurrent builds safely.
// Located in the system temp dir (outside the workspace) to avoid conflicts
// with concurrent workspace builds that set their own CARGO_TARGET_DIR.
fn stable_target_dir() -> PathBuf
{
  let dir = std::env::temp_dir().join( "wca_integration_tests" );
  create_dir_all( &dir ).expect( "failed to create stable target dir" );
  dir
}

#[ test ]
fn help_command_with_optional_params()
{
  let temp = assert_fs ::TempDir ::new().unwrap();

  let toml = format!(
  r#"[package]
name = "wca_hello_test_optional_params"
version = "0.1.0"
edition = "2021"
[dependencies]
wca = {{path = "{}", features = ["enabled"]}}"#,
  env!("CARGO_MANIFEST_DIR").replace('\\', "/")
 );

  let main = r#"use wca :: { Type, VerifiedCommand };
  fn main()
  {
   let ca = wca ::CommandsAggregator ::former()
   .command( "echo" )
  .hint( "prints all subjects and properties" )
  .subject().hint( "Subject" ).kind( Type ::String ).optional( true ).end()
  .property( "property" ).hint( "simple property" ).kind( Type ::String ).optional( true ).end()
  .routine( | o: VerifiedCommand | { println!( "= Args\n{:?}\n\n= Properties\n{:?}\n", o.args, o.props ) } )
  .end()
   .perform();
 
   let args = std ::env ::args().skip( 1 ).collect :: < Vec< String > >();
   ca.perform( args ).unwrap();
 }
  "#;
  File ::create(temp.path().join("Cargo.toml"))
  .unwrap()
  .write_all(toml.as_bytes())
  .unwrap();
  DirBuilder ::new().create(temp.join("src")).unwrap();
  File ::create(temp.path().join("src").join("main.rs"))
  .unwrap()
  .write_all(main.as_bytes())
  .unwrap();
  let target = stable_target_dir();
  let result = start_sync( "cargo", ["r", ".help", "echo"], temp.path(), Some( &target ) );
  assert_eq!
  (
  "Help command\n\n.echo < subjects > < properties > - prints all subjects and properties\n\nSubjects: \n\t- Subject [?String]\nProperties: \n\tproperty - simple property [?String]\n",
  result
 );
}

#[ test ]
fn bug_reproducer_help_should_not_execute_command()
{
  let temp = assert_fs ::TempDir ::new().unwrap();

  let toml = format!(
  r#"[package]
name = "wca_hello_test_help_execution_bug"
version = "0.1.0"
edition = "2021"
[dependencies]
wca = {{path = "{}", features = ["enabled"]}}"#,
  env!("CARGO_MANIFEST_DIR").replace('\\', "/")
 );

  let main = r#"use wca :: { Type, VerifiedCommand };
  fn main()
  {
   let ca = wca ::CommandsAggregator ::former()
   .command( "echo" )
  .hint( "prints all subjects and properties" )
  .subject().hint( "Subject" ).kind( Type ::String ).optional( true ).end()
  .property( "property" ).hint( "simple property" ).kind( Type ::String ).optional( true ).end()
  .routine( | _o: VerifiedCommand | { println!( "COMMAND EXECUTED" ) } )
  .end()
   .perform();

   let args = std ::env ::args().skip( 1 ).collect :: < Vec< String > >();
   ca.perform( args ).unwrap();
 }
  "#;
  File ::create(temp.path().join("Cargo.toml"))
  .unwrap()
  .write_all(toml.as_bytes())
  .unwrap();
  DirBuilder ::new().create(temp.join("src")).unwrap();
  File ::create(temp.path().join("src").join("main.rs"))
  .unwrap()
  .write_all(main.as_bytes())
  .unwrap();

  let target = stable_target_dir();
  let result = start_sync( "cargo", ["r", ".help", "echo"], temp.path(), Some( &target ) );

  // Help for specific command should NOT execute the command
  assert!( !result.contains( "COMMAND EXECUTED" ),
  "Help command should not execute the target command. Output was: {result}" );

  // Should contain help text for the echo command
  assert!( result.contains( "prints all subjects and properties" ),
  "Help output should contain command description. Output was: {result}" );
}

#[ test ]
fn help_command_with_nature_order()
{
  let temp = assert_fs ::TempDir ::new().unwrap();

  let toml = format!(
  r#"[package]
name = "wca_hello_test_nature_order"
version = "0.1.0"
edition = "2021"
[dependencies]
wca = {{path = "{}", features = ["enabled"]}}"#,
  env!("CARGO_MANIFEST_DIR").replace('\\', "/")
 );

  let main = r#"fn main()
 {
   use wca :: { Type, VerifiedCommand, Order };
 
   let ca = wca ::CommandsAggregator ::former()
   .command( "c" )
  .hint( "c" )
  .property( "c-property" ).kind( Type ::String ).optional( true ).end()
  .property( "b-property" ).kind( Type ::String ).optional( true ).end()
  .property( "a-property" ).kind( Type ::String ).optional( true ).end()
  .routine( | _o: VerifiedCommand | { println!("c") } )
  .end()
   .command( "b" )
  .hint( "b" )
  .property( "b-property" ).kind( Type ::String ).optional( true ).end()
  .routine( | _o: VerifiedCommand | { println!("b") } )
  .end()
   .command( "a" )
  .hint( "a" )
  .property( "a-property" ).kind( Type ::String ).optional( true ).end()
  .routine( | _o: VerifiedCommand | { println!("a") } )
  .end()
   .order( Order ::Nature )
 
   .perform();
 
   let args = std ::env ::args().skip( 1 ).collect :: < Vec< String > >();
   ca.perform( args ).unwrap();
 }"#;

  File ::create(temp.path().join("Cargo.toml"))
  .unwrap()
  .write_all(toml.as_bytes())
  .unwrap();
  DirBuilder ::new().create(temp.join("src")).unwrap();
  File ::create(temp.path().join("src").join("main.rs"))
  .unwrap()
  .write_all(main.as_bytes())
  .unwrap();

  let target = stable_target_dir();
  let result = start_sync( "cargo", ["r", ".help"], temp.path(), Some( &target ) );

  assert_eq!( "Help command\n\n.c  - c\n.b  - b\n.a  - a\n", result );

  let result = start_sync( "cargo", ["r", ".help", "c"], temp.path(), Some( &target ) );

  println!( "{result}" );

  assert_eq!(
  "Help command\n\n.c  - c\n\nProperties: \n\tc-property -  [?String]\n\tb-property -  [?String]\n\ta-property -  [?String]\n",
  result
 );
}

#[ test ]
fn help_command_with_lexicography_order()
{
  let temp = assert_fs ::TempDir ::new().unwrap();

  let toml = format!(
  r#"[package]
name = "wca_hello_test_lexicography_order"
version = "0.1.0"
edition = "2021"
[dependencies]
wca = {{path = "{}", features = ["enabled"]}}"#,
  env!("CARGO_MANIFEST_DIR").replace('\\', "/")
 );

  let main = r#"fn main()
 {
   use wca :: { Type, VerifiedCommand, Order };
 
   let ca = wca ::CommandsAggregator ::former()
   .command( "c" )
  .hint( "c" )
  .property( "c-property" ).kind( Type ::String ).optional( true ).end()
  .property( "b-property" ).kind( Type ::String ).optional( true ).end()
  .property( "a-property" ).kind( Type ::String ).optional( true ).end()
  .routine( | _o: VerifiedCommand | { println!("c") } )
  .end()
   .command( "b" )
  .hint( "b" )
  .property( "b-property" ).kind( Type ::String ).optional( true ).end()
  .routine( | _o: VerifiedCommand | { println!("b") } )
  .end()
   .command( "a" )
  .hint( "a" )
  .property( "a-property" ).kind( Type ::String ).optional( true ).end()
  .routine( | _o: VerifiedCommand | { println!("a") } )
  .end()
  .order( Order ::Lexicography )
   .perform();
 
   let args = std ::env ::args().skip( 1 ).collect :: < Vec< String > >();
   ca.perform( args ).unwrap();
 }"#;

  File ::create(temp.path().join("Cargo.toml"))
  .unwrap()
  .write_all(toml.as_bytes())
  .unwrap();
  DirBuilder ::new().create(temp.join("src")).unwrap();
  File ::create(temp.path().join("src").join("main.rs"))
  .unwrap()
  .write_all(main.as_bytes())
  .unwrap();

  let target = stable_target_dir();
  let result = start_sync( "cargo", ["r", ".help"], temp.path(), Some( &target ) );

  assert_eq!( "Help command\n\n.a  - a\n.b  - b\n.c  - c\n", result );

  let result = start_sync( "cargo", ["r", ".help", "c"], temp.path(), Some( &target ) );

  assert_eq!(
  "Help command\n\n.c  - c\n\nProperties: \n\ta-property -  [?String]\n\tb-property -  [?String]\n\tc-property -  [?String]\n",
  result
 );
}
