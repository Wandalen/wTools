/// Example demonstrating external content sources
///
/// This example shows how to use the content source abstraction to:
/// 1. Reference templates from external files
/// 2. Fetch content from URLs (with custom resolver)
/// 3. Mix inline and external content sources
/// 4. Implement custom content resolvers for databases, cloud storage, etc.

use genfile_core::
{
  TemplateArchive,
  ContentSource,
  ContentResolver,
  FileContent,
  WriteMode,
  Value,
  HandlebarsRenderer,
  MemoryFileSystem,
  FileSystem,
  Error,
  FileRef,
  UrlRef,
};
use std::path::{ Path, PathBuf };
use std::collections::HashMap;

/// Example: Custom HTTP resolver that simulates fetching templates from a web service
struct HttpResolver
{
  /// Cached responses (in real impl, would make HTTP requests)
  cache: HashMap< String, String >,
}

impl HttpResolver
{
  fn new() -> Self
  {
    let mut cache = HashMap::new();

    // Simulate some "fetched" templates
    cache.insert(
      "https://templates.example.com/header.hbs".to_string(),
      "=== {{title}} ===".to_string(),
    );

    cache.insert(
      "https://templates.example.com/footer.hbs".to_string(),
      "Copyright {{year}} {{company}}".to_string(),
    );

    Self { cache }
  }
}

impl ContentResolver for HttpResolver
{
  fn resolve( &self, source: &ContentSource ) -> Result< FileContent, Error >
  {
    match source
    {
      ContentSource::Inline { content } =>
      {
        Ok( content.clone() )
      }

      ContentSource::File { path } =>
      {
        // Read from filesystem
        match std::fs::read_to_string( path )
        {
          Ok( text ) => Ok( FileContent::Text( text ) ),
          Err( e ) => Err( Error::Fs( e ) ),
        }
      }

      ContentSource::Url { url } =>
      {
        // Simulate HTTP fetch from cache
        match self.cache.get( url )
        {
          Some( content ) => Ok( FileContent::Text( content.clone() ) ),
          None => Err( Error::Render( format!( "URL not found: {}", url ) ) ),
        }
      }
    }
  }
}

#[ test ]
fn example_external_content_sources()
{
  // Create archive
  let mut archive = TemplateArchive::new( "website-template" );
  archive.set_version( "1.0.0" );
  archive.set_description( "Website template with external sources" );

  // 1. Inline content (traditional approach)
  archive.add_text_file(
    PathBuf::from( "index.html" ),
    r#"<html>
  <body>{{content}}</body>
</html>"#,
    WriteMode::Rewrite
  );

  // 2. Reference template from URL
  archive.add_file_from(
    PathBuf::from( "header.txt" ),
    UrlRef::new( "https://templates.example.com/header.hbs" ),
    WriteMode::Rewrite
  );

  // 3. Reference template from URL
  archive.add_file_from(
    PathBuf::from( "footer.txt" ),
    UrlRef::new( "https://templates.example.com/footer.hbs" ),
    WriteMode::Rewrite
  );

  // Set parameter values
  archive.set_value( "content", Value::String( "Welcome!".into() ) );
  archive.set_value( "title", Value::String( "My Website".into() ) );
  archive.set_value( "year", Value::Number( 2024 ) );
  archive.set_value( "company", Value::String( "Acme Corp".into() ) );

  // Create custom HTTP resolver
  let resolver = HttpResolver::new();

  // Materialize with resolver
  let renderer = HandlebarsRenderer::new();
  let mut filesystem = MemoryFileSystem::new();

  let report = archive.materialize_with_resolver(
    Path::new( "/output" ),
    &renderer,
    &mut filesystem,
    &resolver
  ).unwrap();

  // Verify results
  assert_eq!( report.files_created.len(), 3 );

  let index = filesystem.read( &PathBuf::from( "/output/index.html" ) ).unwrap();
  assert!( index.contains( "<body>Welcome!</body>" ) );

  let header = filesystem.read( &PathBuf::from( "/output/header.txt" ) ).unwrap();
  assert_eq!( header, "=== My Website ===" );

  let footer = filesystem.read( &PathBuf::from( "/output/footer.txt" ) ).unwrap();
  assert_eq!( footer, "Copyright 2024 Acme Corp" );
}

/// Example: Custom database resolver
struct DatabaseResolver
{
  templates: HashMap< String, String >,
}

impl DatabaseResolver
{
  fn new() -> Self
  {
    let mut templates = HashMap::new();

    // Simulate database records
    templates.insert(
      "email_welcome".to_string(),
      "Hello {{username}}, welcome to {{app_name}}!".to_string(),
    );

    templates.insert(
      "email_reset".to_string(),
      "Reset your password: {{reset_link}}".to_string(),
    );

    Self { templates }
  }

  fn get_template( &self, id: &str ) -> Option< String >
  {
    self.templates.get( id ).cloned()
  }
}

impl ContentResolver for DatabaseResolver
{
  fn resolve( &self, source: &ContentSource ) -> Result< FileContent, Error >
  {
    match source
    {
      ContentSource::Inline { content } =>
      {
        Ok( content.clone() )
      }

      ContentSource::File { path } =>
      {
        // Interpret path as database template ID
        let template_id = path.file_stem()
          .and_then( | s | s.to_str() )
          .unwrap_or( "" );

        match self.get_template( template_id )
        {
          Some( content ) => Ok( FileContent::Text( content ) ),
          None => Err( Error::Render( format!( "Template not found in DB: {}", template_id ) ) ),
        }
      }

      ContentSource::Url { url } =>
      {
        Err( Error::Render( format!( "URLs not supported: {}", url ) ) )
      }
    }
  }
}

#[ test ]
fn example_database_templates()
{
  // Create archive with database template references
  let mut archive = TemplateArchive::new( "email-templates" );

  // Use "file" source type to reference database template IDs
  archive.add_file_from(
    PathBuf::from( "welcome.txt" ),
    FileRef::new( PathBuf::from( "email_welcome" ) ),  // Template ID in database
    WriteMode::Rewrite
  );

  archive.add_file_from(
    PathBuf::from( "reset.txt" ),
    FileRef::new( PathBuf::from( "email_reset" ) ),
    WriteMode::Rewrite
  );

  // Set values
  archive.set_value( "username", Value::String( "Alice".into() ) );
  archive.set_value( "app_name", Value::String( "MyApp".into() ) );
  archive.set_value( "reset_link", Value::String( "https://example.com/reset?token=xyz".into() ) );

  // Use database resolver
  let resolver = DatabaseResolver::new();
  let renderer = HandlebarsRenderer::new();
  let mut filesystem = MemoryFileSystem::new();

  archive.materialize_with_resolver(
    Path::new( "/emails" ),
    &renderer,
    &mut filesystem,
    &resolver
  ).unwrap();

  // Verify
  let welcome = filesystem.read( &PathBuf::from( "/emails/welcome.txt" ) ).unwrap();
  assert_eq!( welcome, "Hello Alice, welcome to MyApp!" );

  let reset = filesystem.read( &PathBuf::from( "/emails/reset.txt" ) ).unwrap();
  assert_eq!( reset, "Reset your password: https://example.com/reset?token=xyz" );
}

#[ test ]
fn example_serialization_with_external_sources()
{
  // Create archive with external sources
  let mut archive = TemplateArchive::new( "config-template" );

  archive.add_file_from(
    PathBuf::from( "settings.json" ),
    UrlRef::new( "https://config.example.com/settings.json" ),
    WriteMode::Rewrite
  );

  archive.add_file_from(
    PathBuf::from( "rules.yaml" ),
    FileRef::new( PathBuf::from( "/etc/templates/rules.yaml" ) ),
    WriteMode::Rewrite
  );

  // Serialize to JSON
  let json = archive.to_json_pretty().unwrap();

  println!( "Archive JSON:\n{}", json );

  // Verify it contains source references
  assert!( json.contains( "content_source" ) );
  assert!( json.contains( "https://config.example.com/settings.json" ) );
  assert!( json.contains( "/etc/templates/rules.yaml" ) );

  // Deserialize and verify
  let restored = TemplateArchive::from_json( &json ).unwrap();
  assert_eq!( restored.file_count(), 2 );

  // Verify sources are preserved
  let settings_file = restored.get_file( Path::new( "settings.json" ) ).unwrap();
  assert!( settings_file.content_source.as_ref().unwrap().is_url() );

  let rules_file = restored.get_file( Path::new( "rules.yaml" ) ).unwrap();
  assert!( rules_file.content_source.as_ref().unwrap().is_file() );
}
