//! Memory profiling tests
//!
//! Verifies NFR2: Processing 100 files uses less than 10MB of memory

use genfile_core::
{
  TemplateArchive,
  WriteMode,
  Value,
  HandlebarsRenderer,
  MemoryFileSystem,
};
use std::path::PathBuf;
use memory_stats::memory_stats;

/// Format bytes as human-readable size
fn format_bytes( bytes: usize ) -> String
{
  if bytes < 1024
  {
    format!( "{} bytes", bytes )
  }
  else if bytes < 1024 * 1024
  {
    format!( "{:.2} KB", bytes as f64 / 1024.0 )
  }
  else
  {
    format!( "{:.2} MB", bytes as f64 / ( 1024.0 * 1024.0 ) )
  }
}

/// Test NFR2: Process 100 files with less than 10MB memory
#[ test ]
fn test_memory_usage_100_files()
{
  // Get initial memory usage
  let initial_memory = memory_stats().expect( "couldn't get memory stats" );
  let initial_physical = initial_memory.physical_mem;

  // Create archive with 100 files
  let mut archive = TemplateArchive::new( "memory-test" );

  for i in 0..100
  {
    let content = format!(
      "File {}: {{{{name}}}} - {{{{value_{}}}}}\\nSome template content here.\\n",
      i, i
    );
    archive.add_text_file(
      PathBuf::from( format!( "file_{}.txt", i ) ),
      &content,
      WriteMode::Rewrite
    );
  }

  // Add parameter values
  archive.set_value( "name", Value::String( "TestProject".into() ) );
  for i in 0..100
  {
    archive.set_value( &format!( "value_{}", i ), Value::String( format!( "value_{}", i ) ) );
  }

  // Measure memory after archive creation
  let after_creation = memory_stats().expect( "couldn't get memory stats" );
  let after_creation_physical = after_creation.physical_mem;

  // Materialize to filesystem
  let renderer = HandlebarsRenderer::new();
  let mut fs = MemoryFileSystem::new();

  archive.materialize_with_components(
    PathBuf::from( "/output" ).as_path(),
    &renderer,
    &mut fs
  ).expect( "materialization failed" );

  // Measure memory after materialization
  let after_materialization = memory_stats().expect( "couldn't get memory stats" );
  let after_materialization_physical = after_materialization.physical_mem;

  // Calculate memory deltas
  let creation_delta = after_creation_physical.saturating_sub( initial_physical );
  let materialization_delta = after_materialization_physical.saturating_sub( after_creation_physical );
  let total_delta = after_materialization_physical.saturating_sub( initial_physical );

  // Print results
  println!( "=== Memory Usage Report ===" );
  println!( "Initial memory:              {}", format_bytes( initial_physical ) );
  println!( "After archive creation:      {}", format_bytes( after_creation_physical ) );
  println!( "After materialization:       {}", format_bytes( after_materialization_physical ) );
  println!();
  println!( "Archive creation delta:      {}", format_bytes( creation_delta ) );
  println!( "Materialization delta:       {}", format_bytes( materialization_delta ) );
  println!( "Total memory delta:          {}", format_bytes( total_delta ) );
  println!();

  // Verify NFR2: under 10MB
  let max_allowed = 10 * 1024 * 1024; // 10MB in bytes
  println!( "NFR2 Requirement:            {}", format_bytes( max_allowed ) );
  println!( "Actual memory usage:         {}", format_bytes( total_delta ) );

  if total_delta < max_allowed
  {
    let percentage = ( total_delta as f64 / max_allowed as f64 ) * 100.0;
    println!( "Status:                      ✅ PASS ({:.1}% of limit)", percentage );
  }
  else
  {
    let percentage = ( total_delta as f64 / max_allowed as f64 ) * 100.0;
    println!( "Status:                      ❌ FAIL ({:.1}% of limit)", percentage );
  }

  // Assert the requirement
  assert!(
    total_delta < max_allowed,
    "Memory usage {} exceeds 10MB limit",
    format_bytes( total_delta )
  );
}

/// Test memory usage with larger archives (500 files)
#[ test ]
fn test_memory_usage_500_files()
{
  let initial_memory = memory_stats().expect( "couldn't get memory stats" );
  let initial_physical = initial_memory.physical_mem;

  let mut archive = TemplateArchive::new( "large-test" );

  for i in 0..500
  {
    let content = format!( "File {}: {{{{name}}}}\\n", i );
    archive.add_text_file(
      PathBuf::from( format!( "file_{}.txt", i ) ),
      &content,
      WriteMode::Rewrite
    );
  }

  archive.set_value( "name", Value::String( "LargeProject".into() ) );

  let renderer = HandlebarsRenderer::new();
  let mut fs = MemoryFileSystem::new();

  archive.materialize_with_components(
    PathBuf::from( "/output" ).as_path(),
    &renderer,
    &mut fs
  ).expect( "materialization failed" );

  let after_materialization = memory_stats().expect( "couldn't get memory stats" );
  let total_delta = after_materialization.physical_mem.saturating_sub( initial_physical );

  println!( "Memory usage for 500 files: {}", format_bytes( total_delta ) );

  // This is just informational - we don't enforce a limit for 500 files
}

/// Test memory usage with binary files
#[ test ]
fn test_memory_usage_binary_files()
{
  let initial_memory = memory_stats().expect( "couldn't get memory stats" );
  let initial_physical = initial_memory.physical_mem;

  let mut archive = TemplateArchive::new( "binary-test" );

  // Add 50 text files and 50 binary files (100 total)
  for i in 0..50
  {
    let content = format!( "Text file {}: {{{{name}}}}\\n", i );
    archive.add_text_file(
      PathBuf::from( format!( "text_{}.txt", i ) ),
      &content,
      WriteMode::Rewrite
    );
  }

  for i in 0..50
  {
    // Create a 1KB binary file
    let binary_data: Vec< u8 > = ( 0..1024 ).map( | b | ( b % 256 ) as u8 ).collect();
    archive.add_binary_file(
      PathBuf::from( format!( "binary_{}.bin", i ) ),
      binary_data
    );
  }

  archive.set_value( "name", Value::String( "BinaryProject".into() ) );

  let renderer = HandlebarsRenderer::new();
  let mut fs = MemoryFileSystem::new();

  archive.materialize_with_components(
    PathBuf::from( "/output" ).as_path(),
    &renderer,
    &mut fs
  ).expect( "materialization failed" );

  let after_materialization = memory_stats().expect( "couldn't get memory stats" );
  let total_delta = after_materialization.physical_mem.saturating_sub( initial_physical );

  println!( "Memory usage for 100 files (50 text + 50 binary): {}", format_bytes( total_delta ) );

  // Verify under 10MB
  let max_allowed = 10 * 1024 * 1024;
  assert!(
    total_delta < max_allowed,
    "Memory usage {} exceeds 10MB limit",
    format_bytes( total_delta )
  );
}
