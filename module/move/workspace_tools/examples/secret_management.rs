//! secret management example for `workspace_tools`
//!
//! this example demonstrates secure configuration loading functionality

#[ cfg( feature = "secret_management" ) ]
fn main() -> Result< (), workspace_tools::WorkspaceError >
{
  // ensure we have a workspace path set
  if std::env::var( "WORKSPACE_PATH" ).is_err()
  {
    println!( "setting WORKSPACE_PATH to current directory for demo" );
    std::env::set_var( "WORKSPACE_PATH", std::env::current_dir().unwrap() );
  }

  let ws = workspace_tools::workspace()?;
  
  println!( "workspace root: {}", ws.root().display() );
  
  // create secret directory and example file
  let secret_dir = ws.secret_dir();
  std::fs::create_dir_all( &secret_dir ).map_err( | e | workspace_tools::WorkspaceError::IoError( e.to_string() ) )?;
  
  let secret_file = secret_dir.join( "-secrets.sh" );
  let secret_content = r"# application secrets (shell format)
API_KEY=your_api_key_here
DATABASE_URL=postgresql://user:pass@localhost/db
# optional secrets
REDIS_URL=redis://localhost:6379
";
  
  std::fs::write( &secret_file, secret_content ).map_err( | e | workspace_tools::WorkspaceError::IoError( e.to_string() ) )?;
  
  println!( "created example secret file: {}", secret_file.display() );
  
  // load all secrets from file
  println!( "\nloading secrets from file:" );
  let secrets = ws.load_secrets_from_file( "-secrets.sh" )?;
  
  for ( key, value ) in &secrets
  {
    let masked_value = if value.len() > 8
    {
      format!( "{}...", &value[ ..8 ] )
    }
    else
    {
      "***".to_string()
    };
    println!( "  {key}: {masked_value}" );
  }
  
  // load specific secret key
  println!( "\nloading specific secret keys:" );
  match ws.load_secret_key( "API_KEY", "-secrets.sh" )
  {
    Ok( key ) => println!( "  API_KEY loaded (length: {})", key.len() ),
    Err( e ) => println!( "  failed to load API_KEY: {e}" ),
  }
  
  // demonstrate fallback to environment
  std::env::set_var( "ENV_SECRET", "from_environment" );
  match ws.load_secret_key( "ENV_SECRET", "-secrets.sh" )
  {
    Ok( key ) => println!( "  ENV_SECRET from environment: {key}" ),
    Err( e ) => println!( "  failed to load ENV_SECRET: {e}" ),
  }
  
  // clean up demo files
  let _ = std::fs::remove_file( &secret_file );
  let _ = std::fs::remove_dir( &secret_dir );
  
  Ok( () )
}

#[ cfg( not( feature = "secret_management" ) ) ]
fn main()
{
  println!( "this example requires the 'secret_management' feature" );
  println!( "run with: cargo run --example secret_management --features secret_management" );
}