/// Template archive system for packaging and materializing file templates
use std::collections::{ HashMap, HashSet };
use std::path::{ Path, PathBuf };
use serde::{ Serialize, Deserialize };
use crate::
{
  ParameterDescriptor,
  Parameters,
  Value,
  Values,
  WriteMode,
  Error,
  TemplateRenderer,
  HandlebarsRenderer,
  FileSystem,
  RealFileSystem,
};

//

/// Complete template archive with files, parameters, and metadata
///
/// This is the main entity that owns all template-related data and provides
/// operations for manipulation, analysis, serialization, and materialization.
///
/// # Examples
///
/// ```rust
/// use genfile_core::{ TemplateArchive, FileContent, WriteMode, ParameterDescriptor };
/// use std::path::PathBuf;
///
/// // Create archive
/// let mut archive = TemplateArchive::new("rust-project-template");
/// archive.set_version("1.0.0");
/// archive.set_description("Complete Rust project template");
///
/// // Add text template file
/// archive.add_text_file(
///     PathBuf::from("src/main.rs"),
///     "fn main() {\n    println!(\"{{project_name}}\");\n}",
///     WriteMode::Rewrite
/// );
///
/// // Add binary file
/// let logo_bytes = vec![0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];
/// archive.add_binary_file(
///     PathBuf::from("assets/logo.png"),
///     logo_bytes
/// );
///
/// // Add parameter definition
/// archive.add_parameter(
///     ParameterDescriptor
///     {
///       parameter: "project_name".into(),
///       is_mandatory: true,
///       default_value: None,
///       description: Some("Name of the project".into()),
///     }
/// );
///
/// // Discover all template variables automatically
/// let discovered = archive.discover_parameters();
/// println!("Found variables: {:?}", discovered);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateArchive
{
  /// Archive name/identifier
  pub name: String,

  /// Archive version
  #[serde(default = "default_version")]
  pub version: String,

  /// Optional description
  #[serde(skip_serializing_if = "Option::is_none")]
  pub description: Option< String >,

  /// All files in the archive with their content
  #[serde(default)]
  pub files: Vec< TemplateFile >,

  /// Parameter definitions
  #[serde(default)]
  pub parameters: Parameters,

  /// Current parameter values (can be set before materialization)
  #[serde(skip_serializing_if = "Option::is_none")]
  pub values: Option< Values< Value > >,

  /// Archive metadata
  #[serde(skip_serializing_if = "Option::is_none")]
  pub metadata: Option< ArchiveMetadata >,
}

fn default_version() -> String
{
  "0.1.0".to_string()
}

/// Metadata about the archive
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArchiveMetadata
{
  /// Author of the template
  #[serde(skip_serializing_if = "Option::is_none")]
  pub author: Option< String >,

  /// License
  #[serde(skip_serializing_if = "Option::is_none")]
  pub license: Option< String >,

  /// Tags for categorization
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  pub tags: Vec< String >,

  /// Creation timestamp
  #[serde(skip_serializing_if = "Option::is_none")]
  pub created_at: Option< String >,

  /// Last modified timestamp
  #[serde(skip_serializing_if = "Option::is_none")]
  pub modified_at: Option< String >,
}

/// Single file in the archive
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateFile
{
  /// Relative path within the archive (e.g., "src/lib.rs")
  pub path: PathBuf,

  /// File content (text or binary) - inline content
  pub content: FileContent,

  /// How to write this file during materialization
  pub write_mode: WriteMode,

  /// Optional file-specific metadata
  #[serde(skip_serializing_if = "Option::is_none")]
  pub metadata: Option< FileMetadata >,

  /// Optional external content source
  /// If present, this takes precedence over inline content field
  #[serde(skip_serializing_if = "Option::is_none")]
  pub content_source: Option< crate::ContentSource >,
}

/// File content representation
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum FileContent
{
  /// Text content (templates, source code, configs)
  /// Stored as UTF-8 string
  Text( String ),

  /// Binary content (images, executables, archives)
  /// Serialized as base64-encoded string in JSON/YAML
  #[serde(with = "base64_encoding")]
  Binary( Vec< u8 > ),
}

/// Base64 encoding for binary data in JSON/YAML
mod base64_encoding
{
  use serde::{ Deserialize, Deserializer, Serializer };

  pub fn serialize< S >( bytes: &Vec< u8 >, serializer: S ) -> Result< S::Ok, S::Error >
  where
    S: Serializer,
  {
    use base64::Engine;
    let base64_string = base64::engine::general_purpose::STANDARD.encode( bytes );
    serializer.serialize_str( &base64_string )
  }

  pub fn deserialize< 'de, D >( deserializer: D ) -> Result< Vec< u8 >, D::Error >
  where
    D: Deserializer< 'de >,
  {
    use base64::Engine;
    let base64_string = String::deserialize( deserializer )?;
    base64::engine::general_purpose::STANDARD
      .decode( base64_string )
      .map_err( serde::de::Error::custom )
  }
}

/// Optional metadata for individual files
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileMetadata
{
  /// Unix permissions (e.g., 0o755 for executables)
  #[serde(skip_serializing_if = "Option::is_none")]
  pub permissions: Option< u32 >,

  /// Whether this is a template (contains {{variables}})
  #[serde(default)]
  pub is_template: bool,

  /// File-specific comments
  #[serde(skip_serializing_if = "Option::is_none")]
  pub comment: Option< String >,
}

impl TemplateArchive
{
  // === Creation & Basic Operations ===

  /// Create new empty archive
  ///
  /// # Examples
  ///
  /// ```rust
  /// use genfile_core::TemplateArchive;
  ///
  /// let archive = TemplateArchive::new("my-template");
  /// assert_eq!(archive.name, "my-template");
  /// assert_eq!(archive.version, "0.1.0");
  /// ```
  pub fn new( name: impl Into< String > ) -> Self
  {
    Self
    {
      name: name.into(),
      version: default_version(),
      description: None,
      files: Vec::new(),
      parameters: Parameters::default(),
      values: None,
      metadata: None,
    }
  }

  /// Set archive version
  ///
  /// # Examples
  ///
  /// ```rust
  /// use genfile_core::TemplateArchive;
  ///
  /// let mut archive = TemplateArchive::new("test");
  /// archive.set_version("2.0.0");
  /// assert_eq!(archive.version, "2.0.0");
  /// ```
  pub fn set_version( &mut self, version: impl Into< String > ) -> &mut Self
  {
    self.version = version.into();
    self
  }

  /// Set archive description
  ///
  /// # Examples
  ///
  /// ```rust
  /// use genfile_core::TemplateArchive;
  ///
  /// let mut archive = TemplateArchive::new("test");
  /// archive.set_description("A test template");
  /// assert_eq!(archive.description, Some("A test template".to_string()));
  /// ```
  pub fn set_description( &mut self, desc: impl Into< String > ) -> &mut Self
  {
    self.description = Some( desc.into() );
    self
  }

  /// Set metadata
  pub fn set_metadata( &mut self, metadata: ArchiveMetadata ) -> &mut Self
  {
    self.metadata = Some( metadata );
    self
  }

  // === File Operations ===

  /// Add file to archive
  ///
  /// # Examples
  ///
  /// ```rust
  /// use genfile_core::{ TemplateArchive, FileContent, WriteMode };
  /// use std::path::PathBuf;
  ///
  /// let mut archive = TemplateArchive::new("test");
  /// archive.add_file(
  ///     PathBuf::from("README.md"),
  ///     FileContent::Text("# {{name}}".to_string()),
  ///     WriteMode::Rewrite
  /// );
  /// assert_eq!(archive.file_count(), 1);
  /// ```
  pub fn add_file(
    &mut self,
    path: PathBuf,
    content: FileContent,
    write_mode: WriteMode
  ) -> &mut Self
  {
    self.files.push( TemplateFile
    {
      path,
      content,
      write_mode,
      metadata: None,
      content_source: None,
    });
    self
  }

  /// Add text file (convenience method)
  ///
  /// # Examples
  ///
  /// ```rust
  /// use genfile_core::{ TemplateArchive, WriteMode };
  /// use std::path::PathBuf;
  ///
  /// let mut archive = TemplateArchive::new("test");
  /// archive.add_text_file(
  ///     PathBuf::from("src/main.rs"),
  ///     "fn main() { println!(\"{{greeting}}\"); }",
  ///     WriteMode::Rewrite
  /// );
  /// ```
  pub fn add_text_file(
    &mut self,
    path: PathBuf,
    content: impl Into< String >,
    write_mode: WriteMode
  ) -> &mut Self
  {
    self.add_file( path, FileContent::Text( content.into() ), write_mode )
  }

  /// Add binary file (convenience method)
  ///
  /// # Examples
  ///
  /// ```rust
  /// use genfile_core::TemplateArchive;
  /// use std::path::PathBuf;
  ///
  /// let mut archive = TemplateArchive::new("test");
  /// let image_data = vec![0x89, 0x50, 0x4E, 0x47];
  /// archive.add_binary_file(
  ///     PathBuf::from("logo.png"),
  ///     image_data
  /// );
  /// ```
  pub fn add_binary_file( &mut self, path: PathBuf, content: Vec< u8 > ) -> &mut Self
  {
    self.add_file( path, FileContent::Binary( content ), WriteMode::Rewrite )
  }

  /// Add file with content from any source.
  ///
  /// This is the universal method for adding files to the archive.
  /// Accepts any type implementing `IntoContentSource` trait, allowing
  /// for flexible content sourcing (inline, file references, URLs, custom).
  ///
  /// # Parameters
  ///
  /// - `path`: Destination path in the archive
  /// - `source`: Content source (FileRef, UrlRef, InlineContent, etc.)
  /// - `write_mode`: How to write this file during materialization
  ///
  /// # Examples
  ///
  /// ```rust
  /// use genfile_core::{ TemplateArchive, FileRef, UrlRef, InlineContent, FileContent, WriteMode };
  /// use std::path::PathBuf;
  ///
  /// let mut archive = TemplateArchive::new( "test" );
  ///
  /// // Reference local file
  /// archive.add_file_from(
  ///     PathBuf::from( "output.txt" ),
  ///     FileRef::new( "/templates/main.hbs" ),
  ///     WriteMode::Rewrite
  /// );
  ///
  /// // Reference remote URL
  /// archive.add_file_from(
  ///     PathBuf::from( "config.json" ),
  ///     UrlRef::new( "https://example.com/config.json" ),
  ///     WriteMode::Rewrite
  /// );
  ///
  /// // Inline content
  /// archive.add_file_from(
  ///     PathBuf::from( "inline.txt" ),
  ///     InlineContent::text( "Hello {{name}}" ),
  ///     WriteMode::Rewrite
  /// );
  /// ```
  pub fn add_file_from< S >(
    &mut self,
    path: PathBuf,
    source: S,
    write_mode: WriteMode
  ) -> &mut Self
  where
    S: crate::IntoContentSource,
  {
    let content_source = source.into_content_source();

    self.files.push( TemplateFile
    {
      path,
      content: FileContent::Text( String::new() ), // Placeholder
      write_mode,
      metadata: None,
      content_source: Some( content_source ),
    });
    self
  }

  /// Remove file by path
  pub fn remove_file( &mut self, path: &Path ) -> Option< TemplateFile >
  {
    self.files
      .iter()
      .position( | f | f.path == path )
      .map( | idx | self.files.remove( idx ) )
  }

  /// Get file by path
  pub fn get_file( &self, path: &Path ) -> Option< &TemplateFile >
  {
    self.files.iter().find( | f | f.path == path )
  }

  /// Get mutable file by path
  pub fn get_file_mut( &mut self, path: &Path ) -> Option< &mut TemplateFile >
  {
    self.files.iter_mut().find( | f | f.path == path )
  }

  /// Check if file exists
  ///
  /// # Examples
  ///
  /// ```rust
  /// use genfile_core::{ TemplateArchive, WriteMode };
  /// use std::path::{ Path, PathBuf };
  ///
  /// let mut archive = TemplateArchive::new("test");
  /// archive.add_text_file(PathBuf::from("test.txt"), "content", WriteMode::Rewrite);
  ///
  /// assert!(archive.has_file(Path::new("test.txt")));
  /// assert!(!archive.has_file(Path::new("missing.txt")));
  /// ```
  pub fn has_file( &self, path: &Path ) -> bool
  {
    self.files.iter().any( | f | f.path == path )
  }

  /// List all file paths
  pub fn list_files( &self ) -> Vec< &Path >
  {
    self.files.iter().map( | f | f.path.as_path() ).collect()
  }

  /// Get all directories referenced by files
  ///
  /// # Examples
  ///
  /// ```rust
  /// use genfile_core::{ TemplateArchive, WriteMode };
  /// use std::path::PathBuf;
  ///
  /// let mut archive = TemplateArchive::new("test");
  /// archive.add_text_file(PathBuf::from("src/lib.rs"), "code", WriteMode::Rewrite);
  /// archive.add_text_file(PathBuf::from("src/utils/helper.rs"), "code", WriteMode::Rewrite);
  ///
  /// let dirs = archive.list_directories();
  /// assert!(dirs.contains(&PathBuf::from("src")));
  /// assert!(dirs.contains(&PathBuf::from("src/utils")));
  /// ```
  pub fn list_directories( &self ) -> Vec< PathBuf >
  {
    let mut dirs = HashSet::new();

    for file in &self.files
    {
      let mut current = file.path.as_path();
      while let Some( parent ) = current.parent()
      {
        if parent.as_os_str().is_empty()
        {
          break;
        }
        dirs.insert( parent.to_path_buf() );
        current = parent;
      }
    }

    let mut result: Vec< _ > = dirs.into_iter().collect();
    result.sort();
    result
  }

  // === Parameter Operations ===

  /// Add parameter definition
  ///
  /// # Examples
  ///
  /// ```rust
  /// use genfile_core::{ TemplateArchive, ParameterDescriptor };
  ///
  /// let mut archive = TemplateArchive::new("test");
  /// archive.add_parameter(
  ///     ParameterDescriptor
  ///     {
  ///       parameter: "name".into(),
  ///       is_mandatory: true,
  ///       default_value: None,
  ///       description: None,
  ///     }
  /// );
  /// ```
  pub fn add_parameter( &mut self, param: ParameterDescriptor ) -> &mut Self
  {
    self.parameters.descriptors.push( param );
    self
  }

  /// Remove parameter
  pub fn remove_parameter( &mut self, name: &str ) -> Option< ParameterDescriptor >
  {
    self.parameters
      .descriptors
      .iter()
      .position( | p | p.parameter == name )
      .map( | idx | self.parameters.descriptors.remove( idx ) )
  }

  /// Get parameter definition
  pub fn get_parameter( &self, name: &str ) -> Option< &ParameterDescriptor >
  {
    self.parameters.descriptors.iter().find( | p | p.parameter == name )
  }

  /// List all defined parameters
  pub fn list_parameters( &self ) -> Vec< &str >
  {
    self.parameters.descriptors.iter().map( | p | p.parameter.as_str() ).collect()
  }

  /// List mandatory parameters
  pub fn list_mandatory_parameters( &self ) -> Vec< &str >
  {
    self.parameters.list_mandatory()
  }

  // === Parameter Discovery & Analysis ===

  /// Discover all template variables in file content
  ///
  /// Scans all text files and extracts {{variable}} patterns.
  ///
  /// # Examples
  ///
  /// ```rust
  /// use genfile_core::{ TemplateArchive, WriteMode };
  /// use std::path::PathBuf;
  ///
  /// let mut archive = TemplateArchive::new("test");
  /// archive.add_text_file(
  ///     PathBuf::from("config.txt"),
  ///     "Host: {{host}}, Port: {{port}}",
  ///     WriteMode::Rewrite
  /// );
  ///
  /// let discovered = archive.discover_parameters();
  /// assert!(discovered.contains("host"));
  /// assert!(discovered.contains("port"));
  /// assert_eq!(discovered.len(), 2);
  /// ```
  pub fn discover_parameters( &self ) -> HashSet< String >
  {
    let mut params = HashSet::new();
    let pattern = regex::Regex::new( r"\{\{([a-zA-Z_][a-zA-Z0-9_]*)\}\}" ).unwrap();

    for file in &self.files
    {
      if let FileContent::Text( content ) = &file.content
      {
        for cap in pattern.captures_iter( content )
        {
          if let Some( param_name ) = cap.get( 1 )
          {
            params.insert( param_name.as_str().to_string() );
          }
        }
      }
    }

    params
  }

  /// Find parameters used in templates but not defined
  pub fn get_undefined_parameters( &self ) -> Vec< String >
  {
    let discovered = self.discover_parameters();
    let defined: HashSet< _ > = self.list_parameters().into_iter().collect();

    discovered
      .into_iter()
      .filter( | p | !defined.contains( p.as_str() ) )
      .collect()
  }

  /// Find defined parameters not used in any template
  pub fn get_unused_parameters( &self ) -> Vec< String >
  {
    let discovered = self.discover_parameters();
    let defined = self.list_parameters();

    defined
      .into_iter()
      .filter( | p | !discovered.contains( *p ) )
      .map( String::from )
      .collect()
  }

  /// Analyze parameter usage across files
  pub fn analyze_parameter_usage( &self ) -> HashMap< String, Vec< PathBuf > >
  {
    let mut usage: HashMap< String, Vec< PathBuf > > = HashMap::new();
    let pattern = regex::Regex::new( r"\{\{([a-zA-Z_][a-zA-Z0-9_]*)\}\}" ).unwrap();

    for file in &self.files
    {
      if let FileContent::Text( content ) = &file.content
      {
        for cap in pattern.captures_iter( content )
        {
          if let Some( param_name ) = cap.get( 1 )
          {
            usage
              .entry( param_name.as_str().to_string() )
              .or_insert_with( Vec::new )
              .push( file.path.clone() );
          }
        }
      }
    }

    usage
  }

  // === Value Operations ===

  /// Set parameter value
  pub fn set_value( &mut self, name: impl Into< String >, value: Value ) -> &mut Self
  {
    let name_string = name.into();
    if self.values.is_none()
    {
      self.values = Some( Values::new() );
    }
    self.values.as_mut().unwrap().insert( &name_string, value );
    self
  }

  /// Get parameter value
  pub fn get_value( &self, name: &str ) -> Option< &Value >
  {
    self.values.as_ref().and_then( | v | v.get( name ) )
  }

  /// Set multiple values at once
  pub fn set_values( &mut self, values: HashMap< String, Value > ) -> &mut Self
  {
    for ( name, value ) in values
    {
      self.set_value( name, value );
    }
    self
  }

  /// Get mutable access to values
  pub fn values_mut( &mut self ) -> &mut Values< Value >
  {
    if self.values.is_none()
    {
      self.values = Some( Values::new() );
    }
    self.values.as_mut().unwrap()
  }

  /// Clear all values
  pub fn clear_values( &mut self )
  {
    self.values = None;
  }

  // === Statistics & Inspection ===

  /// Get total number of files
  ///
  /// # Examples
  ///
  /// ```rust
  /// use genfile_core::{ TemplateArchive, WriteMode };
  /// use std::path::PathBuf;
  ///
  /// let mut archive = TemplateArchive::new("test");
  /// assert_eq!(archive.file_count(), 0);
  ///
  /// archive.add_text_file(PathBuf::from("a.txt"), "content", WriteMode::Rewrite);
  /// archive.add_text_file(PathBuf::from("b.txt"), "content", WriteMode::Rewrite);
  /// assert_eq!(archive.file_count(), 2);
  /// ```
  pub fn file_count( &self ) -> usize
  {
    self.files.len()
  }

  /// Get number of text files
  pub fn text_file_count( &self ) -> usize
  {
    self.files.iter().filter( | f | matches!( f.content, FileContent::Text( _ ) ) ).count()
  }

  /// Get number of binary files
  pub fn binary_file_count( &self ) -> usize
  {
    self.files.iter().filter( | f | matches!( f.content, FileContent::Binary( _ ) ) ).count()
  }

  /// Calculate total size of all content in bytes
  pub fn total_size( &self ) -> usize
  {
    self.files.iter().map( | f |
    {
      match &f.content
      {
        FileContent::Text( s ) => s.len(),
        FileContent::Binary( b ) => b.len(),
      }
    }).sum()
  }

  /// Get deepest directory nesting level
  ///
  /// # Examples
  ///
  /// ```rust
  /// use genfile_core::{ TemplateArchive, WriteMode };
  /// use std::path::PathBuf;
  ///
  /// let mut archive = TemplateArchive::new("test");
  /// archive.add_text_file(PathBuf::from("a.txt"), "x", WriteMode::Rewrite);
  /// assert_eq!(archive.max_directory_depth(), 0);
  ///
  /// archive.add_text_file(PathBuf::from("dir/b.txt"), "x", WriteMode::Rewrite);
  /// assert_eq!(archive.max_directory_depth(), 1);
  ///
  /// archive.add_text_file(PathBuf::from("a/b/c/d.txt"), "x", WriteMode::Rewrite);
  /// assert_eq!(archive.max_directory_depth(), 3);
  /// ```
  pub fn max_directory_depth( &self ) -> usize
  {
    self.files
      .iter()
      .map( | f | f.path.components().count().saturating_sub( 1 ) )
      .max()
      .unwrap_or( 0 )
  }

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
  pub fn to_json( &self ) -> Result< String, Error >
  {
    serde_json::to_string( self )
      .map_err( | e | Error::Render( format!( "JSON serialization failed: {}", e ) ) )
  }

  /// Serialize to pretty-printed JSON
  pub fn to_json_pretty( &self ) -> Result< String, Error >
  {
    serde_json::to_string_pretty( self )
      .map_err( | e | Error::Render( format!( "JSON serialization failed: {}", e ) ) )
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
  pub fn from_json( json: &str ) -> Result< Self, Error >
  {
    serde_json::from_str( json )
      .map_err( | e | Error::Render( format!( "JSON deserialization failed: {}", e ) ) )
  }

  /// Serialize to YAML string
  pub fn to_yaml( &self ) -> Result< String, Error >
  {
    serde_yaml::to_string( self )
      .map_err( | e | Error::Render( format!( "YAML serialization failed: {}", e ) ) )
  }

  /// Deserialize from YAML string
  pub fn from_yaml( yaml: &str ) -> Result< Self, Error >
  {
    serde_yaml::from_str( yaml )
      .map_err( | e | Error::Render( format!( "YAML deserialization failed: {}", e ) ) )
  }

  // === Materialization ===

  /// Materialize archive to filesystem at base_path
  ///
  /// Creates all directories and files, applying template rendering
  /// with current parameter values.
  pub fn materialize( &self, base_path: &Path ) -> Result< MaterializationReport, Error >
  {
    let renderer = HandlebarsRenderer::new();
    let mut filesystem = RealFileSystem::new();
    self.materialize_with_components( base_path, &renderer, &mut filesystem )
  }

  /// Materialize with custom components (for testing)
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
    let values = self.values.as_ref().map( | v | v.to_serializable() ).unwrap_or_default();

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
  /// ```rust,ignore
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
    let values = self.values.as_ref().map( | v | v.to_serializable() ).unwrap_or_default();

    // Track directories
    for dir in self.list_directories()
    {
      report.directories_created.push( dir );
    }

    // Process each file
    for file in &self.files
    {
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

  /// Materialize archive using ContentStorage abstraction.
  ///
  /// Instead of using FileSystem trait, uses ContentStorage for maximum
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
  /// ```rust,ignore
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
    let values = self.values.as_ref().map( | v | v.to_serializable() ).unwrap_or_default();

    // Track directories
    for dir in self.list_directories()
    {
      report.directories_created.push( dir );
    }

    // Process each file
    for file in &self.files
    {
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
  /// ```rust,ignore
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
            .map_err( | e | Error::Render( format!( "Path error: {}", e ) ) )?
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
  /// ```rust,ignore
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
  /// # Returns
  ///
  /// Ok(()) on success, Error if files cant be written
  ///
  /// # Examples
  ///
  /// ```rust,ignore
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
  pub fn externalize( &mut self, base_path: &Path ) -> Result< (), Error >
  {
    // Create base directory
    std::fs::create_dir_all( base_path )?;

    for file in &mut self.files
    {
      if file.content_source.is_none()
      {
        // Generate unique filename for content
        let content_filename = format!( "{}.content", file.path.display() ).replace( "/", "_" );
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
  /// # Examples
  ///
  /// ```rust,ignore
  /// use genfile_core::TemplateArchive;
  /// use std::path::Path;
  ///
  /// let archive = TemplateArchive::new( "test" );
  /// archive.save_to_file( Path::new( "archive.json" ) ).unwrap();
  /// ```
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
  /// # Returns
  ///
  /// Loaded archive
  ///
  /// # Examples
  ///
  /// ```rust,ignore
  /// use genfile_core::TemplateArchive;
  /// use std::path::Path;
  ///
  /// let archive = TemplateArchive::load_from_file( Path::new( "archive.json" ) ).unwrap();
  /// ```
  pub fn load_from_file( path: &Path ) -> Result< Self, Error >
  {
    let json = std::fs::read_to_string( path )?;
    Self::from_json( &json )
  }
}

/// Materialization report
#[derive(Debug, Clone, Default)]
pub struct MaterializationReport
{
  /// Files successfully created
  pub files_created: Vec< PathBuf >,

  /// Files successfully updated
  pub files_updated: Vec< PathBuf >,

  /// Files skipped (already exist, no changes)
  pub files_skipped: Vec< PathBuf >,

  /// Directories created
  pub directories_created: Vec< PathBuf >,

  /// Total bytes written
  pub total_bytes_written: usize,

  /// Errors encountered (if any)
  pub errors: Vec< ( PathBuf, String ) >,
}
