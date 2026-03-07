//! Token usage analyzer for Claude Code storage
//!
//! Analyzes token consumption across all projects and sessions.
//!
//! # Usage
//!
//! ```bash
//! cargo run --example token_usage
//! cargo run --example token_usage -- --project abc123
//! cargo run --example token_usage -- --top 10
//! ```

use claude_storage::{ Storage, Result };
use std::env;

fn main() -> Result< () >
{
  let args : Vec< String > = env::args().collect();

  let storage = Storage::new()?;

  // Parse arguments
  let project_filter = args.iter()
    .position( | a | a == "--project" )
    .and_then( | i | args.get( i + 1 ) )
    .map( std::string::String::as_str );

  let top_n = args.iter()
    .position( | a | a == "--top" )
    .and_then( | i | args.get( i + 1 ) )
    .and_then( | s | s.parse::< usize >().ok() )
    .unwrap_or( usize::MAX );

  if let Some( project_id ) = project_filter
  {
    // Analyze specific project
    analyze_project( &storage, project_id )?;
  }
  else
  {
    // Analyze all projects
    analyze_global( &storage, top_n )?;
  }

  Ok( () )
}

fn analyze_global( storage : &Storage, top_n : usize ) -> Result< () >
{
  println!( "Claude Code Token Usage Analysis" );
  println!( "=================================\n" );

  let stats = match storage.global_stats()
  {
    Ok( s ) => s,
    Err( e ) =>
    {
      eprintln!( "Error: Could not analyze storage: {e}" );
      eprintln!();
      eprintln!( "This may indicate:" );
      eprintln!( "  - Corrupted or empty JSONL files in ~/.claude/" );
      eprintln!( "  - Invalid JSON in conversation files" );
      eprintln!( "  - Incompatible format version" );
      eprintln!();
      eprintln!( "Try running with a specific project ID to isolate the issue:" );
      eprintln!( "  cargo run --example token_usage -- --project <project_id>" );
      return Err( e );
    }
  };

  // Global summary
  println!( "Total Projects: {}", stats.total_projects );
  println!( "  UUID Projects: {}", stats.uuid_projects );
  println!( "  Path Projects: {}", stats.path_projects );
  println!();

  println!( "Total Sessions: {}", stats.total_sessions );
  println!( "  Main Sessions: {}", stats.main_sessions );
  println!( "  Agent Sessions: {}", stats.agent_sessions );
  println!();

  println!( "Total Entries: {}", stats.total_entries );
  println!( "  User: {}", stats.total_user_entries );
  println!( "  Assistant: {}", stats.total_assistant_entries );
  println!();

  println!( "Token Usage:" );
  println!( "  Input: {}", format_tokens( stats.total_input_tokens ) );
  println!( "  Output: {}", format_tokens( stats.total_output_tokens ) );
  println!( "  Cache Read: {}", format_tokens( stats.total_cache_read_tokens ) );
  println!( "  Cache Creation: {}", format_tokens( stats.total_cache_creation_tokens ) );
  println!();

  let total_tokens = stats.total_input_tokens + stats.total_output_tokens;
  println!( "Total Tokens: {}", format_tokens( total_tokens ) );
  println!();

  // Top projects by token usage
  let mut projects : Vec< _ > = stats.project_breakdown.values().collect();
  projects.sort_by_key( | p | core::cmp::Reverse( p.total_input_tokens + p.total_output_tokens ) );

  println!( "Top {} Projects by Token Usage:", top_n.min( projects.len() ) );
  println!( "{:<40} {:>12} {:>12} {:>12}", "Project ID", "Sessions", "Input", "Output" );
  println!( "{}", "-".repeat( 80 ) );

  for project in projects.iter().take( top_n )
  {
    println!( "{:<40} {:>12} {:>12} {:>12}",
      truncate_str( &project.project_id, 40 ),
      project.session_count,
      format_tokens( project.total_input_tokens ),
      format_tokens( project.total_output_tokens )
    );
  }

  Ok( () )
}

fn analyze_project( storage : &Storage, project_id : &str ) -> Result< () >
{
  println!( "Project Token Analysis: {project_id}" );
  println!( "=================================\n" );

  let projects = storage.list_projects()?;
  let project = projects.iter()
    .find( | p |
    {
      use claude_storage::ProjectId;
      match p.id()
      {
        ProjectId::Uuid( uuid ) => uuid.contains( project_id ),
        ProjectId::Path( path ) => path.to_string_lossy().contains( project_id ),
      }
    })
    .ok_or_else( || std::io::Error::new( std::io::ErrorKind::NotFound, "Project not found" ) )?;

  let mut all_sessions = project.all_sessions()?;

  println!( "Total Sessions: {}", all_sessions.len() );

  // Calculate per-session statistics
  let mut session_stats_list = Vec::new();
  for session in &mut all_sessions
  {
    let stats = session.stats()?;
    session_stats_list.push( stats );
  }

  // Sort by total tokens
  session_stats_list.sort_by_key( | s |
    core::cmp::Reverse( s.total_input_tokens + s.total_output_tokens )
  );

  println!();
  println!( "{:<20} {:>10} {:>12} {:>12} {:>8}",
    "Session ID", "Entries", "Input", "Output", "Type" );
  println!( "{}", "-".repeat( 70 ) );

  for stats in &session_stats_list
  {
    let session_type = if stats.is_agent_session { "Agent" } else { "Main" };

    println!( "{:<20} {:>10} {:>12} {:>12} {:>8}",
      truncate_str( &stats.session_id, 20 ),
      stats.total_entries,
      format_tokens( stats.total_input_tokens ),
      format_tokens( stats.total_output_tokens ),
      session_type
    );
  }

  // Totals
  println!( "{}", "-".repeat( 70 ) );
  let total_input : u64 = session_stats_list.iter().map( | s | s.total_input_tokens ).sum();
  let total_output : u64 = session_stats_list.iter().map( | s | s.total_output_tokens ).sum();
  let total_entries : usize = session_stats_list.iter().map( | s | s.total_entries ).sum();

  println!( "{:<20} {:>10} {:>12} {:>12}",
    "TOTAL",
    total_entries,
    format_tokens( total_input ),
    format_tokens( total_output )
  );

  Ok( () )
}

/// Format token count with thousands separators
fn format_tokens( tokens : u64 ) -> String
{
  let s = tokens.to_string();
  let mut result = String::new();

  for ( count, c ) in s.chars().rev().enumerate()
  {
    if count > 0 && count % 3 == 0
    {
      result.insert( 0, ',' );
    }
    result.insert( 0, c );
  }

  result
}

/// Truncate string to maximum length
fn truncate_str( s : &str, max_len : usize ) -> String
{
  if s.len() <= max_len
  {
    s.to_string()
  }
  else
  {
    format!( "{}...", &s[ ..max_len - 3 ] )
  }
}
