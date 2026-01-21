//! Reproducing test for file system directory creation bugs in examples
//!
//! ## Root Cause
//!
//! Six example files attempted to write output files to `target/` directory without
//! verifying or creating the directory first. The examples assumed `target/` would exist
//! (which it does at workspace level), but when run from the module directory
//! (`/home/user1/pro/lib/wTools/module/core/benchkit`), the local `target/` directory
//! doesn't exist, causing file write operations to fail with:
//! ```text
//! Error: Os { code: 2, kind: NotFound, message: "No such file or directory" }
//! ```
//!
//! Affected examples:
//! - `plotting_example.rs` (lines 34, 43, 60)
//! - `statistical_analysis_example.rs` (line 103)
//! - `strs_tools_comprehensive_test.rs` (line 408)
//! - `strs_tools_manual_test.rs` (line 286)
//! - `strs_tools_transformation.rs` (line 297)
//! - `unilang_parser_benchkit_integration.rs` (line 462)
//!
//! ## Why Not Caught
//!
//! 1. **Missing Example Tests**: No automated tests existed to verify examples execute successfully
//! 2. **Environment Assumptions**: Examples assumed workspace-level `target/` directory availability
//! 3. **No File System Edge Case Testing**: Corner case checklist didn't include directory creation scenarios
//! 4. **Silent Success in Workspace Context**: Examples worked when run from workspace root, hiding the bug
//!
//! ## Fix Applied
//!
//! Added `std::fs::create_dir_all("target")?;` before file write operations in all affected examples.
//! This ensures the output directory exists before attempting to write files.
//!
//! Example fix pattern:
//! ```rust
//! // Create output directory if it doesn't exist
//! std::fs::create_dir_all("target")?;
//!
//! // Now safe to write files
//! let report_path = "target/report.md";
//! std::fs::write(report_path, &content)?;
//! ```
//!
//! ## Prevention
//!
//! 1. **Example Execution Tests**: Add CI tests that execute all examples to catch runtime failures
//! 2. **File System Utility**: Create helper function for safe file writes that ensures parent directories exist
//! 3. **Documentation**: Add to corner case checklist: "Non-existent parent directory for file operations"
//! 4. **Linting Rule**: Consider adding clippy lint for file operations without directory validation
//!
//! ## Pitfall
//!
//! **Never assume directories exist for file operations**. Always use `std::fs::create_dir_all()`
//! before writing files to ensure parent directories exist. This is especially critical for:
//! - Output files in examples
//! - Temporary files in tests
//! - Report generation in integration workflows
//! - Any file path that includes subdirectories
//!
//! The `create_dir_all()` function is idempotent (safe to call multiple times) and creates
//! all missing parent directories, making it the safest choice for ensuring file write success.

use std ::fs;
use std ::path ::Path;

/// Fix(issue-001): Missing directory creation before file operations
/// Root cause: Examples assumed target/ directory exists without verification
/// Pitfall: Always use `create_dir_all` before file writes; never assume directories exist

#[ test ]
fn test_file_write_without_directory_creation_fails()
{
  // Cleanup: ensure clean test environment
  let test_dir = "test_output_missing_dir_001";
  let _cleanup = TestDirectoryCleanup ::new(test_dir);

  // Attempt to write file to non-existent directory (reproduces bug)
  let file_path = format!("{test_dir}/report.txt");
  let result = fs ::write(&file_path, b"test content");

  // Verify bug reproduces: file write fails with NotFound error
  assert!(result.is_err(), "Expected file write to fail when directory doesn't exist");
  if let Err(e) = result
  {
  assert_eq!(e.kind(), std ::io ::ErrorKind ::NotFound,
   "Expected NotFound error, got: {:?}", e.kind());
 }
}

#[ test ]
fn test_file_write_with_directory_creation_succeeds()
{
  // Cleanup: ensure clean test environment
  let test_dir = "test_output_with_dir_002";
  let _cleanup = TestDirectoryCleanup ::new(test_dir);

  // Apply fix: create directory before file write
  let result = fs ::create_dir_all(test_dir);
  assert!(result.is_ok(), "Directory creation should succeed: {:?}", result.err());

  // Now file write should succeed
  let file_path = format!("{test_dir}/report.txt");
  let result = fs ::write(&file_path, b"test content");
  assert!(result.is_ok(), "File write should succeed after directory creation: {:?}", result.err());

  // Verify file was created with correct content
  let content = fs ::read_to_string(&file_path).expect("Should read file");
  assert_eq!(content, "test content", "File content should match");
}

#[ test ]
fn test_create_dir_all_is_idempotent()
{
  // Cleanup: ensure clean test environment
  let test_dir = "test_output_idempotent_003";
  let _cleanup = TestDirectoryCleanup ::new(test_dir);

  // Create directory multiple times (demonstrates idempotency)
  fs ::create_dir_all(test_dir).expect("First creation should succeed");
  fs ::create_dir_all(test_dir).expect("Second creation should also succeed (idempotent)");
  fs ::create_dir_all(test_dir).expect("Third creation should also succeed (idempotent)");

  // Verify directory exists
  assert!(Path ::new(test_dir).exists(), "Directory should exist");
  assert!(Path ::new(test_dir).is_dir(), "Path should be a directory");
}

#[ test ]
fn test_nested_directory_creation()
{
  // Cleanup: ensure clean test environment
  let test_dir = "test_output_nested_004/level1/level2/level3";
  let _cleanup = TestDirectoryCleanup ::new("test_output_nested_004");

  // Create nested directories in one call
  let result = fs ::create_dir_all(test_dir);
  assert!(result.is_ok(), "Nested directory creation should succeed: {:?}", result.err());

  // Verify all levels exist
  assert!(Path ::new("test_output_nested_004").exists());
  assert!(Path ::new("test_output_nested_004/level1").exists());
  assert!(Path ::new("test_output_nested_004/level1/level2").exists());
  assert!(Path ::new(test_dir).exists());

  // Write file in nested directory
  let file_path = format!("{test_dir}/report.txt");
  let result = fs ::write(&file_path, b"nested content");
  assert!(result.is_ok(), "File write in nested directory should succeed: {:?}", result.err());
}

#[ test ]
fn test_safe_file_write_helper_pattern()
{
  // Cleanup: ensure clean test environment
  let test_dir = "test_output_helper_005";
  let _cleanup = TestDirectoryCleanup ::new(test_dir);

  // Demonstrate recommended safe file write pattern
  fn safe_write_file(path: &str, content: &[u8]) -> std ::io ::Result< () >
  {
  // Extract parent directory from path
  if let Some(parent) = Path ::new(path).parent()
  {
   // Ensure parent directory exists
   fs ::create_dir_all(parent)?;
 }
  // Now safe to write file
  fs ::write(path, content)
 }

  // Use safe helper
  let file_path = format!("{test_dir}/subdir/report.txt");
  let result = safe_write_file(&file_path, b"safe content");
  assert!(result.is_ok(), "Safe file write should succeed: {:?}", result.err());

  // Verify file was created
  let content = fs ::read_to_string(&file_path).expect("Should read file");
  assert_eq!(content, "safe content");
}

// Test cleanup helper to ensure test directories are removed after tests
struct TestDirectoryCleanup
{
  dir: String,
}

impl TestDirectoryCleanup
{
  fn new(dir: &str) -> Self
  {
  // Clean up if directory exists from previous failed test
  let _= fs ::remove_dir_all(dir);
  Self { dir: dir.to_string() }
 }
}

impl Drop for TestDirectoryCleanup
{
  fn drop(&mut self)
  {
  // Clean up test directory after test completes
  let _= fs ::remove_dir_all(&self.dir);
 }
}
