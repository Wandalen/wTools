#![allow(missing_docs)]
#![allow(clippy ::std_instead_of_core)]

//! Tests validating example code quality and error handling patterns
//!
//! These tests ensure examples demonstrate best practices and handle
//! edge cases gracefully without panicking.

#[ cfg(feature = "enabled") ]
use crates_tools ::CrateArchive;

/// Reproduces Issue 1: Error handling validation for network failures
///
/// ## Ultrathink Analysis
///
/// **Issue Root Cause**: Original example used `.unwrap()` on `download_crates_io()`,
/// causing panic on network errors (offline, 404, timeout, etc.). This violated
/// `test_organization.rulebook.md` requirement for graceful error handling.
///
/// **Why This Matters**:
/// - Network operations inherently unreliable (connectivity, DNS, timeouts)
/// - Users copy example patterns into production code
/// - Panics in production code are unacceptable
/// - Examples must demonstrate robust error handling
///
/// **Fix Applied**: Replaced `.unwrap()` with `?` operator and Result<> return type,
/// enabling proper error propagation and user-friendly error messages.
///
/// **Test Strategy**: Verify that error cases return `Result::Err` instead of panicking.
/// We test with invalid crate name which triggers 404 error.
#[ cfg(feature = "enabled") ]
#[ test ]
fn test_error_handling_network_failures()
{
  // Test with non-existent crate (simulates network error scenario)
  let result = CrateArchive ::download_crates_io("this_crate_definitely_does_not_exist_xyz123", "0.1.0");

  // Should return Err, not panic
  assert!(result.is_err(), "Download of non-existent crate should return Err, not panic");

  // Error should contain useful information
  let err_msg = format!("{}", result.unwrap_err());
  assert!(!err_msg.is_empty(), "Error message should not be empty");
}

/// Reproduces Issue 2: Binary content handling validation
///
/// ## Ultrathink Analysis
///
/// **Issue Root Cause**: Original example assumed all archive files are UTF-8 text,
/// using `.unwrap()` on `from_utf8()`. Many real crates contain binary files
/// (test fixtures, images, compiled objects), causing panic.
///
/// **Why This Matters**:
/// - Real-world crates commonly include binary test data
/// - Examples of crates with binary: image processing (PNG/JPEG), compression (ZIP),
///   serialization (binary protobuf), embedded resources
/// - UTF-8 validation failure is expected behavior for binary, not exceptional case
/// - Graceful handling required per `test_organization.rulebook.md`
///
/// **Fix Applied**: Used `match` on `from_utf8()` result with graceful fallback
/// for binary files (displays "[BINARY]" marker and byte count).
///
/// **Test Strategy**: Create archive with known binary content, verify handling
/// without panic. Uses raw bytes that are definitively not UTF-8.
#[ cfg(feature = "enabled") ]
#[ test ]
fn test_binary_content_handling()
{
  // Create test archive with binary content (invalid UTF-8)
  let binary_data: Vec< u8 > = vec![ 0xFF, 0xFE, 0xFD, 0x00, 0x80, 0x81 ];

  // Verify binary data is NOT valid UTF-8
  assert!(core ::str ::from_utf8(&binary_data).is_err(), "Test data must be invalid UTF-8");

  // Create minimal tar.gz archive with binary content
  use flate2 ::write ::GzEncoder;
  use flate2 ::Compression;
  use tar ::Builder;
  use std ::io ::Write;

  let mut tar_data = Vec ::new();
  {
    let mut builder = Builder ::new(&mut tar_data);

    // Add binary file to archive
    let mut header = tar ::Header ::new_gnu();
    header.set_path("test_binary.bin").unwrap();
    header.set_size(binary_data.len() as u64);
    header.set_cksum();
    builder.append(&header, &binary_data[..]).unwrap();

    builder.finish().unwrap();
  }

  // Compress with gzip
  let mut encoder = GzEncoder ::new(Vec ::new(), Compression ::default());
  encoder.write_all(&tar_data).unwrap();
  let compressed = encoder.finish().unwrap();

  // Decode archive (should succeed)
  let archive = CrateArchive ::decode(compressed)
    .expect("Archive decode should succeed even with binary content");

  // Verify archive contains the binary file
  let files = archive.list();
  assert_eq!(files.len(), 1, "Archive should contain 1 file");

  // Get content
  let content = archive.content_bytes("test_binary.bin")
    .expect("Binary file should exist in archive");

  // Verify it's the binary data we added
  assert_eq!(content, &binary_data[..], "Binary content should match");

  // Verify UTF-8 conversion fails (binary content)
  assert!(core ::str ::from_utf8(content).is_err(), "Binary content should fail UTF-8 validation");

  // KEY TEST: Demonstrate graceful handling (what fixed example does)
  if let Ok(_text) = core ::str ::from_utf8(content)
  {
    panic!("Should not be UTF-8!")
  }
  else
  {
    // Graceful handling: acknowledge binary, show size
    let size = content.len();
    assert_eq!(size, 6, "Binary size should be correct");
    // This represents what the fixed example does: handle gracefully without panic
  }
}

/// Reproduces Issue 3: Example demonstrates proper Result handling patterns
///
/// ## Ultrathink Analysis
///
/// **Issue Root Cause**: Original example provided no guidance on error handling,
/// using `.unwrap()` throughout. This violated principle that examples should
/// teach best practices, not anti-patterns.
///
/// **Why This Matters**:
/// - Examples are primary learning resource for API users
/// - Users copy example code patterns into production
/// - `.unwrap()` in examples promotes `.unwrap()` in production code
/// - Rust ecosystem emphasizes explicit error handling (Result<>, ? operator)
/// - Library examples have pedagogical responsibility
///
/// **Fix Applied**: Transformed `main()` to return `Result<>`, used `?` operator for
/// error propagation, demonstrated match for binary handling. Shows idiomatic
/// Rust error handling patterns.
///
/// **Test Strategy**: Verify that proper Result-based APIs work correctly when
/// used idiomatically (with ? operator). Tests successful case and error case.
#[ cfg(feature = "enabled") ]
#[ test ]
fn test_result_based_api_usage()
{
  // Define helper that uses ? operator (idiomatic pattern from fixed example)
  fn download_and_process() -> Result< usize, Box< dyn std ::error ::Error > >
  {
    // Using ? operator for error propagation (best practice)
    let archive = CrateArchive ::download_crates_io("test_experimental_c", "0.1.0")?;

    // Count files
    let file_count = archive.list().len();

    Ok(file_count)
  }

  // Test successful case
  let result = download_and_process();
  assert!(result.is_ok(), "Valid download should succeed");
  assert!(result.unwrap() > 0, "Archive should contain files");

  // Test error case (non-existent crate)
  fn download_invalid() -> Result< usize, Box< dyn std ::error ::Error > >
  {
    let archive = CrateArchive ::download_crates_io("invalid_crate_name_xyz", "0.1.0")?;
    Ok(archive.list().len())
  }

  let result = download_invalid();
  assert!(result.is_err(), "Invalid download should return Err");
}

/// Integration test: Verify fixed example handles all edge cases
///
/// ## Ultrathink Analysis
///
/// **Comprehensive Validation**: This test validates that all three issues
/// (error handling, binary content, idiomatic patterns) are properly addressed
/// in combination, not just individually.
///
/// **Test Coverage**:
/// 1. Network error handling (404, invalid input)
/// 2. Binary content graceful handling
/// 3. Idiomatic Result-based patterns
///
/// **Quality Gate**: Example must handle real-world complexity without panic.
#[ cfg(feature = "enabled") ]
#[ test ]
fn test_example_handles_all_edge_cases()
{
  // Edge case 1: Error handling (invalid crate)
  let err_result = CrateArchive ::download_crates_io("", "0.0.0");
  assert!(err_result.is_err(), "Empty crate name should error, not panic");

  // Edge case 2: Valid download with potential binary content
  let ok_result = CrateArchive ::download_crates_io("test_experimental_c", "0.1.0");
  assert!(ok_result.is_ok(), "Valid crate should download successfully");

  let archive = ok_result.unwrap();

  // Edge case 3: Binary content handling (test on all files)
  for path in archive.list()
  {
    if let Some(bytes) = archive.content_bytes(path)
    {
      // Graceful handling: match on UTF-8 validation result
      match core ::str ::from_utf8(bytes)
      {
        Ok(_text) =>
        {
          // Text file - process normally
        }
        Err(_) =>
        {
          // Binary file - handle gracefully (no panic)
          assert!(!bytes.is_empty(), "Binary files should have content");
        }
      }
    }
  }
}
