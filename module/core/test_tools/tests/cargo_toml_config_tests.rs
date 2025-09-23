//! Tests for Cargo.toml configuration functionality (Task 017)
//!
//! These tests verify that `SmokeModuleTest` can configure temporary project dependencies
//! for both local path-based and published version-based dependencies (FR-5).
//!
//! ## TDD Approach
//! These tests are written FIRST and will initially FAIL, demonstrating
//! the need for implementing Cargo.toml configuration in Task 018.

#[ cfg(test) ]
mod cargo_toml_config_tests 
{
  use test_tools ::SmokeModuleTest;
  use std ::path ::PathBuf;

  /// Test that `SmokeModuleTest` can configure local path dependencies in Cargo.toml
  /// This test verifies FR-5 requirement for local, path-based crate versions
  #[ test ]
  fn test_local_path_dependency_configuration()
  {
  let mut smoke_test = SmokeModuleTest ::new("local_dep_test");
  
  // Configure a local path dependency
  let local_path = PathBuf ::from("/path/to/local/crate");
  
  // This should configure the dependency to use local path
  // Currently expected to fail - implementation needed in Task 018
  let result = smoke_test.dependency_local_path("my_crate", &local_path);
  assert!(result.is_ok(), "Should be able to configure local path dependency");
  
  // Form the project and verify Cargo.toml contains local path dependency
  smoke_test.form().expect("Should be able to form project");
  
  // Read the generated Cargo.toml and verify local path configuration
  let cargo_toml_path = smoke_test.project_path().join("Cargo.toml");
  let cargo_toml_content = std ::fs ::read_to_string(&cargo_toml_path)
   .expect("Should be able to read generated Cargo.toml");
  
  // Verify local path dependency is correctly configured
  assert!(cargo_toml_content.contains("my_crate = { path = \"/path/to/local/crate\" }"),
   "Cargo.toml should contain local path dependency configuration");
  
  // Cleanup
  smoke_test.clean(true).expect("Cleanup should succeed");
 }

  /// Test that `SmokeModuleTest` can configure published version dependencies in Cargo.toml
  /// This test verifies FR-5 requirement for published, version-based crate versions
  #[ test ]
  fn test_published_version_dependency_configuration()
  {
  let mut smoke_test = SmokeModuleTest ::new("version_dep_test");
  
  // Configure a published version dependency
  // This should configure the dependency to use published version
  // Currently expected to fail - implementation needed in Task 018
  let result = smoke_test.dependency_version("serde", "1.0");
  assert!(result.is_ok(), "Should be able to configure version dependency");
  
  // Form the project and verify Cargo.toml contains version dependency
  smoke_test.form().expect("Should be able to form project");
  
  // Read the generated Cargo.toml and verify version configuration
  let cargo_toml_path = smoke_test.project_path().join("Cargo.toml");
  let cargo_toml_content = std ::fs ::read_to_string(&cargo_toml_path)
   .expect("Should be able to read generated Cargo.toml");
  
  // Verify version dependency is correctly configured
  assert!(cargo_toml_content.contains("serde = { version = \"1.0\" }"),
   "Cargo.toml should contain version dependency configuration");
  
  // Cleanup
  smoke_test.clean(true).expect("Cleanup should succeed");
 }

  /// Test that `SmokeModuleTest` generates complete and valid Cargo.toml files
  /// This verifies the overall file generation process for FR-5
  #[ test ]
  fn test_cargo_toml_generation()
  {
  let mut smoke_test = SmokeModuleTest ::new("toml_gen_test");
  
  // Configure multiple dependencies
  // Currently expected to fail - implementation needed in Task 018
  smoke_test.dependency_version("serde", "1.0").expect("Should configure serde");
  
  let local_path = PathBuf ::from("/local/path/test_crate");
  smoke_test.dependency_local_path("test_crate", &local_path)
   .expect("Should configure local path dependency");
  
  // Form the project
  smoke_test.form().expect("Should be able to form project");
  
  // Verify Cargo.toml exists and is valid
  let cargo_toml_path = smoke_test.project_path().join("Cargo.toml");
  assert!(cargo_toml_path.exists(), "Cargo.toml should be generated");
  
  let cargo_toml_content = std ::fs ::read_to_string(&cargo_toml_path)
   .expect("Should be able to read Cargo.toml");
  
  // Verify essential Cargo.toml structure
  assert!(cargo_toml_content.contains("[package]"), "Should contain [package] section");
  assert!(cargo_toml_content.contains("[dependencies]"), "Should contain [dependencies] section");
  assert!(cargo_toml_content.contains("name = \"toml_gen_test_smoke_test\""), "Should contain correct package name");
  
  // Verify both dependency types are present
  assert!(cargo_toml_content.contains("serde = { version = \"1.0\" }"), "Should contain version dependency");
  assert!(cargo_toml_content.contains("test_crate = { path = \"/local/path/test_crate\" }"), 
   "Should contain local path dependency");
  
  // Cleanup
  smoke_test.clean(true).expect("Cleanup should succeed");
 }

  /// Test cross-platform path handling for local dependencies
  /// This ensures proper path escaping and formatting across operating systems
  #[ test ]
  fn test_cross_platform_path_handling()
  {
  let mut smoke_test = SmokeModuleTest ::new("cross_platform_test");
  
  // Test with paths that need proper escaping on different platforms
  #[ cfg(windows) ]
  let test_path = PathBuf ::from("C: \\Users\\test\\my_crate");
  
  #[ cfg(not(windows)) ]
  let test_path = PathBuf ::from("/home/test/my_crate");
  
  // Configure local path dependency with platform-specific path
  // Currently expected to fail - implementation needed in Task 018
  let result = smoke_test.dependency_local_path("platform_crate", &test_path);
  assert!(result.is_ok(), "Should handle platform-specific paths");
  
  // Form the project
  smoke_test.form().expect("Should be able to form project");
  
  // Verify path is properly escaped in Cargo.toml
  let cargo_toml_path = smoke_test.project_path().join("Cargo.toml");
  let cargo_toml_content = std ::fs ::read_to_string(&cargo_toml_path)
   .expect("Should be able to read Cargo.toml");
  
  // Verify the path appears correctly in the TOML (with proper escaping)
  let expected_path_str = test_path.to_string_lossy();
  assert!(cargo_toml_content.contains(&format!("platform_crate = {{ path = \"{expected_path_str}\" }}")),
   "Should contain properly escaped path dependency");
  
  // Cleanup
  smoke_test.clean(true).expect("Cleanup should succeed");
 }

  /// Test version string handling and validation
  /// This ensures version strings are properly formatted and validated
  #[ test ]
  fn test_version_string_handling()
  {
  let mut smoke_test = SmokeModuleTest ::new("version_test");
  
  // Test various version string formats
  // Currently expected to fail - implementation needed in Task 018
  
  // Simple version
  smoke_test.dependency_version("simple", "1.0").expect("Should handle simple version");
  
  // Semver with patch
  smoke_test.dependency_version("patch", "1.2.3").expect("Should handle patch version");
  
  // Range version
  smoke_test.dependency_version("range", "^1.0").expect("Should handle range version");
  
  // Form the project
  smoke_test.form().expect("Should be able to form project");
  
  // Verify all version formats are correctly written
  let cargo_toml_path = smoke_test.project_path().join("Cargo.toml");
  let cargo_toml_content = std ::fs ::read_to_string(&cargo_toml_path)
   .expect("Should be able to read Cargo.toml");
  
  assert!(cargo_toml_content.contains("simple = { version = \"1.0\" }"), "Should contain simple version");
  assert!(cargo_toml_content.contains("patch = { version = \"1.2.3\" }"), "Should contain patch version");
  assert!(cargo_toml_content.contains("range = { version = \"^1.0\" }"), "Should contain range version");
  
  // Cleanup
  smoke_test.clean(true).expect("Cleanup should succeed");
 }

  /// Test dependency configuration with features
  /// This verifies advanced dependency configuration capabilities
  #[ test ]
  fn test_dependency_features_configuration()
  {
  let mut smoke_test = SmokeModuleTest ::new("features_test");
  
  // Configure dependency with features
  // Currently expected to fail - implementation needed in Task 018
  let result = smoke_test.dependency_with_features("tokio", "1.0", &[ "full", "macros"]);
  assert!(result.is_ok(), "Should be able to configure dependency with features");
  
  // Form the project
  smoke_test.form().expect("Should be able to form project");
  
  // Verify features are correctly configured in Cargo.toml
  let cargo_toml_path = smoke_test.project_path().join("Cargo.toml");
  let cargo_toml_content = std ::fs ::read_to_string(&cargo_toml_path)
   .expect("Should be able to read Cargo.toml");
  
  // Verify dependency with features is correctly formatted
  assert!(cargo_toml_content.contains("tokio = { version = \"1.0\", features = [\"full\", \"macros\"] }"),
   "Should contain dependency with features configuration");
  
  // Cleanup
  smoke_test.clean(true).expect("Cleanup should succeed");
 }

  /// Test optional dependencies configuration
  /// This verifies optional dependency handling for conditional compilation
  #[ test ]
  fn test_optional_dependencies_configuration()
  {
  let mut smoke_test = SmokeModuleTest ::new("optional_test");
  
  // Configure optional dependency
  // Currently expected to fail - implementation needed in Task 018
  let result = smoke_test.dependency_optional("optional_crate", "1.0");
  assert!(result.is_ok(), "Should be able to configure optional dependency");
  
  // Form the project
  smoke_test.form().expect("Should be able to form project");
  
  // Verify optional dependency is correctly configured
  let cargo_toml_path = smoke_test.project_path().join("Cargo.toml");
  let cargo_toml_content = std ::fs ::read_to_string(&cargo_toml_path)
   .expect("Should be able to read Cargo.toml");
  
  assert!(cargo_toml_content.contains("optional_crate = { version = \"1.0\", optional = true }"),
   "Should contain optional dependency configuration");
  
  // Cleanup
  smoke_test.clean(true).expect("Cleanup should succeed");
 }

  /// Test development dependencies configuration
  /// This verifies dev-dependency section handling
  #[ test ]
  fn test_dev_dependencies_configuration()
  {
  let mut smoke_test = SmokeModuleTest ::new("dev_deps_test");
  
  // Configure development dependency
  // Currently expected to fail - implementation needed in Task 018
  let result = smoke_test.dev_dependency("criterion", "0.3");
  assert!(result.is_ok(), "Should be able to configure dev dependency");
  
  // Form the project
  smoke_test.form().expect("Should be able to form project");
  
  // Verify dev dependency is in correct section
  let cargo_toml_path = smoke_test.project_path().join("Cargo.toml");
  let cargo_toml_content = std ::fs ::read_to_string(&cargo_toml_path)
   .expect("Should be able to read Cargo.toml");
  
  assert!(cargo_toml_content.contains("[dev-dependencies]"), "Should contain [dev-dependencies] section");
  assert!(cargo_toml_content.contains("criterion = { version = \"0.3\" }"), "Should contain dev dependency");
  
  // Cleanup
  smoke_test.clean(true).expect("Cleanup should succeed");
 }

}