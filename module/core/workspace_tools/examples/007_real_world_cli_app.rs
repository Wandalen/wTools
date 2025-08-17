//! # 007 - Real-World CLI Application
//!
//! complete example of a cli application using `workspace_tools` for
//! configuration, logging, data storage, and resource management

use workspace_tools::workspace;
use std::{ fs, io::Write };

fn main() -> Result< (), Box< dyn core::error::Error > >
{
  println!( "🔧 real-world cli application example\n" );
  
  // 1. initialize application workspace
  let app = CliApp::new()?;
  app.show_info();
  
  // 2. demonstrate core application functionality
  app.run_demo_commands()?;
  
  // 3. cleanup
  app.cleanup()?;
  
  println!( "\n🎯 this example demonstrates:" );
  println!( "   • workspace-based application structure" );
  println!( "   • configuration management" );
  println!( "   • logging setup" );  
  println!( "   • data persistence" );
  println!( "   • resource discovery and management" );
  println!( "   • error handling and recovery" );
  
  println!( "\n🎯 next: run example 008 to see web service integration" );
  
  Ok( () )
}

struct CliApp
{
  workspace : workspace_tools::Workspace,
  config : AppConfig,
}

#[ derive( Debug ) ]
struct AppConfig
{
  app_name : String,
  log_level : String,
  data_retention_days : u32,
  max_cache_size_mb : u64,
}

impl Default for AppConfig
{
  fn default() -> Self
  {
    Self
    {
      app_name : "demo-cli".to_string(),
      log_level : "info".to_string(),
      data_retention_days : 30,
      max_cache_size_mb : 100,
    }
  }
}

impl CliApp
{
  fn new() -> Result< Self, Box< dyn core::error::Error > >
  {
    println!( "1️⃣  initializing cli application..." );
    
    // setup workspace
    if std::env::var( "WORKSPACE_PATH" ).is_err()
    {
      std::env::set_var( "WORKSPACE_PATH", std::env::current_dir()? );
    }
    
    let workspace = workspace()?;
    
    // ensure directory structure exists
    Self::ensure_directory_structure( &workspace )?;
    
    // load configuration
    let config = Self::load_configuration( &workspace )?;
    
    // setup logging
    Self::setup_logging( &workspace, &config )?;
    
    println!( "   ✅ application initialized successfully" );
    
    Ok( Self { workspace, config } )
  }
  
  fn ensure_directory_structure( ws : &workspace_tools::Workspace ) -> Result< (), Box< dyn core::error::Error > >
  {
    println!( "   📁 ensuring directory structure..." );
    
    let dirs = vec!
    [
      ws.config_dir(),
      ws.data_dir(),
      ws.logs_dir(),
      ws.data_dir().join( "cache" ),
      ws.data_dir().join( "exports" ),
    ];
    
    for dir in dirs
    {
      fs::create_dir_all( &dir )?;
      println!( "     created: {}", dir.display() );
    }
    
    Ok( () )
  }
  
  fn load_configuration( ws : &workspace_tools::Workspace ) -> Result< AppConfig, Box< dyn core::error::Error > >
  {
    println!( "   ⚙️  loading configuration..." );
    
    let config_file = ws.config_dir().join( "app.toml" );
    
    let config = if config_file.exists()
    {
      println!( "     loading from: {}", config_file.display() );
      let content = fs::read_to_string( config_file )?;
      Self::parse_config( &content )
    }
    else
    {
      println!( "     creating default config..." );
      let default_config = AppConfig::default();
      let config_content = Self::config_to_toml( &default_config );
      fs::write( &config_file, config_content )?;
      println!( "     saved default config to: {}", config_file.display() );
      default_config
    };
    
    println!( "     ✅ configuration loaded: {config:?}" );
    Ok( config )
  }
  
  fn setup_logging( ws : &workspace_tools::Workspace, config : &AppConfig ) -> Result< (), Box< dyn core::error::Error > >
  {
    println!( "   📋 setting up logging..." );
    
    let log_file = ws.logs_dir().join( format!( "{}.log", config.app_name ) );
    let error_log = ws.logs_dir().join( "error.log" );
    
    println!( "     log file: {}", log_file.display() );
    println!( "     error log: {}", error_log.display() );
    println!( "     log level: {}", config.log_level );
    
    // simulate log setup (in real app, you'd configure tracing/log4rs/etc.)
    writeln!( fs::File::create( &log_file )?, 
      "[{}] application started with workspace: {}", 
      chrono::Utc::now().format( "%Y-%m-%d %H:%M:%S" ),
      ws.root().display()
    )?;
    
    Ok( () )
  }
  
  fn show_info( &self )
  {
    println!( "\n2️⃣  application information:" );
    println!( "   app name: {}", self.config.app_name );
    println!( "   workspace: {}", self.workspace.root().display() );
    println!( "   config: {}", self.workspace.config_dir().display() );
    println!( "   data: {}", self.workspace.data_dir().display() );
    println!( "   logs: {}", self.workspace.logs_dir().display() );
  }
  
  fn run_demo_commands( &self ) -> Result< (), Box< dyn core::error::Error > >
  {
    println!( "\n3️⃣  running demo commands:" );
    
    // command 1: data processing
    self.process_data()?;
    
    // command 2: cache management
    self.manage_cache()?;
    
    // command 3: export functionality
    self.export_data()?;
    
    // command 4: resource discovery
    #[ cfg( feature = "glob" ) ]
    self.discover_resources();
    
    // command 5: maintenance
    self.run_maintenance()?;
    
    Ok( () )
  }
  
  fn process_data( &self ) -> Result< (), Box< dyn core::error::Error > >
  {
    println!( "   📊 processing data..." );
    
    // simulate data processing
    let input_data = r#"{"users": [
      {"id": 1, "name": "alice", "active": true},
      {"id": 2, "name": "bob", "active": false},  
      {"id": 3, "name": "charlie", "active": true}
    ]}"#;
    
    let input_file = self.workspace.data_dir().join( "input.json" );
    let output_file = self.workspace.data_dir().join( "processed_output.json" );
    
    fs::write( &input_file, input_data )?;
    println!( "     created input: {}", input_file.display() );
    
    // simulate processing (count active users)
    let processed_data = r#"{"active_users": 2, "total_users": 3, "processed_at": "2024-01-01T00:00:00Z"}"#;
    fs::write( &output_file, processed_data )?;
    println!( "     created output: {}", output_file.display() );
    
    // log the operation
    let log_file = self.workspace.logs_dir().join( format!( "{}.log", self.config.app_name ) );
    let mut log = fs::OpenOptions::new().append( true ).open( log_file )?;
    writeln!( log, "[{}] processed {} -> {}", 
      chrono::Utc::now().format( "%H:%M:%S" ),
      input_file.file_name().unwrap().to_string_lossy(),
      output_file.file_name().unwrap().to_string_lossy()
    )?;
    
    Ok( () )
  }
  
  fn manage_cache( &self ) -> Result< (), Box< dyn core::error::Error > >
  {
    println!( "   💾 managing cache..." );
    
    let cache_dir = self.workspace.data_dir().join( "cache" );
    
    // simulate cache operations
    let cache_files = vec!
    [
      ( "api_response_123.json", r#"{"data": "cached api response"}"# ),
      ( "user_profile_456.json", r#"{"user": "cached user data"}"# ),
      ( "query_results_789.json", r#"{"results": "cached query data"}"# ),
    ];
    
    for ( filename, content ) in cache_files
    {
      let cache_file = cache_dir.join( filename );
      fs::write( &cache_file, content )?;
      println!( "     cached: {}", cache_file.display() );
    }
    
    // simulate cache size check
    let cache_size = Self::calculate_directory_size( &cache_dir )?;
    println!( "     cache size: {} bytes (limit: {} MB)", 
      cache_size, self.config.max_cache_size_mb
    );
    
    if cache_size > ( self.config.max_cache_size_mb * 1024 * 1024 )
    {
      println!( "     ⚠️  cache size exceeds limit, cleanup recommended" );
    }
    else
    {
      println!( "     ✅ cache size within limits" );
    }
    
    Ok( () )
  }
  
  fn export_data( &self ) -> Result< (), Box< dyn core::error::Error > >
  {
    println!( "   📤 exporting data..." );
    
    let exports_dir = self.workspace.data_dir().join( "exports" );
    let timestamp = chrono::Utc::now().format( "%Y%m%d_%H%M%S" );
    
    // export configuration
    let config_export = exports_dir.join( format!( "config_export_{timestamp}.toml" ) );
    let config_content = Self::config_to_toml( &self.config );
    fs::write( &config_export, config_content )?;
    println!( "     exported config: {}", config_export.display() );
    
    // export data summary
    let data_export = exports_dir.join( format!( "data_summary_{timestamp}.json" ) );
    let summary = format!( r#"{{
  "export_timestamp": "{}",
  "workspace_root": "{}",
  "files_processed": 3,
  "cache_entries": 3,
  "log_entries": 2
}}"#, 
      chrono::Utc::now().to_rfc3339(),
      self.workspace.root().display()
    );
    fs::write( &data_export, summary )?;
    println!( "     exported summary: {}", data_export.display() );
    
    Ok( () )
  }
  
  #[ cfg( feature = "glob" ) ]
  fn discover_resources( &self )
  {
    println!( "   🔍 discovering resources..." );
    
    let patterns = vec!
    [
      ( "**/*.json", "json files" ),
      ( "**/*.toml", "toml files" ),
      ( "**/*.log", "log files" ),
      ( "data/**/*", "data files" ),
    ];
    
    for ( pattern, description ) in patterns
    {
      match self.workspace.find_resources( pattern )
      {
        Ok( files ) => 
        {
          println!( "     {}: {} files", description, files.len() );
          for file in files.iter().take( 3 ) // show first 3
          {
            println!( "       - {}", file.file_name().unwrap().to_string_lossy() );
          }
          if files.len() > 3
          {
            println!( "       ... and {} more", files.len() - 3 );
          }
        }
        Err( e ) => println!( "     {description}: error - {e}" ),
      }
    }
  }
  
  fn run_maintenance( &self ) -> Result< (), Box< dyn core::error::Error > >
  {
    println!( "   🧹 running maintenance..." );
    
    // check workspace health
    match self.workspace.validate()
    {
      Ok( () ) => println!( "     ✅ workspace structure is healthy" ),
      Err( e ) => println!( "     ⚠️  workspace issue: {e}" ),
    }
    
    // check disk usage
    let data_size = Self::calculate_directory_size( &self.workspace.data_dir() )?;
    let log_size = Self::calculate_directory_size( &self.workspace.logs_dir() )?;
    
    println!( "     data directory: {data_size} bytes" );
    println!( "     logs directory: {log_size} bytes" );
    
    // simulate old file cleanup based on retention policy
    let retention_days = self.config.data_retention_days;
    println!( "     retention policy: {retention_days} days" );
    println!( "     (in production: would clean files older than {retention_days} days)" );
    
    Ok( () )
  }
  
  fn cleanup( &self ) -> Result< (), Box< dyn core::error::Error > >
  {
    println!( "\n4️⃣  cleaning up demo files..." );
    
    let demo_dirs = vec![ "data", "logs" ];
    for dir_name in demo_dirs
    {
      let dir_path = self.workspace.join( dir_name );
      if dir_path.exists()
      {
        fs::remove_dir_all( &dir_path )?;
        println!( "   removed: {}", dir_path.display() );
      }
    }
    
    let config_file = self.workspace.config_dir().join( "app.toml" );
    if config_file.exists()
    {
      fs::remove_file( &config_file )?;
      println!( "   removed: {}", config_file.display() );
    }
    
    println!( "   ✅ cleanup completed" );
    
    Ok( () )
  }
  
  // utility methods
  
  fn parse_config( content : &str ) -> AppConfig
  {
    // simple toml-like parsing for demo (in real app, use toml crate)
    let mut config = AppConfig::default();
    
    for line in content.lines()
    {
      if let Some( ( key, value ) ) = line.split_once( " = " )
      {
        let key = key.trim();
        let value = value.trim().trim_matches( '"' );
        
        match key
        {
          "app_name" => config.app_name = value.to_string(),
          "log_level" => config.log_level = value.to_string(),
          "data_retention_days" => config.data_retention_days = value.parse().unwrap_or( 30 ),
          "max_cache_size_mb" => config.max_cache_size_mb = value.parse().unwrap_or( 100 ),
          _ => {}
        }
      }
    }
    
    config
  }
  
  fn config_to_toml( config : &AppConfig ) -> String
  {
    format!( r#"# CLI Application Configuration
app_name = "{}"
log_level = "{}"
data_retention_days = {}
max_cache_size_mb = {}
"#, 
      config.app_name, config.log_level, config.data_retention_days, config.max_cache_size_mb
    )
  }
  
  fn calculate_directory_size( dir : &std::path::Path ) -> Result< u64, Box< dyn core::error::Error > >
  {
    let mut total_size = 0;
    
    if dir.exists()
    {
      for entry in fs::read_dir( dir )?
      {
        let entry = entry?;
        let metadata = entry.metadata()?;
        
        if metadata.is_file()
        {
          total_size += metadata.len();
        }
        else if metadata.is_dir()
        {
          total_size += Self::calculate_directory_size( &entry.path() )?;
        }
      }
    }
    
    Ok( total_size )
  }
}

// add chrono for timestamps
mod chrono
{
  pub struct Utc;
  
  impl Utc
  {
    pub fn now() -> DateTime
    {
      DateTime
    }
  }
  
  pub struct DateTime;
  
  impl DateTime
  {
    #[allow(clippy::unused_self)]
    pub fn format( &self, _fmt : &str ) -> impl core::fmt::Display
    {
      "2024-01-01 12:00:00"
    }
    
    #[allow(clippy::unused_self)]
    pub fn to_rfc3339( &self ) -> String
    {
      "2024-01-01T12:00:00Z".to_string()
    }
  }
}