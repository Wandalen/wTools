use super::*;

// use the_module::*;
use assert_cmd::Command;
use inc::helper::
{
  ProjectBuilder,
  BINARY_NAME,
};

use assert_fs::TempDir;

#[ test ]
fn status_code_1_on_failure()
{
  let temp = TempDir::new().unwrap();
  let temp = &temp;

  let project = ProjectBuilder::new( "status_code" )
  .toml_file( "" )
  .test_file( r#"
    #[test]
    fn should_fail() {
      panic!();
    }
  "#)
  .build( temp )
  .unwrap();

  Command::cargo_bin( BINARY_NAME ).unwrap()
  .args([ ".tests.run", "with_nightly :0" ])
  .current_dir( project )
  .assert()
  .failure();
}

#[ test ]
fn status_code_not_zero_on_failure()
{
  let temp = TempDir::new().unwrap();
  let temp = &temp;

  let project = ProjectBuilder::new( "status_code" )
  .toml_file( "" )
  .test_file( r#"
  #[test]
  fn should_fail() {
    panic!();
  }
  "#)
  .build( temp )
  .unwrap();

  Command::cargo_bin( BINARY_NAME ).unwrap()
  .args([ ".tests.run", "with_nightly :0" ])
  .current_dir( project )
  .assert()
  .failure();
}

#[ test ]
fn status_code_not_zero_on_compile_error()
{
  let temp = TempDir::new().unwrap();
  let temp = &temp;

  let project = ProjectBuilder::new( "status_code" )
  .toml_file( "" )
  .test_file( r#"
  #[test]
  fn should_fail() {
    compile_error!("=-=");
  }
  "#)
  .build( temp )
  .unwrap();

  Command::cargo_bin( BINARY_NAME ).unwrap()
  .args([ ".tests.run", "with_nightly :0" ])
  .current_dir( project )
  .assert()
  .failure();
}
