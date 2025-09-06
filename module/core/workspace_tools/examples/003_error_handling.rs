//! # 003 - Error Handling
//!
//! comprehensive error handling patterns for workspace operations
//! this example shows different error scenarios and how to handle them

use workspace_tools::{ workspace, Workspace, WorkspaceError };

#[allow(clippy::too_many_lines)]
fn main() -> Result< (), Box< dyn core::error::Error > >
{
  println!( "🚨 workspace error handling patterns\n" );
  
  // 1. environment variable missing
  println!( "1️⃣  handling missing environment variable:" );
  std::env::remove_var( "WORKSPACE_PATH" ); // ensure it's not set
  
  match Workspace::resolve()
  {
    Ok( ws ) => println!( "   unexpected success: {}", ws.root().display() ),
    Err( WorkspaceError::EnvironmentVariableMissing( var ) ) =>
    {
      println!( "   ✅ caught missing env var: {var}" );
      println!( "   💡 solution: set WORKSPACE_PATH or use resolve_or_fallback()" );
    }
    Err( e ) => println!( "   unexpected error: {e}" ),
  }
  
  // 2. fallback resolution (never fails)
  println!( "\n2️⃣  using fallback resolution:" );
  let ws = Workspace::resolve_or_fallback();
  println!( "   ✅ fallback workspace: {}", ws.root().display() );
  println!( "   💡 this method always succeeds with some valid workspace" );
  
  // 3. path not found errors  
  println!( "\n3️⃣  handling path not found:" );
  std::env::set_var( "WORKSPACE_PATH", "/nonexistent/directory/path" );
  
  match Workspace::resolve()
  {
    Ok( ws ) => println!( "   unexpected success: {}", ws.root().display() ),
    Err( WorkspaceError::PathNotFound( path ) ) =>
    {
      println!( "   ✅ caught path not found: {}", path.display() );
      println!( "   💡 solution: ensure WORKSPACE_PATH points to existing directory" );
    }
    Err( e ) => println!( "   unexpected error: {e}" ),
  }
  
  // setup valid workspace for remaining examples
  std::env::set_var( "WORKSPACE_PATH", std::env::current_dir()? );
  let ws = workspace()?;
  
  // 4. io errors during operations
  println!( "\n4️⃣  handling io errors:" );
  match ws.normalize_path( "nonexistent/deeply/nested/path.txt" )
  {
    Ok( normalized ) => println!( "   unexpected success: {}", normalized.display() ),
    Err( WorkspaceError::IoError( msg ) ) =>
    {
      println!( "   ✅ caught io error: {msg}" );
      println!( "   💡 normalization requires existing paths" );
    }
    Err( e ) => println!( "   unexpected error type: {e}" ),
  }
  
  // 5. configuration errors
  println!( "\n5️⃣  configuration error example:" );
  // create a file where we expect a directory
  let fake_workspace = std::env::temp_dir().join( "fake_workspace_file" );
  std::fs::write( &fake_workspace, "this is a file, not a directory" )?;
  
  std::env::set_var( "WORKSPACE_PATH", &fake_workspace );
  match Workspace::resolve()
  {
    Ok( ws ) =>
    {
      // this might succeed initially, but validation will catch it
      match ws.validate()
      {
        Ok( () ) => println!( "   unexpected validation success" ),
        Err( WorkspaceError::ConfigurationError( msg ) ) =>
        {
          println!( "   ✅ caught configuration error: {msg}" );
          println!( "   💡 always validate workspace in production" );
        }
        Err( e ) => println!( "   unexpected error: {e}" ),
      }
    }
    Err( e ) => println!( "   error during resolve: {e}" ),
  }
  
  // cleanup
  let _ = std::fs::remove_file( &fake_workspace );
  std::env::set_var( "WORKSPACE_PATH", std::env::current_dir()? );
  
  // 6. comprehensive error matching pattern
  println!( "\n6️⃣  comprehensive error handling pattern:" );
  
  fn handle_workspace_operation() -> Result< (), WorkspaceError >
  {
    let ws = workspace()?;
    ws.validate()?;
    let _config = ws.normalize_path( "config/app.toml" )?;
    Ok( () )
  }
  
  match handle_workspace_operation()
  {
    Ok( () ) => println!( "   ✅ operation succeeded" ),
    Err( WorkspaceError::EnvironmentVariableMissing( var ) ) =>
      println!( "   handle missing env: {var}" ),
    Err( WorkspaceError::PathNotFound( path ) ) =>
      println!( "   handle missing path: {}", path.display() ),
    Err( WorkspaceError::ConfigurationError( msg ) ) =>
      println!( "   handle config error: {msg}" ),
    Err( WorkspaceError::IoError( msg ) ) =>
      println!( "   handle io error: {msg}" ),
    #[ cfg( feature = "glob" ) ]
    Err( WorkspaceError::GlobError( msg ) ) =>
      println!( "   handle glob error: {msg}" ),
    Err( WorkspaceError::PathOutsideWorkspace( path ) ) =>
      println!( "   handle security violation: {}", path.display() ),
    
    // handle new error types from cargo and serde integration
    #[ cfg( feature = "serde" ) ]
    Err( WorkspaceError::CargoError( msg ) ) =>
      println!( "   handle cargo error: {msg}" ),
    
    #[ cfg( feature = "serde" ) ]  
    Err( WorkspaceError::TomlError( msg ) ) =>
      println!( "   handle toml error: {msg}" ),
    
    #[ cfg( feature = "serde" ) ]
    Err( WorkspaceError::SerdeError( msg ) ) =>
      println!( "   handle serde error: {msg}" ),
    
    // catch-all for any future error variants (required due to #[non_exhaustive])
    Err( e ) => println!( "   handle unknown error: {e}" ),
  }
  
  println!( "\n💡 error handling best practices:" );
  println!( "   • use specific error matching instead of generic Error" );
  println!( "   • provide helpful error messages to users" );
  println!( "   • validate workspace early in application lifecycle" );
  println!( "   • consider using resolve_or_fallback() for flexibility" );
  println!( "   • handle path not found gracefully" );
  
  println!( "\n🎯 next: run example 004 to learn about resource discovery" );
  
  Ok( () )
}