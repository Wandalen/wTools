use assert_cmd ::Command;
use crate ::inc ::
{
  endpoints ::tests_run ::ProjectBuilder,
  commands ::BINARY_NAME,
};

use assert_fs ::TempDir;

#[ test ]
fn status_code_1_on_failure()
{
  let temp = TempDir ::new().unwrap();
  let temp = &temp;

  let project = ProjectBuilder ::new( "status_code" )
  .toml_file( "" )
  .test_file( r#"
    #[test]
    fn should_fail() {
      panic!();
    }
  "#)
  .build( temp )
  .unwrap();

  Command ::cargo_bin( BINARY_NAME ).unwrap()
  .args([ ".tests.run", "with_nightly :0" ])
  .current_dir( project )
  .assert()
  .failure();
}
