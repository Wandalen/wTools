//! List all Claude Code projects
//!
//! A simple example demonstrating basic storage access.
//!
//! # Usage
//!
//! ```bash
//! cargo run --example list_projects
//! ```

use claude_storage::{ Storage, ProjectId, Result };

fn main() -> Result< () >
{
  println!( "Claude Code Projects" );
  println!( "===================\n" );

  let storage = Storage::new()?;
  let projects = storage.list_projects()?;

  println!( "Found {} projects:", projects.len() );
  println!();

  for project in projects
  {
    match project.id()
    {
      ProjectId::Uuid( uuid ) =>
      {
        println!( "UUID Project: {uuid}" );
      },
      ProjectId::Path( path ) =>
      {
        println!( "Path Project: {}", path.display() );
      },
    }

    // Try to list sessions (may fail for corrupted projects)
    match project.sessions()
    {
      Ok( sessions ) =>
      {
        println!( "  Sessions: {}", sessions.len() );

        for session in sessions.iter().take( 5 )
        {
          println!( "    - {}", session.id() );
        }

        if sessions.len() > 5
        {
          println!( "    ... and {} more", sessions.len() - 5 );
        }
      },
      Err( e ) =>
      {
        println!( "  Sessions: Error reading ({e})" );
      }
    }

    println!();
  }

  Ok( () )
}
