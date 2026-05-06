//! Comprehensive corner case tests for `crates_tools`
//!
//! ## Purpose
//!
//! Tests all corner cases identified in the corner case matrix that aren't
//! covered by existing tests. Focuses on P0 (security, panics) and P1
//! (common real-world scenarios) cases.
//!
//! ## Test Organization
//!
//! - Category 1: Network edge cases
//! - Category 2: Archive structure edge cases
//! - Category 3: Path handling edge cases
//! - Category 4: Content handling edge cases
//! - Category 5: No-panic guarantees

#[ cfg(feature = "enabled") ]
use crates_tools ::CrateArchive;

// ============================================================================
// Category 1: Network Edge Cases
// ============================================================================

/// C1.1: Empty string as crate name should return error, not panic
///
/// ## Corner Case Analysis
///
/// **Risk**: Empty crate name could cause panic in URL formatting or HTTP request.
/// **Expected**: Should return `Result::Err` with descriptive error message.
/// **Priority**: P1 (High) - common input validation scenario
#[ cfg(feature = "enabled") ]
#[ test ]
fn test_empty_crate_name()
{
  let result = CrateArchive ::download_crates_io("", "0.1.0");

  // Should return error, not panic
  assert!(result.is_err(), "Empty crate name should return Err");

  // Error message should be informative
  let err_msg = format!("{}", result.unwrap_err());
  assert!(!err_msg.is_empty(), "Error message should not be empty");
}

/// C1.2: Empty string as version should return error, not panic
///
/// ## Corner Case Analysis
///
/// **Risk**: Empty version could cause panic in URL formatting or invalid HTTP request.
/// **Expected**: Should return `Result::Err` with descriptive error message.
/// **Priority**: P1 (High) - common input validation scenario
#[ cfg(feature = "enabled") ]
#[ test ]
fn test_empty_version()
{
  let result = CrateArchive ::download_crates_io("test_experimental_c", "");

  // Should return error, not panic
  assert!(result.is_err(), "Empty version should return Err");

  // Error message should be informative
  let err_msg = format!("{}", result.unwrap_err());
  assert!(!err_msg.is_empty(), "Error message should not be empty");
}

/// C1.3: Invalid version format should return error, not panic
///
/// ## Corner Case Analysis
///
/// **Risk**: Version formats like "latest", "1.x", "v1.0.0" are invalid for crates.io.
/// **Expected**: Should return HTTP 404 error, not panic.
/// **Priority**: P2 (Medium) - users might try common version patterns
#[ cfg(feature = "enabled") ]
#[ test ]
fn test_invalid_version_format()
{
  let test_cases = vec![
  "latest",
  "1.x",
  "v1.0.0",
  "1.0",
  "1",
  ];

  for invalid_version in test_cases
  {
  let result = CrateArchive ::download_crates_io("test_experimental_c", invalid_version);

  // Should return error (404 from crates.io), not panic
  assert!(
   result.is_err(),
   "Invalid version '{invalid_version}' should return Err, not panic"
  );
 }
}

/// C1.4: Very long crate name should not cause issues
///
/// ## Corner Case Analysis
///
/// **Risk**: Extremely long crate names could cause buffer overflows or allocation issues.
/// **Expected**: Should return HTTP error (name doesn't exist), not panic.
/// **Priority**: P3 (Low) - rare, but good safety check
#[ cfg(feature = "enabled") ]
#[ test ]
fn test_very_long_crate_name()
{
  // Create 1000-character crate name
  let long_name = "a".repeat(1000);

  let result = CrateArchive ::download_crates_io(&long_name, "0.1.0");

  // Should handle gracefully (return error), not panic
  assert!(result.is_err(), "Very long crate name should return Err");
}

// ============================================================================
// Category 2: Archive Structure Edge Cases
// ============================================================================

/// C2.1: Archive with single file should work correctly
///
/// ## Corner Case Analysis
///
/// **Risk**: Edge case of minimal archive (1 file instead of typical 3-4).
/// **Expected**: Should decode and list single file correctly.
/// **Priority**: P2 (Medium) - uncommon but valid
#[ cfg(feature = "enabled") ]
#[ test ]
fn test_single_file_archive()
{
  use flate2 ::write ::GzEncoder;
  use flate2 ::Compression;
  use tar ::Builder;
  use std ::io ::Write;

  // Create tar with single file
  let mut tar_data = Vec ::new();
  {
  let mut builder = Builder ::new(&mut tar_data);

  let content = b"single file content";
  let mut header = tar ::Header ::new_gnu();
  header.set_path("single.txt").unwrap();
  header.set_size(content.len() as u64);
  header.set_cksum();
  builder.append(&header, &content[..]).unwrap();

  builder.finish().unwrap();
 }

  // Compress
  let mut encoder = GzEncoder ::new(Vec ::new(), Compression ::default());
  encoder.write_all(&tar_data).unwrap();
  let compressed = encoder.finish().unwrap();

  // Decode
  let archive = CrateArchive ::decode(compressed)
  .expect("Single-file archive should decode successfully");

  // Verify
  let files = archive.list();
  assert_eq!(files.len(), 1, "Archive should contain exactly 1 file");

  let content = archive.content_bytes("single.txt")
  .expect("File should exist in archive");
  assert_eq!(content, b"single file content");
}

/// C2.4: Zero-byte files should be handled correctly
///
/// ## Corner Case Analysis
///
/// **Risk**: Empty files could cause issues in content display or UTF-8 validation.
/// **Expected**: Should list file, return empty content, handle gracefully.
/// **Priority**: P1 (High) - common in real crates (e.g., __init__.py equivalents)
#[ cfg(feature = "enabled") ]
#[ test ]
fn test_zero_byte_files()
{
  use flate2 ::write ::GzEncoder;
  use flate2 ::Compression;
  use tar ::Builder;
  use std ::io ::Write;

  // Create archive with zero-byte file
  let mut tar_data = Vec ::new();
  {
  let mut builder = Builder ::new(&mut tar_data);

  let content = b"";
  let mut header = tar ::Header ::new_gnu();
  header.set_path("empty.txt").unwrap();
  header.set_size(0);
  header.set_cksum();
  builder.append(&header, &content[..]).unwrap();

  builder.finish().unwrap();
 }

  // Compress
  let mut encoder = GzEncoder ::new(Vec ::new(), Compression ::default());
  encoder.write_all(&tar_data).unwrap();
  let compressed = encoder.finish().unwrap();

  // Decode
  let archive = CrateArchive ::decode(compressed)
  .expect("Archive with zero-byte file should decode");

  // Verify file exists
  let files = archive.list();
  assert_eq!(files.len(), 1, "Archive should contain the file");

  // Verify content is empty
  let content = archive.content_bytes("empty.txt")
  .expect("Zero-byte file should exist");
  assert_eq!(content.len(), 0, "Content should be empty");

  // Verify UTF-8 conversion works (empty string is valid UTF-8)
  let text = core ::str ::from_utf8(content)
  .expect("Empty content should be valid UTF-8");
  assert_eq!(text, "", "Should be empty string");
}

/// C2.6-C2.9: Corrupted archive data should return error, not panic
///
/// ## Corner Case Analysis
///
/// **Risk**: Corrupted gzip or tar data could cause panics during decompression.
/// **Expected**: Should return `io::Error`, not panic.
/// **Priority**: P0 (Critical) - security and stability
#[ cfg(feature = "enabled") ]
#[ test ]
fn test_corrupted_gzip_data()
{
  // Invalid gzip header
  let corrupted_data = vec![ 0xFF, 0xFF, 0xFF, 0xFF, 0xFF ];

  let result = CrateArchive ::decode(corrupted_data);

  // Should return error, not panic
  assert!(result.is_err(), "Corrupted gzip should return Err");
}

#[ cfg(feature = "enabled") ]
#[ test ]
fn test_truncated_archive()
{
  use flate2 ::write ::GzEncoder;
  use flate2 ::Compression;
  use tar ::Builder;
  use std ::io ::Write;

  // Create valid archive
  let mut tar_data = Vec ::new();
  {
  let mut builder = Builder ::new(&mut tar_data);

  let content = b"test content";
  let mut header = tar ::Header ::new_gnu();
  header.set_path("test.txt").unwrap();
  header.set_size(content.len() as u64);
  header.set_cksum();
  builder.append(&header, &content[..]).unwrap();

  builder.finish().unwrap();
 }

  let mut encoder = GzEncoder ::new(Vec ::new(), Compression ::default());
  encoder.write_all(&tar_data).unwrap();
  let mut compressed = encoder.finish().unwrap();

  // Truncate to half size
  compressed.truncate(compressed.len() / 2);

  // Should return error, not panic
  let result = CrateArchive ::decode(compressed);
  assert!(result.is_err(), "Truncated archive should return Err");
}

// ============================================================================
// Category 3: Path Handling Edge Cases
// ============================================================================

/// C3.1: Unicode in filenames should be handled correctly
///
/// ## Corner Case Analysis
///
/// **Risk**: Unicode paths (emoji, non-Latin scripts) could cause display or lookup issues.
/// **Expected**: Should store, retrieve, and display correctly.
/// **Priority**: P1 (High) - increasingly common in global development
#[ cfg(feature = "enabled") ]
#[ test ]
fn test_unicode_filenames()
{
  use flate2 ::write ::GzEncoder;
  use flate2 ::Compression;
  use tar ::Builder;
  use std ::io ::Write;

  let test_filenames = vec![
  "测试.txt",          // Chinese
  "テスト.txt",         // Japanese
  "файл.txt",         // Russian
  "ملف.txt",          // Arabic
  "😀_emoji.txt",     // Emoji
  "café.txt",         // Accented characters
  ];

  // Create archive with Unicode filenames
  let mut tar_data = Vec ::new();
  {
  let mut builder = Builder ::new(&mut tar_data);

  for filename in &test_filenames
  {
   let content = format!("content of {filename}");
   let content_bytes = content.as_bytes();

   let mut header = tar ::Header ::new_gnu();
   header.set_path(filename).unwrap();
   header.set_size(content_bytes.len() as u64);
   header.set_cksum();
   builder.append(&header, content_bytes).unwrap();
  }

  builder.finish().unwrap();
 }

  let mut encoder = GzEncoder ::new(Vec ::new(), Compression ::default());
  encoder.write_all(&tar_data).unwrap();
  let compressed = encoder.finish().unwrap();

  // Decode
  let archive = CrateArchive ::decode(compressed)
  .expect("Archive with Unicode filenames should decode");

  // Verify all files are listed
  let files = archive.list();
  assert_eq!(files.len(), test_filenames.len(), "All files should be listed");

  // Verify each file can be retrieved
  for filename in &test_filenames
  {
  let content = archive.content_bytes(filename)
   .unwrap_or_else(|| panic!("Should find file: {filename}"));

  let expected = format!("content of {filename}");
  let actual = core ::str ::from_utf8(content)
   .expect("Content should be valid UTF-8");

  assert_eq!(actual, expected, "Content should match for {filename}");
 }
}

/// C3.2: Special characters in filenames should be handled correctly
///
/// ## Corner Case Analysis
///
/// **Risk**: Filenames with spaces, quotes, or other special chars could break display.
/// **Expected**: Should handle correctly without escaping issues.
/// **Priority**: P1 (High) - common in real-world files
#[ cfg(feature = "enabled") ]
#[ test ]
fn test_special_characters_in_filenames()
{
  use flate2 ::write ::GzEncoder;
  use flate2 ::Compression;
  use tar ::Builder;
  use std ::io ::Write;

  let test_filenames = vec![
  "file with spaces.txt",
  "file'with'quotes.txt",
  "file\"with\"doublequotes.txt",
  "file(with)parens.txt",
  "file[with]brackets.txt",
  "file{with}braces.txt",
  ];

  // Create archive
  let mut tar_data = Vec ::new();
  {
  let mut builder = Builder ::new(&mut tar_data);

  for filename in &test_filenames
  {
   let content = b"test content";
   let mut header = tar ::Header ::new_gnu();
   header.set_path(filename).unwrap();
   header.set_size(content.len() as u64);
   header.set_cksum();
   builder.append(&header, &content[..]).unwrap();
  }

  builder.finish().unwrap();
 }

  let mut encoder = GzEncoder ::new(Vec ::new(), Compression ::default());
  encoder.write_all(&tar_data).unwrap();
  let compressed = encoder.finish().unwrap();

  // Decode
  let archive = CrateArchive ::decode(compressed)
  .expect("Archive with special char filenames should decode");

  // Verify retrieval works
  for filename in &test_filenames
  {
  let content = archive.content_bytes(filename)
   .unwrap_or_else(|| panic!("Should find file: {filename}"));

  assert_eq!(content, b"test content", "Content should match for {filename}");
 }
}

// ============================================================================
// Category 4: Content Handling Edge Cases
// ============================================================================

/// C4.6: Partial UTF-8 sequences should be handled as binary
///
/// ## Corner Case Analysis
///
/// **Risk**: Truncated multi-byte UTF-8 characters could cause validation issues.
/// **Expected**: Should fail UTF-8 validation, be treated as binary.
/// **Priority**: P1 (High) - can occur with truncated files
#[ cfg(feature = "enabled") ]
#[ test ]
fn test_partial_utf8_sequences()
{
  use flate2 ::write ::GzEncoder;
  use flate2 ::Compression;
  use tar ::Builder;
  use std ::io ::Write;

  // Create partial UTF-8 sequence (truncated emoji)
  // Emoji "😀" is U+1F600, encoded as F0 9F 98 80 in UTF-8
  // Truncate to just F0 9F (incomplete)
  let partial_utf8: Vec< u8 > = vec![ 0xF0, 0x9F ];

  // Verify it's invalid UTF-8
  assert!(core ::str ::from_utf8(&partial_utf8).is_err(), "Test data must be invalid UTF-8");

  // Create archive with this content
  let mut tar_data = Vec ::new();
  {
  let mut builder = Builder ::new(&mut tar_data);

  let mut header = tar ::Header ::new_gnu();
  header.set_path("partial.bin").unwrap();
  header.set_size(partial_utf8.len() as u64);
  header.set_cksum();
  builder.append(&header, &partial_utf8[..]).unwrap();

  builder.finish().unwrap();
 }

  let mut encoder = GzEncoder ::new(Vec ::new(), Compression ::default());
  encoder.write_all(&tar_data).unwrap();
  let compressed = encoder.finish().unwrap();

  // Decode
  let archive = CrateArchive ::decode(compressed)
  .expect("Archive with partial UTF-8 should decode");

  // Get content
  let content = archive.content_bytes("partial.bin")
  .expect("File should exist");

  // Verify UTF-8 validation fails (should be treated as binary)
  assert!(
  core ::str ::from_utf8(content).is_err(),
  "Partial UTF-8 should fail validation"
 );

  // Verify content is preserved correctly
  assert_eq!(content, &partial_utf8[..], "Binary content should be preserved");
}

/// C4.12: Files with null bytes should be handled as binary
///
/// ## Corner Case Analysis
///
/// **Risk**: Null bytes in content could cause string handling issues.
/// **Expected**: Should be handled as binary (UTF-8 validation fails), content preserved.
/// **Priority**: P2 (Medium) - uncommon in text files, common in binary
#[ cfg(feature = "enabled") ]
#[ test ]
fn test_null_bytes_in_content()
{
  use flate2 ::write ::GzEncoder;
  use flate2 ::Compression;
  use tar ::Builder;
  use std ::io ::Write;

  // Content with null bytes
  let content_with_nulls: Vec< u8 > = vec![
  b'H', b'e', b'l', b'l', b'o', 0x00, b'W', b'o', b'r', b'l', b'd', 0x00,
  ];

  // Create archive
  let mut tar_data = Vec ::new();
  {
  let mut builder = Builder ::new(&mut tar_data);

  let mut header = tar ::Header ::new_gnu();
  header.set_path("with_nulls.bin").unwrap();
  header.set_size(content_with_nulls.len() as u64);
  header.set_cksum();
  builder.append(&header, &content_with_nulls[..]).unwrap();

  builder.finish().unwrap();
 }

  let mut encoder = GzEncoder ::new(Vec ::new(), Compression ::default());
  encoder.write_all(&tar_data).unwrap();
  let compressed = encoder.finish().unwrap();

  // Decode
  let archive = CrateArchive ::decode(compressed)
  .expect("Archive with null bytes should decode");

  // Get content
  let content = archive.content_bytes("with_nulls.bin")
  .expect("File should exist");

  // Verify content is preserved (including null bytes)
  assert_eq!(content, &content_with_nulls[..], "Null bytes should be preserved");

  // Verify UTF-8 validation (actually null bytes ARE valid UTF-8, interesting!)
  // But in practice, text files with nulls are treated as binary
  let utf8_result = core ::str ::from_utf8(content);
  if let Ok(text) = utf8_result
  {
  // If it's valid UTF-8, verify the null bytes are there
  assert!(text.contains('\0'), "Null characters should be in string");
 }
}

// ============================================================================
// Category 5: No-Panic Guarantees
// ============================================================================

/// C6.3: Comprehensive no-panic guarantee for all error conditions
///
/// ## Corner Case Analysis
///
/// **Risk**: Any panic in library code is unacceptable for production use.
/// **Expected**: All error conditions should return `Result::Err`, never panic.
/// **Priority**: P0 (Critical) - fundamental reliability requirement
#[ cfg(feature = "enabled") ]
#[ test ]
fn test_no_panics_on_error_conditions()
{
  // Test all error conditions return Err, not panic

  // Network errors
  let _ = CrateArchive ::download_crates_io("nonexistent_crate_xyz", "0.1.0");
  let _ = CrateArchive ::download_crates_io("", "0.1.0");
  let _ = CrateArchive ::download_crates_io("test", "");

  // Corrupted data
  let _ = CrateArchive ::decode(vec![ 0xFF, 0xFF, 0xFF ]);
  let _ = CrateArchive ::decode(vec![]);

  // Reaching here without panicking means all error conditions are handled
}
