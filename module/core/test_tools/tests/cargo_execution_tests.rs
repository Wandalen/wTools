//! Tests for `SmokeModuleTest` cargo command execution functionality (Task 020)
//!
//! These tests verify that `SmokeModuleTest` executes cargo test and cargo run commands
//! with proper success assertions according to FR-6 specification requirements.

use test_tools::*;

#[cfg(test)]
mod cargo_execution_tests 
{
  use super::*;

  /// Test that cargo test executes successfully in temporary project
  #[test]
  fn test_cargo_test_execution_success()
  {
    let mut smoke_test = SmokeModuleTest::new("serde");
    smoke_test.version("1.0");
    
    // Set up a simple test project with a well-known external crate
    smoke_test.code("use serde::*;".to_string());
    
    // Create the project structure
    smoke_test.form().expect("form() should succeed");
    
    // Execute perform() which runs cargo test and cargo run
    let result = smoke_test.perform();
    
    // Clean up regardless of test result
    smoke_test.clean(true).expect("cleanup should succeed");
    
    // Verify that perform() succeeded (both cargo test and cargo run passed)
    assert!(result.is_ok(), "perform() should succeed when project builds correctly");
  }

  /// Test that cargo run executes successfully in temporary project
  #[test]
  fn test_cargo_run_execution_success()
  {
    let mut smoke_test = SmokeModuleTest::new("serde");
    smoke_test.version("1.0");
    
    // Set up code that should run successfully
    smoke_test.code("println!(\"Cargo run test successful\");".to_string());
    
    smoke_test.form().expect("form() should succeed");
    
    let result = smoke_test.perform();
    
    smoke_test.clean(true).expect("cleanup should succeed");
    
    assert!(result.is_ok(), "perform() should succeed with valid code");
  }

  /// Test success assertion mechanisms work correctly
  #[test]
  fn test_success_assertion_mechanisms()
  {
    let mut smoke_test = SmokeModuleTest::new("serde");
    smoke_test.version("1.0");
    
    // Code that should compile and run successfully
    smoke_test.code("
      use serde::*;
      println!(\"Testing success assertion mechanisms\");
    ".to_string());
    
    smoke_test.form().expect("form() should succeed");
    
    let result = smoke_test.perform();
    
    smoke_test.clean(true).expect("cleanup should succeed");
    
    // Should succeed because code is valid
    assert!(result.is_ok(), "Success assertion should pass for valid code");
  }

  /// Test proper command output handling
  #[test]
  fn test_command_output_handling()
  {
    let mut smoke_test = SmokeModuleTest::new("serde");
    smoke_test.version("1.0");
    
    // Code that produces output
    smoke_test.code("
      println!(\"Standard output message\");
      eprintln!(\"Standard error message\");
    ".to_string());
    
    smoke_test.form().expect("form() should succeed");
    
    // Note: The current implementation prints output but doesn't return it
    // This test verifies that the perform() method handles output correctly
    let result = smoke_test.perform();
    
    smoke_test.clean(true).expect("cleanup should succeed");
    
    assert!(result.is_ok(), "Command output should be handled correctly");
  }

  /// Test error case handling for invalid code
  #[test]
  fn test_error_case_handling_invalid_code()
  {
    let mut smoke_test = SmokeModuleTest::new("serde");
    smoke_test.version("1.0");
    
    // Code that should fail to compile
    smoke_test.code("this_is_invalid_rust_code_that_should_not_compile;".to_string());
    
    smoke_test.form().expect("form() should succeed");
    
    let result = smoke_test.perform();
    
    smoke_test.clean(true).expect("cleanup should succeed");
    
    // Should fail because code is invalid
    assert!(result.is_err(), "Error case should be handled correctly for invalid code");
  }

  /// Test error case handling for missing dependencies
  #[test]
  fn test_error_case_handling_missing_dependency()
  {
    let mut smoke_test = SmokeModuleTest::new("nonexistent_crate_name_12345");
    smoke_test.version("99.99.99"); // Non-existent version
    
    // This should fail at the form() stage or perform() stage
    let form_result = smoke_test.form();
    
    if form_result.is_ok() {
      // If form succeeded, perform should fail
      let perform_result = smoke_test.perform();
      smoke_test.clean(true).expect("cleanup should succeed");
      assert!(perform_result.is_err(), "Should fail with missing dependency");
    } else {
      // Form failed as expected due to missing dependency
      // Note: current implementation might succeed at form() and fail at perform()
      assert!(form_result.is_err(), "Should handle missing dependency error");
    }
  }

  /// Test that both cargo test and cargo run are executed
  #[test]
  fn test_both_commands_executed()
  {
    let mut smoke_test = SmokeModuleTest::new("serde");
    smoke_test.version("1.0");
    
    // Create code that works for both cargo test and cargo run
    smoke_test.code("
      use serde::*;
      
      #[cfg(test)]
      mod tests {
        use super::*;
        
        #[test]
        fn dummy_test() {
          assert!(true);
        }
      }
      
      println!(\"Main function executed\");
    ".to_string());
    
    smoke_test.form().expect("form() should succeed");
    
    // perform() should run both cargo test and cargo run
    let result = smoke_test.perform();
    
    smoke_test.clean(true).expect("cleanup should succeed");
    
    assert!(result.is_ok(), "Both cargo test and cargo run should execute successfully");
  }

  /// Test working directory management during command execution
  #[test]
  fn test_working_directory_management()
  {
    let mut smoke_test = SmokeModuleTest::new("serde");
    smoke_test.version("1.0");
    
    // Store current directory to verify it doesn't change
    let original_dir = std::env::current_dir().unwrap();
    
    smoke_test.code("println!(\"Testing working directory management\");".to_string());
    
    smoke_test.form().expect("form() should succeed");
    
    let result = smoke_test.perform();
    
    // Verify current directory hasn't changed
    let current_dir = std::env::current_dir().unwrap();
    assert_eq!(original_dir, current_dir, "Working directory should not change");
    
    smoke_test.clean(true).expect("cleanup should succeed");
    
    assert!(result.is_ok(), "Working directory should be managed correctly");
  }
}