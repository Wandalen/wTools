/// Content source abstraction for external data references
///
/// This module provides abstractions for retrieving and storing file content
/// from various sources (inline, filesystem, remote URLs, custom providers).
/// This allows archive data to be stored externally while maintaining
/// references in the archive structure.
use std::path::{ Path, PathBuf };
use serde::{ Serialize, Deserialize };
use crate::{ FileContent, Error };

/// Source of file content - inline or external reference.
///
/// Represents where file content comes from, allowing flexibility in
/// how archives store their data. Content can be embedded directly,
/// referenced from filesystem, fetched from URLs, or retrieved via
/// custom user-defined mechanisms.
///
/// # Variants
///
/// - `Inline`: Content embedded directly in the archive (default behavior)
/// - `File`: Reference to a file in the local filesystem
/// - `Url`: Reference to a remote resource (HTTP/HTTPS)
///
/// # Examples
///
/// ```rust
/// use genfile_core::{ ContentSource, FileContent };
/// use std::path::PathBuf;
///
/// // Inline content
/// let inline = ContentSource::Inline
/// {
///   content: FileContent::Text( "Hello {{name}}".into() ),
/// };
///
/// // File reference
/// let file_ref = ContentSource::File
/// {
///   path: PathBuf::from( "/templates/main.hbs" ),
/// };
///
/// // Remote URL
/// let remote = ContentSource::Url
/// {
///   url: "https://example.com/template.hbs".into(),
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "source_type")]
pub enum ContentSource
{
  /// Content embedded directly in archive
  Inline
  {
    /// The actual file content
    content: FileContent,
  },

  /// Reference to file in local filesystem
  File
  {
    /// Path to the file
    path: PathBuf,
  },

  /// Reference to remote resource
  Url
  {
    /// URL to fetch content from
    url: String,
  },
}

/// Trait for types that can be converted into a `ContentSource`.
///
/// Implement this trait to create custom content source types that work
/// seamlessly with the archive API.
///
/// # Examples
///
/// ```rust
/// use genfile_core::{ IntoContentSource, ContentSource, FileContent };
///
/// struct CustomSource(String);
///
/// impl IntoContentSource for CustomSource
/// {
///   fn into_content_source( self ) -> ContentSource
///   {
///     ContentSource::Inline
///     {
///       content: FileContent::Text( self.0 ),
///     }
///   }
/// }
/// ```
pub trait IntoContentSource
{
  /// Convert self into a `ContentSource`
  fn into_content_source( self ) -> ContentSource;
}

impl IntoContentSource for ContentSource
{
  fn into_content_source( self ) -> ContentSource
  {
    self
  }
}

/// File reference source type.
///
/// Represents a reference to a file in the local filesystem.
///
/// # Examples
///
/// ```rust
/// use genfile_core::FileRef;
/// use std::path::PathBuf;
///
/// let source = FileRef::new( "/templates/main.hbs" );
/// let source = FileRef( PathBuf::from( "/templates/main.hbs" ) );
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileRef( pub PathBuf );

impl FileRef
{
  /// Create a new file reference
  pub fn new( path: impl Into< PathBuf > ) -> Self
  {
    Self( path.into() )
  }
}

impl IntoContentSource for FileRef
{
  fn into_content_source( self ) -> ContentSource
  {
    ContentSource::File { path: self.0 }
  }
}

impl From< PathBuf > for FileRef
{
  fn from( path: PathBuf ) -> Self
  {
    Self( path )
  }
}

impl From< &str > for FileRef
{
  fn from( path: &str ) -> Self
  {
    Self( PathBuf::from( path ) )
  }
}

/// URL reference source type.
///
/// Represents a reference to a remote resource accessible via URL.
///
/// # Examples
///
/// ```rust
/// use genfile_core::UrlRef;
///
/// let source = UrlRef::new( "https://example.com/template.hbs" );
/// let source = UrlRef( "https://example.com/template.hbs".to_string() );
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UrlRef( pub String );

impl UrlRef
{
  /// Create a new URL reference
  pub fn new( url: impl Into< String > ) -> Self
  {
    Self( url.into() )
  }
}

impl IntoContentSource for UrlRef
{
  fn into_content_source( self ) -> ContentSource
  {
    ContentSource::Url { url: self.0 }
  }
}

impl From< String > for UrlRef
{
  fn from( url: String ) -> Self
  {
    Self( url )
  }
}

impl From< &str > for UrlRef
{
  fn from( url: &str ) -> Self
  {
    Self( url.to_string() )
  }
}

/// Inline content source type.
///
/// Represents content that is embedded directly in the archive.
///
/// # Examples
///
/// ```rust
/// use genfile_core::{ InlineContent, FileContent };
///
/// let source = InlineContent::new( FileContent::Text( "Hello {{name}}".into() ) );
/// let source = InlineContent( FileContent::Text( "Hello {{name}}".into() ) );
/// ```
#[derive(Debug, Clone)]
pub struct InlineContent( pub FileContent );

impl InlineContent
{
  /// Create a new inline content source
  #[must_use] 
  pub fn new( content: FileContent ) -> Self
  {
    Self( content )
  }

  /// Create inline text content
  pub fn text( text: impl Into< String > ) -> Self
  {
    Self( FileContent::Text( text.into() ) )
  }

  /// Create inline binary content
  #[must_use] 
  pub fn binary( bytes: Vec< u8 > ) -> Self
  {
    Self( FileContent::Binary( bytes ) )
  }
}

impl IntoContentSource for InlineContent
{
  fn into_content_source( self ) -> ContentSource
  {
    ContentSource::Inline { content: self.0 }
  }
}

impl From< FileContent > for InlineContent
{
  fn from( content: FileContent ) -> Self
  {
    Self( content )
  }
}

impl ContentSource
{

  /// Check if this is inline content
  #[must_use] 
  pub fn is_inline( &self ) -> bool
  {
    matches!( self, Self::Inline { .. } )
  }

  /// Check if this is a file reference
  #[must_use] 
  pub fn is_file( &self ) -> bool
  {
    matches!( self, Self::File { .. } )
  }

  /// Check if this is a URL reference
  #[must_use] 
  pub fn is_url( &self ) -> bool
  {
    matches!( self, Self::Url { .. } )
  }

  /// Get inline content if available
  #[must_use] 
  pub fn as_inline( &self ) -> Option< &FileContent >
  {
    if let Self::Inline { content } = self
    {
      Some( content )
    }
    else
    {
      None
    }
  }

  /// Get file path if this is a file reference
  #[must_use] 
  pub fn as_file_path( &self ) -> Option< &Path >
  {
    if let Self::File { path } = self
    {
      Some( path )
    }
    else
    {
      None
    }
  }

  /// Get URL if this is a URL reference
  #[must_use] 
  pub fn as_url( &self ) -> Option< &str >
  {
    if let Self::Url { url } = self
    {
      Some( url )
    }
    else
    {
      None
    }
  }
}

/// Trait for resolving content from various sources.
///
/// Implement this trait to provide custom content retrieval logic,
/// such as fetching from databases, cloud storage, or custom protocols.
///
/// # Examples
///
/// ```rust
/// use genfile_core::{ ContentResolver, ContentSource, FileContent, Error };
///
/// struct MyResolver;
///
/// impl ContentResolver for MyResolver
/// {
///   fn resolve( &self, source: &ContentSource ) -> Result< FileContent, Error >
///   {
///     match source
///     {
///       ContentSource::Inline { content } => Ok( content.clone() ),
///       ContentSource::File { path } =>
///       {
///         // Custom file reading logic
///         let data = std::fs::read( path )?;
///         Ok( FileContent::Binary( data ) )
///       }
///       ContentSource::Url { url } =>
///       {
///         // Custom URL fetching logic
///         Err( Error::Render( format!( "URL not supported: {}", url ) ) )
///       }
///     }
///   }
/// }
/// ```
pub trait ContentResolver
{
  /// Resolve content from a source.
  ///
  /// # Parameters
  ///
  /// - `source`: The content source to resolve
  ///
  /// # Returns
  ///
  /// The resolved file content or an error
  ///
  /// # Errors
  ///
  /// Returns error if content cannot be retrieved from the source
  fn resolve( &self, source: &ContentSource ) -> Result< FileContent, Error >;
}

/// Trait for storing content to various destinations.
///
/// Implement this trait to provide custom content storage logic,
/// such as writing to databases, cloud storage, or custom backends.
///
/// # Examples
///
/// ```rust
/// use genfile_core::{ ContentStorage, FileContent, Error };
/// use std::path::Path;
///
/// struct MyStorage;
///
/// impl ContentStorage for MyStorage
/// {
///   fn store( &mut self, path: &Path, content: &FileContent ) -> Result< (), Error >
///   {
///     // Custom storage logic
///     println!( "Storing to {}: {:?}", path.display(), content );
///     Ok( () )
///   }
/// }
/// ```
pub trait ContentStorage
{
  /// Store content to a destination.
  ///
  /// # Parameters
  ///
  /// - `path`: Destination path for the content
  /// - `content`: The content to store
  ///
  /// # Returns
  ///
  /// Ok(()) on success, Error on failure
  ///
  /// # Errors
  ///
  /// Returns error if content cannot be written to destination
  fn store( &mut self, path: &Path, content: &FileContent ) -> Result< (), Error >;
}

/// Default content resolver implementation.
///
/// Handles inline content, local filesystem files, and provides
/// basic URL support (returns error by default - users should
/// implement custom resolver for URL fetching).
///
/// # Examples
///
/// ```rust
/// use genfile_core::{ DefaultContentResolver, ContentResolver, ContentSource, FileContent };
///
/// let resolver = DefaultContentResolver::new();
///
/// let source = ContentSource::Inline
/// {
///   content: FileContent::Text( "test".into() ),
/// };
///
/// let content = resolver.resolve( &source ).unwrap();
/// ```
#[derive(Debug)]
pub struct DefaultContentResolver;

impl DefaultContentResolver
{
  /// Create a new default content resolver
  #[must_use] 
  pub fn new() -> Self
  {
    Self
  }
}

impl Default for DefaultContentResolver
{
  fn default() -> Self
  {
    Self::new()
  }
}

impl ContentResolver for DefaultContentResolver
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
        // Read file from filesystem
        let data = std::fs::read( path )?;

        // Try to detect if its text or binary
        // Simple heuristic: if valid UTF-8, treat as text
        match String::from_utf8( data.clone() )
        {
          Ok( text ) => Ok( FileContent::Text( text ) ),
          Err( _ ) => Ok( FileContent::Binary( data ) ),
        }
      }

      ContentSource::Url { url } =>
      {
        // Default implementation doesnt support URL fetching
        // Users should implement custom resolver with HTTP client
        Err( Error::Render( format!(
          "URL fetching not supported in default resolver. \
          Implement custom ContentResolver to fetch from: {url}"
        ) ) )
      }
    }
  }
}

/// Default content storage implementation.
///
/// Stores content to local filesystem, creating parent directories
/// as needed. Binary content is written as-is, text content is
/// written as UTF-8.
///
/// # Examples
///
/// ```rust,ignore
/// use genfile_core::{ DefaultContentStorage, ContentStorage, FileContent };
/// use std::path::Path;
///
/// let mut storage = DefaultContentStorage::new();
///
/// storage.store(
///   Path::new( "output.txt" ),
///   &FileContent::Text( "Hello".into() )
/// ).unwrap();
/// ```
#[derive(Debug)]
pub struct DefaultContentStorage;

impl DefaultContentStorage
{
  /// Create a new default content storage
  #[must_use] 
  pub fn new() -> Self
  {
    Self
  }
}

impl Default for DefaultContentStorage
{
  fn default() -> Self
  {
    Self::new()
  }
}

impl ContentStorage for DefaultContentStorage
{
  fn store( &mut self, path: &Path, content: &FileContent ) -> Result< (), Error >
  {
    // Create parent directories
    if let Some( parent ) = path.parent()
    {
      std::fs::create_dir_all( parent )?;
    }

    // Write content based on type
    match content
    {
      FileContent::Text( text ) =>
      {
        std::fs::write( path, text )?;
      }

      FileContent::Binary( bytes ) =>
      {
        std::fs::write( path, bytes )?;
      }
    }

    Ok( () )
  }
}
