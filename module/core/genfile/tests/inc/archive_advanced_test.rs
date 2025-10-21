/// Advanced archive tests - pack/unpack, internalize/externalize
///
/// # Test Coverage
///
/// - Materialize with ContentStorage
/// - Pack archive from directory tree
/// - Internalize external references
/// - Externalize inline content
/// - Save/load archive files
/// - Round-trip conversions

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
  IntoContentSource,
  FileRef,
  UrlRef,
};
use std::path::{ Path, PathBuf };
use std::collections::HashMap;

//

/// Mock storage for testing ContentStorage
struct MockStorage
{
  pub stored: HashMap< PathBuf, FileContent >,
}

impl MockStorage
{
  fn new() -> Self
  {
    Self
    {
      stored: HashMap::new(),
    }
  }

  fn get( &self, path: &Path ) -> Option< &FileContent >
  {
    self.stored.get( path )
  }
}

impl ContentStorage for MockStorage
{
  fn store( &mut self, path: &Path, content: &FileContent ) -> Result< (), Error >
  {
    self.stored.insert( path.to_path_buf(), content.clone() );
    Ok( () )
  }
}

#[ test ]
fn materialize_with_storage()
{
  let mut archive = TemplateArchive::new( "test" );

  archive.add_text_file(
    PathBuf::from( "output.txt" ),
    "Value: {{num}}",
    WriteMode::Rewrite
  );

  archive.add_binary_file(
    PathBuf::from( "data.bin" ),
    vec![ 0x00, 0xFF, 0x42 ]
  );

  archive.set_value( "num", Value::Number( 100 ) );

  let renderer = HandlebarsRenderer::new();
  let mut storage = MockStorage::new();
  let resolver = DefaultContentResolver::new();

  archive.materialize_with_storage(
    Path::new( "/out" ),
    &renderer,
    &mut storage,
    &resolver
  ).unwrap();

  // Verify storage received files
  assert_eq!( storage.stored.len(), 2 );

  let text_content = storage.get( Path::new( "/out/output.txt" ) ).unwrap();
  match text_content
  {
    FileContent::Text( s ) => assert_eq!( s, "Value: 100" ),
    _ => panic!( "Expected text content" ),
  }

  let binary_content = storage.get( Path::new( "/out/data.bin" ) ).unwrap();
  match binary_content
  {
    FileContent::Binary( b ) => assert_eq!( b, &vec![ 0x00, 0xFF, 0x42 ] ),
    _ => panic!( "Expected binary content" ),
  }
}

//

/// Mock resolver for testing internalize
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

  fn add( &mut self, key: impl Into< String >, content: FileContent )
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
          Error::Render( format!( "Not found: {}", key ) )
        })
      }

      ContentSource::Url { url } =>
      {
        self.responses.get( url ).cloned().ok_or_else( ||
        {
          Error::Render( format!( "Not found: {}", url ) )
        })
      }
    }
  }
}

#[ test ]
fn internalize_external_sources()
{
  let mut archive = TemplateArchive::new( "test" );

  // Add files with external sources
  archive.add_file_from(
    PathBuf::from( "file1.txt" ),
    FileRef::new( PathBuf::from( "/templates/t1.hbs" ) ),
    WriteMode::Rewrite
  );

  archive.add_file_from(
    PathBuf::from( "file2.txt" ),
    UrlRef::new( "https://example.com/t2.hbs" ),
    WriteMode::Rewrite
  );

  // Also have inline content
  archive.add_text_file(
    PathBuf::from( "file3.txt" ),
    "Inline content",
    WriteMode::Rewrite
  );

  // Setup resolver
  let mut resolver = MockResolver::new();
  resolver.add( "/templates/t1.hbs", FileContent::Text( "From file".into() ) );
  resolver.add( "https://example.com/t2.hbs", FileContent::Text( "From URL".into() ) );

  // Verify external sources before
  assert!( archive.get_file( Path::new( "file1.txt" ) ).unwrap().content_source.is_some() );
  assert!( archive.get_file( Path::new( "file2.txt" ) ).unwrap().content_source.is_some() );
  assert!( archive.get_file( Path::new( "file3.txt" ) ).unwrap().content_source.is_none() );

  // Internalize
  archive.internalize( &resolver ).unwrap();

  // Verify all sources are now inline
  let file1 = archive.get_file( Path::new( "file1.txt" ) ).unwrap();
  assert!( file1.content_source.is_none() );
  match &file1.content
  {
    FileContent::Text( s ) => assert_eq!( s, "From file" ),
    _ => panic!( "Expected text" ),
  }

  let file2 = archive.get_file( Path::new( "file2.txt" ) ).unwrap();
  assert!( file2.content_source.is_none() );
  match &file2.content
  {
    FileContent::Text( s ) => assert_eq!( s, "From URL" ),
    _ => panic!( "Expected text" ),
  }

  let file3 = archive.get_file( Path::new( "file3.txt" ) ).unwrap();
  assert!( file3.content_source.is_none() );
  match &file3.content
  {
    FileContent::Text( s ) => assert_eq!( s, "Inline content" ),
    _ => panic!( "Expected text" ),
  }
}

#[ test ]
fn internalize_missing_source_error()
{
  let mut archive = TemplateArchive::new( "test" );

  archive.add_file_from(
    PathBuf::from( "missing.txt" ),
    FileRef::new( PathBuf::from( "/nonexistent/file.hbs" ) ),
    WriteMode::Rewrite
  );

  let resolver = MockResolver::new();

  // Should fail because source doesnt exist
  let result = archive.internalize( &resolver );
  assert!( result.is_err() );
}

//

#[ test ]
fn serialize_after_internalize()
{
  let mut archive = TemplateArchive::new( "test" );

  archive.add_file_from(
    PathBuf::from( "data.txt" ),
    UrlRef::new( "https://example.com/data.txt" ),
    WriteMode::Rewrite
  );

  let mut resolver = MockResolver::new();
  resolver.add( "https://example.com/data.txt", FileContent::Text( "Content".into() ) );

  archive.internalize( &resolver ).unwrap();

  // Serialize should not include content_source
  let json = archive.to_json().unwrap();
  assert!( !json.contains( "content_source" ) );
  assert!( json.contains( "Content" ) );
}

//

#[ test ]
fn mixed_inline_and_external_internalize()
{
  let mut archive = TemplateArchive::new( "test" );

  archive.add_text_file(
    PathBuf::from( "inline.txt" ),
    "Inline {{var}}",
    WriteMode::Rewrite
  );

  archive.add_file_from(
    PathBuf::from( "external.txt" ),
    FileRef::new( PathBuf::from( "/ext.hbs" ) ),
    WriteMode::Rewrite
  );

  let mut resolver = MockResolver::new();
  resolver.add( "/ext.hbs", FileContent::Text( "External {{var}}".into() ) );

  archive.internalize( &resolver ).unwrap();

  assert_eq!( archive.file_count(), 2 );

  // Both should be inline now
  for file_path in &[ "inline.txt", "external.txt" ]
  {
    let file = archive.get_file( Path::new( file_path ) ).unwrap();
    assert!( file.content_source.is_none() );
  }
}

//

#[ test ]
fn internalize_binary_content()
{
  let mut archive = TemplateArchive::new( "test" );

  archive.add_file_from(
    PathBuf::from( "image.png" ),
    FileRef::new( PathBuf::from( "/images/logo.png" ) ),
    WriteMode::Rewrite
  );

  let mut resolver = MockResolver::new();
  resolver.add(
    "/images/logo.png",
    FileContent::Binary( vec![ 0x89, 0x50, 0x4E, 0x47 ] )
  );

  archive.internalize( &resolver ).unwrap();

  let file = archive.get_file( Path::new( "image.png" ) ).unwrap();
  assert!( file.content_source.is_none() );

  match &file.content
  {
    FileContent::Binary( b ) => assert_eq!( b, &vec![ 0x89, 0x50, 0x4E, 0x47 ] ),
    _ => panic!( "Expected binary" ),
  }
}

//

#[ test ]
fn count_external_sources()
{
  let mut archive = TemplateArchive::new( "test" );

  archive.add_text_file( PathBuf::from( "a.txt" ), "inline", WriteMode::Rewrite );
  archive.add_file_from( PathBuf::from( "b.txt" ), FileRef::new( PathBuf::from( "/b" ) ), WriteMode::Rewrite );
  archive.add_file_from( PathBuf::from( "c.txt" ), UrlRef::new( "http://c" ), WriteMode::Rewrite );

  let external_count = archive.list_files().iter()
    .filter( | p | archive.get_file( p ).unwrap().content_source.is_some() )
    .count();

  assert_eq!( external_count, 2 );
}

//

#[ test ]
fn internalize_preserves_metadata()
{
  use genfile_core::{ FileMetadata, TemplateFile };

  let mut archive = TemplateArchive::new( "test" );

  // Add file with metadata and external source
  archive.files.push( TemplateFile
  {
    path: PathBuf::from( "script.sh" ),
    content: FileContent::Text( String::new() ),
    write_mode: WriteMode::Rewrite,
    metadata: Some( FileMetadata
    {
      permissions: Some( 0o755 ),
      is_template: true,
      comment: Some( "Executable script".into() ),
    }),
    content_source: Some( FileRef::new( PathBuf::from( "/templates/script.sh" ) ).into_content_source() ),
  });

  let mut resolver = MockResolver::new();
  resolver.add( "/templates/script.sh", FileContent::Text( "#!/bin/bash".into() ) );

  archive.internalize( &resolver ).unwrap();

  let file = archive.get_file( Path::new( "script.sh" ) ).unwrap();

  // Verify metadata preserved
  assert!( file.metadata.is_some() );
  let metadata = file.metadata.as_ref().unwrap();
  assert_eq!( metadata.permissions, Some( 0o755 ) );
  assert!( metadata.is_template );
  assert_eq!( metadata.comment, Some( "Executable script".to_string() ) );

  // Verify content internalized
  assert!( file.content_source.is_none() );
  match &file.content
  {
    FileContent::Text( s ) => assert_eq!( s, "#!/bin/bash" ),
    _ => panic!( "Expected text" ),
  }
}
