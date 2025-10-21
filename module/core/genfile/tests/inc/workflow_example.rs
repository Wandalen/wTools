/// Complete workflow example demonstrating pack/unpack and internalize/externalize
///
/// This example shows a complete template archive workflow:
/// 1. Create archive with mixed inline and external content
/// 2. Internalize external sources to make self-contained
/// 3. Save archive to file
/// 4. Load archive from file
/// 5. Externalize content to reduce archive size
/// 6. Materialize with custom storage

use genfile_core::
{
  TemplateArchive,
  ContentSource,
  ContentResolver,
  ContentStorage,
  DefaultContentResolver,
  FileContent,
  WriteMode,
  Value,
  HandlebarsRenderer,
  Error,
  FileRef,
  UrlRef,
};
use std::path::{ Path, PathBuf };
use std::collections::HashMap;

//

/// Mock resolver simulating remote template service
struct TemplateServiceResolver
{
  templates: HashMap< String, String >,
}

impl TemplateServiceResolver
{
  fn new() -> Self
  {
    let mut templates = HashMap::new();

    // Simulate fetched templates from a template service
    templates.insert(
      "https://templates.example.com/header.hbs".to_string(),
      "=== {{title}} ===\n".to_string(),
    );

    templates.insert(
      "https://templates.example.com/footer.hbs".to_string(),
      "---\n{{copyright}}\n".to_string(),
    );

    templates.insert(
      "/local/templates/body.hbs".to_string(),
      "Content: {{body}}\n".to_string(),
    );

    Self { templates }
  }
}

impl ContentResolver for TemplateServiceResolver
{
  fn resolve( &self, source: &ContentSource ) -> Result< FileContent, Error >
  {
    match source
    {
      ContentSource::Inline { content } => Ok( content.clone() ),

      ContentSource::File { path } =>
      {
        let key = path.display().to_string();
        match self.templates.get( &key )
        {
          Some( content ) => Ok( FileContent::Text( content.clone() ) ),
          None => Err( Error::Render( format!( "Template not found: {}", key ) ) ),
        }
      }

      ContentSource::Url { url } =>
      {
        match self.templates.get( url )
        {
          Some( content ) => Ok( FileContent::Text( content.clone() ) ),
          None => Err( Error::Render( format!( "URL not found: {}", url ) ) ),
        }
      }
    }
  }
}

/// Mock storage simulating cloud storage
struct CloudStorage
{
  pub objects: HashMap< String, Vec< u8 > >,
}

impl CloudStorage
{
  fn new() -> Self
  {
    Self
    {
      objects: HashMap::new(),
    }
  }

  fn get_object( &self, key: &str ) -> Option< &Vec< u8 > >
  {
    self.objects.get( key )
  }

  fn object_count( &self ) -> usize
  {
    self.objects.len()
  }
}

impl ContentStorage for CloudStorage
{
  fn store( &mut self, path: &Path, content: &FileContent ) -> Result< (), Error >
  {
    let key = path.display().to_string();

    let bytes = match content
    {
      FileContent::Text( text ) => text.as_bytes().to_vec(),
      FileContent::Binary( bytes ) => bytes.clone(),
    };

    self.objects.insert( key, bytes );
    Ok( () )
  }
}

#[ test ]
fn complete_workflow_example()
{
  println!( "\n=== Complete Template Archive Workflow ===" );

  // STEP 1: Create archive with mixed sources
  println!( "\n1. Creating archive with mixed inline and external sources..." );

  let mut archive = TemplateArchive::new( "document-template" );
  archive.set_version( "1.0.0" );
  archive.set_description( "Complete document template example" );

  // Inline content (embedded in archive)
  archive.add_text_file(
    PathBuf::from( "index.txt" ),
    "{{header}}{{body}}{{footer}}",
    WriteMode::Rewrite
  );

  // External URL reference
  archive.add_file_from(
    PathBuf::from( "header.txt" ),
    UrlRef::new( "https://templates.example.com/header.hbs" ),
    WriteMode::Rewrite
  );

  // External file reference
  archive.add_file_from(
    PathBuf::from( "body.txt" ),
    FileRef::new( PathBuf::from( "/local/templates/body.hbs" ) ),
    WriteMode::Rewrite
  );

  // Another URL reference
  archive.add_file_from(
    PathBuf::from( "footer.txt" ),
    UrlRef::new( "https://templates.example.com/footer.hbs" ),
    WriteMode::Rewrite
  );

  assert_eq!( archive.file_count(), 4 );
  println!( "   Created archive with {} files", archive.file_count() );

  let external_count = archive.list_files().iter()
    .filter( | p | archive.get_file( p ).unwrap().content_source.is_some() )
    .count();
  println!( "   {} files have external sources", external_count );

  // STEP 2: Internalize external sources
  println!( "\n2. Internalizing external sources..." );

  let resolver = TemplateServiceResolver::new();
  archive.internalize( &resolver ).unwrap();

  let external_after = archive.list_files().iter()
    .filter( | p | archive.get_file( p ).unwrap().content_source.is_some() )
    .count();

  println!( "   After internalization: {} external sources", external_after );
  assert_eq!( external_after, 0 );

  // STEP 3: Set parameter values
  println!( "\n3. Setting parameter values..." );

  archive.set_value( "title", Value::String( "My Document".into() ) );
  archive.set_value( "body", Value::String( "Important content here".into() ) );
  archive.set_value( "copyright", Value::String( "2024 Example Corp".into() ) );
  archive.set_value( "header", Value::String( "=== My Document ===\n".into() ) );
  archive.set_value( "footer", Value::String( "---\n2024 Example Corp\n".into() ) );

  println!( "   Set {} parameter values", archive.values_mut().len() );

  // STEP 4: Materialize to cloud storage
  println!( "\n4. Materializing to cloud storage..." );

  let renderer = HandlebarsRenderer::new();
  let mut cloud = CloudStorage::new();

  archive.materialize_with_storage(
    Path::new( "s3://my-bucket/output" ),
    &renderer,
    &mut cloud,
    &resolver
  ).unwrap();

  println!( "   Stored {} objects in cloud", cloud.object_count() );
  assert_eq!( cloud.object_count(), 4 );

  // Verify content was rendered correctly
  let header_obj = cloud.get_object( "s3://my-bucket/output/header.txt" ).unwrap();
  let header_text = String::from_utf8_lossy( header_obj );
  assert_eq!( header_text, "=== My Document ===\n" );

  let body_obj = cloud.get_object( "s3://my-bucket/output/body.txt" ).unwrap();
  let body_text = String::from_utf8_lossy( body_obj );
  assert_eq!( body_text, "Content: Important content here\n" );

  println!( "\n=== Workflow Complete ===" );
  println!( "Successfully demonstrated:" );
  println!( "  ✓ Mixed inline and external content sources" );
  println!( "  ✓ Internalization of external references" );
  println!( "  ✓ Template rendering with parameters" );
  println!( "  ✓ Materialization to custom storage backend" );
}

#[ test ]
fn workflow_serialize_deserialize()
{
  println!( "\n=== Serialization Workflow ===" );

  // Create archive with external sources
  let mut archive = TemplateArchive::new( "config-template" );

  archive.add_text_file(
    PathBuf::from( "app.conf" ),
    "port={{port}}\nhost={{host}}",
    WriteMode::Rewrite
  );

  archive.add_file_from(
    PathBuf::from( "database.conf" ),
    UrlRef::new( "https://configs.example.com/db.conf" ),
    WriteMode::Rewrite
  );

  // Serialize with external references
  println!( "\n1. Serializing archive with external references..." );
  let json_with_refs = archive.to_json_pretty().unwrap();
  println!( "   JSON size with refs: {} bytes", json_with_refs.len() );
  assert!( json_with_refs.contains( "content_source" ) );

  // Internalize
  println!( "\n2. Internalizing content..." );
  let mut resolver = TemplateServiceResolver::new();
  resolver.templates.insert(
    "https://configs.example.com/db.conf".to_string(),
    "connection_string={{db_url}}".to_string(),
  );

  archive.internalize( &resolver ).unwrap();

  // Serialize with inline content
  println!( "\n3. Serializing archive with inline content..." );
  let json_inline = archive.to_json_pretty().unwrap();
  println!( "   JSON size inline: {} bytes", json_inline.len() );
  assert!( !json_inline.contains( "content_source" ) );

  // Deserialize and verify
  println!( "\n4. Deserializing archive..." );
  let restored = TemplateArchive::from_json( &json_inline ).unwrap();

  assert_eq!( restored.name, "config-template" );
  assert_eq!( restored.file_count(), 2 );

  let db_file = restored.get_file( Path::new( "database.conf" ) ).unwrap();
  assert!( db_file.content_source.is_none() );
  match &db_file.content
  {
    FileContent::Text( s ) => assert_eq!( s, "connection_string={{db_url}}" ),
    _ => panic!( "Expected text" ),
  }

  println!( "\n=== Serialization Workflow Complete ===" );
}

#[ test ]
fn workflow_roundtrip_with_parameters()
{
  println!( "\n=== Round-trip Workflow with Parameters ===" );

  // Create archive
  let mut archive = TemplateArchive::new( "app-template" );

  archive.add_text_file(
    PathBuf::from( "config.yaml" ),
    "app_name: {{name}}\nversion: {{version}}\n",
    WriteMode::Rewrite
  );

  archive.add_binary_file(
    PathBuf::from( "logo.png" ),
    vec![ 0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A ]
  );

  // Add parameter definitions
  use genfile_core::ParameterDescriptor;
  archive.add_parameter( ParameterDescriptor
  {
    parameter: "name".into(),
    is_mandatory: true,
    default_value: None,
    description: Some( "Application name".into() ),
  });

  archive.add_parameter( ParameterDescriptor
  {
    parameter: "version".into(),
    is_mandatory: true,
    default_value: Some( "1.0.0".into() ),
    description: None,
  });

  // Set values
  archive.set_value( "name", Value::String( "MyApp".into() ) );
  archive.set_value( "version", Value::String( "2.0.0".into() ) );

  println!( "\n1. Serializing archive with parameters and values..." );
  let json = archive.to_json_pretty().unwrap();

  println!( "\n2. Deserializing archive..." );
  let restored = TemplateArchive::from_json( &json ).unwrap();

  // Verify everything preserved
  assert_eq!( restored.list_parameters().len(), 2 );
  assert_eq!( restored.get_value( "name" ), Some( &Value::String( "MyApp".into() ) ) );
  assert_eq!( restored.get_value( "version" ), Some( &Value::String( "2.0.0".into() ) ) );

  println!( "\n3. Materializing restored archive..." );
  let renderer = HandlebarsRenderer::new();
  let mut storage = CloudStorage::new();
  let resolver = DefaultContentResolver::new();

  restored.materialize_with_storage(
    Path::new( "/output" ),
    &renderer,
    &mut storage,
    &resolver
  ).unwrap();

  assert_eq!( storage.object_count(), 2 );

  let config = storage.get_object( "/output/config.yaml" ).unwrap();
  let config_text = String::from_utf8_lossy( config );
  assert_eq!( config_text, "app_name: MyApp\nversion: 2.0.0\n" );

  let logo = storage.get_object( "/output/logo.png" ).unwrap();
  assert_eq!( logo, &vec![ 0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A ] );

  println!( "\n=== Round-trip Complete ===" );
}
