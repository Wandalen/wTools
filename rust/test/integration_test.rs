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

  fn version( &mut self, version : &'a str ) -> &mut IntegrationModuleTest< 'a >
  {
    self.version = version;
    self
  }

  fn code( &mut self, code : &'a str ) -> &mut IntegrationModuleTest< 'a >
  {
    self.code = code;
    self
  }

  fn form( &self ) -> Result<(), &'static str>
  {
    std::fs::create_dir( &self.test_path ).unwrap();
    let output = std::process::Command::new( "cargo" )
    .current_dir( &self.test_path )
    .args( [ "new", "--bin", self.dependency_name ] )
    .output()
    .expect("Failed to execute command");
    println!( "{:?}", output );

    Ok( () )
  }

  fn perform( &self ) -> Result<(), &'static str>
  {
    let mut test_path = self.test_path.clone();
    test_path.push( self.dependency_name );

    let output = std::process::Command::new( "cargo" )
    .current_dir( test_path )
    .arg( "run --release" )
    .output()
    .expect("Failed to execute command");
    println!( "{:?}", output );

    Ok( () )
  }

  fn clean( &self ) -> Result<(), &'static str>
  {
    std::fs::remove_dir_all( &self.test_path ).unwrap();
    Ok( () )
  }
}

//

fn run_tests_test()
{
  let t = IntegrationModuleTest::new( "wtools" );
  t.form().unwrap();
  t.clean().unwrap();
}

//

test_suite!
{
  run_tests,
  // run_samples,
}

/* xxx */
