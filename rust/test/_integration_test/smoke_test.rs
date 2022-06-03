// #[ allow( unused_imports ) ]
// use test_tools::test_suite;

//

#[ derive( Debug ) ]
struct SmokeModuleTest< 'a >
{
  pub dependency_name : &'a str,
  pub version : &'a str,
  pub local_path : &'a str,
  pub code : String,
  pub test_path : std::path::PathBuf,
  pub test_postfix : &'a str,
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
      code : "".to_string(),
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

  fn test_postfix( &mut self, src : &'a str )
  {
    self.test_postfix = src;
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
    if self.code == ""
    {
      self.code = format!( "use {}::*;", self.dependency_name );
    }
    let code = format!
    (
      "#[ allow( unused_imports ) ]\n\
      fn main()\n\
      {{\n  \
        {}\n\
      }}",
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
//     local_path,
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
  t.clean( true ).unwrap();
  t.test_postfix( test_name );

  let data;
  if code_path.exists()
  {
    data = std::fs::read_to_string( code_path ).unwrap();
    t.code( data );
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

// #[ test ]
// fn local_smoke_test()
// {
//   if let Ok( value ) = std::env::var( "WITH_SMOKE" )
//   {
//     match value.as_str()
//     {
//       "false" => {},
//       "local" => smoke_test_run( "local_smoke_test", true ),
//       "published" => {},
//       _ =>
//       {
//         smoke_test_run( "local_smoke_test", true );
//       },
//     }
//   }
// }

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
      "published" => smoke_test_run( "published_smoke_test", false ),
      _ =>
      {
        smoke_test_run( "published_smoke_test", false );
      },
    }
  }
}
