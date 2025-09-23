//! basic usage example for `workspace_tools`
//!
//! this example demonstrates the core functionality of workspace path resolution

use workspace_tools :: { workspace, WorkspaceError };

fn main() -> Result< (), WorkspaceError >
{
  // ensure we have a workspace path set
  if std ::env ::var( "WORKSPACE_PATH" ).is_err()
  {
  println!( "setting WORKSPACE_PATH to current directory for demo" );
  std ::env ::set_var( "WORKSPACE_PATH", std ::env ::current_dir().unwrap() );
 }

  // get workspace instance
  println!( "resolving workspace..." );
  let ws = workspace()?;
  
  println!( "workspace root: {}", ws.root().display() );
  
  // demonstrate standard directory access
  println!( "\nstandard directories: " );
  println!( "  config: {}", ws.config_dir().display() );
  println!( "  data: {}", ws.data_dir().display() );
  println!( "  logs: {}", ws.logs_dir().display() );
  println!( "  docs: {}", ws.docs_dir().display() );
  println!( "  tests: {}", ws.tests_dir().display() );
  
  // demonstrate path joining
  println!( "\npath joining examples: " );
  let app_config = ws.join( "config/app.toml" );
  let cache_file = ws.join( "data/cache.db" );
  let log_file = ws.join( "logs/application.log" );
  
  println!( "  app config: {}", app_config.display() );
  println!( "  cache file: {}", cache_file.display() );
  println!( "  log file: {}", log_file.display() );
  
  // demonstrate workspace boundary checking
  println!( "\nworkspace boundary checking: " );
  println!( "  app_config in workspace: {}", ws.is_workspace_file( &app_config ) );
  println!( "  /etc/passwd in workspace: {}", ws.is_workspace_file( "/etc/passwd" ) );
  
  // validate workspace
  println!( "\nvalidating workspace..." );
  match ws.validate()
  {
  Ok( () ) => println!( "  workspace structure is valid" ),
  Err( e ) => println!( "  workspace validation failed: {e}" ),
 }
  
  Ok( () )
}