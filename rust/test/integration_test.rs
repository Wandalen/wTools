#[ allow( unused_imports ) ]
use test_tools::test_suite;

#[ derive( Debug ) ]
struct IntegrationModuleTest< 'a >
{
  dependency_name : &'a str,
  version : &'a str,
  code : &'a str,
  test_path : std::path::PathBuf,
}

impl< 'a > IntegrationModuleTest< 'a >
{
  fn new( dependency_name : &'a str ) -> IntegrationModuleTest< 'a >
  {
    let mut test_path = std::env::temp_dir();
    test_path.push( dependency_name );

    IntegrationModuleTest
    {
      dependency_name,
      version : "*",
      code : "",
      test_path,
    }
  }

  #[ allow( dead_code ) ]
  fn version( &mut self, version : &'a str ) -> &mut IntegrationModuleTest< 'a >
  {
    self.version = version;
    self
  }

  #[ allow( dead_code ) ]
  fn code( &mut self, code : &'a str ) -> &mut IntegrationModuleTest< 'a >
  {
    self.code = code;
    self
  }

  fn form( &self ) -> Result<(), &'static str>
  {
    std::fs::create_dir( &self.test_path ).unwrap();

    let mut test_path = self.test_path.clone();

    /* create binary test module */
    let test_name = format!( "{}_test", self.dependency_name );
    let output = std::process::Command::new( "cargo" )
    .current_dir( &test_path )
    .args([ "new", "--bin", &test_name ])
    .output()
    .expect( "Failed to execute command" );
    println!( "{}", std::str::from_utf8( &output.stderr ).expect( "Found invalid UTF-8" ) );

    test_path.push( test_name );

    /* setup config */
    let dependency_name = format!( "dependencies.{}", self.dependency_name );
    let selector = self.selector_global_or_temporary();
    let output = std::process::Command::new( &selector )
    .current_dir( &test_path )
    .args([ "set", "./Cargo.toml", &dependency_name, "*" ])
    .output()
    .expect( "Failed to execute command" );
    let output = std::str::from_utf8( &output.stdout ).expect( "Found invalid UTF-8" );
    let mut config_path = test_path.clone();
    config_path.push( "Cargo.toml" );
    std::fs::write( config_path, output ).unwrap();

    /* write code */
    test_path.push( "src" );
    test_path.push( "main.rs" );
    let code = format!( "#[ allow( unused_imports ) ]\nfn main()\n{{\n  use {}::*;\n  {}\n}}", self.dependency_name, self.code );
    std::fs::write( &test_path, code ).unwrap();

    Ok( () )
  }

  fn selector_global_or_temporary( &self ) -> String
  {
    let output = std::process::Command::new( "selector" )
    .current_dir( &self.test_path )
    .arg( "--help" )
    .output();

    if output.is_ok()
    {
      "selector".to_owned()
    }
    else
    {
      let mut tmp_path = self.test_path.clone();
      tmp_path.push( ".cargo" );

      std::fs::create_dir( &tmp_path ).unwrap();

      let _output = std::process::Command::new( "cargo" )
      .current_dir( &tmp_path )
      .args([ "install", "wselector", "--root", tmp_path.to_str().unwrap() ])
      .output()
      .expect( "Failed to install local version of selector" );

      tmp_path.push( "bin" );
      tmp_path.push( "selector" );
      let selector_str = tmp_path.to_str().unwrap();
      selector_str.to_owned()
    }
  }

  fn perform( &self ) -> Result<(), &'static str>
  {
    let mut test_path = self.test_path.clone();
    let test_name = format!( "{}_test", self.dependency_name );
    test_path.push( test_name );

    let output = std::process::Command::new( "cargo" )
    .current_dir( test_path )
    .args([ "test", "--release" ])
    .output()
    .unwrap();
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

macro_rules! module_integration_test
{
  ( $( $name : ident , )* ) =>
  {
    $(
      #[ test ]
      fn $name()
      {
        let t = IntegrationModuleTest::new( stringify!( $name ) );
        t.clean( true ).unwrap();
        t.form().unwrap();
        t.perform().unwrap();
        t.clean( false ).unwrap();
      }
    )*
  }
}

//

module_integration_test!
{
  wtools,
  wtest_basic,
}
