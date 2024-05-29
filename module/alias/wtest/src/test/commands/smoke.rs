use std::env::current_dir;
use std::path::PathBuf;
use wtools::error::BasicError;
use ::wpublisher::manifest::Manifest;
use wca::{ Args, Props, Type };
use wtools::error::Result;

pub( crate ) fn smoke_command() -> wca::Command
{
  wca::Command::former()
  .hint( "Perform smoke testing on module." )
  .long_hint( "Perform smoke testing on module." )
  .phrase( "smoke" )
  .property( "smoke", "A variant of smoke testing of module. It can be:\n  local - local module in directory.\n  published - module published on `crates.io`. true - local and published version.\n  Default is \"local\"", Type::String, true )
  .property( "code_path", "A path to code snippet to test. By default utility imports module into binary.", Type::Path, true )
  .property( "version", "A string version of module. By default \"*\"", Type::String, true )
  .form()
}

pub( crate ) fn smoke_with_subject_command() -> wca::Command
{
  wca::Command::former()
  .hint( "Perform smoke testing on module by path." )
  .long_hint( "Perform smoke testing on module by path." )
  .phrase( "smoke" )
  .subject( "A path to module. Should be a directory with file `Cargo.toml`. Default is current directory.", Type::Path, true )
  .property( "smoke", "A variant of smoke testing of module. It can be:\n  local - local module in directory.\n  published - module published on `crates.io`. true - local and published version.\n  Default is \"local\"", Type::String, true )
  .property( "code_path", "A path to code snippet to test. By default utility imports module into binary.", Type::Path, true )
  .property( "version", "A string version of module. By default \"*\"", Type::String, true )
  .form()
}

///
/// Perform smoke testing.
///

pub fn smoke( ( args, props ) : ( Args, Props ) ) -> Result< () >
{
  println!( "Command \".smoke\"" );
  let mut current_path = current_dir().unwrap();

  let subject_path = args.get_owned::< PathBuf >( 0 ).unwrap_or_default();
  let module_path = if subject_path.is_relative()
  {
    current_path.push( args.get_owned::< PathBuf >( 0 ).unwrap_or_default() );
    current_path
  }
  else
  {
    subject_path
  };

  let mut manifest_path = module_path.clone();
  manifest_path.push( "Cargo.toml" );

  if !manifest_path.exists()
  {
    let msg = format!( "Current directory {:?} has no file \"Cargo.toml\"", module_path.canonicalize().unwrap() );
    return Err( BasicError::new( msg ) );
  }

  let mut manifest = Manifest::new();
  manifest.manifest_path_from_str( &manifest_path ).unwrap();
  manifest.load().unwrap();
  let data = manifest.manifest_data.as_deref().unwrap();

  /* */

  let module_name = &data[ "package" ][ "name" ].clone();
  let module_name = module_name.as_str().unwrap();

  let code_path = match props.get_owned( "code_path" )
  {
    Some( path ) => path,
    None => PathBuf::default(),
  };

  let mut data = None;
  if code_path.exists()
  {
    data = Some( std::fs::read_to_string( code_path ).unwrap() );
  }

  let version = match props.get_owned( "version" )
  {
    Some( x ) => x,
    None => "*".to_string(),
  };

  let smoke = match props.get_owned( "smoke" )
  {
    Some( x ) => x,
    None =>
    {
      if let Ok( x ) = std::env::var( "WITH_SMOKE" )
      {
        x
      }
      else
      {
        "local".to_string()
      }
    },
  };

  /* */

  if smoke != "false" && smoke != "0"
  {
    let mut threads = vec![];
    if smoke == "local" || smoke != "published"
    {
      let module_name = module_name.to_owned();
      let data = data.clone();
      let version = version.clone();
      let thread = std::thread::spawn( move ||
      {
        let mut t = SmokeModuleTest::new( module_name );
        t.test_postfix( "_test_local" );
        if data.is_some()
        {
          t.code( data.as_ref().unwrap() );
        }
        t.version( version.as_str() );
        t.local_path_clause( module_path.to_str().unwrap() );

        t.clean( true ).unwrap();
        t.form().unwrap();
        t.perform().unwrap();
        t.clean( false ).unwrap();
      });
      threads.push( thread );
    }

    if smoke == "published" || smoke != "local"
    {
      let module_name = module_name.to_owned();
      let data = data;
      let version = version;
      let thread = std::thread::spawn( move ||
      {
        let mut t = SmokeModuleTest::new( module_name );
        t.test_postfix( "_test_published" );
        if data.is_some()
        {
          t.code( data.as_ref().unwrap() );
        }
        t.version( version.as_str() );

        t.clean( true ).unwrap();
        t.form().unwrap();
        t.perform().unwrap();
        t.clean( false ).unwrap();
      });
      threads.push( thread );
    }

    for thread in threads
    {
      thread.join().unwrap();
    }
  }

  Ok( () )
}

//

#[ derive( Debug ) ]
struct SmokeModuleTest< 'a >
{
  pub dependency_name : String,
  pub version : &'a str,
  pub local_path_clause : &'a str,
  pub code : String,
  pub test_path : std::path::PathBuf,
  pub test_postfix : &'a str,
}

impl< 'a > SmokeModuleTest< 'a >
{
  fn new( dependency_name : String ) -> SmokeModuleTest< 'a >
  {
    let test_postfix = "_smoke_test";
    let smoke_test_path = format!( "{}{}", dependency_name, test_postfix );
    let mut test_path = std::env::temp_dir();
    test_path.push( smoke_test_path );

    SmokeModuleTest
    {
      dependency_name,
      version : "*",
      local_path_clause : "",
      code : "".to_string(),
      test_path,
      test_postfix,
    }
  }

  fn version( &mut self, version : &'a str ) -> &mut SmokeModuleTest< 'a >
  {
    self.version = version;
    self
  }

  fn local_path_clause( &mut self, local_path_clause : &'a str ) -> &mut SmokeModuleTest< 'a >
  {
    self.local_path_clause = local_path_clause;
    self
  }

  fn test_postfix( &mut self, test_postfix : &'a str ) -> &mut SmokeModuleTest< 'a >
  {
    self.test_postfix = test_postfix;
    let smoke_test_path = format!( "{}{}", self.dependency_name, test_postfix );
    self.test_path.pop();
    self.test_path.push( smoke_test_path );
    self
  }

  fn code( &mut self, code : impl AsRef< str > + 'a ) -> &mut SmokeModuleTest< 'a >
  {
    self.code = code.as_ref().into();
    self
  }

  fn form( &mut self ) -> Result< (), &'static str >
  {
    std::fs::create_dir( &self.test_path ).unwrap();

    let mut test_path = self.test_path.clone();

    /* create binary test module */
    let test_name = format!( "{}{}", self.dependency_name, self.test_postfix );
    let output = std::process::Command::new( "cargo" )
    .current_dir( &test_path )
    .args([ "new", "--bin", &test_name ])
    .output()
    .expect( "Failed to execute command" );
    println!( "Creating smoke binary module :\n\n{}", std::str::from_utf8( &output.stderr ).expect( "Found invalid UTF-8" ) );

    test_path.push( &test_name );

    /* setup config */
    #[ cfg( target_os = "windows" ) ]
    let local_path_clause = if self.local_path_clause.is_empty() { "".to_string() } else { format!( ", path = \"{}\"", self.local_path_clause.escape_default() ) };
    #[ cfg( not( target_os = "windows" ) ) ]
    let local_path_clause = if self.local_path_clause.is_empty() { "".to_string() } else { format!( ", path = \"{}\"", self.local_path_clause ) };
    let dependencies_section = format!( "{} = {{ version = \"{}\" {} }}", self.dependency_name, self.version, &local_path_clause );
    let config_data = format!
    (
      "[package]
      edition = \"2021\"
      name = \"{}_smoke_test\"
      version = \"0.0.1\"

      [dependencies]
      {}",
      &self.dependency_name,
      &dependencies_section
    );
    let mut config_path = test_path.clone();
    config_path.push( "Cargo.toml" );
    println!( "Manifest of module \"{}\" :\n\n      {}\n", test_name, config_data );
    std::fs::write( config_path, config_data ).unwrap();

    /* write code */
    test_path.push( "src" );
    test_path.push( "main.rs" );
    if self.code.is_empty()
    {
      self.code = format!( "use ::{}::*;", self.dependency_name );
    }
    let code = format!
    (
      "#[ allow( unused_imports ) ]
      fn main()
      {{
        {}
      }}",
      self.code,
    );
    self.code = code;
    std::fs::write( &test_path, &self.code ).unwrap();

    Ok( () )
  }

  fn perform( &self ) -> Result<(), BasicError>
  {
    let mut test_path = self.test_path.clone();
    let test_name = format!( "{}{}", self.dependency_name, self.test_postfix );
    test_path.push( test_name );

    let output = std::process::Command::new( "cargo" )
    .current_dir( test_path )
    .args([ "run", "--release" ])
    .output()
    .unwrap();
    println!( "{}", std::str::from_utf8( &output.stdout ).expect( "Found invalid UTF-8" ) );
    println!( "{}", std::str::from_utf8( &output.stderr ).expect( "Found invalid UTF-8" ) );
    println!( "Process status :\n  {}\n", output.status );
    println!( "Code :\n\n      {}\n", self.code );

    if !output.status.success()
    {
      return Err( BasicError::new( "Smoke test failed" ) );
    }

    Ok( () )
  }

  fn clean( &self, force : bool ) -> Result<(), &'static str>
  {
    let result = std::fs::remove_dir_all( &self.test_path );
    if force
    {
      result.unwrap_or_default();
    }
    else
    {
      let msg = format!( "Cannot remove temporary directory {}. Please, remove it manually", &self.test_path.display() );
      result.expect( &msg );
    }
    Ok( () )
  }

}
