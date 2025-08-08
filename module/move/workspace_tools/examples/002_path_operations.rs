//! # 002 - Path Operations
//!
//! essential path operations for workspace-relative file access
//! this example demonstrates joining, validation, and boundary checking

use workspace_tools::{ workspace, WorkspaceError };

fn main() -> Result< (), WorkspaceError >
{
  // setup workspace
  if std::env::var( "WORKSPACE_PATH" ).is_err()
  {
    std::env::set_var( "WORKSPACE_PATH", std::env::current_dir().unwrap() );
  }

  let ws = workspace()?;
  
  println!( "🛠️  workspace path operations" );
  println!( "workspace root: {}\n", ws.root().display() );
  
  // 1. path joining - the most common operation
  println!( "1️⃣  path joining:" );
  let config_file = ws.join( "config/app.toml" );
  let data_file = ws.join( "data/cache.db" );  
  let nested_path = ws.join( "data/user/profile.json" );
  
  println!( "   config file: {}", config_file.display() );
  println!( "   data file:   {}", data_file.display() );
  println!( "   nested path: {}", nested_path.display() );
  
  // 2. boundary checking - ensure paths are within workspace
  println!( "\n2️⃣  boundary checking:" );
  println!( "   config in workspace: {}", ws.is_workspace_file( &config_file ) );
  println!( "   data in workspace:   {}", ws.is_workspace_file( &data_file ) );
  println!( "   /tmp in workspace:   {}", ws.is_workspace_file( "/tmp/outside" ) );
  println!( "   /etc in workspace:   {}", ws.is_workspace_file( "/etc/passwd" ) );
  
  // 3. convenient standard directory access
  println!( "\n3️⃣  standard directory shortcuts:" );
  let log_file = ws.logs_dir().join( "application.log" );
  let test_fixture = ws.tests_dir().join( "fixtures/sample.json" );
  
  println!( "   log file:     {}", log_file.display() );
  println!( "   test fixture: {}", test_fixture.display() );
  
  // 4. workspace validation
  println!( "\n4️⃣  workspace validation:" );
  match ws.validate()
  {
    Ok( () ) => println!( "   ✅ workspace structure is valid and accessible" ),
    Err( e ) => println!( "   ❌ workspace validation failed: {e}" ),
  }
  
  // 5. path normalization (resolves .., symlinks, etc.)
  println!( "\n5️⃣  path normalization:" );
  let messy_path = "config/../data/./cache.db";
  println!( "   messy path: {messy_path}" );
  
  match ws.normalize_path( messy_path )
  {
    Ok( normalized ) => println!( "   normalized: {}", normalized.display() ),
    Err( e ) => println!( "   normalization failed: {e}" ),
  }
  
  println!( "\n💡 key principles:" );
  println!( "   • always use ws.join() instead of manual path construction" );
  println!( "   • check boundaries with is_workspace_file() for security" );
  println!( "   • use standard directories for predictable layouts" );
  println!( "   • validate workspace in production applications" );
  
  println!( "\n🎯 next: run example 003 to learn about error handling" );
  
  Ok( () )
}