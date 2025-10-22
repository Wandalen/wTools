/// Template materialization orchestrator
use crate ::
{
  TemplateValue,
  TemplateRenderer,
  FileSystem,
  Values,
  FileDescriptor,
  WriteMode,
  Error,
};

/// Template holder for materializing templates into files.
///
/// Orchestrates the entire template processing pipeline: reads templates,
/// applies values, renders content, and writes to files using pluggable
/// filesystem and renderer implementations.
///
/// Generic over value type `V` and renderer type `R` for maximum flexibility.
///
/// # Type Parameters
///
/// - `V`: Value type implementing `TemplateValue` trait
/// - `R`: Renderer implementing `TemplateRenderer` trait
/// - `FS`: File system implementing `FileSystem` trait
///
/// # Examples
///
/// ```rust,ignore
/// use genfile_core::{ Template, HandlebarsRenderer, MemoryFileSystem, Value, FileDescriptor, WriteMode };
/// use std::path::PathBuf;
///
/// let renderer = HandlebarsRenderer::new();
/// let mut filesystem = MemoryFileSystem::new();
///
/// // Set up template
/// filesystem.write( &PathBuf::from( "template.hbs" ), "Hello {{name}}!" )?;
///
/// let mut template = Template::new( renderer, filesystem );
/// template.insert_value( "name", Value::String( "World".into() ) );
///
/// template.add_file( FileDescriptor
/// {
///   file_path: PathBuf::from( "output.txt" ),
///   template_path: PathBuf::from( "template.hbs" ),
///   write_mode: WriteMode::Rewrite,
/// });
///
/// template.materialize()?;
/// # Ok::<(), genfile_core::Error>(())
/// ```
#[ derive( Debug ) ]
pub struct Template< V, R, FS >
where
  V: TemplateValue + serde ::Serialize + serde ::de ::DeserializeOwned,
  R: TemplateRenderer,
  FS: FileSystem,
{
  /// Value storage for template substitution
  values: Values< V >,

  /// File descriptors defining what to generate
  files: Vec< FileDescriptor >,

  /// Template renderer
  renderer: R,

  /// File system for I/O
  filesystem: FS,
}

impl< V, R, FS > Template< V, R, FS >
where
  V: TemplateValue + serde ::Serialize + serde ::de ::DeserializeOwned,
  R: TemplateRenderer,
  FS: FileSystem,
{
  /// Creates a new template with the given renderer and filesystem.
  ///
  /// # Parameters
  ///
  /// - `renderer`: Template rendering engine
  /// - `filesystem`: File system for I/O operations
  ///
  /// # Returns
  ///
  /// New Template instance
  ///
  /// # Examples
  ///
  /// ```rust
  /// use genfile_core::{ Template, HandlebarsRenderer, MemoryFileSystem, Value };
  ///
  /// let renderer = HandlebarsRenderer::new();
  /// let filesystem = MemoryFileSystem::new();
  /// let template: Template< Value, _, _ > = Template::new( renderer, filesystem );
  /// ```
  pub fn new( renderer: R, filesystem: FS ) -> Self
  {
    Self
    {
      values: Values ::new(),
      files: Vec ::new(),
      renderer,
      filesystem,
    }
  }

  /// Inserts a value for template substitution.
  ///
  /// # Parameters
  ///
  /// - `key`: Parameter name
  /// - `value`: Value to substitute
  ///
  /// # Examples
  ///
  /// ```rust
  /// use genfile_core::{ Template, HandlebarsRenderer, MemoryFileSystem, Value };
  ///
  /// let mut template: Template< Value, _, _ > = Template::new( HandlebarsRenderer::new(), MemoryFileSystem::new() );
  /// template.insert_value( "name", Value::String( "test".into() ) );
  /// ```
  pub fn insert_value( &mut self, key: &str, value: V )
  {
    self.values.insert( key, value );
  }

  /// Checks if a value exists for the given key.
  ///
  /// # Parameters
  ///
  /// - `key`: Parameter name to check
  ///
  /// # Returns
  ///
  /// `true` if value exists, `false` otherwise
  pub fn has_value( &self, key: &str ) -> bool
  {
    self.values.has_value( key )
  }

  /// Adds a file descriptor specifying what file to generate.
  ///
  /// # Parameters
  ///
  /// - `descriptor`: File descriptor with paths and write mode
  ///
  /// # Examples
  ///
  /// ```rust
  /// use genfile_core::{ Template, HandlebarsRenderer, MemoryFileSystem, Value, FileDescriptor, WriteMode };
  /// use std::path::PathBuf;
  ///
  /// let mut template: Template< Value, _, _ > = Template::new( HandlebarsRenderer::new(), MemoryFileSystem::new() );
  /// template.add_file( FileDescriptor
  /// {
  ///   file_path: PathBuf::from( "output.txt" ),
  ///   template_path: PathBuf::from( "template.hbs" ),
  ///   write_mode: WriteMode::Rewrite,
  /// });
  /// ```
  pub fn add_file( &mut self, descriptor: FileDescriptor )
  {
    self.files.push( descriptor );
  }

  /// Materializes all templates into files.
  ///
  /// Reads each template, renders it with values, and writes to the target
  /// file using the appropriate write mode.
  ///
  /// # Returns
  ///
  /// Ok(()) on success, Error if any operation fails
  ///
  /// # Errors
  ///
  /// Returns error if:
  /// - Template file can't be read
  /// - Template rendering fails
  /// - Output file can't be written
  /// - TOML merging fails (for `TomlExtend` mode)
  ///
  /// # Examples
  ///
  /// ```rust,ignore
  /// template.materialize()?;
  /// ```
  pub fn materialize( &mut self ) -> Result< (), Error >
  {
    let serialized_values = self.values.to_serializable();

    for file_desc in &self.files
    {
      // Read template
      let template_content = self.filesystem.read( &file_desc.template_path )?;

      // Render
      let rendered = self.renderer.render( &template_content, &serialized_values )?;

      // Write based on mode
      match file_desc.write_mode
      {
        WriteMode ::Rewrite =>
        {
          self.filesystem.write( &file_desc.file_path, &rendered )?;
        }
        WriteMode ::TomlExtend =>
        {
          // TODO: Implement TOML smart merging (FR12, FR19)
          // For now, just write (will implement in follow-up)
          self.filesystem.write( &file_desc.file_path, &rendered )?;
        }
      }
    }

    Ok(())
  }

  /// Returns a reference to the filesystem.
  ///
  /// Useful for testing to verify generated files.
  ///
  /// # Returns
  ///
  /// Reference to the filesystem
  pub fn filesystem( &self ) -> &FS
  {
    &self.filesystem
  }
}
