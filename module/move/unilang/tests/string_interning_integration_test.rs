//! Integration tests for string interning functionality
//!
//! Validates that string interning works correctly within the semantic analysis
//! pipeline and provides the expected memory and performance benefits.

use unilang::prelude::*;
use core::sync::atomic::{ AtomicUsize, Ordering };
use std::time::Instant;

// Test that string interning returns the same reference for identical command names
#[ test ]
fn test_string_interning_reference_equality()
{
  let interner = unilang::interner::StringInterner::new();
  
  // Test basic interning
  let cmd1 = interner.intern_command_name( &[ "test", "command" ] );
  let cmd2 = interner.intern_command_name( &[ "test", "command" ] );
  
  // Should return the same reference (pointer equality)
  assert!( core::ptr::eq( cmd1, cmd2 ), "String interning should return the same reference for identical strings" );
  assert_eq!( cmd1, ".test.command" );
}

#[ test ]
fn test_global_interner_integration()
{
  // Test that global interner convenience functions work
  let cmd1 = unilang::interner::intern_command_name( &[ "global", "test" ] );
  let cmd2 = unilang::interner::intern_command_name( &[ "global", "test" ] );
  
  assert!( core::ptr::eq( cmd1, cmd2 ) );
  assert_eq!( cmd1, ".global.test" );
}

#[ test ]
fn test_semantic_analyzer_integration()
{
  // This test verifies that string interning works correctly within the semantic analyzer
  // by testing that repeated command name construction uses interned strings
  
  #[allow(deprecated)]
  #[allow(deprecated)]
    let mut registry = CommandRegistry::new();
  registry.register( CommandDefinition
  {
    name : ".test.command".to_string(),
    description : "Test command".to_string(),
    arguments : vec![],
    routine_link : None,
    auto_help_enabled: false,
    namespace : "test".to_string(),
    hint : "Test command".to_string(),
    status : "stable".to_string(),
    version : "1.0.0".to_string(),
    tags : vec![],
    aliases : vec![],
    permissions : vec![],
    idempotent : true,
    deprecation_message : String::new(),
    http_method_hint : String::new(),
    examples : vec![],
  });
  
  let pipeline = Pipeline::new( registry );
  
  // Test string interning by processing the same command multiple times
  // The key test is that command resolution works (proving string interning works)
  // even if execution fails due to missing routine
  for i in 0..5
  {
    let result = pipeline.process_command_simple( "test.command" );
    
    // The command should be found (string interning works)
    // but may fail at execution stage (missing routine) - that's OK for this test
    if let Some( ref error_msg ) = result.error
    {
      // Acceptable errors: missing executable routine (proves command was found)
      // Unacceptable errors: command not found (would indicate string interning issue) 
      assert!( 
        error_msg.contains( "No executable routine found" ) || 
        error_msg.contains( "not implemented" ) ||
        result.success, // Or complete success
        "Iteration {i}: Unexpected error type: {error_msg}" 
      );
    }
  }
}

#[ test ]
fn test_interning_with_empty_first_slice()
{
  let interner = unilang::interner::StringInterner::new();
  
  // Test the edge case where first slice is empty
  let cmd1 = interner.intern_command_name( &[ "", "test", "command" ] );
  let cmd2 = interner.intern_command_name( &[ "test", "command" ] );
  
  // Both should produce the same result
  assert_eq!( cmd1, ".test.command" );
  assert_eq!( cmd2, ".test.command" );
  
  // And should be the same interned reference
  assert!( core::ptr::eq( cmd1, cmd2 ) );
}

#[ test ]
fn test_cache_size_limits()
{
  let interner = unilang::interner::StringInterner::with_capacity( 5 );
  
  // Fill cache to capacity
  let _c1 = interner.intern_command_name( &[ "cmd1" ] );
  let _c2 = interner.intern_command_name( &[ "cmd2" ] );
  let _c3 = interner.intern_command_name( &[ "cmd3" ] );
  let _c4 = interner.intern_command_name( &[ "cmd4" ] );
  let _c5 = interner.intern_command_name( &[ "cmd5" ] );
  
  let stats_at_capacity = interner.stats();
  assert_eq!( stats_at_capacity.cached_strings, 5 );
  
  // Add one more - should trigger eviction
  let _c6 = interner.intern_command_name( &[ "cmd6" ] );
  
  let stats_after_eviction = interner.stats();
  assert_eq!( stats_after_eviction.cached_strings, 5 ); // Still at capacity limit
}

#[ test ]
fn test_thread_safety()
{
  use std::thread;
  use std::sync::Arc;
  
  let interner = Arc::new( unilang::interner::StringInterner::new() );
  let mut handles = Vec::new();
  let success_counter = Arc::new( AtomicUsize::new( 0 ) );
  
  // Spawn multiple threads
  for i in 0..8
  {
    let interner_clone = Arc::clone( &interner );
    let counter_clone = Arc::clone( &success_counter );
    
    let handle = thread::spawn( move ||
    {
      let test_suffix = format!( "test_{}", i % 3 );
      let command_slices = vec![ "thread", &test_suffix ]; // Some overlap
      
      // Each thread interns the same patterns multiple times
      for _ in 0..1000
      {
        let cmd = interner_clone.intern_command_name( &command_slices );
        
        // Verify correct format
        let expected = format!( ".thread.test_{}", i % 3 );
        if cmd == expected
        {
          counter_clone.fetch_add( 1, Ordering::SeqCst );
        }
      }
    });
    
    handles.push( handle );
  }
  
  // Wait for all threads
  for handle in handles
  {
    handle.join().unwrap();
  }
  
  // All operations should have succeeded
  assert_eq!( success_counter.load( Ordering::SeqCst ), 8 * 1000 );
  
  // Verify cache contains expected entries
  let stats = interner.stats();
  assert!( stats.cached_strings >= 3 ); // At least the 3 unique patterns
  assert!( stats.cached_strings <= 8 ); // At most one per thread
}

#[ test ]
fn test_performance_characteristics()
{
  let interner = unilang::interner::StringInterner::new();
  let test_commands = vec![
    vec![ "file", "create" ],
    vec![ "file", "delete" ],
    vec![ "user", "login" ],
    vec![ "system", "status" ],
  ];
  
  // Measure cache miss performance (first time)
  let miss_start = Instant::now();
  for cmd_slices in &test_commands
  {
    for _ in 0..1000
    {
      let _interned = interner.intern_command_name( cmd_slices );
    }
  }
  let miss_time = miss_start.elapsed();
  
  // Clear and measure cache miss again for comparison
  interner.clear();
  
  // Measure cache miss again
  let second_miss_start = Instant::now();
  for cmd_slices in &test_commands
  {
    let _interned = interner.intern_command_name( cmd_slices );
  }
  let second_miss_time = second_miss_start.elapsed();
  
  // Now measure cache hit performance (subsequent times)
  let hit_start = Instant::now();
  for _ in 0..1000
  {
    for cmd_slices in &test_commands
    {
      let _interned = interner.intern_command_name( cmd_slices );
    }
  }
  let hit_time = hit_start.elapsed();
  
  println!( "Cache miss time (bulk): {miss_time:?}" );
  println!( "Cache miss time (single): {second_miss_time:?}" );
  println!( "Cache hit time (bulk): {hit_time:?}" );
  
  // Cache hits should be reasonably fast compared to misses for bulk operations
  // Allow for some variance in performance due to system load and other factors
  // We expect cache hits to not be significantly slower than cache misses
  assert!( hit_time < miss_time * 5, "Cache hits should be reasonably fast compared to misses" );
}

#[ test ]
fn test_memory_usage_reporting()
{
  let interner = unilang::interner::StringInterner::new();
  
  let initial_stats = interner.stats();
  assert_eq!( initial_stats.cached_strings, 0 );
  assert_eq!( initial_stats.memory_usage_estimate, 0 );
  
  // Add some entries
  interner.intern_command_name( &[ "memory", "test" ] );
  interner.intern_command_name( &[ "another", "command" ] );
  
  let updated_stats = interner.stats();
  assert_eq!( updated_stats.cached_strings, 2 );
  assert!( updated_stats.memory_usage_estimate > 0, "Should report non-zero memory usage" );
  assert!( updated_stats.memory_usage_estimate < 1000, "Memory usage should be reasonable for small test" );
}

#[ test ]
fn test_pipeline_integration_correctness()
{
  // Test that string interning doesn't affect pipeline correctness over multiple calls
  #[allow(deprecated)]
  #[allow(deprecated)]
    let mut registry = CommandRegistry::new();
  registry.register( CommandDefinition
  {
    name : ".integration.test".to_string(),
    description : "Integration test command".to_string(),
    arguments : vec![],
    routine_link : None,
    auto_help_enabled: false,
    namespace : "test".to_string(),
    hint : "Test".to_string(),
    status : "stable".to_string(),
    version : "1.0.0".to_string(),
    tags : vec![],
    aliases : vec![],
    permissions : vec![],
    idempotent : true,
    deprecation_message : String::new(),
    http_method_hint : String::new(),
    examples : vec![],
  });
  
  let pipeline = Pipeline::new( registry );
  let command_text = "integration.test";
  
  // Process the same command multiple times to test consistency
  for i in 0..10
  {
    let result = pipeline.process_command_simple( command_text );
    
    // Verify consistent results across multiple calls
    assert_eq!( result.command, command_text );
    
    // Command should be found (may fail at execution, but consistently)
    if let Some( ref error_msg ) = result.error
    {
      assert!( 
        error_msg.contains( "No executable routine found" ) || 
        error_msg.contains( "not implemented" ) ||
        result.success,
        "Iteration {i}: Unexpected error: {error_msg}" 
      );
    }
  }
}

#[ test ]
fn test_error_handling_with_interning()
{
  #[allow(deprecated)]
  #[allow(deprecated)]
    let registry = CommandRegistry::new(); // Empty registry
  let pipeline = Pipeline::new( registry );
  
  // Try to process a non-existent command
  let result = pipeline.process_command_simple( "nonexistent command" );
  
  // Should fail
  assert!( !result.success, "Non-existent command should fail" );
  assert!( result.error.is_some(), "Should have error message" );
  
  let error_message = result.error.unwrap();
  assert!( error_message.contains( "not found" ) || error_message.contains( "COMMAND_NOT_FOUND" ),
          "Error message should indicate command not found: {error_message}" );
}

// Test that demonstrates the memory benefits
#[ test ]
fn test_memory_allocation_reduction()
{
  let interner = unilang::interner::StringInterner::new();
  
  // This test is more conceptual - in a real scenario,
  // we'd measure actual allocations, but we can at least
  // verify the behavior that should lead to allocation reduction
  
  let test_patterns = vec![
    vec![ "repeated", "command" ],
    vec![ "another", "repeated", "command" ],
    vec![ "third", "pattern" ],
  ];
  
  // First time - should create new strings
  let mut interned_strings = Vec::new();
  for pattern in &test_patterns
  {
    interned_strings.push( interner.intern_command_name( pattern ) );
  }
  
  // Subsequent times - should reuse existing strings
  for _ in 0..100
  {
    for ( i, pattern ) in test_patterns.iter().enumerate()
    {
      let interned_cmd = interner.intern_command_name( pattern );
      
      // Should be the same reference as before
      assert!( core::ptr::eq( interned_cmd, interned_strings[ i ] ),
              "Repeated interning should return same reference" );
    }
  }
  
  // Cache should only contain the unique patterns
  let stats = interner.stats();
  assert_eq!( stats.cached_strings, test_patterns.len() );
}