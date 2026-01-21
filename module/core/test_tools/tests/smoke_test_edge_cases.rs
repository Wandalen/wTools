//! Edge case tests for SmokeModuleTest
//!
//! These tests cover corner cases and boundary conditions that go beyond
//! the standard functional requirements testing.

use test_tools::*;

#[cfg(test)]
mod smoke_test_edge_cases
{
  use super::*;

  /// Test empty code string - should handle gracefully
  #[test]
  fn test_empty_code_string()
  {
    let mut smoke_test = SmokeModuleTest::new("serde");
    smoke_test.version("1.0");
    smoke_test.code("".to_string());

    let form_result = smoke_test.form();
    assert!(form_result.is_ok(), "Empty code should succeed at form() stage");

    // perform() may fail (no main function) or succeed - both are acceptable
    let _perform_result = smoke_test.perform();

    smoke_test.clean(true).expect("cleanup should succeed");
  }

  /// Test code with only whitespace
  #[test]
  fn test_whitespace_only_code()
  {
    let mut smoke_test = SmokeModuleTest::new("serde");
    smoke_test.version("1.0");
    smoke_test.code("   \n\n\t\t  \n".to_string());

    let form_result = smoke_test.form();
    assert!(form_result.is_ok(), "Whitespace-only code should succeed at form() stage");

    let _perform_result = smoke_test.perform();

    smoke_test.clean(true).expect("cleanup should succeed");
  }

  /// Test code with runtime panic - should fail at perform() stage
  #[test]
  fn test_code_with_runtime_panic()
  {
    let mut smoke_test = SmokeModuleTest::new("serde");
    smoke_test.version("1.0");
    smoke_test.code(r#"
      fn main() {
        panic!("Intentional panic for testing");
      }
    "#.to_string());

    smoke_test.form().expect("form() should succeed");

    let perform_result = smoke_test.perform();

    smoke_test.clean(true).expect("cleanup should succeed");

    assert!(perform_result.is_err(), "Code with panic should fail at perform() stage");
  }

  /// Test accessing non-existent items from dependency
  #[test]
  fn test_nonexistent_dependency_item()
  {
    let mut smoke_test = SmokeModuleTest::new("serde");
    smoke_test.version("1.0");
    smoke_test.code(r#"
      use serde::ThisTypeDoesNotExist;
      fn main() {}
    "#.to_string());

    smoke_test.form().expect("form() should succeed");

    let perform_result = smoke_test.perform();

    smoke_test.clean(true).expect("cleanup should succeed");

    assert!(perform_result.is_err(), "Non-existent type access should fail at perform() stage");
  }

  /// Test very old version of dependency
  #[test]
  fn test_very_old_version()
  {
    let mut smoke_test = SmokeModuleTest::new("serde");
    smoke_test.version("0.1.0");
    smoke_test.code("fn main() {}".to_string());

    let form_result = smoke_test.form();

    if form_result.is_ok()
    {
      let _perform_result = smoke_test.perform();
      smoke_test.clean(true).ok();
    }

    // Old versions may not exist - either outcome is acceptable
  }

  /// Test code with compile-time type error
  #[test]
  fn test_type_error()
  {
    let mut smoke_test = SmokeModuleTest::new("serde");
    smoke_test.version("1.0");
    smoke_test.code(r#"
      fn main() {
        let x: u32 = "not a number";
      }
    "#.to_string());

    smoke_test.form().expect("form() should succeed");

    let perform_result = smoke_test.perform();

    smoke_test.clean(true).expect("cleanup should succeed");

    assert!(perform_result.is_err(), "Type error should fail at perform() stage");
  }

  /// Test multiple cleanup calls - should be idempotent or return error
  #[test]
  fn test_multiple_cleanup_calls()
  {
    let mut smoke_test = SmokeModuleTest::new("serde");
    smoke_test.version("1.0");
    smoke_test.code("fn main() {}".to_string());

    smoke_test.form().expect("form() should succeed");
    smoke_test.perform().expect("perform() should succeed");

    let first_cleanup = smoke_test.clean(true);
    assert!(first_cleanup.is_ok(), "First cleanup should succeed");

    let second_cleanup = smoke_test.clean(true);

    // Second cleanup may fail (directory doesn't exist) or succeed (idempotent)
    // Both behaviors are acceptable
    match second_cleanup
    {
      Ok(_) => {
        // Idempotent cleanup - acceptable
      }
      Err(_) => {
        // Fails because already cleaned - also acceptable
      }
    }
  }

  /// Test cleanup without prior form() call
  #[test]
  fn test_cleanup_without_form()
  {
    let mut smoke_test = SmokeModuleTest::new("serde");
    smoke_test.version("1.0");
    smoke_test.code("fn main() {}".to_string());

    // Skip form() and perform(), go directly to cleanup
    let cleanup_result = smoke_test.clean(true);

    // Should either succeed (nothing to do) or fail gracefully
    // Both behaviors are acceptable for this edge case
    match cleanup_result
    {
      Ok(_) => {
        // Handles missing directory gracefully - acceptable
      }
      Err(_) => {
        // Fails because form() never called - also acceptable
      }
    }
  }

  /// Test code with unsafe block - should compile if syntactically correct
  #[test]
  fn test_code_with_unsafe()
  {
    let mut smoke_test = SmokeModuleTest::new("serde");
    smoke_test.version("1.0");
    smoke_test.code(r#"
      fn main() {
        unsafe {
          let x: i32 = 42;
          let ptr = &x as *const i32;
          let value = *ptr;
          println!("Value: {}", value);
        }
      }
    "#.to_string());

    smoke_test.form().expect("form() should succeed");

    let perform_result = smoke_test.perform();

    smoke_test.clean(true).expect("cleanup should succeed");

    assert!(perform_result.is_ok(), "Valid unsafe code should compile and run");
  }

  /// Test code with macro invocation
  #[test]
  fn test_code_with_macros()
  {
    let mut smoke_test = SmokeModuleTest::new("serde");
    smoke_test.version("1.0");
    smoke_test.code(r#"
      macro_rules! my_macro {
        ($x:expr) => { println!("Value: {}", $x); };
      }

      fn main() {
        my_macro!(42);
        my_macro!("hello");
      }
    "#.to_string());

    smoke_test.form().expect("form() should succeed");

    let perform_result = smoke_test.perform();

    smoke_test.clean(true).expect("cleanup should succeed");

    assert!(perform_result.is_ok(), "Code with macros should compile and run");
  }
}
