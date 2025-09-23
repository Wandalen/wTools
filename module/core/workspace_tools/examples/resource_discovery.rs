//! resource discovery example for `workspace_tools`
//!
//! this example demonstrates glob-based file finding functionality

#[ cfg( feature = "glob" ) ]
fn main() -> Result< (), workspace_tools ::WorkspaceError >
{
  // ensure we have a workspace path set
  if std ::env ::var( "WORKSPACE_PATH" ).is_err()
  {
  println!( "setting WORKSPACE_PATH to current directory for demo" );
  std ::env ::set_var( "WORKSPACE_PATH", std ::env ::current_dir().unwrap() );
 }

  let ws = workspace_tools ::workspace()?;
  
  println!( "workspace root: {}", ws.root().display() );
  
  // create example directory structure
  let demo_dirs = vec!
  [
  ws.join( "src" ),
  ws.join( "tests" ),
  ws.join( "config" ),
  ws.join( "assets/images" ),
  ws.join( "assets/fonts" ),
 ];
  
  for dir in &demo_dirs
  {
  std ::fs ::create_dir_all( dir ).map_err( | e | workspace_tools ::WorkspaceError ::IoError( e.to_string() ) )?;
 }
  
  // create example files
  let demo_files = vec!
  [
  ( "src/lib.rs", "// main library code" ),
  ( "src/main.rs", "// main application" ),
  ( "src/utils.rs", "// utility functions" ),
  ( "tests/integration_test.rs", "// integration tests" ),
  ( "tests/unit_test.rs", "// unit tests" ),
  ( "config/app.toml", "[app]\nname = \"demo\"" ),
  ( "config/database.yaml", "host: localhost" ),
  ( "assets/images/logo.png", "fake png data" ),
  ( "assets/images/icon.svg", "< svg >fake svg< /svg >" ),
  ( "assets/fonts/main.ttf", "fake font data" ),
 ];
  
  for ( path, content ) in &demo_files
  {
  let file_path = ws.join( path );
  std ::fs ::write( &file_path, content ).map_err( | e | workspace_tools ::WorkspaceError ::IoError( e.to_string() ) )?;
 }
  
  println!( "created example project structure" );
  
  // demonstrate resource discovery
  println!( "\nfinding rust source files: " );
  let rust_files = ws.find_resources( "src/**/*.rs" )?;
  for file in &rust_files
  {
  println!( "  {}", file.display() );
 }
  
  println!( "\nfinding test files: " );
  let test_files = ws.find_resources( "tests/**/*.rs" )?;
  for file in &test_files
  {
  println!( "  {}", file.display() );
 }
  
  println!( "\nfinding configuration files: " );
  let config_files = ws.find_resources( "config/**/*" )?;
  for file in &config_files
  {
  println!( "  {}", file.display() );
 }
  
  println!( "\nfinding image assets: " );
  let image_files = ws.find_resources( "assets/images/*" )?;
  for file in &image_files
  {
  println!( "  {}", file.display() );
 }
  
  // demonstrate config file discovery
  println!( "\nfinding specific config files: " );
  match ws.find_config( "app" )
  {
  Ok( config ) => println!( "  app config: {}", config.display() ),
  Err( e ) => println!( "  app config not found: {e}" ),
 }
  
  match ws.find_config( "database" )
  {
  Ok( config ) => println!( "  database config: {}", config.display() ),
  Err( e ) => println!( "  database config not found: {e}" ),
 }
  
  match ws.find_config( "nonexistent" )
  {
  Ok( config ) => println!( "  nonexistent config: {}", config.display() ),
  Err( e ) => println!( "  nonexistent config not found (expected) : {e}" ),
 }
  
  // clean up demo files
  println!( "\ncleaning up demo files..." );
  for dir in demo_dirs.iter().rev() // reverse order to delete children first
  {
  let _ = std ::fs ::remove_dir_all( dir );
 }
  
  Ok( () )
}

#[ cfg( not( feature = "glob" ) ) ]
fn main()
{
  println!( "this example requires the 'glob' feature" );
  println!( "run with: cargo run --example resource_discovery --features glob" );
}