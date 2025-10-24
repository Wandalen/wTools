//! Integration tests for LogfmtFormatter
//!
//! ## What This Tests
//!
//! Comprehensive test coverage for `LogfmtFormatter` ensuring correct
//! logfmt output format and proper value escaping.
//!
//! ## Test Categories
//!
//! 1. **Basic formatting** - Simple key=value output
//! 2. **Value escaping** - Spaces, quotes, newlines, tabs
//! 3. **Edge cases** - Empty values, single rows, unicode, special chars
//! 4. **Configuration** - Custom separators, multiple columns
//! 5. **Real-world scenarios** - Log-like data structures
//!
//! ## Why Logfmt Format Matters
//!
//! Logfmt is crucial for:
//! - **Structured logging**: Human-readable AND machine-parseable
//! - **Observability**: Compatible with Prometheus, Loki, Elasticsearch
//! - **Grep-friendliness**: Easy search with standard Unix tools
//! - **Streaming**: Can be parsed line-by-line with constant memory
//!
//! ## Critical Test Cases
//!
//! ### Escaping Spaces (test_logfmt_escaping_spaces)
//! Values with spaces must be quoted:
//! - Input: `msg` = `"hello world"`
//! - Output: `msg="hello world"`
//!
//! ### Escaping Quotes (test_logfmt_escaping_quotes)
//! Quotes must be backslash-escaped:
//! - Input: `msg` = `say "hello"`
//! - Output: `msg="say \"hello\""`
//!
//! ### Escaping Newlines (test_logfmt_newlines)
//! Newlines replaced with literal \n:
//! - Input: `msg` = `"line1\nline2"`
//! - Output: `msg="line1\nline2"`

#![ allow( clippy::all, clippy::pedantic, clippy::nursery, warnings ) ]

use tree_fmt::{ RowBuilder, LogfmtFormatter, Format };

// =============================================================================
// Basic Functionality Tests
// =============================================================================

#[ test ]
fn test_logfmt_basic_single_column()
{
  let view = RowBuilder::new( vec![ "name".into() ] )
    .add_row( vec![ "Alice".into() ] )
    .add_row( vec![ "Bob".into() ] )
    .build_view();

  let formatter = LogfmtFormatter::new();
  let output = formatter.format( &view ).unwrap();

  println!( "Basic single column:\n{output}" );

  assert!( output.contains( "name=Alice" ) );
  assert!( output.contains( "name=Bob" ) );
  assert_eq!( output.lines().count(), 2 );
}

#[ test ]
fn test_logfmt_basic_multiple_columns()
{
  let view = RowBuilder::new( vec![ "name".into(), "age".into(), "city".into() ] )
    .add_row( vec![ "Alice".into(), "30".into(), "NYC".into() ] )
    .add_row( vec![ "Bob".into(), "25".into(), "LA".into() ] )
    .build_view();

  let formatter = LogfmtFormatter::new();
  let output = formatter.format( &view ).unwrap();

  println!( "Basic multiple columns:\n{output}" );

  let lines : Vec< &str > = output.lines().collect();
  assert_eq!( lines.len(), 2 );

  assert!( lines[ 0 ].contains( "name=Alice" ) );
  assert!( lines[ 0 ].contains( "age=30" ) );
  assert!( lines[ 0 ].contains( "city=NYC" ) );

  assert!( lines[ 1 ].contains( "name=Bob" ) );
  assert!( lines[ 1 ].contains( "age=25" ) );
  assert!( lines[ 1 ].contains( "city=LA" ) );
}

#[ test ]
fn test_logfmt_field_order_preserved()
{
  let view = RowBuilder::new( vec![ "first".into(), "second".into(), "third".into() ] )
    .add_row( vec![ "a".into(), "b".into(), "c".into() ] )
    .build_view();

  let formatter = LogfmtFormatter::new();
  let output = formatter.format( &view ).unwrap();

  // Check fields appear in correct order
  let first_pos = output.find( "first=" ).unwrap();
  let second_pos = output.find( "second=" ).unwrap();
  let third_pos = output.find( "third=" ).unwrap();

  assert!( first_pos < second_pos );
  assert!( second_pos < third_pos );
}

// =============================================================================
// Value Escaping Tests
// =============================================================================

#[ test ]
fn test_logfmt_escaping_spaces()
{
  let view = RowBuilder::new( vec![ "msg".into() ] )
    .add_row( vec![ "hello world".into() ] )
    .add_row( vec![ "foo bar baz".into() ] )
    .build_view();

  let formatter = LogfmtFormatter::new();
  let output = formatter.format( &view ).unwrap();

  println!( "Escaping spaces:\n{output}" );

  // Values with spaces must be quoted
  assert!( output.contains( "msg=\"hello world\"" ) );
  assert!( output.contains( "msg=\"foo bar baz\"" ) );
}

#[ test ]
fn test_logfmt_escaping_tabs()
{
  let view = RowBuilder::new( vec![ "data".into() ] )
    .add_row( vec![ "hello\tworld".into() ] )
    .build_view();

  let formatter = LogfmtFormatter::new();
  let output = formatter.format( &view ).unwrap();

  println!( "Escaping tabs:\n{output}" );

  // Values with tabs must be quoted
  assert!( output.contains( "data=\"hello\tworld\"" ) );
}

#[ test ]
fn test_logfmt_escaping_quotes()
{
  let view = RowBuilder::new( vec![ "msg".into() ] )
    .add_row( vec![ "say \"hello\"".into() ] )
    .add_row( vec![ "\"quoted\"".into() ] )
    .build_view();

  let formatter = LogfmtFormatter::new();
  let output = formatter.format( &view ).unwrap();

  println!( "Escaping quotes:\n{output}" );

  // Quotes must be backslash-escaped
  assert!( output.contains( "msg=\"say \\\"hello\\\"\"" ) );
  assert!( output.contains( "msg=\"\\\"quoted\\\"\"" ) );
}

#[ test ]
fn test_logfmt_escaping_newlines()
{
  let view = RowBuilder::new( vec![ "msg".into() ] )
    .add_row( vec![ "line1\nline2".into() ] )
    .add_row( vec![ "a\nb\nc".into() ] )
    .build_view();

  let formatter = LogfmtFormatter::new();
  let output = formatter.format( &view ).unwrap();

  println!( "Escaping newlines:\n{output}" );

  // Newlines replaced with literal \n
  assert!( output.contains( "msg=\"line1\\nline2\"" ) );
  assert!( output.contains( "msg=\"a\\nb\\nc\"" ) );

  // Output should still be 2 lines (not broken by newlines in values)
  assert_eq!( output.lines().count(), 2 );
}

#[ test ]
fn test_logfmt_escaping_combined()
{
  // Test value with spaces AND quotes AND newlines
  let view = RowBuilder::new( vec![ "msg".into() ] )
    .add_row( vec![ "say \"hello\nworld\" here".into() ] )
    .build_view();

  let formatter = LogfmtFormatter::new();
  let output = formatter.format( &view ).unwrap();

  println!( "Combined escaping:\n{output}" );

  assert!( output.contains( "msg=\"say \\\"hello\\nworld\\\" here\"" ) );
}

#[ test ]
fn test_logfmt_no_escaping_simple_values()
{
  let view = RowBuilder::new( vec![ "key".into() ] )
    .add_row( vec![ "simple_value".into() ] )
    .add_row( vec![ "123".into() ] )
    .add_row( vec![ "true".into() ] )
    .build_view();

  let formatter = LogfmtFormatter::new();
  let output = formatter.format( &view ).unwrap();

  println!( "No escaping needed:\n{output}" );

  // Simple values should NOT be quoted
  assert!( output.contains( "key=simple_value" ) );
  assert!( output.contains( "key=123" ) );
  assert!( output.contains( "key=true" ) );
  assert!( !output.contains( "key=\"simple_value\"" ) );
}

// =============================================================================
// Edge Cases
// =============================================================================

#[ test ]
fn test_logfmt_empty_values()
{
  let view = RowBuilder::new( vec![ "name".into(), "status".into() ] )
    .add_row( vec![ "server1".into(), "".into() ] )
    .add_row( vec![ "".into(), "ok".into() ] )
    .build_view();

  let formatter = LogfmtFormatter::new();
  let output = formatter.format( &view ).unwrap();

  println!( "Empty values:\n{output}" );

  // Empty values should be output as key= (no value)
  assert!( output.contains( "name=server1 status=" ) );
  assert!( output.contains( "name= status=ok" ) );
}

#[ test ]
fn test_logfmt_single_row()
{
  let view = RowBuilder::new( vec![ "msg".into() ] )
    .add_row( vec![ "hello".into() ] )
    .build_view();

  let formatter = LogfmtFormatter::new();
  let output = formatter.format( &view ).unwrap();

  println!( "Single row:\n{output}" );

  assert_eq!( output.trim(), "msg=hello" );
}

#[ test ]
fn test_logfmt_unicode_values()
{
  let view = RowBuilder::new( vec![ "name".into(), "emoji".into() ] )
    .add_row( vec![ "测试".into(), "🎉".into() ] )
    .add_row( vec![ "Привет".into(), "🚀".into() ] )
    .build_view();

  let formatter = LogfmtFormatter::new();
  let output = formatter.format( &view ).unwrap();

  println!( "Unicode values:\n{output}" );

  assert!( output.contains( "name=测试" ) );
  assert!( output.contains( "emoji=🎉" ) );
  assert!( output.contains( "name=Привет" ) );
  assert!( output.contains( "emoji=🚀" ) );
}

#[ test ]
fn test_logfmt_special_characters()
{
  let view = RowBuilder::new( vec![ "data".into() ] )
    .add_row( vec![ "key=value".into() ] )  // Equals sign in value
    .add_row( vec![ "a|b|c".into() ] )      // Pipes
    .add_row( vec![ "x@y.com".into() ] )    // Email
    .build_view();

  let formatter = LogfmtFormatter::new();
  let output = formatter.format( &view ).unwrap();

  println!( "Special characters:\n{output}" );

  // These special chars should NOT trigger quoting
  assert!( output.contains( "data=key=value" ) );
  assert!( output.contains( "data=a|b|c" ) );
  assert!( output.contains( "data=x@y.com" ) );
}

// =============================================================================
// Configuration Tests
// =============================================================================

#[ test ]
fn test_logfmt_custom_separator()
{
  let view = RowBuilder::new( vec![ "key".into() ] )
    .add_row( vec![ "value".into() ] )
    .build_view();

  let formatter = LogfmtFormatter::with_separator( ":" );
  let output = formatter.format( &view ).unwrap();

  println!( "Custom separator:\n{output}" );

  assert_eq!( output.trim(), "key:value" );
}

#[ test ]
fn test_logfmt_many_columns()
{
  let view = RowBuilder::new( vec![
    "col1".into(), "col2".into(), "col3".into(),
    "col4".into(), "col5".into(), "col6".into(),
  ])
    .add_row( vec![
      "a".into(), "b".into(), "c".into(),
      "d".into(), "e".into(), "f".into(),
    ])
    .build_view();

  let formatter = LogfmtFormatter::new();
  let output = formatter.format( &view ).unwrap();

  println!( "Many columns:\n{output}" );

  assert!( output.contains( "col1=a" ) );
  assert!( output.contains( "col6=f" ) );

  // Count field separators (should be 5 spaces between 6 fields)
  let space_count = output.trim().matches( ' ' ).count();
  assert_eq!( space_count, 5 );
}

// =============================================================================
// Real-World Scenarios
// =============================================================================

#[ test ]
fn test_logfmt_application_logs()
{
  // Simulate application log entries
  let view = RowBuilder::new( vec![
    "timestamp".into(),
    "level".into(),
    "msg".into(),
    "user_id".into(),
    "duration".into()
  ])
    .add_row( vec![
      "2025-01-15T10:30:00Z".into(),
      "info".into(),
      "user login".into(),
      "12345".into(),
      "0.043".into()
    ])
    .add_row( vec![
      "2025-01-15T10:30:01Z".into(),
      "error".into(),
      "database timeout".into(),
      "67890".into(),
      "5.234".into()
    ])
    .build_view();

  let formatter = LogfmtFormatter::new();
  let output = formatter.format( &view ).unwrap();

  println!( "Application logs:\n{output}" );

  let lines : Vec< &str > = output.lines().collect();
  assert_eq!( lines.len(), 2 );

  // First log line
  assert!( lines[ 0 ].contains( "timestamp=2025-01-15T10:30:00Z" ) );
  assert!( lines[ 0 ].contains( "level=info" ) );
  assert!( lines[ 0 ].contains( "msg=\"user login\"" ) );
  assert!( lines[ 0 ].contains( "user_id=12345" ) );
  assert!( lines[ 0 ].contains( "duration=0.043" ) );

  // Second log line
  assert!( lines[ 1 ].contains( "level=error" ) );
  assert!( lines[ 1 ].contains( "msg=\"database timeout\"" ) );
}

#[ test ]
fn test_logfmt_metric_export()
{
  // Simulate metric export data
  let view = RowBuilder::new( vec![
    "metric".into(),
    "value".into(),
    "host".into(),
    "environment".into()
  ])
    .add_row( vec![ "cpu_usage".into(), "0.75".into(), "server1".into(), "prod".into() ] )
    .add_row( vec![ "memory_usage".into(), "0.82".into(), "server1".into(), "prod".into() ] )
    .add_row( vec![ "disk_usage".into(), "0.45".into(), "server2".into(), "staging".into() ] )
    .build_view();

  let formatter = LogfmtFormatter::new();
  let output = formatter.format( &view ).unwrap();

  println!( "Metric export:\n{output}" );

  assert_eq!( output.lines().count(), 3 );
  assert!( output.contains( "metric=cpu_usage value=0.75 host=server1 environment=prod" ) );
  assert!( output.contains( "metric=memory_usage" ) );
  assert!( output.contains( "metric=disk_usage" ) );
}

#[ test ]
fn test_logfmt_http_requests()
{
  // Simulate HTTP request logs
  let view = RowBuilder::new( vec![
    "method".into(),
    "path".into(),
    "status".into(),
    "duration_ms".into(),
    "user_agent".into()
  ])
    .add_row( vec![
      "GET".into(),
      "/api/users".into(),
      "200".into(),
      "45".into(),
      "Mozilla/5.0".into()
    ])
    .add_row( vec![
      "POST".into(),
      "/api/login".into(),
      "401".into(),
      "12".into(),
      "curl/7.68.0".into()
    ])
    .build_view();

  let formatter = LogfmtFormatter::new();
  let output = formatter.format( &view ).unwrap();

  println!( "HTTP requests:\n{output}" );

  assert!( output.contains( "method=GET path=/api/users status=200" ) );
  assert!( output.contains( "method=POST path=/api/login status=401" ) );
  assert!( output.contains( "user_agent=Mozilla/5.0" ) );
  assert!( output.contains( "user_agent=curl/7.68.0" ) );
}

#[ test ]
fn test_logfmt_visual_verification()
{
  // Create realistic structured log output for visual inspection
  let view = RowBuilder::new( vec![
    "ts".into(),
    "level".into(),
    "service".into(),
    "msg".into(),
    "error".into()
  ])
    .add_row( vec![
      "2025-01-15T10:30:00.123Z".into(),
      "INFO".into(),
      "api".into(),
      "request received".into(),
      "".into()
    ])
    .add_row( vec![
      "2025-01-15T10:30:00.456Z".into(),
      "ERROR".into(),
      "database".into(),
      "connection failed".into(),
      "timeout after 5s".into()
    ])
    .build_view();

  let formatter = LogfmtFormatter::new();
  let output = formatter.format( &view ).unwrap();

  println!( "\n=== Visual Verification of Logfmt Output ===" );
  println!( "{output}" );
  println!( "===========================================\n" );

  // Verify it's parseable (no unescaped special chars breaking format)
  for line in output.lines()
  {
    // Each line should have correct number of = signs (one per field)
    let equals_count = line.matches( '=' ).count();
    assert_eq!( equals_count, 5, "Each line should have 5 fields" );
  }
}
