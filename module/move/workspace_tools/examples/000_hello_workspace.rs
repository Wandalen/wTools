//! # 000 - Hello Workspace
//!
//! the most basic introduction to workspace_tools
//! this example shows the fundamental concept of workspace resolution

use workspace_tools::{ workspace, WorkspaceError };

fn main() -> Result< (), WorkspaceError >
{
  // workspace_tools works by reading the WORKSPACE_PATH environment variable
  // if it's not set, we'll set it to current directory for this demo
  if std::env::var( "WORKSPACE_PATH" ).is_err()
  {
    let current_dir = std::env::current_dir().unwrap();
    std::env::set_var( "WORKSPACE_PATH", &current_dir );
    println!( "📍 set WORKSPACE_PATH to: {}", current_dir.display() );
  }

  // the fundamental operation: get a workspace instance
  println!( "🔍 resolving workspace..." );
  let ws = workspace()?;
  
  // every workspace has a root directory
  println!( "✅ workspace root: {}", ws.root().display() );
  
  // that's it! you now have reliable, workspace-relative path resolution
  // no more brittle "../../../config/file.toml" paths
  
  println!( "\n🎉 workspace resolution successful!" );
  println!( "next: run example 001 to learn about standard directories" );
  
  Ok( () )
}