//! # 001 - Standard Directory Layout
//!
//! `workspace_tools` promotes a consistent directory structure
//! this example shows the standard directories and their intended uses

use workspace_tools :: { workspace, WorkspaceError };

fn main() -> Result< (), WorkspaceError >
{
  // setup workspace for demo
  if std ::env ::var( "WORKSPACE_PATH" ).is_err()
  {
  std ::env ::set_var( "WORKSPACE_PATH", std ::env ::current_dir().unwrap() );
 }

  let ws = workspace()?;
  
  println!( "🏗️  standard directory layout for: {}", ws.root().display() );
  println!();
  
  // configuration files - app settings, service configs, etc.
  let config_dir = ws.config_dir();
  println!( "⚙️  config: {} ", config_dir.display() );
  println!( "   └── app.toml, database.yaml, services.json" );
  
  // application data - databases, caches, user data
  let data_dir = ws.data_dir();
  println!( "💾 data: {}", data_dir.display() );
  println!( "   └── cache.db, state.json, user_data/" );
  
  // log files - application logs, debug output  
  let logs_dir = ws.logs_dir();
  println!( "📋 logs: {}", logs_dir.display() );
  println!( "   └── app.log, error.log, access.log" );
  
  // documentation - readme, guides, api docs
  let docs_dir = ws.docs_dir();
  println!( "📚 docs: {}", docs_dir.display() );
  println!( "   └── readme.md, api/, guides/" );
  
  // test resources - test data, fixtures, mock files
  let tests_dir = ws.tests_dir();
  println!( "🧪 tests: {}", tests_dir.display() );
  println!( "   └── fixtures/, test_data.json" );
  
  // workspace metadata - internal workspace state
  let workspace_dir = ws.workspace_dir();
  println!( "🗃️  meta: {}", workspace_dir.display() );
  println!( "   └── .workspace metadata" );
  
  println!();
  println!( "💡 benefits of standard layout: " );
  println!( "   • predictable file locations across projects" );
  println!( "   • easy deployment and packaging" );
  println!( "   • consistent backup and maintenance" );
  println!( "   • team collaboration without confusion" );
  
  println!( "\n🎯 next: run example 002 to learn path operations" );
  
  Ok( () )
}