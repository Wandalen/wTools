//! Tests for `SmokeModuleTest` creation functionality (Task 014)
//!
//! These tests verify that `SmokeModuleTest` can create temporary, isolated Cargo projects
//! in the filesystem according to FR-4 specification requirements.

use test_tools :: *;

#[ cfg(test) ]
mod smoke_module_test_creation_tests 
{
  use super :: *;

  /// Test that `SmokeModuleTest` creates a temporary directory structure
  #[ test ]
  fn test_creates_temporary_directory_structure()
  {
  let mut smoke_test = SmokeModuleTest ::new("test_crate");
  
  // Before form() is called, the directory should not exist
  assert!(!smoke_test.test_path.exists(), "Temporary directory should not exist before form()");
  
  // Call form() to create the project structure
  smoke_test.form().expect("form() should succeed");
  
  // After form(), the directory structure should exist
  assert!(smoke_test.test_path.exists(), "Temporary directory should exist after form()");
  
  // Verify the basic project structure
  let test_name = format!("{}{}", smoke_test.dependency_name, smoke_test.test_postfix);
  let project_path = smoke_test.test_path.join(&test_name);
  assert!(project_path.exists(), "Project directory should exist");
  assert!(project_path.join("Cargo.toml").exists(), "Cargo.toml should exist");
  assert!(project_path.join("src").exists(), "src directory should exist");
  assert!(project_path.join("src/main.rs").exists(), "main.rs should exist");
  
  // Clean up
  smoke_test.clean(true).expect("cleanup should succeed");
 }

  /// Test that temporary projects are isolated from the main project
  #[ test ]
  fn test_isolation_from_main_project()
  {
  let smoke_test = SmokeModuleTest ::new("isolated_test");
  
  // The temporary path should be in the system temp directory, not the current project
  let temp_dir = std ::env ::temp_dir();
  assert!(smoke_test.test_path.starts_with(&temp_dir), 
  "Test path should be in system temp directory for isolation");
  
  // The path should contain a random component for uniqueness
  let path_str = smoke_test.test_path.to_string_lossy();
  assert!(path_str.contains("isolated_test"), "Path should contain dependency name");
  assert!(path_str.contains("_smoke_test_"), "Path should contain test postfix");
  
  // Verify path doesn't conflict with current working directory
  let current_dir = std ::env ::current_dir().unwrap();
  assert!(!smoke_test.test_path.starts_with(&current_dir), 
  "Test path should not be within current working directory");
  
  // Test multiple instances create different paths (isolation between tests)
  let smoke_test2 = SmokeModuleTest ::new("isolated_test");
  assert_ne!(smoke_test.test_path, smoke_test2.test_path, 
   "Multiple test instances should have different paths");
 }

  /// Test that Cargo project is properly initialized
  #[ test ]
  fn test_proper_cargo_project_initialization()
  {
  let mut smoke_test = SmokeModuleTest ::new("cargo_init_test");
  smoke_test.form().expect("form() should succeed");
  
  let test_name = format!("{}{}", smoke_test.dependency_name, smoke_test.test_postfix);
  let project_path = smoke_test.test_path.join(&test_name);
  
  // Read and verify Cargo.toml content
  let cargo_toml_path = project_path.join("Cargo.toml");
  let cargo_content = std ::fs ::read_to_string(&cargo_toml_path)
   .expect("Should be able to read Cargo.toml");
  
  // Verify package section
  assert!(cargo_content.contains("[package]"), "Should have [package] section");
  assert!(cargo_content.contains("edition = \"2021\""), "Should use 2021 edition");
  assert!(cargo_content.contains(&format!("name = \"{}_smoke_test\"", smoke_test.dependency_name)), 
  "Should have correct package name");
  assert!(cargo_content.contains("version = \"0.0.1\""), "Should have version");
  
  // Verify dependencies section
  assert!(cargo_content.contains("[dependencies]"), "Should have [dependencies] section");
  assert!(cargo_content.contains(&format!("{} = {{", smoke_test.dependency_name)), 
  "Should have dependency on test crate");
  
  // Read and verify main.rs content
  let main_rs_path = project_path.join("src/main.rs");
  let main_content = std ::fs ::read_to_string(&main_rs_path)
   .expect("Should be able to read main.rs");
  
  assert!(main_content.contains("fn main()"), "Should have main function");
  assert!(main_content.contains("#[ allow( unused_imports ) ]"), "Should allow unused imports");
  
  // Clean up
  smoke_test.clean(true).unwrap();
 }

  /// Test filesystem permissions and access
  #[ test ]
  fn test_filesystem_permissions_and_access()
  {
  let mut smoke_test = SmokeModuleTest ::new("permissions_test");
  
  // Should be able to create directory
  smoke_test.form().expect("Should have permission to create directories");
  
  let test_name = format!("{}{}", smoke_test.dependency_name, smoke_test.test_postfix);
  let project_path = smoke_test.test_path.join(&test_name);
  
  // Should be able to read created files
  let cargo_toml = project_path.join("Cargo.toml");
  assert!(cargo_toml.exists() && cargo_toml.is_file(), "Cargo.toml should be readable file");
  
  let main_rs = project_path.join("src/main.rs");
  assert!(main_rs.exists() && main_rs.is_file(), "main.rs should be readable file");
  
  // Should be able to write to the directory (test by creating a test file)
  let test_file = project_path.join("test_write.txt");
  std ::fs ::write(&test_file, "test content").expect("Should be able to write to project directory");
  assert!(test_file.exists(), "Test file should be created");
  
  // Should be able to clean up (delete)
  smoke_test.clean(false).expect("Should be able to clean up directories");
  assert!(!smoke_test.test_path.exists(), "Directory should be removed after cleanup");
 }

  /// Test custom configuration options
  #[ test ]
  fn test_custom_configuration_options()
  {
  let mut smoke_test = SmokeModuleTest ::new("config_test");
  
  // Test version configuration
  smoke_test.version("1.2.3");
  assert_eq!(smoke_test.version, "1.2.3", "Should set version correctly");
  
  // Test local path configuration
  let test_path = "/path/to/local/crate";
  smoke_test.local_path_clause(test_path);
  assert_eq!(smoke_test.local_path_clause, test_path, "Should set local path correctly");
  
  // Test custom code configuration
  let custom_code = "println!(\"Custom test code\");".to_string();
  smoke_test.code(custom_code.clone());
  assert_eq!(smoke_test.code, custom_code, "Should set custom code correctly");
  
  // Test custom postfix
  let custom_postfix = "_custom_test";
  let original_path = smoke_test.test_path.clone();
  smoke_test.test_postfix(custom_postfix);
  assert_eq!(smoke_test.test_postfix, custom_postfix, "Should set custom postfix");
  assert_ne!(smoke_test.test_path, original_path, "Path should change when postfix changes");
  
  let path_str = smoke_test.test_path.to_string_lossy();
  assert!(path_str.contains(custom_postfix), "New path should contain custom postfix");
 }

  /// Test error handling for invalid scenarios
  #[ test ]
  #[ should_panic(expected = "File exists") ]
  fn test_error_handling_for_repeated_form_calls()
  {
  // Test that form() fails when called multiple times (this is the current behavior)
  // This test documents the current limitation - form() should ideally return an error
  // instead of panicking when called on an already-formed test
  let mut smoke_test = SmokeModuleTest ::new("error_test");
  smoke_test.form().expect("First form() should succeed");
  
  // Second call currently panics due to unwrap() - this is the documented behavior
  smoke_test.form().expect("Second form() call should fail gracefully in future versions");
 }

  /// Test clean functionality
  #[ test ]  
  fn test_clean_functionality()
  {
  // Test normal cleanup
  let mut smoke_test = SmokeModuleTest ::new("clean_test");
  smoke_test.form().expect("form() should succeed");
  assert!(smoke_test.test_path.exists(), "Directory should exist after form()");
  
  smoke_test.clean(false).expect("clean() should succeed");
  assert!(!smoke_test.test_path.exists(), "Directory should not exist after clean()");
  
  // Test clean() with force=true on non-existent directory
  let smoke_test2 = SmokeModuleTest ::new("clean_test2");
  let clean_result = smoke_test2.clean(true);
  assert!(clean_result.is_ok(), "clean(true) should succeed even on non-existent directory");
 }

  /// Test that random path generation works correctly
  #[ test ]
  fn test_random_path_generation()
  {
  let smoke_test1 = SmokeModuleTest ::new("random_test");
  let smoke_test2 = SmokeModuleTest ::new("random_test");
  let smoke_test3 = SmokeModuleTest ::new("random_test");
  
  // All paths should be different due to random component
  assert_ne!(smoke_test1.test_path, smoke_test2.test_path, "Paths should be unique");
  assert_ne!(smoke_test2.test_path, smoke_test3.test_path, "Paths should be unique");
  assert_ne!(smoke_test1.test_path, smoke_test3.test_path, "Paths should be unique");
  
  // All paths should contain the same base name but different random suffixes
  let path1_str = smoke_test1.test_path.to_string_lossy();
  let path2_str = smoke_test2.test_path.to_string_lossy();
  let path3_str = smoke_test3.test_path.to_string_lossy();
  
  assert!(path1_str.contains("random_test_smoke_test_"), "Should contain base name");
  assert!(path2_str.contains("random_test_smoke_test_"), "Should contain base name");
  assert!(path3_str.contains("random_test_smoke_test_"), "Should contain base name");
 }
}