//! Parse a real Claude Code session to validate format compatibility

use std::path::PathBuf;
use claude_storage_core::Session;

fn main() -> Result< (), Box< dyn core::error::Error > >
{
  // Direct session file path
  let session_path = PathBuf::from( "/home/user1/.claude/projects/-home-user1-pro-lib-willbe-module-claude-storage--default-topic/bc14c4bf-eb06-406f-82ed-4349dd1f93a3.jsonl" );

  println!( "=== Real Session Parse Test ===" );
  println!( "Session file: {}", session_path.display() );

  let mut session = Session::load( &session_path )?;

  println!( "Session ID: {}", session.id() );

  let stats = session.stats()?;

  println!( "\nSession Statistics:" );
  println!( "  Total entries: {}", stats.total_entries );
  println!( "  User entries: {}", stats.user_entries );
  println!( "  Assistant entries: {}", stats.assistant_entries );
  println!( "  Total input tokens: {}", stats.total_input_tokens );
  println!( "  Total output tokens: {}", stats.total_output_tokens );

  if let Some( first ) = &stats.first_timestamp
  {
    println!( "  First message: {first}" );
  }

  if let Some( last ) = &stats.last_timestamp
  {
    println!( "  Last message: {last}" );
  }

  println!( "\n✅ Successfully parsed real Claude Code v2.0.31 session!" );
  println!( "✅ Graceful skip working (loaded {} conversation entries)", stats.total_entries );
  println!( "✅ Parser handled {} user + {} assistant entries", stats.user_entries, stats.assistant_entries );

  Ok( () )
}
