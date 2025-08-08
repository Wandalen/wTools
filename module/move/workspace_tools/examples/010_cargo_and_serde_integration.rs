//! Cargo Integration and Serde Integration Example
//!
//! This example demonstrates the new cargo integration and serde integration features:
//! 1. Automatic cargo workspace detection
//! 2. Configuration loading with automatic format detection
//! 3. Configuration saving and updating
//! 4. Layered configuration management
//!
//! Run with: cargo run --example 010_cargo_and_serde_integration --features full

use workspace_tools::{ Workspace, WorkspaceError };

#[ cfg( feature = "serde_integration" ) ]
use serde::{ Deserialize, Serialize };
#[ cfg( feature = "serde_integration" ) ]
use workspace_tools::ConfigMerge;

#[ cfg( feature = "serde_integration" ) ]
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
struct AppConfig
{
  name : String,
  version : String,
  port : u16,
  debug : bool,
  database : DatabaseConfig,
  features : Vec< String >,
}

#[ cfg( feature = "serde_integration" ) ]
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
struct DatabaseConfig
{
  host : String,
  port : u16,
  name : String,
  ssl : bool,
}

#[ cfg( feature = "serde_integration" ) ]
impl ConfigMerge for AppConfig
{
  fn merge( mut self, other : Self ) -> Self
  {
    // merge strategy: other config overrides self
    self.name = other.name;
    self.version = other.version;
    self.port = other.port;
    self.debug = other.debug;
    self.database = other.database;
    
    // combine features from both configs
    self.features.extend( other.features );
    self.features.sort();
    self.features.dedup();
    
    self
  }
}

fn main() -> Result< (), Box< dyn std::error::Error > >
{
  println!( "🚀 Cargo Integration and Serde Integration Demo\n" );

  // demonstrate cargo integration
  #[ cfg( feature = "cargo_integration" ) ]
  cargo_integration_demo()?;

  // demonstrate serde integration
  #[ cfg( feature = "serde_integration" ) ]
  serde_integration_demo()?;

  Ok( () )
}

#[ cfg( feature = "cargo_integration" ) ]
fn cargo_integration_demo() -> Result< (), Box< dyn std::error::Error > >
{
  println!( "📦 Cargo Integration Features:" );
  
  // try to detect cargo workspace automatically
  match Workspace::from_cargo_workspace()
  {
    Ok( workspace ) =>
    {
      println!( "  ✅ Auto-detected cargo workspace at: {}", workspace.root().display() );
      
      // check if this is a cargo workspace
      if workspace.is_cargo_workspace()
      {
        println!( "  ✅ Confirmed: This is a valid cargo workspace" );
        
        // get cargo metadata
        match workspace.cargo_metadata()
        {
          Ok( metadata ) =>
          {
            println!( "  📊 Cargo Metadata:" );
            println!( "     Workspace root: {}", metadata.workspace_root.display() );
            println!( "     Members: {} packages", metadata.members.len() );
            
            for member in &metadata.members
            {
              println!( "       • {} v{} at {}", 
                member.name, 
                member.version, 
                member.package_root.display() 
              );
            }
            
            if !metadata.workspace_dependencies.is_empty()
            {
              println!( "     Workspace dependencies:" );
              for ( name, version ) in &metadata.workspace_dependencies
              {
                println!( "       • {} = {}", name, version );
              }
            }
          }
          Err( e ) =>
          {
            println!( "  ⚠️  Failed to get cargo metadata: {}", e );
          }
        }
        
        // get workspace members
        match workspace.workspace_members()
        {
          Ok( members ) =>
          {
            println!( "  📁 Workspace member directories:" );
            for member_dir in members
            {
              println!( "     • {}", member_dir.display() );
            }
          }
          Err( e ) =>
          {
            println!( "  ⚠️  Failed to get workspace members: {}", e );
          }
        }
      }
      else
      {
        println!( "  ⚠️  Directory exists but is not a cargo workspace" );
      }
    }
    Err( e ) =>
    {
      println!( "  ⚠️  No cargo workspace detected: {}", e );
      println!( "     Falling back to standard workspace detection..." );
    }
  }
  
  // demonstrate resolve_or_fallback with cargo priority
  let workspace = Workspace::resolve_or_fallback();
  println!( "  🎯 Final workspace location: {}", workspace.root().display() );
  
  println!();
  Ok( () )
}

#[ cfg( feature = "serde_integration" ) ]
fn serde_integration_demo() -> Result< (), Box< dyn std::error::Error > >
{
  println!( "🔧 Serde Integration Features:" );
  
  let workspace = Workspace::resolve_or_fallback();
  
  // ensure config directory exists
  let config_dir = workspace.config_dir();
  std::fs::create_dir_all( &config_dir )?;
  
  // 1. demonstrate saving configurations in different formats
  println!( "  💾 Saving configurations in multiple formats..." );
  
  let app_config = AppConfig {
    name : "demo_app".to_string(),
    version : "1.0.0".to_string(),
    port : 8080,
    debug : true,
    database : DatabaseConfig {
      host : "localhost".to_string(),
      port : 5432,
      name : "demo_db".to_string(),
      ssl : false,
    },
    features : vec![ "logging".to_string(), "metrics".to_string() ],
  };
  
  // save as TOML
  workspace.save_config_to( config_dir.join( "app.toml" ), &app_config )?;
  println!( "     ✅ Saved app.toml" );
  
  // save as JSON
  workspace.save_config_to( config_dir.join( "app.json" ), &app_config )?;
  println!( "     ✅ Saved app.json" );
  
  // save as YAML
  workspace.save_config_to( config_dir.join( "app.yaml" ), &app_config )?;
  println!( "     ✅ Saved app.yaml" );
  
  // 2. demonstrate loading with automatic format detection
  println!( "  📂 Loading configurations with automatic format detection..." );
  
  // load TOML
  let toml_config : AppConfig = workspace.load_config( "app" )?;
  println!( "     ✅ Loaded from app.toml: {} v{}", toml_config.name, toml_config.version );
  
  // load from specific JSON file
  let json_config : AppConfig = workspace.load_config_from( config_dir.join( "app.json" ) )?;
  println!( "     ✅ Loaded from app.json: {} on port {}", json_config.name, json_config.port );
  
  // load from specific YAML file
  let yaml_config : AppConfig = workspace.load_config_from( config_dir.join( "app.yaml" ) )?;
  println!( "     ✅ Loaded from app.yaml: {} with {} features", 
    yaml_config.name, yaml_config.features.len() );
  
  // 3. demonstrate layered configuration
  println!( "  🔄 Layered configuration management..." );
  
  // create base configuration
  let base_config = AppConfig {
    name : "base_app".to_string(),
    version : "1.0.0".to_string(),
    port : 3000,
    debug : false,
    database : DatabaseConfig {
      host : "db.example.com".to_string(),
      port : 5432,
      name : "production_db".to_string(),
      ssl : true,
    },
    features : vec![ "auth".to_string(), "logging".to_string() ],
  };
  workspace.save_config( "base", &base_config )?;
  
  // create environment-specific override
  let dev_config = AppConfig {
    name : "dev_app".to_string(),
    version : "1.0.0-dev".to_string(),
    port : 8080,
    debug : true,
    database : DatabaseConfig {
      host : "localhost".to_string(),
      port : 5432,
      name : "dev_db".to_string(),
      ssl : false,
    },
    features : vec![ "debug_toolbar".to_string(), "hot_reload".to_string() ],
  };
  workspace.save_config( "development", &dev_config )?;
  
  // load layered configuration
  let layered_config : AppConfig = workspace.load_config_layered( &[ "base", "development" ] )?;
  println!( "     ✅ Merged configuration: {} v{} on port {}", 
    layered_config.name, layered_config.version, layered_config.port );
  println!( "        Features: {:?}", layered_config.features );
  println!( "        Database: {}:{} (ssl: {})", 
    layered_config.database.host, 
    layered_config.database.port,
    layered_config.database.ssl 
  );
  
  // 4. demonstrate partial configuration updates
  println!( "  🔄 Partial configuration updates..." );
  
  let updates = serde_json::json!({
    "port": 9090,
    "debug": false,
    "database": {
      "ssl": true
    }
  });
  
  let updated_config : AppConfig = workspace.update_config( "app", updates )?;
  println!( "     ✅ Updated configuration: {} now running on port {} (debug: {})", 
    updated_config.name, updated_config.port, updated_config.debug );
  println!( "        Database SSL: {}", updated_config.database.ssl );
  
  // 5. demonstrate error handling
  println!( "  ⚠️  Error handling demonstration..." );
  
  match workspace.load_config::< AppConfig >( "nonexistent" )
  {
    Ok( _ ) => println!( "     Unexpected success!" ),
    Err( e ) => println!( "     ✅ Properly handled missing config: {}", e ),
  }
  
  println!();
  Ok( () )
}

#[ cfg( not( any( feature = "cargo_integration", feature = "serde_integration" ) ) ) ]
fn main()
{
  println!( "🔧 This example requires cargo_integration and/or serde_integration features." );
  println!( "   Run with: cargo run --example 010_cargo_and_serde_integration --features full" );
}