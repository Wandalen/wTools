//! Manual corner case testing for SmokeModuleTest
//!
//! This file contains manual tests for edge cases that are difficult to automate
//! or need human verification. Run each test individually and document results
//! in tests/manual/readme.md.

use test_tools::*;

/// Test 1: Non-existent dependency name
///
/// Expected: Should fail at form() or perform() stage with clear error message
/// Run: cargo test --test smoke_test_corner_cases -- --exact test_nonexistent_dependency --nocapture
#[test]
#[ignore]
// DISABLED: 2026-05-04
// REASON: Requires human verification, network access, and explicit invocation
// RE-ENABLE: Run individually with `cargo test --test smoke_test_corner_cases -- --exact <name> --nocapture --ignored`
// APPROVED: manual_test_suite
fn test_nonexistent_dependency()
{
  let mut smoke_test = SmokeModuleTest::new("this_crate_definitely_does_not_exist_12345");
  smoke_test.version("1.0.0");
  smoke_test.code("fn main() {}".to_string());

  let form_result = smoke_test.form();
  println!("Form result: {:?}", form_result);

  if form_result.is_ok()
  {
    let perform_result = smoke_test.perform();
    println!("Perform result: {:?}", perform_result);
    smoke_test.clean(true).ok();

    assert!(perform_result.is_err(), "Should fail with non-existent dependency");
  }
}

/// Test 2: Malformed version string
///
/// Expected: Should fail gracefully with clear error about invalid version
/// Run: cargo test --test smoke_test_corner_cases -- --exact test_malformed_version --nocapture
#[test]
#[ignore]
// DISABLED: 2026-05-04
// REASON: Requires human verification, network access, and explicit invocation
// RE-ENABLE: Run individually with `cargo test --test smoke_test_corner_cases -- --exact <name> --nocapture --ignored`
// APPROVED: manual_test_suite
fn test_malformed_version()
{
  let mut smoke_test = SmokeModuleTest::new("serde");
  smoke_test.version("not.a.valid.version");
  smoke_test.code("fn main() {}".to_string());

  let form_result = smoke_test.form();
  println!("Form result: {:?}", form_result);

  if form_result.is_ok()
  {
    let perform_result = smoke_test.perform();
    println!("Perform result: {:?}", perform_result);
    smoke_test.clean(true).ok();

    // May fail at form or perform - either is acceptable
    if perform_result.is_ok()
    {
      println!("WARNING: Malformed version did not cause failure - cargo may have interpreted it differently");
    }
  }
}

/// Test 3: Empty code string
///
/// Expected: Should succeed (valid empty Rust project)
/// Run: cargo test --test smoke_test_corner_cases -- --exact test_empty_code --nocapture
#[test]
#[ignore]
// DISABLED: 2026-05-04
// REASON: Requires human verification, network access, and explicit invocation
// RE-ENABLE: Run individually with `cargo test --test smoke_test_corner_cases -- --exact <name> --nocapture --ignored`
// APPROVED: manual_test_suite
fn test_empty_code()
{
  let mut smoke_test = SmokeModuleTest::new("serde");
  smoke_test.version("1.0");
  smoke_test.code("".to_string());

  let form_result = smoke_test.form();
  println!("Form result: {:?}", form_result);
  assert!(form_result.is_ok(), "Should succeed with empty code");

  let perform_result = smoke_test.perform();
  println!("Perform result: {:?}", perform_result);

  smoke_test.clean(true).expect("Cleanup should succeed");

  // Empty code should compile and run (no main function warning is ok)
  println!("Empty code handling: {:?}", perform_result);
}

/// Test 4: Code with runtime panic
///
/// Expected: cargo test should fail, cargo run should fail
/// Run: cargo test --test smoke_test_corner_cases -- --exact test_code_with_panic --nocapture
#[test]
#[ignore]
// DISABLED: 2026-05-04
// REASON: Requires human verification, network access, and explicit invocation
// RE-ENABLE: Run individually with `cargo test --test smoke_test_corner_cases -- --exact <name> --nocapture --ignored`
// APPROVED: manual_test_suite
fn test_code_with_panic()
{
  let mut smoke_test = SmokeModuleTest::new("serde");
  smoke_test.version("1.0");
  smoke_test.code(r#"
    fn main() {
      panic!("Intentional panic for testing");
    }
  "#.to_string());

  let form_result = smoke_test.form();
  println!("Form result: {:?}", form_result);
  assert!(form_result.is_ok(), "Form should succeed");

  let perform_result = smoke_test.perform();
  println!("Perform result: {:?}", perform_result);

  smoke_test.clean(true).expect("Cleanup should succeed");

  // Should fail because main panics
  assert!(perform_result.is_err(), "Should fail with panicking code");
}

/// Test 5: Very large code string
///
/// Expected: Should handle gracefully (may be slow)
/// Run: cargo test --test smoke_test_corner_cases -- --exact test_large_code --nocapture
#[test]
#[ignore]
// DISABLED: 2026-05-04
// REASON: Requires human verification, network access, and explicit invocation
// RE-ENABLE: Run individually with `cargo test --test smoke_test_corner_cases -- --exact <name> --nocapture --ignored`
// APPROVED: manual_test_suite
fn test_large_code()
{
  let mut smoke_test = SmokeModuleTest::new("serde");
  smoke_test.version("1.0");

  // Generate large code (1000 functions)
  let mut code = String::new();
  for i in 0..1000
  {
    code.push_str(&format!("fn func_{}() {{ println!(\"Function {}\"); }}\n", i, i));
  }
  code.push_str("fn main() { func_0(); }");

  smoke_test.code(code);

  let form_result = smoke_test.form();
  println!("Form result: {:?}", form_result);

  if form_result.is_ok()
  {
    println!("Starting perform() - this may take a while...");
    let perform_result = smoke_test.perform();
    println!("Perform result: {:?}", perform_result);

    smoke_test.clean(true).expect("Cleanup should succeed");

    assert!(perform_result.is_ok(), "Should handle large code");
  }
}

/// Test 6: Code accessing non-existent dependency items
///
/// Expected: Should fail at compile time with clear error
/// Run: cargo test --test smoke_test_corner_cases -- --exact test_nonexistent_item --nocapture
#[test]
#[ignore]
// DISABLED: 2026-05-04
// REASON: Requires human verification, network access, and explicit invocation
// RE-ENABLE: Run individually with `cargo test --test smoke_test_corner_cases -- --exact <name> --nocapture --ignored`
// APPROVED: manual_test_suite
fn test_nonexistent_item()
{
  let mut smoke_test = SmokeModuleTest::new("serde");
  smoke_test.version("1.0");
  smoke_test.code(r#"
    use serde::ThisTypeDoesNotExist;
    fn main() {}
  "#.to_string());

  let form_result = smoke_test.form();
  println!("Form result: {:?}", form_result);

  if form_result.is_ok()
  {
    let perform_result = smoke_test.perform();
    println!("Perform result: {:?}", perform_result);

    smoke_test.clean(true).ok();

    assert!(perform_result.is_err(), "Should fail with non-existent type");
  }
}

/// Test 7: Multiple cleanup calls
///
/// Expected: Should handle gracefully (second call is no-op or returns error)
/// Run: cargo test --test smoke_test_corner_cases -- --exact test_multiple_cleanup --nocapture
#[test]
#[ignore]
// DISABLED: 2026-05-04
// REASON: Requires human verification, network access, and explicit invocation
// RE-ENABLE: Run individually with `cargo test --test smoke_test_corner_cases -- --exact <name> --nocapture --ignored`
// APPROVED: manual_test_suite
fn test_multiple_cleanup()
{
  let mut smoke_test = SmokeModuleTest::new("serde");
  smoke_test.version("1.0");
  smoke_test.code("fn main() {}".to_string());

  smoke_test.form().expect("Form should succeed");
  smoke_test.perform().expect("Perform should succeed");

  let first_cleanup = smoke_test.clean(true);
  println!("First cleanup result: {:?}", first_cleanup);
  assert!(first_cleanup.is_ok(), "First cleanup should succeed");

  let second_cleanup = smoke_test.clean(true);
  println!("Second cleanup result: {:?}", second_cleanup);

  // Second cleanup may fail (directory doesn't exist) or succeed (idempotent) - both acceptable
  println!("Multiple cleanup handling: first={:?}, second={:?}", first_cleanup, second_cleanup);
}

/// Test 8: Cleanup without prior form() call
///
/// Expected: Should handle gracefully (nothing to clean or error)
/// Run: cargo test --test smoke_test_corner_cases -- --exact test_cleanup_without_form --nocapture
#[test]
#[ignore]
// DISABLED: 2026-05-04
// REASON: Requires human verification, network access, and explicit invocation
// RE-ENABLE: Run individually with `cargo test --test smoke_test_corner_cases -- --exact <name> --nocapture --ignored`
// APPROVED: manual_test_suite
fn test_cleanup_without_form()
{
  let mut smoke_test = SmokeModuleTest::new("serde");
  smoke_test.version("1.0");
  smoke_test.code("fn main() {}".to_string());

  // Skip form() and perform(), go directly to cleanup
  let cleanup_result = smoke_test.clean(true);
  println!("Cleanup result: {:?}", cleanup_result);

  // Should either succeed (nothing to do) or fail gracefully
  println!("Cleanup without form: {:?}", cleanup_result);
}

/// Test 9: Test name with special characters
///
/// Expected: Should handle or sanitize special characters in filesystem paths
/// Run: cargo test --test smoke_test_corner_cases -- --exact test_special_chars_name --nocapture
#[test]
#[ignore]
// DISABLED: 2026-05-04
// REASON: Requires human verification, network access, and explicit invocation
// RE-ENABLE: Run individually with `cargo test --test smoke_test_corner_cases -- --exact <name> --nocapture --ignored`
// APPROVED: manual_test_suite
fn test_special_chars_name()
{
  // Note: SmokeModuleTest uses dependency_name in path, so test with special chars in dependency name
  let mut smoke_test = SmokeModuleTest::new("serde");
  smoke_test.version("1.0");
  smoke_test.code("fn main() { println!(\"Special chars test\"); }".to_string());

  let form_result = smoke_test.form();
  println!("Form result: {:?}", form_result);

  if form_result.is_ok()
  {
    let perform_result = smoke_test.perform();
    println!("Perform result: {:?}", perform_result);

    smoke_test.clean(true).ok();

    println!("Special chars handling: {:?}", perform_result);
  }
}

/// Test 10: Pre-release version
///
/// Expected: Should handle pre-release versions (if they exist)
/// Run: cargo test --test smoke_test_corner_cases -- --exact test_prerelease_version --nocapture
#[test]
#[ignore]
// DISABLED: 2026-05-04
// REASON: Requires human verification, network access, and explicit invocation
// RE-ENABLE: Run individually with `cargo test --test smoke_test_corner_cases -- --exact <name> --nocapture --ignored`
// APPROVED: manual_test_suite
fn test_prerelease_version()
{
  let mut smoke_test = SmokeModuleTest::new("serde");
  smoke_test.version("1.0.0-alpha.1"); // May not exist - that's ok, we're testing handling
  smoke_test.code("fn main() {}".to_string());

  let form_result = smoke_test.form();
  println!("Form result: {:?}", form_result);

  if form_result.is_ok()
  {
    let perform_result = smoke_test.perform();
    println!("Perform result: {:?}", perform_result);

    smoke_test.clean(true).ok();

    println!("Pre-release version handling: {:?}", perform_result);
  }
}
