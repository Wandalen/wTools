/// Serialization and file I/O operations for [`TemplateArchive`].
///
/// Provides JSON/YAML serialization, file save/load, directory packing,
/// and content internalize/externalize operations.
/// Split from mod.rs to keep both files within source file size limits.
use std ::path ::Path;

use crate ::
{
  Error,
  FileSystem,
  HandlebarsRenderer,
  RealFileSystem,
  TemplateRenderer,
  Values,
  WriteMode,
  validate_path,
};

use super ::
{
  FileContent,
  MaterializationReport,
  TemplateArchive,
};

impl TemplateArchive
{
  // === Serialization ===

  /// Serialize to JSON string
  ///
  /// # Examples
  ///
  /// ```rust
  /// use genfile_core::TemplateArchive;
  ///
  /// let archive = TemplateArchive::new("test");
  /// let json = archive.to_json().unwrap();
  /// assert!(json.contains("\"name\":\"test\""));
  /// ```
  ///
  /// # Errors
  ///
  /// Returns error if JSON serialization fails.
  #[cfg(feature = "json")]
  pub fn to_json( &self ) -> Result< String, Error >
  {
    serde_json::to_string( self )
      .map_err( | e | Error::Render( format!( "JSON serialization failed: {e}" ) ) )
  }

  /// Serialize to pretty-printed JSON
  ///
  /// # Errors
  ///
  /// Returns error if JSON serialization fails.
  #[cfg(feature = "json")]
  pub fn to_json_pretty( &self ) -> Result< String, Error >
  {
    serde_json::to_string_pretty( self )
      .map_err( | e | Error::Render( format!( "JSON serialization failed: {e}" ) ) )
  }

  /// Deserialize from JSON string
  ///
  /// # Examples
  ///
  /// ```rust
  /// use genfile_core::TemplateArchive;
  ///
  /// let json = r#"{"name":"test","version":"1.0.0","files":[],"parameters":{"descriptors":[]}}"#;
  /// let archive = TemplateArchive::from_json(json).unwrap();
  /// assert_eq!(archive.name, "test");
  /// ```
  ///
  /// # Errors
  ///
  /// Returns error if JSON deserialization fails.
  #[cfg(feature = "json")]
  pub fn from_json( json: &str ) -> Result< Self, Error >
  {
    serde_json::from_str( json )
      .map_err( | e | Error::Render( format!( "JSON deserialization failed: {e}" ) ) )
  }

  /// Serialize to YAML string
  ///
  /// # Errors
  ///
  /// Returns error if YAML serialization fails.
  #[cfg(feature = "yaml")]
  pub fn to_yaml( &self ) -> Result< String, Error >
  {
    serde_yaml::to_string( self )
      .map_err( | e | Error::Render( format!( "YAML serialization failed: {e}" ) ) )
  }

  /// Deserialize from YAML string
  ///
  /// # Errors
  ///
  /// Returns error if YAML deserialization fails.
  #[cfg(feature = "yaml")]
  pub fn from_yaml( yaml: &str ) -> Result< Self, Error >
  {
    serde_yaml::from_str( yaml )
      .map_err( | e | Error::Render( format!( "YAML deserialization failed: {e}" ) ) )
  }

  // === Materialization ===

  /// Materialize archive to filesystem at `base_path`
  ///
  /// Creates all directories and files, applying template rendering
  /// with current parameter values.
  ///
  /// # Errors
  ///
  /// Returns error if template rendering or file creation fails.
  pub fn materialize( &self, base_path: &Path ) -> Result< MaterializationReport, Error >
  {
    let renderer = HandlebarsRenderer::new();
    let mut filesystem = RealFileSystem::new();
    self.materialize_with_components( base_path, &renderer, &mut filesystem )
  }

  /// Materialize with custom components (for testing)
  ///
  /// # Errors
  ///
  /// Returns error if template rendering, file creation, or path validation fails.
  pub fn materialize_with_components<R, FS>(
    &self,
    base_path: &Path,
    renderer: &R,
    filesystem: &mut FS
  ) -> Result< MaterializationReport, Error >
  where
    R: TemplateRenderer,
    FS: FileSystem,
  {
    let mut report = MaterializationReport::default();

    // Get values or use empty if none set
    let values = self.values.as_ref().map( Values::to_serializable ).unwrap_or_default();

    // Track directories that will be created
    // RealFileSystem creates directories automatically in write()
    // MemoryFileSystem doesnt need directory creation
    for dir in self.list_directories()
    {
      report.directories_created.push( dir );
    }

    // Process each file
    for file in &self.files
    {
      // Validate path for security (prevent directory traversal)
      validate_path( &file.path )?;

      let full_path = base_path.join( &file.path );

      let final_content = match &file.content
      {
        FileContent::Text( template ) =>
        {
          // Render template
          renderer.render( template, &values )?
        }
        FileContent::Binary( bytes ) =>
        {
          // Convert binary to string for filesystem write
          // In real impl, FileSystem trait would need write_bytes method
          String::from_utf8_lossy( bytes ).to_string()
        }
      };

      let existed = filesystem.exists( &full_path );

      // Write file (RealFileSystem creates parent dirs automatically)
      filesystem.write( &full_path, &final_content )?;

      report.total_bytes_written += final_content.len();

      if existed
      {
        report.files_updated.push( file.path.clone() );
      }
      else
      {
        report.files_created.push( file.path.clone() );
      }
    }

    Ok( report )
  }

  /// Materialize archive with custom content resolver.
  ///
  /// Allows using external content sources (files, URLs, custom providers)
  /// instead of only inline content.
  ///
  /// # Parameters
  ///
  /// - `base_path`: Base directory for output files
  /// - `renderer`: Template rendering engine
  /// - `filesystem`: File system for writing output
  /// - `resolver`: Content resolver for external sources
  ///
  /// # Examples
  ///
  /// ```rust,no_run
  /// use genfile_core::{ TemplateArchive, DefaultContentResolver, HandlebarsRenderer, RealFileSystem };
  /// use std::path::Path;
  ///
  /// let archive = TemplateArchive::new( "test" );
  /// let resolver = DefaultContentResolver::new();
  /// let renderer = HandlebarsRenderer::new();
  /// let mut filesystem = RealFileSystem::new();
  ///
  /// archive.materialize_with_resolver(
  ///     Path::new( "/output" ),
  ///     &renderer,
  ///     &mut filesystem,
  ///     &resolver
  /// ).unwrap();
  /// ```
  ///
  /// # Errors
  ///
  /// Returns error if content resolution, template rendering, or file creation fails.
  #[cfg(feature = "external_content")]
  pub fn materialize_with_resolver<R, FS, CR>(
    &self,
    base_path: &Path,
    renderer: &R,
    filesystem: &mut FS,
    resolver: &CR
  ) -> Result< MaterializationReport, Error >
  where
    R: TemplateRenderer,
    FS: FileSystem,
    CR: crate::ContentResolver,
  {
    let mut report = MaterializationReport::default();

    // Get values or use empty if none set
    let values = self.values.as_ref().map( Values::to_serializable ).unwrap_or_default();

    // Track directories
    for dir in self.list_directories()
    {
      report.directories_created.push( dir );
    }

    // Process each file
    for file in &self.files
    {
      // Validate path for security (prevent directory traversal)
      validate_path( &file.path )?;

      let full_path = base_path.join( &file.path );

      // Resolve content from source (external or inline)
      let content = if let Some( source ) = &file.content_source
      {
        // Use external source via resolver
        resolver.resolve( source )?
      }
      else
      {
        // Use inline content
        file.content.clone()
      };

      // Render content
      let final_content = match &content
      {
        FileContent::Text( template ) =>
        {
          renderer.render( template, &values )?
        }
        FileContent::Binary( bytes ) =>
        {
          String::from_utf8_lossy( bytes ).to_string()
        }
      };

      let existed = filesystem.exists( &full_path );

      // Write file
      filesystem.write( &full_path, &final_content )?;

      report.total_bytes_written += final_content.len();

      if existed
      {
        report.files_updated.push( file.path.clone() );
      }
      else
      {
        report.files_created.push( file.path.clone() );
      }
    }

    Ok( report )
  }

  /// Materialize archive using `ContentStorage` abstraction.
  ///
  /// Instead of using `FileSystem` trait, uses `ContentStorage` for maximum
  /// flexibility. This allows writing to databases, cloud storage, etc.
  ///
  /// # Parameters
  ///
  /// - `base_path`: Base directory for output files
  /// - `renderer`: Template rendering engine
  /// - `storage`: Content storage backend
  /// - `resolver`: Content resolver for external sources
  ///
  /// # Examples
  ///
  /// ```rust,no_run
  /// use genfile_core::{ TemplateArchive, DefaultContentResolver, DefaultContentStorage, HandlebarsRenderer };
  /// use std::path::Path;
  ///
  /// let archive = TemplateArchive::new( "test" );
  /// let resolver = DefaultContentResolver::new();
  /// let mut storage = DefaultContentStorage::new();
  /// let renderer = HandlebarsRenderer::new();
  ///
  /// archive.materialize_with_storage(
  ///     Path::new( "/output" ),
  ///     &renderer,
  ///     &mut storage,
  ///     &resolver
  /// ).unwrap();
  /// ```
  ///
  /// # Errors
  ///
  /// Returns error if content resolution, storage operations, or file creation fails.
  #[cfg(feature = "external_content")]
  pub fn materialize_with_storage<R, CS, CR>(
    &self,
    base_path: &Path,
    renderer: &R,
    storage: &mut CS,
    resolver: &CR
  ) -> Result< MaterializationReport, Error >
  where
    R: TemplateRenderer,
    CS: crate::ContentStorage,
    CR: crate::ContentResolver,
  {
    let mut report = MaterializationReport::default();

    // Get values or use empty if none set
    let values = self.values.as_ref().map( Values::to_serializable ).unwrap_or_default();

    // Track directories
    for dir in self.list_directories()
    {
      report.directories_created.push( dir );
    }

    // Process each file
    for file in &self.files
    {
      // Validate path for security (prevent directory traversal)
      validate_path( &file.path )?;

      let full_path = base_path.join( &file.path );

      // Resolve content from source (external or inline)
      let content = if let Some( source ) = &file.content_source
      {
        resolver.resolve( source )?
      }
      else
      {
        file.content.clone()
      };

      // Render content
      let rendered_content = match &content
      {
        FileContent::Text( template ) =>
        {
          FileContent::Text( renderer.render( template, &values )? )
        }
        FileContent::Binary( bytes ) =>
        {
          FileContent::Binary( bytes.clone() )
        }
      };

      // Store using ContentStorage
      storage.store( &full_path, &rendered_content )?;

      report.total_bytes_written += match &rendered_content
      {
        FileContent::Text( s ) => s.len(),
        FileContent::Binary( b ) => b.len(),
      };

      report.files_created.push( file.path.clone() );
    }

    Ok( report )
  }

  // === Pack/Unpack Operations ===

  /// Create archive from directory tree.
  ///
  /// Scans a directory recursively and packs all files into an archive.
  /// Text files are stored as text content, binary files as binary content.
  ///
  /// # Parameters
  ///
  /// - `name`: Archive name
  /// - `base_path`: Root directory to pack
  ///
  /// # Returns
  ///
  /// New archive containing all files from directory tree
  ///
  /// # Errors
  ///
  /// Returns error if directory cant be read or files cant be accessed
  ///
  /// # Examples
  ///
  /// ```rust,no_run
  /// use genfile_core::TemplateArchive;
  /// use std::path::Path;
  ///
  /// let archive = TemplateArchive::pack_from_dir(
  ///     "my-template",
  ///     Path::new( "/templates/project" )
  /// ).unwrap();
  ///
  /// println!( "Packed {} files", archive.file_count() );
  /// ```
  pub fn pack_from_dir( name: impl Into< String >, base_path: &Path ) -> Result< Self, Error >
  {
    let mut archive = Self::new( name );

    // Recursively walk directory
    fn visit_dir( archive: &mut TemplateArchive, base: &Path, current: &Path ) -> Result< (), Error >
    {
      for entry in std::fs::read_dir( current )?
      {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir()
        {
          visit_dir( archive, base, &path )?;
        }
        else if path.is_file()
        {
          // Get relative path from base
          let rel_path = path.strip_prefix( base )
            .map_err( | e | Error::Render( format!( "Path error: {e}" ) ) )?
            .to_path_buf();

          // Read file
          let data = std::fs::read( &path )?;

          // Detect if text or binary
          let content = match String::from_utf8( data.clone() )
          {
            Ok( text ) => FileContent::Text( text ),
            Err( _ ) => FileContent::Binary( data ),
          };

          archive.add_file( rel_path, content, WriteMode::Rewrite );
        }
      }

      Ok( () )
    }

    visit_dir( &mut archive, base_path, base_path )?;

    Ok( archive )
  }

  /// Internalize all external content references.
  ///
  /// Resolves all external sources (files, URLs) and converts them to
  /// inline content. This makes the archive self-contained.
  ///
  /// # Parameters
  ///
  /// - `resolver`: Content resolver to fetch external content
  ///
  /// # Returns
  ///
  /// Ok(()) on success, Error if any external source cant be resolved
  ///
  /// # Examples
  ///
  /// ```rust,no_run
  /// use genfile_core::{ TemplateArchive, DefaultContentResolver };
  ///
  /// let mut archive = TemplateArchive::new( "test" );
  /// // ... add files with external sources ...
  ///
  /// let resolver = DefaultContentResolver::new();
  /// archive.internalize( &resolver ).unwrap();
  ///
  /// // Now all content is inline
  /// ```
  ///
  /// # Errors
  ///
  /// Returns error if content resolution fails for any external reference.
  #[cfg(feature = "external_content")]
  pub fn internalize< CR >( &mut self, resolver: &CR ) -> Result< (), Error >
  where
    CR: crate::ContentResolver,
  {
    for file in &mut self.files
    {
      if let Some( source ) = &file.content_source
      {
        // Resolve external content
        let content = resolver.resolve( source )?;

        // Replace inline content
        file.content = content;

        // Remove external source reference
        file.content_source = None;
      }
    }

    Ok( () )
  }

  /// Externalize inline content to file references.
  ///
  /// Writes all inline content to files in the specified directory
  /// and replaces inline content with file references. This reduces
  /// archive size when serialized.
  ///
  /// # Parameters
  ///
  /// - `base_path`: Directory where content files will be written
  ///
  /// # Errors
  ///
  /// Returns error if directory creation or file writing fails.
  ///
  /// # Examples
  ///
  /// ```rust,no_run
  /// use genfile_core::TemplateArchive;
  /// use std::path::Path;
  ///
  /// let mut archive = TemplateArchive::new( "test" );
  /// // ... add files with inline content ...
  ///
  /// archive.externalize( Path::new( "/archive-content" ) ).unwrap();
  ///
  /// // Now content is stored in /archive-content/ and referenced
  /// ```
  #[cfg(feature = "external_content")]
  pub fn externalize( &mut self, base_path: &Path ) -> Result< (), Error >
  {
    // Create base directory
    std::fs::create_dir_all( base_path )?;

    for file in &mut self.files
    {
      if file.content_source.is_none()
      {
        // Generate unique filename for content
        let content_filename = format!( "{}.content", file.path.display() ).replace( '/', "_" );
        let content_path = base_path.join( &content_filename );

        // Write content to file
        match &file.content
        {
          FileContent::Text( text ) =>
          {
            std::fs::write( &content_path, text )?;
          }
          FileContent::Binary( bytes ) =>
          {
            std::fs::write( &content_path, bytes )?;
          }
        }

        // Replace with file reference
        file.content_source = Some( crate::ContentSource::File { path: content_path } );

        // Clear inline content (or leave placeholder)
        file.content = FileContent::Text( String::new() );
      }
    }

    Ok( () )
  }

  /// Save archive to file using JSON format.
  ///
  /// # Parameters
  ///
  /// - `path`: File path where archive will be saved
  ///
  /// # Errors
  ///
  /// Returns error if JSON serialization or file writing fails.
  ///
  /// # Examples
  ///
  /// ```rust,no_run
  /// use genfile_core::TemplateArchive;
  /// use std::path::Path;
  ///
  /// let archive = TemplateArchive::new( "test" );
  /// archive.save_to_file( Path::new( "archive.json" ) ).unwrap();
  /// ```
  #[cfg(feature = "json")]
  pub fn save_to_file( &self, path: &Path ) -> Result< (), Error >
  {
    let json = self.to_json_pretty()?;
    std::fs::write( path, json )?;
    Ok( () )
  }

  /// Load archive from JSON file.
  ///
  /// # Parameters
  ///
  /// - `path`: File path to load archive from
  ///
  /// # Errors
  ///
  /// Returns error if file reading or JSON deserialization fails.
  ///
  /// # Examples
  ///
  /// ```rust,no_run
  /// use genfile_core::TemplateArchive;
  /// use std::path::Path;
  ///
  /// let archive = TemplateArchive::load_from_file( Path::new( "archive.json" ) ).unwrap();
  /// ```
  #[cfg(feature = "json")]
  pub fn load_from_file( path: &Path ) -> Result< Self, Error >
  {
    let json = std::fs::read_to_string( path )?;
    Self::from_json( &json )
  }
}
