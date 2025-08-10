//! # 004 - Resource Discovery (glob feature)
//!
//! find files and directories using powerful glob patterns
//! this example requires the "glob" feature to be enabled

#[ cfg( feature = "glob" ) ]
fn main() -> Result< (), workspace_tools::WorkspaceError >
{
  println!( "🔍 workspace resource discovery with glob patterns\n" );
  
  // setup workspace
  if std::env::var( "WORKSPACE_PATH" ).is_err()
  {
    std::env::set_var( "WORKSPACE_PATH", std::env::current_dir().unwrap() );
  }

  let ws = workspace_tools::workspace()?;
  
  // create a demo project structure for discovery
  setup_demo_structure( &ws )?;
  
  println!( "📁 created demo project structure" );
  println!( "workspace: {}\n", ws.root().display() );
  
  // 1. find rust source files
  println!( "1️⃣  finding rust source files:" );
  let rust_files = ws.find_resources( "src/**/*.rs" )?;
  print_files( &rust_files, "   " );
  
  // 2. find all test files 
  println!( "\n2️⃣  finding test files:" );
  let test_files = ws.find_resources( "tests/**/*.rs" )?;
  print_files( &test_files, "   " );
  
  // 3. find configuration files
  println!( "\n3️⃣  finding configuration files:" );
  let config_files = ws.find_resources( "config/*" )?;
  print_files( &config_files, "   " );
  
  // 4. find documentation
  println!( "\n4️⃣  finding documentation:" );
  let doc_files = ws.find_resources( "docs/**/*.md" )?;
  print_files( &doc_files, "   " );
  
  // 5. find assets by type
  println!( "\n5️⃣  finding image assets:" );
  let image_files = ws.find_resources( "assets/**/*.{png,jpg,svg}" )?;
  print_files( &image_files, "   " );
  
  // 6. smart configuration discovery
  println!( "\n6️⃣  smart config file discovery:" );
  
  let configs = vec![ "app", "database", "logging", "nonexistent" ];
  for config_name in configs
  {
    match ws.find_config( config_name )
    {
      Ok( config_path ) => 
        println!( "   {} config: {}", config_name, config_path.display() ),
      Err( _ ) => 
        println!( "   {config_name} config: not found" ),
    }
  }
  
  // 7. advanced glob patterns
  println!( "\n7️⃣  advanced glob patterns:" );
  
  let patterns = vec!
  [
    ( "**/*.toml", "all toml files recursively" ),
    ( "src/**/mod.rs", "module files in src" ),  
    ( "**/test_*.rs", "test files anywhere" ),
    ( "assets/**", "all assets recursively" ),
    ( "config/*.{yml,yaml}", "yaml configs only" ),
  ];
  
  for ( pattern, description ) in patterns
  {
    match ws.find_resources( pattern )
    {
      Ok( files ) => println!( "   {}: {} files", description, files.len() ),
      Err( e ) => println!( "   {description}: error - {e}" ),
    }
  }
  
  // 8. filtering results
  println!( "\n8️⃣  filtering and processing results:" );
  let all_rust_files = ws.find_resources( "**/*.rs" )?;
  
  // filter by directory
  let src_files: Vec< _ > = all_rust_files.iter()
    .filter( | path | path.to_string_lossy().contains( "/src/" ) )
    .collect();
    
  let test_files: Vec< _ > = all_rust_files.iter()
    .filter( | path | path.to_string_lossy().contains( "/tests/" ) )
    .collect();
    
  println!( "   total rust files: {}", all_rust_files.len() );
  println!( "   source files: {}", src_files.len() );
  println!( "   test files: {}", test_files.len() );
  
  // cleanup demo structure
  cleanup_demo_structure( &ws );
  
  println!( "\n💡 resource discovery best practices:" );
  println!( "   • use specific patterns to avoid finding too many files" );
  println!( "   • prefer find_config() for configuration discovery" );
  println!( "   • handle glob errors gracefully (invalid patterns)" );
  println!( "   • filter results in rust rather than complex glob patterns" );
  println!( "   • cache results if you'll reuse them frequently" );
  
  println!( "\n🎯 next: run example 005 to learn about secret management" );
  
  Ok( () )
}

#[ cfg( feature = "glob" ) ]
fn setup_demo_structure( ws : &workspace_tools::Workspace ) -> Result< (), workspace_tools::WorkspaceError >
{
  use std::fs;
  
  // create directory structure
  let dirs = vec!
  [
    "src/modules",
    "src/utils", 
    "tests/integration",
    "tests/unit",
    "config",
    "docs/api",
    "docs/guides", 
    "assets/images",
    "assets/fonts",
  ];
  
  for dir in dirs
  {
    let path = ws.join( dir );
    fs::create_dir_all( &path )
      .map_err( | e | workspace_tools::WorkspaceError::IoError( e.to_string() ) )?;
  }
  
  // create demo files
  let files = vec!
  [
    // rust source files
    ( "src/lib.rs", "//! main library\npub mod utils;" ),
    ( "src/main.rs", "fn main() { println!(\"hello\"); }" ),
    ( "src/modules/auth.rs", "// authentication module" ),
    ( "src/modules/mod.rs", "pub mod auth;" ),
    ( "src/utils/helpers.rs", "// helper functions" ),
    ( "src/utils/mod.rs", "pub mod helpers;" ),
    
    // test files
    ( "tests/integration/test_auth.rs", "#[test] fn test_auth() {}" ),
    ( "tests/unit/test_helpers.rs", "#[test] fn test_helpers() {}" ),
    
    // config files
    ( "config/app.toml", "[app]\nname = \"demo\"\nport = 8080" ),
    ( "config/database.yaml", "host: localhost\nport: 5432" ),
    ( "config/logging.yml", "level: info" ),
    
    // documentation
    ( "docs/readme.md", "# project documentation" ),
    ( "docs/api/auth.md", "# authentication api" ),
    ( "docs/guides/setup.md", "# setup guide" ),
    
    // assets
    ( "assets/images/logo.png", "fake png data" ),
    ( "assets/images/icon.svg", "<svg>icon</svg>" ),
    ( "assets/fonts/main.ttf", "fake font data" ),
  ];
  
  for ( path, content ) in files
  {
    let file_path = ws.join( path );
    fs::write( &file_path, content )
      .map_err( | e | workspace_tools::WorkspaceError::IoError( e.to_string() ) )?;
  }
  
  Ok( () )
}

#[ cfg( feature = "glob" ) ]
fn cleanup_demo_structure( ws : &workspace_tools::Workspace )
{
  use std::fs;
  
  let dirs = vec![ "src", "tests", "config", "docs", "assets" ];
  
  for dir in dirs
  {
    let path = ws.join( dir );
    let _ = fs::remove_dir_all( path ); // ignore errors during cleanup
  }
}

#[ cfg( feature = "glob" ) ]
fn print_files( files : &[ std::path::PathBuf ], indent : &str )
{
  if files.is_empty()
  {
    println!( "{indent}(no files found)" );
  }
  else
  {
    for file in files
    {
      println!( "{}{}", indent, file.display() );
    }
  }
}

#[ cfg( not( feature = "glob" ) ) ]
fn main()
{
  println!( "🚨 this example requires the 'glob' feature" );
  println!( "run with: cargo run --example 004_resource_discovery --features glob" );
  println!();
  println!( "to enable glob feature permanently, add to cargo.toml:" );
  println!( r#"[dependencies]"# );
  println!( r#"workspace_tools = { version = "0.1", features = ["glob"] }"# );
}