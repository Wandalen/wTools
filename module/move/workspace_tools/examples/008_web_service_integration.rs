//! # 008 - Web Service Integration  
//!
//! demonstrates workspace_tools integration with web services
//! shows asset serving, config loading, logging, and deployment patterns

use workspace_tools::workspace;
use std::fs;

fn main() -> Result< (), Box< dyn std::error::Error > >
{
  println!( "🌐 web service integration example\n" );
  
  let service = WebService::new()?;
  service.demonstrate_features()?;
  service.cleanup()?;
  
  println!( "\n🎯 this example demonstrates:" );
  println!( "   • web service workspace structure" );
  println!( "   • static asset management" );
  println!( "   • configuration for different environments" );
  println!( "   • template and view resolution" );
  println!( "   • upload and media handling" );  
  println!( "   • deployment-ready patterns" );
  
  println!( "\n🎯 next: run example 009 to see advanced patterns and plugins" );
  
  Ok( () )
}

struct WebService
{
  workspace : workspace_tools::Workspace,
  config : ServiceConfig,
}

#[ derive( Debug ) ]
struct ServiceConfig
{
  name : String,
  host : String,
  port : u16,
  environment : String,
  static_cache_ttl : u32,
  upload_max_size_mb : u32,
}

impl Default for ServiceConfig
{
  fn default() -> Self
  {
    Self
    {
      name : "demo-web-service".to_string(),
      host : "127.0.0.1".to_string(),
      port : 8080,
      environment : "development".to_string(),
      static_cache_ttl : 3600,
      upload_max_size_mb : 10,
    }
  }
}

impl WebService
{
  fn new() -> Result< Self, Box< dyn std::error::Error > >
  {
    println!( "1️⃣  initializing web service..." );
    
    // setup workspace
    if std::env::var( "WORKSPACE_PATH" ).is_err()
    {
      std::env::set_var( "WORKSPACE_PATH", std::env::current_dir()? );
    }
    
    let workspace = workspace()?;
    
    // create web service directory structure
    Self::setup_web_structure( &workspace )?;
    
    // load configuration
    let config = Self::load_config( &workspace )?;
    
    println!( "   ✅ web service initialized" );
    
    Ok( Self { workspace, config } )
  }
  
  fn setup_web_structure( ws : &workspace_tools::Workspace ) -> Result< (), Box< dyn std::error::Error > >
  {
    println!( "   🏗️  setting up web service structure..." );
    
    let web_dirs = vec!
    [
      // standard workspace dirs
      ws.config_dir(),
      ws.data_dir(),
      ws.logs_dir(),
      
      // web-specific directories
      ws.join( "static" ),          // css, js, images
      ws.join( "static/css" ),
      ws.join( "static/js" ),
      ws.join( "static/images" ),
      ws.join( "templates" ),       // html templates
      ws.join( "uploads" ),         // user uploads
      ws.join( "media" ),           // generated media
      ws.join( "cache" ),           // web cache
      ws.join( "sessions" ),        // session storage
    ];
    
    for dir in web_dirs
    {
      fs::create_dir_all( &dir )?;
      println!( "     created: {}", dir.display() );
    }
    
    Ok( () )
  }
  
  fn load_config( ws : &workspace_tools::Workspace ) -> Result< ServiceConfig, Box< dyn std::error::Error > >
  {
    println!( "   ⚙️  loading service configuration..." );
    
    // try environment-specific config first
    let env = std::env::var( "ENVIRONMENT" ).unwrap_or( "development".to_string() );
    let config_file = ws.config_dir().join( format!( "{}.toml", env ) );
    
    let config = if config_file.exists()
    {
      println!( "     loading {}: {}", env, config_file.display() );
      let content = fs::read_to_string( config_file )?;
      Self::parse_config( &content, &env )?
    }
    else
    {
      println!( "     creating default {} config", env );
      let default_config = Self::create_default_config( &env );
      let config_content = Self::config_to_toml( &default_config );
      fs::write( &config_file, config_content )?;
      default_config
    };
    
    // load secrets if available
    Self::load_secrets( ws, &config )?;
    
    println!( "     ✅ configuration loaded: {:?}", config );
    Ok( config )
  }
  
  #[ cfg( feature = "secret_management" ) ]
  fn load_secrets( ws : &workspace_tools::Workspace, config : &ServiceConfig ) -> Result< (), Box< dyn std::error::Error > >
  {
    println!( "   🔒 loading service secrets..." );
    
    let secret_file = format!( "-{}.sh", config.environment );
    
    match ws.load_secret_key( "DATABASE_URL", &secret_file )
    {
      Ok( _ ) => println!( "     ✅ database connection configured" ),
      Err( _ ) => println!( "     ℹ️  no database secrets (using default)" ),
    }
    
    match ws.load_secret_key( "JWT_SECRET", &secret_file )
    {
      Ok( _ ) => println!( "     ✅ jwt signing configured" ),
      Err( _ ) => println!( "     ⚠️  no jwt secret (generate for production!)" ),
    }
    
    Ok( () )
  }
  
  #[ cfg( not( feature = "secret_management" ) ) ]
  fn load_secrets( _ws : &workspace_tools::Workspace, _config : &ServiceConfig ) -> Result< (), Box< dyn std::error::Error > >
  {
    println!( "   ℹ️  secret management not enabled" );
    Ok( () )
  }
  
  fn demonstrate_features( &self ) -> Result< (), Box< dyn std::error::Error > >
  {
    println!( "\n2️⃣  demonstrating web service features:" );
    
    self.setup_static_assets()?;
    self.create_templates()?;
    self.simulate_request_handling()?;
    self.demonstrate_uploads()?;
    self.show_deployment_config()?;
    
    Ok( () )
  }
  
  fn setup_static_assets( &self ) -> Result< (), Box< dyn std::error::Error > >
  {
    println!( "   📄 setting up static assets..." );
    
    // create css files
    let css_content = r#"/* main stylesheet */
body {
  font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
  margin: 0;
  padding: 20px;
  background: #f8f9fa;
}

.container {
  max-width: 1200px;
  margin: 0 auto;
  background: white;
  padding: 20px;
  border-radius: 8px;
  box-shadow: 0 2px 4px rgba(0,0,0,0.1);
}

.header {
  border-bottom: 1px solid #dee2e6;
  margin-bottom: 20px;
  padding-bottom: 10px;
}
"#;
    
    let css_file = self.workspace.join( "static/css/main.css" );
    fs::write( &css_file, css_content )?;
    println!( "     created: {}", css_file.display() );
    
    // create javascript
    let js_content = r#"// main application javascript
document.addEventListener('DOMContentLoaded', function() {
    console.log('workspace_tools demo app loaded');
    
    // simulate dynamic content loading
    const loadData = async () => {
        try {
            const response = await fetch('/api/data');
            const data = await response.json();
            document.querySelector('#data-display').innerHTML = JSON.stringify(data, null, 2);
        } catch (error) {
            console.error('failed to load data:', error);
        }
    };
    
    // setup event listeners
    document.querySelector('#load-data')?.addEventListener('click', loadData);
});
"#;
    
    let js_file = self.workspace.join( "static/js/app.js" );
    fs::write( &js_file, js_content )?;
    println!( "     created: {}", js_file.display() );
    
    // create placeholder images
    let image_data = b"fake-image-data-for-demo";
    let logo_file = self.workspace.join( "static/images/logo.png" );
    fs::write( &logo_file, image_data )?;
    println!( "     created: {}", logo_file.display() );
    
    Ok( () )
  }
  
  fn create_templates( &self ) -> Result< (), Box< dyn std::error::Error > >
  {
    println!( "   📋 creating html templates..." );
    
    // base template
    let base_template = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{{title}} - Workspace Tools Demo</title>
    <link rel="stylesheet" href="/static/css/main.css">
</head>
<body>
    <div class="container">
        <header class="header">
            <h1>{{title}}</h1>
            <nav>
                <a href="/">home</a> |
                <a href="/about">about</a> |
                <a href="/upload">upload</a>
            </nav>
        </header>
        
        <main>
            {{content}}
        </main>
        
        <footer>
            <p>powered by workspace_tools | workspace: {{workspace_root}}</p>
        </footer>
    </div>
    
    <script src="/static/js/app.js"></script>
</body>
</html>"#;
    
    let base_file = self.workspace.join( "templates/base.html" );
    fs::write( &base_file, base_template )?;
    println!( "     created: {}", base_file.display() );
    
    // home page template
    let home_template = r#"<h2>welcome to the demo service</h2>

<p>this service demonstrates workspace_tools integration in web applications.</p>

<div>
    <h3>service information</h3>
    <ul>
        <li>environment: {{environment}}</li>
        <li>host: {{host}}:{{port}}</li>
        <li>workspace: {{workspace_root}}</li>
    </ul>
</div>

<div>
    <h3>dynamic data</h3>
    <button id="load-data">load data</button>
    <pre id="data-display">click button to load data...</pre>
</div>"#;
    
    let home_file = self.workspace.join( "templates/home.html" );
    fs::write( &home_file, home_template )?;
    println!( "     created: {}", home_file.display() );
    
    // upload template
    let upload_template = r#"<h2>file upload</h2>

<form action="/upload" method="post" enctype="multipart/form-data">
    <div>
        <label for="file">choose file:</label>
        <input type="file" id="file" name="file" required>
    </div>
    
    <div>
        <label for="description">description:</label>
        <textarea id="description" name="description" rows="3"></textarea>
    </div>
    
    <button type="submit">upload file</button>
</form>

<p>maximum file size: {{max_upload_size}} mb</p>

<div id="upload-status"></div>"#;
    
    let upload_file = self.workspace.join( "templates/upload.html" );
    fs::write( &upload_file, upload_template )?;
    println!( "     created: {}", upload_file.display() );
    
    Ok( () )
  }
  
  fn simulate_request_handling( &self ) -> Result< (), Box< dyn std::error::Error > >
  {
    println!( "   🌐 simulating request handling..." );
    
    // simulate different request types and their handling
    let requests = vec!
    [
      ( "GET", "/", "serve home page" ),
      ( "GET", "/static/css/main.css", "serve static css" ),
      ( "GET", "/static/js/app.js", "serve static js" ),  
      ( "GET", "/api/data", "serve json api response" ),
      ( "POST", "/upload", "handle file upload" ),
      ( "GET", "/admin/logs", "serve log files" ),
    ];
    
    for ( method, path, description ) in requests
    {
      let response = self.handle_request( method, path )?;
      println!( "     {} {} -> {} ({})", method, path, response, description );
    }
    
    Ok( () )
  }
  
  fn handle_request( &self, method : &str, path : &str ) -> Result< String, Box< dyn std::error::Error > >
  {
    match ( method, path )
    {
      ( "GET", "/" ) =>
      {
        let template_path = self.workspace.join( "templates/home.html" );
        if template_path.exists()
        {
          Ok( "200 ok (rendered template)".to_string() )
        }
        else
        {
          Ok( "404 not found".to_string() )
        }
      }
      
      ( "GET", static_path ) if static_path.starts_with( "/static/" ) =>
      {
        let file_path = self.workspace.join( &static_path[ 1.. ] ); // remove leading /
        if file_path.exists()
        {
          let size = fs::metadata( &file_path )?.len();
          Ok( format!( "200 ok ({} bytes, cache: {}s)", size, self.config.static_cache_ttl ) )
        }
        else
        {
          Ok( "404 not found".to_string() )
        }
      }
      
      ( "GET", "/api/data" ) =>
      {
        // simulate api response generation
        let data_file = self.workspace.data_dir().join( "api_data.json" );
        let api_data = r#"{"status": "ok", "data": ["item1", "item2", "item3"], "timestamp": "2024-01-01T00:00:00Z"}"#;
        fs::write( &data_file, api_data )?;
        Ok( "200 ok (json response)".to_string() )
      }
      
      ( "POST", "/upload" ) =>
      {
        let uploads_dir = self.workspace.join( "uploads" );
        if uploads_dir.exists()
        {
          Ok( format!( "200 ok (max size: {}mb)", self.config.upload_max_size_mb ) )
        }
        else
        {
          Ok( "500 server error".to_string() )
        }
      }
      
      ( "GET", "/admin/logs" ) =>
      {
        let logs_dir = self.workspace.logs_dir();
        if logs_dir.exists()
        {
          Ok( "200 ok (log files served)".to_string() )
        }
        else
        {
          Ok( "404 not found".to_string() )
        }
      }
      
      _ => Ok( "404 not found".to_string() ),
    }
  }
  
  fn demonstrate_uploads( &self ) -> Result< (), Box< dyn std::error::Error > >
  {
    println!( "   📤 demonstrating upload handling..." );
    
    let uploads_dir = self.workspace.join( "uploads" );
    
    // simulate file uploads
    let demo_uploads = vec!
    [
      ( "user_avatar.jpg", b"fake-jpeg-data" as &[ u8 ] ),
      ( "document.pdf", b"fake-pdf-data" ),
      ( "data_export.csv", b"id,name,value\n1,alice,100\n2,bob,200" ),
    ];
    
    for ( filename, data ) in demo_uploads
    {
      let upload_path = uploads_dir.join( filename );
      fs::write( &upload_path, data )?;
      
      let size = data.len();
      let size_mb = size as f64 / 1024.0 / 1024.0;
      
      if size_mb > self.config.upload_max_size_mb as f64
      {
        println!( "     ❌ {} rejected: {:.2}mb > {}mb limit", 
          filename, size_mb, self.config.upload_max_size_mb
        );
        fs::remove_file( &upload_path )?; // reject the upload
      }
      else
      {
        println!( "     ✅ {} accepted: {:.2}mb", filename, size_mb );
      }
    }
    
    Ok( () )
  }
  
  fn show_deployment_config( &self ) -> Result< (), Box< dyn std::error::Error > >
  {
    println!( "   🚀 generating deployment configurations..." );
    
    // docker configuration
    let dockerfile = format!( r#"FROM rust:alpine

# set workspace environment
ENV WORKSPACE_PATH=/app
ENV ENVIRONMENT=production

WORKDIR /app

# copy application
COPY . .

# build application
RUN cargo build --release

# create required directories
RUN mkdir -p config data logs static templates uploads cache sessions

# expose port
EXPOSE {}

# run application
CMD ["./target/release/{}"]
"#, self.config.port, self.config.name.replace( "-", "_" ) );
    
    let dockerfile_path = self.workspace.join( "dockerfile" );
    fs::write( &dockerfile_path, dockerfile )?;
    println!( "     created: {}", dockerfile_path.display() );
    
    // docker compose
    let compose = format!( r#"version: '3.8'
services:
  web:
    build: .
    ports:
      - "{}:{}"
    environment:
      - WORKSPACE_PATH=/app
      - ENVIRONMENT=production
    volumes:
      - ./data:/app/data
      - ./logs:/app/logs
      - ./uploads:/app/uploads
      - ./config:/app/config:ro
    restart: unless-stopped
    
  db:
    image: postgres:15
    environment:
      - POSTGRES_DB=app
      - POSTGRES_USER=app
      - POSTGRES_PASSWORD_FILE=/run/secrets/db_password
    volumes:
      - postgres_data:/var/lib/postgresql/data
    secrets:
      - db_password
      
volumes:
  postgres_data:
  
secrets:
  db_password:
    file: ./.secret/-production.sh
"#, self.config.port, self.config.port );
    
    let compose_path = self.workspace.join( "docker-compose.yml" );
    fs::write( &compose_path, compose )?;
    println!( "     created: {}", compose_path.display() );
    
    // nginx configuration
    let nginx = format!( r#"server {{
    listen 80;
    server_name example.com;
    
    # static files
    location /static/ {{
        alias /app/static/;
        expires {}s;
        add_header Cache-Control "public, immutable";
    }}
    
    # uploads (with access control)
    location /uploads/ {{
        alias /app/uploads/;
        expires 24h;
        # add authentication check here
    }}
    
    # application
    location / {{
        proxy_pass http://127.0.0.1:{};
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }}
}}
"#, self.config.static_cache_ttl, self.config.port );
    
    let nginx_path = self.workspace.join( "nginx.conf" );
    fs::write( &nginx_path, nginx )?;
    println!( "     created: {}", nginx_path.display() );
    
    Ok( () )
  }
  
  fn cleanup( &self ) -> Result< (), Box< dyn std::error::Error > >
  {
    println!( "\n3️⃣  cleaning up demo files..." );
    
    let cleanup_dirs = vec!
    [
      "static", "templates", "uploads", "media", "cache", "sessions", "data", "logs"
    ];
    
    for dir_name in cleanup_dirs
    {
      let dir_path = self.workspace.join( dir_name );
      if dir_path.exists()
      {
        fs::remove_dir_all( &dir_path )?;
        println!( "   removed: {}", dir_path.display() );
      }
    }
    
    let cleanup_files = vec![ "dockerfile", "docker-compose.yml", "nginx.conf" ];
    for file_name in cleanup_files
    {
      let file_path = self.workspace.join( file_name );
      if file_path.exists()
      {
        fs::remove_file( &file_path )?;
        println!( "   removed: {}", file_path.display() );
      }
    }
    
    // clean up config files
    let config_files = vec![ "development.toml", "production.toml" ];
    for config_file in config_files
    {
      let config_path = self.workspace.config_dir().join( config_file );
      if config_path.exists()
      {
        fs::remove_file( &config_path )?;
        println!( "   removed: {}", config_path.display() );
      }
    }
    
    println!( "   ✅ cleanup completed" );
    
    Ok( () )
  }
  
  // utility methods
  
  fn create_default_config( environment : &str ) -> ServiceConfig
  {
    let mut config = ServiceConfig::default();
    config.environment = environment.to_string();
    
    // adjust defaults based on environment
    match environment
    {
      "production" =>
      {
        config.host = "0.0.0.0".to_string();
        config.static_cache_ttl = 86400; // 24 hours
        config.upload_max_size_mb = 50;
      }
      "staging" =>
      {
        config.port = 8081;
        config.static_cache_ttl = 3600; // 1 hour
        config.upload_max_size_mb = 25;
      }
      _ => {} // development defaults
    }
    
    config
  }
  
  fn parse_config( content : &str, environment : &str ) -> Result< ServiceConfig, Box< dyn std::error::Error > >
  {
    let mut config = Self::create_default_config( environment );
    
    for line in content.lines()
    {
      if let Some( ( key, value ) ) = line.split_once( " = " )
      {
        let key = key.trim();
        let value = value.trim().trim_matches( '"' );
        
        match key
        {
          "name" => config.name = value.to_string(),
          "host" => config.host = value.to_string(),
          "port" => config.port = value.parse().unwrap_or( 8080 ),
          "static_cache_ttl" => config.static_cache_ttl = value.parse().unwrap_or( 3600 ),
          "upload_max_size_mb" => config.upload_max_size_mb = value.parse().unwrap_or( 10 ),
          _ => {}
        }
      }
    }
    
    Ok( config )
  }
  
  fn config_to_toml( config : &ServiceConfig ) -> String
  {
    format!( r#"# web service configuration - {} environment
name = "{}"
host = "{}"  
port = {}
static_cache_ttl = {}
upload_max_size_mb = {}
"#, 
      config.environment, config.name, config.host, config.port, 
      config.static_cache_ttl, config.upload_max_size_mb
    )
  }
}