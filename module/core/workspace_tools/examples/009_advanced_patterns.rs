//! # 009 - Advanced Patterns and Extensibility
//!
//! advanced usage patterns, extensibility, and integration with other rust ecosystem tools
//! demonstrates `workspace_tools` as a foundation for more complex applications

use workspace_tools :: { workspace, Workspace };
use std :: { fs, collections ::HashMap };

fn main() -> Result< (), Box< dyn core ::error ::Error > >
{
  println!( "🚀 advanced workspace patterns and extensibility\n" );
  
  let manager = AdvancedWorkspaceManager ::new()?;
  manager.demonstrate_patterns()?;
  manager.cleanup()?;
  
  println!( "\n🎯 this example demonstrates: " );
  println!( "   • workspace plugin architecture" );
  println!( "   • configuration overlays and environments" );
  println!( "   • workspace templates and scaffolding" );
  println!( "   • integration with other rust tools" );
  println!( "   • advanced path resolution patterns" );
  println!( "   • workspace composition and multi-workspace setups" );
  
  println!( "\n✅ congratulations! you've completed all workspace_tools examples" );
  println!( "   you now have a comprehensive understanding of workspace-relative development" );
  println!( "   start using workspace_tools in your projects to eliminate path resolution pain!" );
  
  Ok( () )
}

struct AdvancedWorkspaceManager
{
  workspace: Workspace,
  plugins: Vec< Box< dyn WorkspacePlugin > >,
  environments: HashMap< String, EnvironmentConfig >,
}

trait WorkspacePlugin: Send + Sync
{
  fn name( &self ) -> &str;
  fn initialize( &mut self, workspace: &Workspace ) -> Result< (), Box< dyn core ::error ::Error > >;
  fn process( &self, workspace: &Workspace ) -> Result< PluginResult, Box< dyn core ::error ::Error > >;
}

struct PluginResult
{
  success: bool,
  message: String,
  data: HashMap< String, String >,
}

#[ derive( Clone ) ]
struct EnvironmentConfig
{
  #[ allow( dead_code ) ]
  name: String,
  variables: HashMap< String, String >,
  paths: HashMap< String, String >,
  features: Vec< String >,
}

impl AdvancedWorkspaceManager
{
  fn new() -> Result< Self, Box< dyn core ::error ::Error > >
  {
  println!( "1️⃣  initializing advanced workspace manager..." );
  
  if std ::env ::var( "WORKSPACE_PATH" ).is_err()
  {
   std ::env ::set_var( "WORKSPACE_PATH", std ::env ::current_dir()? );
 }
  
  let workspace = workspace()?;
  
  // initialize plugin system
  let mut plugins = Self ::create_plugins();
  for plugin in &mut plugins
  {
   plugin.initialize( &workspace )?;
   println!( "   initialized plugin: {}", plugin.name() );
 }
  
  // setup environments
  let environments = Self ::create_environments();
  
  // create advanced directory structure
  Self ::setup_advanced_structure( &workspace )?;
  
  println!( "   ✅ advanced manager initialized with {} plugins", plugins.len() );
  
  Ok( Self { workspace, plugins, environments } )
 }
  
  fn demonstrate_patterns( &self ) -> Result< (), Box< dyn core ::error ::Error > >
  {
  println!( "\n2️⃣  demonstrating advanced patterns: " );
  
  self.demonstrate_plugin_system();
  self.demonstrate_environment_overlays()?;
  self.demonstrate_workspace_templates()?;
  self.demonstrate_tool_integration()?;
  self.demonstrate_multi_workspace_composition()?;
  
  Ok( () )
 }
  
  fn demonstrate_plugin_system( &self )
  {
  println!( "   🔌 plugin system demonstration: " );
  
  for plugin in &self.plugins
  {
   match plugin.process( &self.workspace )
   {
  Ok( result ) =>
  {
   println!( "     {} -> {} ({})", 
  plugin.name(), 
  if result.success 
  { "✅" } else { "❌" },
  result.message
 );
   
   for ( key, value ) in result.data
   {
  println!( "       {key} : {value}" );
 }
 }
  Err( e ) => println!( "     {} -> error: {}", plugin.name(), e ),
 }
 }
 }
  
  fn demonstrate_environment_overlays( &self ) -> Result< (), Box< dyn core ::error ::Error > >
  {
  println!( "\n   🏗️  environment overlay system: " );
  
  for ( env_name, env_config ) in &self.environments
  {
   println!( "     environment: {env_name}" );
   
   // create environment-specific configuration
   let env_dir = self.workspace.config_dir().join( "environments" ).join( env_name );
   fs ::create_dir_all( &env_dir )?;
   
   // base configuration
   let base_config = format!( r#"# base configuration for {}
debug = {}
log_level = "{}"
cache_enabled = {}
"#, 
  env_name,
  env_name == "development",
  env_config.variables.get( "LOG_LEVEL" ).unwrap_or( &"info".to_string() ),
  env_name != "testing"
 );
   
   fs ::write( env_dir.join( "base.toml" ), base_config )?;
   
   // feature-specific overlays
   for feature in &env_config.features
   {
  let feature_config = format!( r#"# {feature} feature configuration
[{feature}]
enabled = true
config_file = "config/features/{feature}.toml"
"# );
  
  fs ::write( env_dir.join( format!( "{feature}.toml" ) ), feature_config )?;
  println!( "       created overlay: {env_name}/{feature}.toml" );
 }
   
   // apply environment variables
   for ( key, value ) in &env_config.variables
   {
  println!( "       env {key} : {value}" );
 }
   
   // resolve environment-specific paths
   for ( path_name, path_value ) in &env_config.paths
   {
  let resolved_path = self.workspace.join( path_value );
  println!( "       path {} : {}", path_name, resolved_path.display() );
 }
 }
  
  Ok( () )
 }
  
  fn demonstrate_workspace_templates( &self ) -> Result< (), Box< dyn core ::error ::Error > >
  {
  println!( "\n   📋 workspace template system: " );
  
  let templates = vec!
  [
   ( "rust-cli", Self ::create_cli_template() ),
   ( "web-service", Self ::create_web_template() ),
   ( "data-pipeline", Self ::create_pipeline_template() ),
   ( "desktop-app", Self ::create_desktop_template() ),
 ];
  
  let templates_dir = self.workspace.join( "templates" );
  fs ::create_dir_all( &templates_dir )?;
  
  for ( template_name, template_config ) in templates
  {
   let template_path = templates_dir.join( template_name );
   fs ::create_dir_all( &template_path )?;
   
   // create template metadata
   let metadata = format!( r#"# workspace template: {}
name = "{}"
description = "{}"
version = "1.0.0"
author = "workspace_tools"

[directories]
{}

[files]
{}
"#, 
  template_name,
  template_name,
  template_config.description,
  template_config.directories.join( "\n" ),
  template_config.files.iter()
   .map( | ( name, _ ) | format!( r#""{name}" = "template""# ) )
   .collect :: < Vec< _ > >()
   .join( "\n" )
 );
   
   fs ::write( template_path.join( "template.toml" ), metadata )?;
   
   // create template files
   let file_count = template_config.files.len();
   for ( filename, content ) in &template_config.files
   {
  let file_path = template_path.join( filename );
  if let Some( parent ) = file_path.parent()
  {
   fs ::create_dir_all( parent )?;
 }
  fs ::write( file_path, content )?;
 }
   
   println!( "     created template: {template_name}" );
   println!( "       directories: {}", template_config.directories.len() );
   println!( "       files: {file_count}" );
 }
  
  Ok( () )
 }
  
  fn demonstrate_tool_integration( &self ) -> Result< (), Box< dyn core ::error ::Error > >
  {
  println!( "\n   🔧 rust ecosystem tool integration: " );
  
  // cargo integration
  let cargo_config = format!( r#"# cargo configuration with workspace_tools
[env]
WORKSPACE_PATH = {{ value = ".", relative = true }}

[build]  
target-dir = "{}/target"

[install]
root = "{}/bin"
"#, 
   self.workspace.data_dir().display(),
   self.workspace.join( "tools" ).display()
 );
  
  let cargo_dir = self.workspace.join( ".cargo" );
  fs ::create_dir_all( &cargo_dir )?;
  fs ::write( cargo_dir.join( "config.toml" ), cargo_config )?;
  println!( "     ✅ cargo integration configured" );
  
  // justfile integration
  let justfile = format!( r#"# justfile with workspace_tools integration
# set workspace for all recipes
export WORKSPACE_PATH: = justfile_directory()

# default recipe
default :
  @just --list

# development tasks
dev :
  cargo run --example hello_workspace

test :
  cargo test --workspace

# build tasks  
build :
  cargo build --release
  
# deployment tasks
deploy env="staging" :
  echo "deploying to {{{{env}}}}"
  echo "workspace: $WORKSPACE_PATH"
  
# cleanup tasks
clean :
  cargo clean
  rm -rf {}/target
  rm -rf {}/logs/*
"#, 
   self.workspace.data_dir().display(),
   self.workspace.logs_dir().display()
 );
  
  fs ::write( self.workspace.join( "justfile" ), justfile )?;
  println!( "     ✅ just integration configured" );
  
  // serde integration example
  let serde_example = r#"// serde integration with workspace_tools
use serde :: { Deserialize, Serialize };
use workspace_tools ::workspace;

#[ derive(Serialize, Deserialize) ]
struct AppConfig 
{
  name: String,
  version: String,
  database_url: String,
}

fn load_config() -> Result< AppConfig, Box<dyn std ::error ::Error >> 
{
  let ws = workspace()?;
  let config_path = ws.find_config("app")?;
  let config_str = std ::fs ::read_to_string(config_path)?;
  let config: AppConfig = toml ::from_str(&config_str)?;
  Ok(config)
}
"#;
  
  let examples_dir = self.workspace.join( "integration_examples" );
  fs ::create_dir_all( &examples_dir )?;
  fs ::write( examples_dir.join( "serde_integration.rs" ), serde_example )?;
  println!( "     ✅ serde integration example created" );
  
  // tracing integration
  let tracing_example = r#"// tracing integration with workspace_tools
use tracing :: { info, warn, error };
use tracing_appender ::rolling :: { RollingFileAppender, Rotation };
use workspace_tools ::workspace;

fn setup_logging() -> Result< (), Box<dyn std ::error ::Error >> 
{
  let ws = workspace()?;
  let log_dir = ws.logs_dir();
  std ::fs ::create_dir_all(&log_dir)?;
  
  let file_appender = RollingFileAppender ::new(
  Rotation ::DAILY,
  log_dir,
  "app.log"
 );
  
  // configure tracing subscriber with workspace-aware file output
  // tracing_subscriber setup would go here...
  
  info!("logging initialized with workspace: {}", ws.root().display());
  Ok(())
}
"#;
  
  fs ::write( examples_dir.join( "tracing_integration.rs" ), tracing_example )?;
  println!( "     ✅ tracing integration example created" );
  
  Ok( () )
 }
  
  fn demonstrate_multi_workspace_composition( &self ) -> Result< (), Box< dyn core ::error ::Error > >
  {
  println!( "\n   🏗️  multi-workspace composition: " );
  
  // create sub-workspaces for different components
  let sub_workspaces = vec!
  [
   ( "frontend", "web frontend components" ),
   ( "backend", "api and business logic" ),
   ( "shared", "shared libraries and utilities" ),
   ( "tools", "development and deployment tools" ),
 ];
  
  for ( workspace_name, description ) in sub_workspaces
  {
   let sub_ws_dir = self.workspace.join( "workspaces" ).join( workspace_name );
   fs ::create_dir_all( &sub_ws_dir )?;
   
   // create sub-workspace cargo configuration
   let sub_cargo_dir = sub_ws_dir.join( ".cargo" );
   fs ::create_dir_all( &sub_cargo_dir )?;
   
   let sub_cargo_config = r#"[env]
WORKSPACE_PATH = { value = ".", relative = true }
PARENT_WORKSPACE = { value = "../..", relative = true }

[alias]
parent-test = "test --manifest-path ../../Cargo.toml"
"#.to_string();
   
   fs ::write( sub_cargo_dir.join( "config.toml" ), sub_cargo_config )?;
   
   // create workspace composition manifest
   let composition_manifest = format!( r#"# workspace composition manifest
name = "{workspace_name}"
description = "{description}"
parent_workspace = "../.."

[dependencies.internal]
shared = {{ path = "../shared" }}

[dependencies.external]
# external dependencies specific to this workspace

[directories]
config = "config"
data = "data" 
logs = "logs"
src = "src"

[integration]
parent_config = true
parent_secrets = true
isolated_data = true
"# );
   
   fs ::write( sub_ws_dir.join( "workspace.toml" ), composition_manifest )?;
   
   // create standard structure for sub-workspace
   for dir in &[ "config", "data", "logs", "src" ]
   {
  fs ::create_dir_all( sub_ws_dir.join( dir ) )?;
 }
   
   println!( "     created sub-workspace: {workspace_name} ({description})" );
 }
  
  // create workspace orchestration script
  let orchestration_script = r#"#!/bin/bash
# workspace orchestration script
set -e

PARENT_WS="$WORKSPACE_PATH"
echo "orchestrating multi-workspace build..."
echo "parent workspace: $PARENT_WS"

# build shared components first
echo "building shared workspace..."
cd workspaces/shared
export WORKSPACE_PATH="$(pwd)"
cargo build

# build backend
echo "building backend workspace..."
cd ../backend
export WORKSPACE_PATH="$(pwd)"
cargo build

# build frontend  
echo "building frontend workspace..."
cd ../frontend
export WORKSPACE_PATH="$(pwd)"
cargo build

# build tools
echo "building tools workspace..."
cd ../tools
export WORKSPACE_PATH="$(pwd)"
cargo build

echo "multi-workspace build completed!"
"#;
  
  let scripts_dir = self.workspace.join( "scripts" );
  fs ::create_dir_all( &scripts_dir )?;
  fs ::write( scripts_dir.join( "build-all.sh" ), orchestration_script )?;
  println!( "     ✅ orchestration script created" );
  
  Ok( () )
 }
  
  fn cleanup( &self ) -> Result< (), Box< dyn core ::error ::Error > >
  {
  println!( "\n3️⃣  cleaning up advanced demo..." );
  
  let cleanup_dirs = vec!
  [
   "templates", "workspaces", "scripts", "integration_examples", 
   "tools", "bin", "target", ".cargo"
 ];
  
  for dir_name in cleanup_dirs
  {
   let dir_path = self.workspace.join( dir_name );
   if dir_path.exists()
   {
  fs ::remove_dir_all( &dir_path )?;
  println!( "   removed: {}", dir_path.display() );
 }
 }
  
  let cleanup_files = vec![ "justfile" ];
  for file_name in cleanup_files
  {
   let file_path = self.workspace.join( file_name );
   if file_path.exists()
   {
  fs ::remove_file( &file_path )?;
  println!( "   removed: {}", file_path.display() );
 }
 }
  
  // clean up config directories
  let config_cleanup = vec![ "environments", "features" ];
  for dir_name in config_cleanup
  {
   let dir_path = self.workspace.config_dir().join( dir_name );
   if dir_path.exists()
   {
  fs ::remove_dir_all( &dir_path )?;
  println!( "   removed: {}", dir_path.display() );
 }
 }
  
  println!( "   ✅ cleanup completed" );
  
  Ok( () )
 }
  
  // factory methods
  
  fn create_plugins() -> Vec< Box< dyn WorkspacePlugin > >
  {
  vec!
  [
   Box ::new( ConfigValidatorPlugin ::new() ),
   Box ::new( AssetOptimizerPlugin ::new() ),
   Box ::new( SecurityScannerPlugin ::new() ),
   Box ::new( DocumentationGeneratorPlugin ::new() ),
 ]
 }
  
  fn create_environments() -> HashMap< String, EnvironmentConfig >
  {
  let mut environments = HashMap ::new();
  
  // development environment
  let mut dev_vars = HashMap ::new();
  dev_vars.insert( "LOG_LEVEL".to_string(), "debug".to_string() );
  dev_vars.insert( "DEBUG".to_string(), "true".to_string() );
  
  let mut dev_paths = HashMap ::new();
  dev_paths.insert( "temp".to_string(), "data/dev_temp".to_string() );
  dev_paths.insert( "cache".to_string(), "data/dev_cache".to_string() );
  
  environments.insert( "development".to_string(), EnvironmentConfig
  {
   name: "development".to_string(),
   variables: dev_vars,
   paths: dev_paths,
   features: vec![ "hot_reload".to_string(), "debug_ui".to_string() ],
 } );
  
  // production environment
  let mut prod_vars = HashMap ::new();
  prod_vars.insert( "LOG_LEVEL".to_string(), "info".to_string() );
  prod_vars.insert( "DEBUG".to_string(), "false".to_string() );
  
  let mut prod_paths = HashMap ::new();
  prod_paths.insert( "temp".to_string(), "data/temp".to_string() );
  prod_paths.insert( "cache".to_string(), "data/cache".to_string() );
  
  environments.insert( "production".to_string(), EnvironmentConfig
  {
   name: "production".to_string(),
   variables: prod_vars,
   paths: prod_paths,  
   features: vec![ "metrics".to_string(), "monitoring".to_string() ],
 } );
  
  environments
 }
  
  fn setup_advanced_structure( ws: &Workspace ) -> Result< (), Box< dyn core ::error ::Error > >
  {
  let advanced_dirs = vec!
  [
   "plugins", "templates", "environments", "scripts", "integration_examples",
   "config/environments", "config/features", "config/plugins",
   "data/plugins", "logs/plugins",
 ];
  
  for dir in advanced_dirs
  {
   let dir_path = ws.join( dir );
   fs ::create_dir_all( dir_path )?;
 }
  
  Ok( () )
 }
  
  fn create_cli_template() -> WorkspaceTemplate
  {
  WorkspaceTemplate
  {
   description: "command-line interface application".to_string(),
   directories: vec!
   [
  "src".to_string(), "tests".to_string(), "config".to_string(),
  "data".to_string(), "logs".to_string(), "docs".to_string()
 ],
   files: vec!
   [
  ( "src/main.rs".to_string(), "// cli application main".to_string() ),
  ( "src/cli.rs".to_string(), "// command line interface".to_string() ),
  ( "config/app.toml".to_string(), "# cli configuration".to_string() ),
  ( "Cargo.toml".to_string(), "# cargo manifest".to_string() ),
 ],
 }
 }
  
  fn create_web_template() -> WorkspaceTemplate
  {
  WorkspaceTemplate
  {
   description: "web service application".to_string(),
   directories: vec!
   [
  "src".to_string(), "templates".to_string(), "static".to_string(),
  "uploads".to_string(), "config".to_string(), "data".to_string()
 ],
   files: vec!
   [
  ( "src/main.rs".to_string(), "// web service main".to_string() ),
  ( "src/handlers.rs".to_string(), "// request handlers".to_string() ),
  ( "templates/base.html".to_string(), "< !-- base template -- >".to_string() ),
  ( "static/css/main.css".to_string(), "/* main styles */".to_string() ),
 ],
 }
 }
  
  fn create_pipeline_template() -> WorkspaceTemplate
  {
  WorkspaceTemplate
  {
   description: "data processing pipeline".to_string(),
   directories: vec!
   [
  "src".to_string(), "pipelines".to_string(), "data/input".to_string(),
  "data/output".to_string(), "data/temp".to_string(), "config".to_string()
 ],
   files: vec!
   [
  ( "src/main.rs".to_string(), "// pipeline runner".to_string() ),
  ( "src/processors.rs".to_string(), "// data processors".to_string() ),
  ( "pipelines/etl.toml".to_string(), "# etl pipeline config".to_string() ),
 ],
 }
 }
  
  fn create_desktop_template() -> WorkspaceTemplate
  {
  WorkspaceTemplate
  {
   description: "desktop gui application".to_string(),
   directories: vec!
   [
  "src".to_string(), "assets".to_string(), "resources".to_string(),
  "config".to_string(), "data".to_string(), "plugins".to_string()
 ],
   files: vec!
   [
  ( "src/main.rs".to_string(), "// desktop app main".to_string() ),
  ( "src/ui.rs".to_string(), "// user interface".to_string() ),
  ( "assets/icon.png".to_string(), "// app icon data".to_string() ),
 ],
 }
 }
}

struct WorkspaceTemplate
{
  description: String,
  directories: Vec< String >,
  files: Vec< ( String, String ) >,
}

// plugin implementations

struct ConfigValidatorPlugin
{
  initialized: bool,
}

impl ConfigValidatorPlugin
{
  fn new() -> Self
  {
  Self { initialized: false }
 }
}

impl WorkspacePlugin for ConfigValidatorPlugin
{
  fn name( &self ) -> &'static str { "config-validator" }
  
  fn initialize( &mut self, _workspace: &Workspace ) -> Result< (), Box< dyn core ::error ::Error > >
  {
  self.initialized = true;
  Ok( () )
 }
  
  fn process( &self, workspace: &Workspace ) -> Result< PluginResult, Box< dyn core ::error ::Error > >
  {
  let config_dir = workspace.config_dir();
  let config_count = if config_dir.exists()
  {
   fs ::read_dir( &config_dir )?.count()
 }
  else { 0 };
  
  let mut data = HashMap ::new();
  data.insert( "config_files".to_string(), config_count.to_string() );
  data.insert( "config_dir".to_string(), config_dir.display().to_string() );
  
  Ok( PluginResult
  {
   success: config_count > 0,
   message: format!( "found {config_count} config files" ),
   data,
 } )
 }
}

struct AssetOptimizerPlugin;
impl AssetOptimizerPlugin 
{ fn new() -> Self { Self } }
impl WorkspacePlugin for AssetOptimizerPlugin
{
  fn name( &self ) -> &'static str { "asset-optimizer" }
  fn initialize( &mut self, _workspace: &Workspace ) -> Result< (), Box< dyn core ::error ::Error > > { Ok( () ) }
  fn process( &self, workspace: &Workspace ) -> Result< PluginResult, Box< dyn core ::error ::Error > >
  {
  let static_dir = workspace.join( "static" );
  let asset_count = if static_dir.exists() { fs ::read_dir( static_dir )?.count() } else { 0 };
  
  let mut data = HashMap ::new();
  data.insert( "assets_found".to_string(), asset_count.to_string() );
  
  Ok( PluginResult
  {
   success: true,
   message: format!( "optimized {asset_count} assets" ),
   data,
 } )
 }
}

struct SecurityScannerPlugin;
impl SecurityScannerPlugin 
{ fn new() -> Self { Self } }
impl WorkspacePlugin for SecurityScannerPlugin
{
  fn name( &self ) -> &'static str { "security-scanner" }  
  fn initialize( &mut self, _workspace: &Workspace ) -> Result< (), Box< dyn core ::error ::Error > > { Ok( () ) }
  fn process( &self, #[ cfg_attr( not( feature = "secrets" ), allow( unused_variables ) ) ] workspace: &Workspace ) -> Result< PluginResult, Box< dyn core ::error ::Error > >
  {
  #[ cfg_attr( not( feature = "secrets" ), allow( unused_mut ) ) ]
  let mut issues = 0;
  let mut data = HashMap ::new();

  // simulate security checks
  #[ cfg( feature = "secrets" ) ]
  {
   let secret_dir = workspace.secret_dir();
   if secret_dir.exists()
   {
  // check permissions, etc.
  data.insert( "secret_dir_secure".to_string(), "true".to_string() );
 }
   else
   {
  issues += 1;
  data.insert( "secret_dir_missing".to_string(), "true".to_string() );
 }
 }
  
  data.insert( "security_issues".to_string(), issues.to_string() );
  
  Ok( PluginResult
  {
   success: issues == 0,
   message: format!( "security scan: {issues} issues found" ),
   data,
 } )
 }
}

struct DocumentationGeneratorPlugin;
impl DocumentationGeneratorPlugin 
{ fn new() -> Self { Self } }
impl WorkspacePlugin for DocumentationGeneratorPlugin
{
  fn name( &self ) -> &'static str { "doc-generator" }
  fn initialize( &mut self, _workspace: &Workspace ) -> Result< (), Box< dyn core ::error ::Error > > { Ok( () ) }
  fn process( &self, workspace: &Workspace ) -> Result< PluginResult, Box< dyn core ::error ::Error > >
  {
  let docs_dir = workspace.docs_dir();
  fs ::create_dir_all( &docs_dir )?;
  
  // generate workspace documentation
  let workspace_doc = format!( r"# workspace documentation

generated by workspace_tools documentation plugin

## workspace information
- root: {}
- config: {}  
- data: {}
- logs: {}

## structure
this workspace follows the standard workspace_tools layout for consistent development.
", 
   workspace.root().display(),
   workspace.config_dir().display(), 
   workspace.data_dir().display(),
   workspace.logs_dir().display()
 );
  
  fs ::write( docs_dir.join( "workspace.md" ), workspace_doc )?;
  
  let mut data = HashMap ::new();
  data.insert( "docs_generated".to_string(), "1".to_string() );
  data.insert( "docs_path".to_string(), docs_dir.display().to_string() );
  
  Ok( PluginResult
  {
   success: true,
   message: "generated workspace documentation".to_string(),
   data,
 } )
 }
}