use std::
{
  io::Write,
  path::Path,
  fs::{ DirBuilder, File },
  process::{ Command, Stdio },
};

pub fn start_sync< AP, Args, Arg, P >
(
  application : AP,
  args: Args,
  path : P,
) -> String where AP : AsRef< Path >, Args : IntoIterator< Item = Arg >, Arg : AsRef< std::ffi::OsStr >, P : AsRef< Path >,
{
  let ( application, path ) = ( application.as_ref(), path.as_ref() );
  let args: Vec< std::ffi::OsString > = args.into_iter().map( | a | a.as_ref().into() ).collect();
  let child = Command::new( application ).args( &args ).stdout( Stdio::piped() ).stderr( Stdio::piped() ).current_dir( path ).spawn().unwrap();
  let output = child.wait_with_output().unwrap();
  
  if !output.status.success()
  {
    println!( "{}", String::from_utf8( output.stderr ).unwrap() );
  }

  String::from_utf8( output.stdout ).unwrap()
}

#[ test ]
fn help_command_with_optional_params()
{
  let temp = assert_fs::TempDir::new().unwrap();

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
  
  let main = r#"use wca::{ Type, VerifiedCommand };
  fn main(){
   let ca = wca::CommandsAggregator::former()
   .command( "echo" )
     .hint( "prints all subjects and properties" )
     .subject().hint( "Subject" ).kind( Type::String ).optional( true ).end()
     .property( "property" ).hint( "simple property" ).kind( Type::String ).optional( true ).end()
     .routine( | o : VerifiedCommand | { println!( "= Args\n{:?}\n\n= Properties\n{:?}\n", o.args, o.props ) } )
     .end()
   .perform();
 
   let args = std::env::args().skip( 1 ).collect::< Vec< String > >();
   ca.perform( args ).unwrap();
   }
  "#;
  File::create( temp.path().join( "Cargo.toml" ) ).unwrap().write_all( toml.as_bytes() ).unwrap();
  DirBuilder::new().create( temp.join( "src" ) ).unwrap();
  File::create( temp.path().join( "src" ).join( "main.rs" ) ).unwrap().write_all( main.as_bytes() ).unwrap();
  let result = start_sync( "cargo", [ "r", ".help", "echo" ], temp.path() );
  assert_eq!
  (
    "Help command\n\n.echo < subjects > < properties > - prints all subjects and properties\n\nSubjects:\n\t- Subject [?String]\nProperties:\n\tproperty - simple property [?String]\n",
    result
  );
}

#[ test ]
fn help_command_with_nature_order()
{
  let temp = assert_fs::TempDir::new().unwrap();

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

  let main = r#"fn main()
 {
   use wca::{ Type, VerifiedCommand, Order };
 
   let ca = wca::CommandsAggregator::former()
   .command( "c" )
     .hint( "c" )
     .property( "c-property" ).kind( Type::String ).optional( true ).end()
     .property( "b-property" ).kind( Type::String ).optional( true ).end()
     .property( "a-property" ).kind( Type::String ).optional( true ).end()
     .routine( | o : VerifiedCommand | { println!("c") } )
     .end()
   .command( "b" )
     .hint( "b" )
     .property( "b-property" ).kind( Type::String ).optional( true ).end()
     .routine( | o : VerifiedCommand | { println!("b") } )
     .end()
   .command( "a" )
     .hint( "a" )
     .property( "a-property" ).kind( Type::String ).optional( true ).end()
     .routine( | o : VerifiedCommand | { println!("a") } )
     .end()
   .order( Order::Nature )
 
   .perform();
 
   let args = std::env::args().skip( 1 ).collect::< Vec< String > >();
   ca.perform( args ).unwrap();
 }"#;

  File::create( temp.path().join( "Cargo.toml" ) ).unwrap().write_all( toml.as_bytes() ).unwrap();
  DirBuilder::new().create( temp.join( "src" ) ).unwrap();
  File::create( temp.path().join( "src" ).join( "main.rs" ) ).unwrap().write_all( main.as_bytes() ).unwrap();
  
  let result = start_sync( "cargo", [ "r", ".help" ], temp.path() );

  assert_eq!
  (
    "Help command\n\n.c  - c\n.b  - b\n.a  - a\n",
    result
  );

  let result = start_sync( "cargo", [ "r", ".help", "c" ], temp.path() );

  println!( "{result}" );
  
  assert_eq!
  (
    "Help command\n\n.c  - c\n\nProperties:\n\tc-property -  [?String]\n\tb-property -  [?String]\n\ta-property -  [?String]\n",
    result
  );
}

#[ test ]
fn help_command_with_lexicography_order()
{
  let temp = assert_fs::TempDir::new().unwrap();

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

  let main = r#"fn main()
 {
   use wca::{ Type, VerifiedCommand, Order };
 
   let ca = wca::CommandsAggregator::former()
   .command( "c" )
     .hint( "c" )
     .property( "c-property" ).kind( Type::String ).optional( true ).end()
     .property( "b-property" ).kind( Type::String ).optional( true ).end()
     .property( "a-property" ).kind( Type::String ).optional( true ).end()
     .routine( | o : VerifiedCommand | { println!("c") } )
     .end()
   .command( "b" )
     .hint( "b" )
     .property( "b-property" ).kind( Type::String ).optional( true ).end()
     .routine( | o : VerifiedCommand | { println!("b") } )
     .end()
   .command( "a" )
     .hint( "a" )
     .property( "a-property" ).kind( Type::String ).optional( true ).end()
     .routine( | o : VerifiedCommand | { println!("a") } )
     .end()
     .order( Order::Lexicography )
   .perform();
 
   let args = std::env::args().skip( 1 ).collect::< Vec< String > >();
   ca.perform( args ).unwrap();
 }"#;

  File::create( temp.path().join( "Cargo.toml" ) ).unwrap().write_all( toml.as_bytes() ).unwrap();
  DirBuilder::new().create( temp.join( "src" ) ).unwrap();
  File::create( temp.path().join( "src" ).join( "main.rs" ) ).unwrap().write_all( main.as_bytes() ).unwrap();
  
  let result = start_sync( "cargo", [ "r", ".help" ], temp.path() );

  assert_eq!
  (
    "Help command\n\n.a  - a\n.b  - b\n.c  - c\n",
    result
  );

  let result = start_sync( "cargo", [ "r", ".help", "c" ], temp.path() );

  assert_eq!
  (
    "Help command\n\n.c  - c\n\nProperties:\n\ta-property -  [?String]\n\tb-property -  [?String]\n\tc-property -  [?String]\n",
    result
  );
}
