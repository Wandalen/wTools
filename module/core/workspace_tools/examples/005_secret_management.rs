//! # 005 - Secret Management (`secrets` feature)  
//!
//! secure configuration loading with environment fallbacks
//! this example requires the "`secrets`" feature

#[ cfg( feature = "secrets" ) ]
fn main() -> Result< (), workspace_tools::WorkspaceError >
{
  println!( "🔒 workspace secret management\n" );
  
  // setup workspace
  if std::env::var( "WORKSPACE_PATH" ).is_err()
  {
    std::env::set_var( "WORKSPACE_PATH", std::env::current_dir().unwrap() );
  }

  let ws = workspace_tools::workspace()?;
  
  // 1. setup secret directory and files
  println!( "1️⃣  setting up secret directory:" );
  let secret_dir = ws.secret_dir();
  std::fs::create_dir_all( &secret_dir )
    .map_err( | e | workspace_tools::WorkspaceError::IoError( e.to_string() ) )?;
  
  println!( "   secret dir: {}", secret_dir.display() );
  println!( "   💡 this directory should be in .gitignore!" );
  
  // 2. create different secret files for different environments
  setup_secret_files( &ws )?;
  
  // 3. load all secrets from a file
  println!( "\n3️⃣  loading all secrets from file:" );
  let secrets = ws.load_secrets_from_file( "-secrets.sh" )?;
  
  println!( "   loaded {} secret keys:", secrets.len() );
  for ( key, value ) in &secrets
  {
    let masked = mask_secret( value );
    println!( "   {key}: {masked}" );
  }
  
  // 4. load specific secret keys
  println!( "\n4️⃣  loading specific secret keys:" );
  
  let secret_keys = vec![ "API_KEY", "DATABASE_URL", "REDIS_URL", "JWT_SECRET" ];
  
  for key in secret_keys
  {
    match ws.load_secret_key( key, "-secrets.sh" )
    {
      Ok( value ) => 
        println!( "   {}: {} (length: {})", key, mask_secret( &value ), value.len() ),
      Err( e ) => 
        println!( "   {key}: ❌ {e}" ),
    }
  }
  
  // 5. environment variable fallback
  println!( "\n5️⃣  environment variable fallback:" );
  
  // set some environment variables
  std::env::set_var( "ENV_ONLY_SECRET", "from_environment_only" );
  std::env::set_var( "OVERRIDE_SECRET", "env_value_overrides_file" );
  
  let fallback_keys = vec![ "ENV_ONLY_SECRET", "OVERRIDE_SECRET", "MISSING_KEY" ];
  
  for key in fallback_keys
  {
    match ws.load_secret_key( key, "-secrets.sh" )
    {
      Ok( value ) => 
        println!( "   {}: {} (source: {})", 
          key, 
          mask_secret( &value ),
          if secrets.contains_key( key ) { "file" } else { "environment" }
        ),
      Err( e ) => 
        println!( "   {key}: ❌ {e}" ),
    }
  }
  
  // 6. different secret file formats
  println!( "\n6️⃣  different secret file formats:" );
  
  let file_formats = vec![ "production.env", "development.env", "testing.env" ];
  
  for file_format in file_formats
  {
    match ws.load_secrets_from_file( file_format )
    {
      Ok( file_secrets ) => 
        println!( "   {}: loaded {} secrets", file_format, file_secrets.len() ),
      Err( _ ) => 
        println!( "   {file_format}: not found or empty" ),
    }
  }
  
  // 7. secret validation and security
  println!( "\n7️⃣  secret validation patterns:" );
  
  validate_secrets( &ws );
  
  // 8. practical application configuration
  println!( "\n8️⃣  practical application configuration:" );
  
  demonstrate_app_config( &ws )?;
  
  // cleanup
  cleanup_secret_files( &ws );
  
  println!( "\n🔒 secret management best practices:" );
  println!( "   • never commit secret files to version control" );
  println!( "   • add .secret/ to .gitignore" );
  println!( "   • use different files for different environments" );
  println!( "   • validate secrets early in application startup" );
  println!( "   • prefer environment variables in production" );
  println!( "   • rotate secrets regularly" );
  println!( "   • use proper file permissions (600) for secret files" );
  
  println!( "\n🎯 next: run example 006 to learn about testing integration" );
  
  Ok( () )
}

#[ cfg( feature = "secrets" ) ]
fn setup_secret_files( ws : &workspace_tools::Workspace ) -> Result< (), workspace_tools::WorkspaceError >
{
  use std::fs;
  
  println!( "\n2️⃣  creating example secret files:" );
  
  // main secrets file (shell format)
  let main_secrets = r#"# main application secrets (shell script format)
# database configuration
DATABASE_URL="postgresql://user:pass@localhost:5432/myapp"
REDIS_URL="redis://localhost:6379/0"

# external apis  
API_KEY="sk-1234567890abcdef"
STRIPE_SECRET="sk_test_1234567890"

# authentication
JWT_SECRET="your-256-bit-secret-here"
SESSION_SECRET="another-secret-key"

# optional services
SENTRY_DSN="https://key@sentry.io/project"
"#;

  let secrets_file = ws.secret_file( "-secrets.sh" );
  fs::write( &secrets_file, main_secrets )
    .map_err( | e | workspace_tools::WorkspaceError::IoError( e.to_string() ) )?;
  println!( "   created: {}", secrets_file.display() );
  
  // production environment
  let prod_secrets = r"# production environment secrets
DATABASE_URL=postgresql://prod-user:prod-pass@prod-db:5432/myapp_prod
API_KEY=sk-prod-abcdef1234567890
DEBUG=false
";

  let prod_file = ws.secret_file( "production.env" );
  fs::write( &prod_file, prod_secrets )
    .map_err( | e | workspace_tools::WorkspaceError::IoError( e.to_string() ) )?;
  println!( "   created: {}", prod_file.display() );
  
  // development environment  
  let dev_secrets = r"# development environment secrets
DATABASE_URL=postgresql://dev:dev@localhost:5432/myapp_dev
API_KEY=sk-dev-test1234567890
DEBUG=true
LOG_LEVEL=debug
";

  let dev_file = ws.secret_file( "development.env" );
  fs::write( &dev_file, dev_secrets )
    .map_err( | e | workspace_tools::WorkspaceError::IoError( e.to_string() ) )?;
  println!( "   created: {}", dev_file.display() );
  
  Ok( () )
}

#[ cfg( feature = "secrets" ) ]
fn validate_secrets( ws : &workspace_tools::Workspace )
{
  let required_secrets = vec![ "DATABASE_URL", "API_KEY", "JWT_SECRET" ];
  let optional_secrets = vec![ "REDIS_URL", "SENTRY_DSN" ];
  
  println!( "   validating required secrets:" );
  for secret in required_secrets
  {
    match ws.load_secret_key( secret, "-secrets.sh" )
    {
      Ok( value ) => 
      {
        if value.len() < 10 
        {
          println!( "     ⚠️  {} is too short ({})", secret, value.len() );
        }
        else
        {
          println!( "     ✅ {secret} is valid" );
        }
      }
      Err( _ ) => 
        println!( "     ❌ {secret} is missing (required)" ),
    }
  }
  
  println!( "   validating optional secrets:" );
  for secret in optional_secrets
  {
    match ws.load_secret_key( secret, "-secrets.sh" )
    {
      Ok( _ ) => println!( "     ✅ {secret} is available" ),
      Err( _ ) => println!( "     ℹ️  {secret} not configured (optional)" ),
    }
  }
}

#[ cfg( feature = "secrets" ) ]
fn demonstrate_app_config( ws : &workspace_tools::Workspace ) -> Result< (), workspace_tools::WorkspaceError >
{
  // simulate loading configuration with secrets
  struct AppConfig
  {
    database_url : String,
    api_key : String,
    jwt_secret : String,
    redis_url : Option< String >,
    debug : bool,
  }
  
  let config = AppConfig
  {
    database_url : ws.load_secret_key( "DATABASE_URL", "-secrets.sh" )?,
    api_key : ws.load_secret_key( "API_KEY", "-secrets.sh" )?,
    jwt_secret : ws.load_secret_key( "JWT_SECRET", "-secrets.sh" )?,
    redis_url : ws.load_secret_key( "REDIS_URL", "-secrets.sh" ).ok(),
    debug : std::env::var( "DEBUG" ).unwrap_or( "false".to_string() ) == "true",
  };
  
  println!( "   loaded application configuration:" );
  println!( "     database: {}", mask_secret( &config.database_url ) );
  println!( "     api key: {}", mask_secret( &config.api_key ) );
  println!( "     jwt secret: {}", mask_secret( &config.jwt_secret ) );
  println!( "     redis: {}", 
    config.redis_url
      .as_ref()
      .map_or( "not configured".to_string(), | url | mask_secret( url ) )
  );
  println!( "     debug: {}", config.debug );
  
  Ok( () )
}

#[ cfg( feature = "secrets" ) ]
fn cleanup_secret_files( ws : &workspace_tools::Workspace )
{
  let _ = std::fs::remove_dir_all( ws.secret_dir() );
}

#[ cfg( feature = "secrets" ) ]
fn mask_secret( value : &str ) -> String
{
  if value.len() <= 8
  {
    "*".repeat( value.len() )
  }
  else
  {
    format!( "{}...{}", 
      &value[ ..3 ], 
      "*".repeat( value.len() - 6 )
    )
  }
}

#[ cfg( not( feature = "secrets" ) ) ]
fn main()
{
  println!( "🚨 this example requires the 'secrets' feature" );
  println!( "run with: cargo run --example 005_secrets --features secrets" );
  println!();
  println!( "to enable secrets feature permanently, add to cargo.toml:" );
  println!( r#"[dependencies]"# );
  println!( r#"workspace_tools = {{ version = "0.1", features = ["secrets"] }}"# );
}