// module/move/willbe/tests/inc/action_tests/crate_doc_test.rs
use super :: *;
use crate ::the_module :: { action, CrateDir, path ::AbsolutePath, action ::CrateDocError, Workspace };
use crate ::inc ::helper ::ProjectBuilder;
use assert_fs ::prelude :: *;
use predicates ::prelude :: *;
use std ::
{
  path ::PathBuf,
  fs as std_fs,
  env, // Import env to get current_dir
};

#[ test ]
fn basic_test() 
{
  // Arrange
  let temp = assert_fs ::TempDir ::new().unwrap();
  let crate_name = "dummy_crate";
  let project = ProjectBuilder ::new(crate_name)
  .toml_file("")
  .lib_file("/// A dummy function.\npub fn dummy() {}")
  .build(&temp)
  .unwrap();

  let crate_dir = CrateDir ::try_from(project.as_path()).expect("Failed to create CrateDir");
  let workspace = Workspace ::try_from(crate_dir.clone()).expect("Failed to load workspace");
  // Expected output is now in workspace target/doc
  let expected_output_path = workspace
  .target_directory()
  .join("doc")
  .join(format!("{crate_name}_doc.md"));

  // Act
  let result = action ::crate_doc ::doc(&workspace, &crate_dir, None);

  // Assert
  assert!(result.is_ok(), "Action failed: {:?}", result.err());
  let report = result.unwrap();

  assert!(
  report.status.contains("successfully"),
  "Report status is not successful: {}",
  report.status
 );
  assert_eq!(report.crate_dir.as_ref(), Some(&crate_dir));
  assert_eq!(report.output_path.as_ref(), Some(&expected_output_path));

  // Check file existence and content in the workspace target dir
  assert!(
  expected_output_path.is_file(),
  "Output file not found at expected location: {}",
  expected_output_path.display()
 );
  let content = std_fs ::read_to_string(&expected_output_path).expect("Failed to read output file");

  assert!(!content.is_empty(), "Output file is empty");
  assert!(content.contains("# Crate Documentation"), "Output file missing main header");
  assert!(
  content.contains("# Module `dummy_crate`"),
  "Output file missing module header"
 );
  assert!(content.contains("## Functions"), "Output file missing Functions section");
  assert!(
  content.contains("### Function `dummy`"),
  "Output file missing function header"
 );
  assert!(
  content.contains("A dummy function."),
  "Output file missing function doc comment"
 );
}

#[ test ]
fn output_option_test() 
{
  // Arrange
  let temp = assert_fs ::TempDir ::new().unwrap();
  let crate_name = "output_option_crate";
  let project = ProjectBuilder ::new(crate_name)
  .toml_file("")
  .lib_file("/// Another function.\npub fn another() {}")
  .build(&temp)
  .unwrap();

  let crate_dir = CrateDir ::try_from(project.as_path()).expect("Failed to create CrateDir");
  let workspace = Workspace ::try_from(crate_dir.clone()).expect("Failed to load workspace");
  // Define a custom output path relative to the CWD
  let custom_output_rel_path = PathBuf ::from("docs/custom_doc.md");
  // Expected path is resolved relative to CWD where the test runs
  let expected_output_abs_path = env ::current_dir().unwrap().join(&custom_output_rel_path);
  // Ensure the target directory exists for the test assertion later
  std_fs ::create_dir_all(expected_output_abs_path.parent().unwrap()).unwrap();

  // Act
  let result = action ::crate_doc ::doc(&workspace, &crate_dir, Some(custom_output_rel_path.clone()));

  // Assert
  assert!(result.is_ok(), "Action failed: {:?}", result.err());
  let report = result.unwrap();

  assert!(
  report.status.contains("successfully"),
  "Report status is not successful: {}",
  report.status
 );
  assert_eq!(report.crate_dir.as_ref(), Some(&crate_dir));
  // Check if the report contains the correct absolute output path resolved from CWD
  assert_eq!(report.output_path.as_ref(), Some(&expected_output_abs_path));

  // Check file existence at the custom path (relative to CWD) and content
  assert!(
  expected_output_abs_path.is_file(),
  "Output file not found at expected location: {}",
  expected_output_abs_path.display()
 );
  let content = std_fs ::read_to_string(&expected_output_abs_path).expect("Failed to read output file");
  assert!(!content.is_empty(), "Output file is empty");
  assert!(content.contains("# Crate Documentation"), "Output file missing main header");
  assert!(
  content.contains(&format!("# Module `{crate_name}`")),
  "Output file missing module header"
 );
  assert!(
  content.contains("### Function `another`"),
  "Output file missing function header"
 );
  assert!(
  content.contains("Another function."),
  "Output file missing function doc comment"
 );

  // Ensure the default file (in target/doc) was NOT created
  assert!(!workspace
  .target_directory()
  .join("doc")
  .join(format!("{crate_name}_doc.md"))
  .exists());

  // Clean up the created file/directory relative to CWD
  if expected_output_abs_path.exists() 
  {
  std_fs ::remove_file(&expected_output_abs_path).unwrap();
 }
  if expected_output_abs_path
  .parent()
  .unwrap()
  .read_dir()
  .unwrap()
  .next()
  .is_none()
  {
  std_fs ::remove_dir(expected_output_abs_path.parent().unwrap()).unwrap();
 }
}

#[ test ]
fn non_crate_dir_test() 
{
  // Arrange
  let temp = assert_fs ::TempDir ::new().unwrap();
  temp.child("not_a_dir").touch().unwrap();
  let empty_dir_path = temp.path().join("empty_dir");
  std_fs ::create_dir(&empty_dir_path).unwrap();

  // Attempt to create CrateDir from the empty directory path
  let crate_dir_result = CrateDir ::try_from(empty_dir_path.as_path());
  assert!(
  crate_dir_result.is_err(),
  "CrateDir ::try_from should fail for a directory without Cargo.toml"
 );
}

#[ test ]
fn cargo_doc_fail_test() 
{
  // Arrange
  let temp = assert_fs ::TempDir ::new().unwrap();
  let crate_name = "fail_crate";
  let project = ProjectBuilder ::new( crate_name )
  .toml_file( "" )
  .lib_file( "pub fn bad_code() -> { }" ) // Syntax error
  .build( &temp )
  .unwrap();

  let crate_dir = CrateDir ::try_from(project.as_path()).expect("Failed to create CrateDir");
  let workspace = Workspace ::try_from(crate_dir.clone()).expect("Failed to load workspace");

  // Act
  let result = action ::crate_doc ::doc(&workspace, &crate_dir, None);

  // Assert
  assert!(result.is_err(), "Action should fail when cargo doc fails");
  let (report, error) = result.err().unwrap();

  assert!(
  matches!(error, CrateDocError ::Command(_)),
  "Expected Command error, got {error:?}"
 );
  assert!(
  report
   .status
   .contains(&format!("Failed during `cargo doc` execution for `{crate_name}`.")),
  "Report status mismatch: {}",
  report.status
 );
  assert!(report.cargo_doc_report.is_some());
  assert!(
  report.cargo_doc_report.unwrap().error.is_err(),
  "Cargo doc report should indicate an error"
 );

  // Check that no output file was created (check default location)
  assert!(!workspace
  .target_directory()
  .join("doc")
  .join(format!("{crate_name}_doc.md"))
  .exists());
}
