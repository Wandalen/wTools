/// Tests for ContentSource and external data references
///
/// # Test Coverage
///
/// - ContentSource creation (inline, file, URL)
/// - DefaultContentResolver for inline and file sources
/// - Custom ContentResolver implementations
/// - Archive with external file references
/// - Archive with URL references
/// - Materialization with content resolver
/// - Custom content storage implementations

use genfile_core::
{
  ContentSource,
  ContentResolver,
  ContentStorage,
  DefaultContentResolver,
  DefaultContentStorage,
  TemplateArchive,
  FileContent,
  WriteMode,
  Value,
  HandlebarsRenderer,
  MemoryFileSystem,
  FileSystem,
  Error,
  IntoContentSource,
  FileRef,
  UrlRef,
  InlineContent,
};
use std::path::{ Path, PathBuf };
use std::collections::HashMap;

//

#[ test ]
fn content_source_inline()
{
  let source = InlineContent::text( "Hello {{name}}" ).into_content_source();

  assert!( source.is_inline() );
  assert!( !source.is_file() );
  assert!( !source.is_url() );

  let content = source.as_inline().unwrap();
  match content
  {
    FileContent::Text( text ) => assert_eq!( text, "Hello {{name}}" ),
    _ => panic!( "Expected text content" ),
  }
}

#[ test ]
fn content_source_file()
{
  let source = FileRef::new( "/templates/main.hbs" ).into_content_source();

  assert!( !source.is_inline() );
  assert!( source.is_file() );
  assert!( !source.is_url() );

  let path = source.as_file_path().unwrap();
  assert_eq!( path, Path::new( "/templates/main.hbs" ) );
}

#[ test ]
fn content_source_url()
{
  let source = UrlRef::new( "https://example.com/template.hbs" ).into_content_source();

  assert!( !source.is_inline() );
  assert!( !source.is_file() );
  assert!( source.is_url() );

  let url = source.as_url().unwrap();
  assert_eq!( url, "https://example.com/template.hbs" );
}

//

#[ test ]
fn default_resolver_inline()
{
  let resolver = DefaultContentResolver::new();

  let source = ContentSource::Inline
  {
    content: FileContent::Text( "test content".into() ),
  };

  let content = resolver.resolve( &source ).unwrap();

  match content
  {
    FileContent::Text( text ) => assert_eq!( text, "test content" ),
    _ => panic!( "Expected text content" ),
  }
}

#[ test ]
fn default_resolver_url_not_supported()
{
  let resolver = DefaultContentResolver::new();

  let source = ContentSource::Url
  {
    url: "https://example.com/data.json".into(),
  };

  let result = resolver.resolve( &source );
  assert!( result.is_err() );

  // Should contain helpful error message
  let err = result.unwrap_err();
  match err
  {
    Error::Render( msg ) =>
    {
      assert!( msg.contains( "URL fetching not supported" ) );
      assert!( msg.contains( "https://example.com/data.json" ) );
    }
    _ => panic!( "Expected render error" ),
  }
}

//

/// Custom resolver for testing that returns predefined content
struct MockResolver
{
  responses: HashMap< String, FileContent >,
}

impl MockResolver
{
  fn new() -> Self
  {
    Self
    {
      responses: HashMap::new(),
    }
  }

  fn add_response( &mut self, key: impl Into< String >, content: FileContent )
  {
    self.responses.insert( key.into(), content );
  }
}

impl ContentResolver for MockResolver
{
  fn resolve( &self, source: &ContentSource ) -> Result< FileContent, Error >
  {
    match source
    {
      ContentSource::Inline { content } => Ok( content.clone() ),

      ContentSource::File { path } =>
      {
        let key = path.display().to_string();
        self.responses.get( &key ).cloned().ok_or_else( ||
        {
          Error::Render( format!( "Mock: file not found: {}", key ) )
        })
      }

      ContentSource::Url { url } =>
      {
        self.responses.get( url ).cloned().ok_or_else( ||
        {
          Error::Render( format!( "Mock: URL not found: {}", url ) )
        })
      }
    }
  }
}

#[ test ]
fn custom_resolver_file()
{
  let mut resolver = MockResolver::new();
  resolver.add_response(
    "/templates/test.hbs",
    FileContent::Text( "Mocked content {{var}}".into() )
  );

  let source = ContentSource::File
  {
    path: PathBuf::from( "/templates/test.hbs" ),
  };

  let content = resolver.resolve( &source ).unwrap();

  match content
  {
    FileContent::Text( text ) => assert_eq!( text, "Mocked content {{var}}" ),
    _ => panic!( "Expected text content" ),
  }
}

#[ test ]
fn custom_resolver_url()
{
  let mut resolver = MockResolver::new();
  resolver.add_response(
    "https://example.com/config.json",
    FileContent::Text( r#"{"key":"{{value}}"}"#.into() )
  );

  let source = ContentSource::Url
  {
    url: "https://example.com/config.json".into(),
  };

  let content = resolver.resolve( &source ).unwrap();

  match content
  {
    FileContent::Text( text ) => assert_eq!( text, r#"{"key":"{{value}}"}"# ),
    _ => panic!( "Expected text content" ),
  }
}

//

#[ test ]
fn archive_add_file_source()
{
  let mut archive = TemplateArchive::new( "test" );

  archive.add_file_from(
    PathBuf::from( "output.txt" ),
    FileRef::new( PathBuf::from( "/templates/main.hbs" ) ),
    WriteMode::Rewrite
  );

  assert_eq!( archive.file_count(), 1 );

  let file = archive.get_file( Path::new( "output.txt" ) ).unwrap();
  assert!( file.content_source.is_some() );

  let source = file.content_source.as_ref().unwrap();
  assert!( source.is_file() );
  assert_eq!( source.as_file_path().unwrap(), Path::new( "/templates/main.hbs" ) );
}

#[ test ]
fn archive_add_file_ref()
{
  let mut archive = TemplateArchive::new( "test" );

  archive.add_file_from(
    PathBuf::from( "README.md" ),
    FileRef::new( PathBuf::from( "/templates/readme.hbs" ) ),
    WriteMode::Rewrite
  );

  let file = archive.get_file( Path::new( "README.md" ) ).unwrap();
  let source = file.content_source.as_ref().unwrap();

  assert!( source.is_file() );
}

#[ test ]
fn archive_add_file_url()
{
  let mut archive = TemplateArchive::new( "test" );

  archive.add_file_from(
    PathBuf::from( "config.json" ),
    UrlRef::new( "https://example.com/config.json" ),
    WriteMode::Rewrite
  );

  let file = archive.get_file( Path::new( "config.json" ) ).unwrap();
  let source = file.content_source.as_ref().unwrap();

  assert!( source.is_url() );
  assert_eq!( source.as_url().unwrap(), "https://example.com/config.json" );
}

//

#[ test ]
fn materialize_with_resolver_inline()
{
  let mut archive = TemplateArchive::new( "test" );

  // Inline content (old behavior)
  archive.add_text_file(
    PathBuf::from( "output.txt" ),
    "Hello {{name}}!",
    WriteMode::Rewrite
  );

  archive.set_value( "name", Value::String( "World".into() ) );

  let renderer = HandlebarsRenderer::new();
  let mut filesystem = MemoryFileSystem::new();
  let resolver = DefaultContentResolver::new();

  archive.materialize_with_resolver(
    Path::new( "/out" ),
    &renderer,
    &mut filesystem,
    &resolver
  ).unwrap();

  let content = filesystem.read( &PathBuf::from( "/out/output.txt" ) ).unwrap();
  assert_eq!( content, "Hello World!" );
}

#[ test ]
fn materialize_with_resolver_external()
{
  let mut archive = TemplateArchive::new( "test" );

  // External content source
  archive.add_file_from(
    PathBuf::from( "greeting.txt" ),
    FileRef::new( PathBuf::from( "/templates/greeting.hbs" ) ),
    WriteMode::Rewrite
  );

  archive.set_value( "user", Value::String( "Alice".into() ) );

  // Mock resolver with template content
  let mut resolver = MockResolver::new();
  resolver.add_response(
    "/templates/greeting.hbs",
    FileContent::Text( "Hi {{user}}!".into() )
  );

  let renderer = HandlebarsRenderer::new();
  let mut filesystem = MemoryFileSystem::new();

  archive.materialize_with_resolver(
    Path::new( "/out" ),
    &renderer,
    &mut filesystem,
    &resolver
  ).unwrap();

  let content = filesystem.read( &PathBuf::from( "/out/greeting.txt" ) ).unwrap();
  assert_eq!( content, "Hi Alice!" );
}

#[ test ]
fn materialize_with_resolver_url_source()
{
  let mut archive = TemplateArchive::new( "test" );

  archive.add_file_from(
    PathBuf::from( "remote.txt" ),
    UrlRef::new( "https://example.com/template.hbs" ),
    WriteMode::Rewrite
  );

  archive.set_value( "msg", Value::String( "from URL".into() ) );

  // Mock resolver simulating URL fetch
  let mut resolver = MockResolver::new();
  resolver.add_response(
    "https://example.com/template.hbs",
    FileContent::Text( "Data {{msg}}".into() )
  );

  let renderer = HandlebarsRenderer::new();
  let mut filesystem = MemoryFileSystem::new();

  archive.materialize_with_resolver(
    Path::new( "/out" ),
    &renderer,
    &mut filesystem,
    &resolver
  ).unwrap();

  let content = filesystem.read( &PathBuf::from( "/out/remote.txt" ) ).unwrap();
  assert_eq!( content, "Data from URL" );
}

#[ test ]
fn materialize_mixed_sources()
{
  let mut archive = TemplateArchive::new( "test" );

  // Mix of inline and external sources
  archive.add_text_file(
    PathBuf::from( "inline.txt" ),
    "Inline: {{value}}",
    WriteMode::Rewrite
  );

  archive.add_file_from(
    PathBuf::from( "external.txt" ),
    FileRef::new( PathBuf::from( "/templates/ext.hbs" ) ),
    WriteMode::Rewrite
  );

  archive.set_value( "value", Value::Number( 42 ) );

  let mut resolver = MockResolver::new();
  resolver.add_response(
    "/templates/ext.hbs",
    FileContent::Text( "External: {{value}}".into() )
  );

  let renderer = HandlebarsRenderer::new();
  let mut filesystem = MemoryFileSystem::new();

  archive.materialize_with_resolver(
    Path::new( "/out" ),
    &renderer,
    &mut filesystem,
    &resolver
  ).unwrap();

  let inline_content = filesystem.read( &PathBuf::from( "/out/inline.txt" ) ).unwrap();
  assert_eq!( inline_content, "Inline: 42" );

  let external_content = filesystem.read( &PathBuf::from( "/out/external.txt" ) ).unwrap();
  assert_eq!( external_content, "External: 42" );
}

//

/// Custom storage for testing
struct MockStorage
{
  pub writes: Vec< ( PathBuf, FileContent ) >,
}

impl MockStorage
{
  fn new() -> Self
  {
    Self
    {
      writes: Vec::new(),
    }
  }
}

impl ContentStorage for MockStorage
{
  fn store( &mut self, path: &Path, content: &FileContent ) -> Result< (), Error >
  {
    self.writes.push( ( path.to_path_buf(), content.clone() ) );
    Ok( () )
  }
}

#[ test ]
fn custom_storage()
{
  let mut storage = MockStorage::new();

  storage.store(
    Path::new( "test.txt" ),
    &FileContent::Text( "content".into() )
  ).unwrap();

  assert_eq!( storage.writes.len(), 1 );
  assert_eq!( storage.writes[ 0 ].0, PathBuf::from( "test.txt" ) );

  match &storage.writes[ 0 ].1
  {
    FileContent::Text( text ) => assert_eq!( text, "content" ),
    _ => panic!( "Expected text content" ),
  }
}

#[ test ]
fn default_storage()
{
  let _storage = DefaultContentStorage::new();
  // Just verify it can be created
}

//

#[ test ]
fn json_serialization_with_content_source()
{
  let mut archive = TemplateArchive::new( "test" );

  archive.add_file_from(
    PathBuf::from( "output.txt" ),
    FileRef::new( PathBuf::from( "/templates/main.hbs" ) ),
    WriteMode::Rewrite
  );

  let json = archive.to_json().unwrap();

  // Verify content_source is serialized
  assert!( json.contains( "content_source" ) );
  assert!( json.contains( "source_type" ) );
  assert!( json.contains( "File" ) );

  // Deserialize and verify
  let restored = TemplateArchive::from_json( &json ).unwrap();
  assert_eq!( restored.file_count(), 1 );

  let file = restored.get_file( Path::new( "output.txt" ) ).unwrap();
  assert!( file.content_source.is_some() );

  let source = file.content_source.as_ref().unwrap();
  assert!( source.is_file() );
}

#[ test ]
fn yaml_serialization_with_content_source()
{
  let mut archive = TemplateArchive::new( "test" );

  archive.add_file_from(
    PathBuf::from( "data.json" ),
    UrlRef::new( "https://api.example.com/data.json" ),
    WriteMode::Rewrite
  );

  let yaml = archive.to_yaml().unwrap();

  // Verify content_source is serialized
  assert!( yaml.contains( "content_source" ) );
  assert!( yaml.contains( "source_type" ) );
  assert!( yaml.contains( "Url" ) );

  // Deserialize and verify
  let restored = TemplateArchive::from_yaml( &yaml ).unwrap();

  let file = restored.get_file( Path::new( "data.json" ) ).unwrap();
  let source = file.content_source.as_ref().unwrap();

  assert!( source.is_url() );
  assert_eq!( source.as_url().unwrap(), "https://api.example.com/data.json" );
}
