//! Example: Structured logging output with `LogfmtFormatter`
//!
//! Demonstrates logfmt format for application logs, suitable for:
//! - Observability tool ingestion (Prometheus, Loki, Elasticsearch)
//! - Grep-friendly log analysis
//! - Machine-parseable structured logging
//!
//! Run with: `cargo run --example logfmt_format`

use tree_fmt::{ RowBuilder, LogfmtFormatter, Format };

#[ allow( clippy::too_many_lines ) ]
fn main()
{
  println!( "=== Logfmt Structured Logging Example ===\n" );

  // Example 1: Application logs with timestamps and levels
  println!( "1. Application Logs:" );
  let logs = RowBuilder::new( vec!
  [
    "timestamp".into(),
    "level".into(),
    "service".into(),
    "msg".into(),
    "user_id".into(),
    "duration_ms".into(),
  ])
    .add_row( vec!
    [
      "2025-01-15T10:30:00Z".into(),
      "info".into(),
      "api".into(),
      "user login successful".into(),
      "12345".into(),
      "43".into(),
    ])
    .add_row( vec!
    [
      "2025-01-15T10:30:01Z".into(),
      "warn".into(),
      "api".into(),
      "rate limit approaching".into(),
      "12345".into(),
      "2".into(),
    ])
    .add_row( vec!
    [
      "2025-01-15T10:30:05Z".into(),
      "error".into(),
      "database".into(),
      "connection timeout".into(),
      "67890".into(),
      "5234".into(),
    ])
    .build_view();

  let formatter = LogfmtFormatter::new();
  let output = formatter.format( &logs ).unwrap();
  println!( "{output}" );

  // Example 2: HTTP request logs
  println!( "\n2. HTTP Request Logs:" );
  let requests = RowBuilder::new( vec!
  [
    "method".into(),
    "path".into(),
    "status".into(),
    "duration_ms".into(),
    "remote_ip".into(),
  ])
    .add_row( vec![
      "GET".into(),
      "/api/users".into(),
      "200".into(),
      "45".into(),
      "192.168.1.100".into(),
    ])
    .add_row( vec![
      "POST".into(),
      "/api/auth/login".into(),
      "401".into(),
      "12".into(),
      "10.0.0.50".into(),
    ])
    .add_row( vec![
      "DELETE".into(),
      "/api/users/123".into(),
      "204".into(),
      "89".into(),
      "192.168.1.100".into(),
    ])
    .build_view();

  let output = formatter.format( &requests ).unwrap();
  println!( "{output}" );

  // Example 3: System metrics
  println!( "\n3. System Metrics:" );
  let metrics = RowBuilder::new( vec![
    "metric".into(),
    "value".into(),
    "unit".into(),
    "host".into(),
    "environment".into(),
  ])
    .add_row( vec![
      "cpu_usage".into(),
      "0.75".into(),
      "percent".into(),
      "server1".into(),
      "production".into(),
    ])
    .add_row( vec![
      "memory_used".into(),
      "8.2".into(),
      "GB".into(),
      "server1".into(),
      "production".into(),
    ])
    .add_row( vec![
      "disk_io_read".into(),
      "125.5".into(),
      "MB/s".into(),
      "server2".into(),
      "staging".into(),
    ])
    .build_view();

  let output = formatter.format( &metrics ).unwrap();
  println!( "{output}" );

  // Example 4: Error logs with stack traces (multiline values)
  println!( "\n4. Error Logs with Escaped Newlines:" );
  let errors = RowBuilder::new( vec![
    "timestamp".into(),
    "level".into(),
    "error_type".into(),
    "message".into(),
    "stack_trace".into(),
  ])
    .add_row( vec![
      "2025-01-15T10:35:00Z".into(),
      "error".into(),
      "DatabaseError".into(),
      "failed to connect".into(),
      "at db.connect(db.js:45)\nat retry(utils.js:12)".into(),
    ])
    .build_view();

  let output = formatter.format( &errors ).unwrap();
  println!( "{output}" );

  // Example 5: Custom separator (colon instead of equals)
  println!( "\n5. Custom Separator (using colon):" );
  let custom = RowBuilder::new( vec![ "name".into(), "status".into() ] )
    .add_row( vec![ "service-a".into(), "healthy".into() ] )
    .add_row( vec![ "service-b".into(), "degraded".into() ] )
    .build_view();

  let custom_formatter = LogfmtFormatter::with_separator( ":" );
  let output = custom_formatter.format( &custom ).unwrap();
  println!( "{output}" );

  println!( "\n=== Grep Examples ===" );
  println!( "Try these commands to parse the logfmt output:" );
  println!( "  grep 'level=error' logs.txt" );
  println!( "  awk '{{for(i=1;i<=NF;i++) if($i~/^duration_ms=/) print $i}}' logs.txt" );
  println!( "  sed -n '/msg=.*timeout/p' logs.txt" );
}
