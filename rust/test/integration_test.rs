// #[ allow( unused_imports ) ]
// use test_tools::test_suite;

#[ derive( Debug ) ]
struct SmokeModuleTest< 'a >
{
  dependency_name : &'a str,
  version : &'a str,
  local_path : &'a str,
  code : &'a str,
  test_path : std::path::PathBuf,
  test_postfix : &'a str,
}

impl< 'a > SmokeModuleTest< 'a >
{
  fn new( dependency_name : &'a str ) -> SmokeModuleTest< 'a >
  {
    let mut test_path = std::env::temp_dir();
    test_path.push( dependency_name );

    SmokeModuleTest
    {
      dependency_name,
      version : "*",
      local_path : "",
      code : "",
      test_path,
      test_postfix : "_smoke_test",
    }
  }

  fn version( &mut self, version : &'a str ) -> &mut SmokeModuleTest< 'a >
  {
    self.version = version;
    self
  }

  fn local_path( &mut self, local_path : &'a str ) -> &mut SmokeModuleTest< 'a >
  {
    self.local_path = local_path;
    self
  }

  fn code( &mut self, code : &'a str ) -> &mut SmokeModuleTest< 'a >
  {
    self.code = code;
    self
  }

  fn form( &self ) -> Result<(), &'static str>
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
    let local_path = if self.local_path == "" { "".to_string() } else { format!( ", path = \"{}\"", self.local_path ) };
    let dependencies_section = format!( "{} = {{ version = \"{}\"{} }}", self.dependency_name, self.version, &local_path );
    let config_data = format!
    (
      "[package]\n\
      edition = \"2018\"\n\
      name = \"{}_smoke_test\"\n\
      version = \"0.0.1\"\n\
      \n\
      [dependencies]\n\
      {}",
      &self.dependency_name,
      &dependencies_section
    );
    let mut config_path = test_path.clone();
    config_path.push( "Cargo.toml" );
    std::fs::write( config_path, config_data ).unwrap();

    /* write code */
    test_path.push( "src" );
    test_path.push( "main.rs" );
    let code = format!
    (
      "#[ allow( unused_imports ) ]\n\
      fn main()\n\
      {{\n  \
        use {}::*;\n  \
        {}\n\
      }}",
      self.dependency_name,
      self.code
    );
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
    assert!( output.status.success(), "Smoke test failed" );
    println!( "{}", std::str::from_utf8( &output.stdout ).expect( "Found invalid UTF-8" ) );
    println!( "{}", std::str::from_utf8( &output.stderr ).expect( "Found invalid UTF-8" ) );

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

fn smoke_test( local : bool )
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
  t.clean( true ).unwrap();

  let data;
  if code_path.exists()
  {
    data = std::fs::read_to_string( code_path ).unwrap();
    t.code( &data );
  }
  t.version( "*" );
  if local
  {
    t.local_path( module_path.as_str() );
  }
  t.form().unwrap();
  t.perform().unwrap();
  t.clean( false ).unwrap();
}

//

#[ test ]
fn integration()
{
  if let Ok( value ) = std::env::var( "WITH_SMOKE" )
  {
    match value.as_str()
    {
      "false" => {},
      "local" => smoke_test( true ),
      "published" => smoke_test( false ),
      _ =>
      {
        smoke_test( true );
        smoke_test( false );
      },
    }
  }
}

