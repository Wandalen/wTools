use crate::*;
use assert_cmd::Command;
use inc::
{
  action::test::ProjectBuilder,
  // aaa : for Petro : move to helper. don't reuse test-rs files in command and endpoints
  // aaa : move to helper module
  helpers::BINARY_NAME,
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

#[ test ]
fn plan_test()
{
  let temp = TempDir::new().unwrap();
  let temp = &temp;

  let project = ProjectBuilder::new( "tttest" )
  .toml_file( "" )
  .test_file( r#"
  #[test]
  fn should_fail() {
    panic!();
  }
  "#)
  .build( temp )
  .unwrap();

  let with_default = Command::cargo_bin( BINARY_NAME ).unwrap()
  .args([ ".test" ])
  .current_dir( project.clone() )
  .assert();
  let out = String::from_utf8( with_default.get_output().stdout.clone() ).unwrap();

  assert!
  (
    out.contains
    (
      r#"  [ optimization : debug | channel : stable | feature : no-features ]
  [ optimization : debug | channel : nightly | feature : no-features ]
  [ optimization : release | channel : stable | feature : no-features ]
  [ optimization : release | channel : nightly | feature : no-features ]
"#
    )
  );
}
