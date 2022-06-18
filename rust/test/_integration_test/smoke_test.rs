
//

#[ derive( Debug ) ]
struct SmokeModuleTest< 'a >
{
  pub dependency_name : &'a str,
  pub version : &'a str,
  pub local_path_clause : &'a str,
  pub code : String,
  pub test_path : std::path::PathBuf,
  pub test_postfix : &'a str,
}

impl< 'a > SmokeModuleTest< 'a >
{
  fn new( dependency_name : &'a str ) -> SmokeModuleTest< 'a >
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

  fn code( &mut self, code : String ) -> &mut SmokeModuleTest< 'a >
  {
    self.code = code;
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
    println!( "{}", std::str::from_utf8( &output.stderr ).expect( "Found invalid UTF-8" ) );

    test_path.push( test_name );

    /* setup config */
    #[ cfg( target_os = "windows" ) ]
    let local_path_clause = if self.local_path_clause == "" { "".to_string() } else { format!( ", path = \"{}\"", self.local_path_clause.escape_default() ) };
    #[ cfg( not( target_os = "windows" ) ) ]
    let local_path_clause = if self.local_path_clause == "" { "".to_string() } else { format!( ", path = \"{}\"", self.local_path_clause ) };
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
    println!( "\n{}\n", config_data );
    std::fs::write( config_path, config_data ).unwrap();

    /* write code */
    test_path.push( "src" );
    test_path.push( "main.rs" );
    if self.code == ""
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
    println!( "\n{}\n", code );
    std::fs::write( &test_path, code ).unwrap();

    Ok( () )
  }

  fn perform( &self ) -> Result<(), &'static str>
  {
    let mut test_path = self.test_path.clone();
    let test_name = format!( "{}{}", self.dependency_name, self.test_postfix );
    test_path.push( test_name );

    let output = std::process::Command::new( "cargo" )
    .current_dir( test_path )
    .args([ "run", "--release" ])
    .output()
    .unwrap();
    println!( "status : {}", output.status );
    println!( "{}", std::str::from_utf8( &output.stdout ).expect( "Found invalid UTF-8" ) );
    println!( "{}", std::str::from_utf8( &output.stderr ).expect( "Found invalid UTF-8" ) );
    assert!( output.status.success(), "Smoke test failed" );

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

//
//   index!
//   {
//
//     new,
//     version,
//     local_path_clause,
//     code,
//     form,
//     perform,
//     clean,
//
//   }
//
//

fn smoke_test_run( test_name : &'static str, local : bool )
{
  let module_name = std::env::var( "CARGO_PKG_NAME" ).unwrap();
  let module_path = std::env::var( "CARGO_MANIFEST_DIR" ).unwrap();

  let mut code_path = std::path::PathBuf::from( module_path.clone() );
  code_path.push( "rust" );
  code_path.push( "test" );
  code_path.push( if module_name.starts_with( "w" ) { &module_name[ 1.. ] } else { module_name.as_str() } );
  code_path.push( "_asset" );
  code_path.push( "smoke.rs" );

  let mut t = SmokeModuleTest::new( module_name.as_str() );
  t.test_postfix( test_name );
  t.clean( true ).unwrap();

  let data;
  if code_path.exists()
  {
    data = std::fs::read_to_string( code_path ).unwrap();
    t.code( data );
  }

  t.version( "*" );
  if local
  {
    t.local_path_clause( module_path.as_str() );
  }

  t.form().unwrap();
  t.perform().unwrap();
  t.clean( false ).unwrap();
}

//

// qqq : make it working
#[ test ]
fn local_smoke_test()
{
  if let Ok( value ) = std::env::var( "WITH_SMOKE" )
  {
    match value.as_str()
    {
      "false" => {},
      "local" => smoke_test_run( "_local_smoke_test", true ),
      "published" => {},
      _ =>
      {
        smoke_test_run( "_local_smoke_test", true );
      },
    }
  }
}

//

#[ test ]
fn published_smoke_test()
{
  if let Ok( value ) = std::env::var( "WITH_SMOKE" )
  {
    match value.as_str()
    {
      "false" | "0" => {},
      "local" => {},
      "published" => smoke_test_run( "_published_smoke_test", false ),
      _ =>
      {
        smoke_test_run( "_published_smoke_test", false );
      },
    }
  }
}
