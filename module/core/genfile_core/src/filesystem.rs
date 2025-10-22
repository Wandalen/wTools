/// File system abstractions for testability
use std ::path ::Path;
use std ::collections ::HashMap;
use crate ::Error;

/// File system operations trait.
///
/// Abstracts file I/O operations to enable both real filesystem usage and
/// in-memory testing without touching the actual disk.
///
/// # Methods
///
/// - `read()`: Read file contents as UTF-8 string
/// - `write()`: Write string content to file
/// - `exists()`: Check if file exists
///
/// # Examples
///
/// ```rust,ignore
/// use genfile_core::{ FileSystem, MemoryFileSystem };
/// use std::path::PathBuf;
///
/// let mut fs = MemoryFileSystem::new();
/// fs.write( &PathBuf::from( "test.txt" ), "Hello" )?;
/// assert_eq!( fs.read( &PathBuf::from( "test.txt" ) )?, "Hello" );
/// # Ok::<(), genfile_core::Error>(())
/// ```
pub trait FileSystem
{
  /// Reads file contents as a UTF-8 string.
  ///
  /// # Parameters
  ///
  /// - `path`: Path to file to read
  ///
  /// # Returns
  ///
  /// File contents on success, Error on failure
  ///
  /// # Errors
  ///
  /// Returns `Error::Fs` if file doesn't exist or can't be read
  fn read( &self, path: &Path ) -> Result< String, Error >;

  /// Writes string content to a file.
  ///
  /// # Parameters
  ///
  /// - `path`: Path where content should be written
  /// - `content`: UTF-8 string content to write
  ///
  /// # Returns
  ///
  /// Ok(()) on success, Error on failure
  ///
  /// # Errors
  ///
  /// Returns `Error::Fs` if file can't be written
  fn write( &mut self, path: &Path, content: &str ) -> Result< (), Error >;

  /// Checks if a file exists.
  ///
  /// # Parameters
  ///
  /// - `path`: Path to check
  ///
  /// # Returns
  ///
  /// `true` if file exists, `false` otherwise
  fn exists( &self, path: &Path ) -> bool;
}

/// In-memory file system for testing.
///
/// Stores files in a `HashMap` for fast, isolated testing without disk I/O.
/// Useful for unit tests and integration tests that need predictable,
/// reproducible file system state.
///
/// # Examples
///
/// ```rust
/// use genfile_core::{ FileSystem, MemoryFileSystem };
/// use std::path::PathBuf;
///
/// let mut fs = MemoryFileSystem::new();
///
/// // Write and read without touching disk
/// fs.write( &PathBuf::from( "config.toml" ), "key = \"value\"" ).unwrap();
/// let content = fs.read( &PathBuf::from( "config.toml" ) ).unwrap();
/// assert_eq!( content, "key = \"value\"" );
/// ```
#[ derive( Debug, Default ) ]
pub struct MemoryFileSystem
{
  /// In-memory storage of file paths to contents
  files: HashMap< std ::path ::PathBuf, String >,
}

impl MemoryFileSystem
{
  /// Creates a new empty in-memory file system.
  ///
  /// # Returns
  ///
  /// New `MemoryFileSystem` with no files
  ///
  /// # Examples
  ///
  /// ```rust
  /// use genfile_core::MemoryFileSystem;
  ///
  /// let fs = MemoryFileSystem::new();
  /// ```
  #[must_use] 
  pub fn new() -> Self
  {
    Self
    {
      files: HashMap ::new(),
    }
  }
}

impl FileSystem for MemoryFileSystem
{
  fn read( &self, path: &Path ) -> Result< String, Error >
  {
    self
      .files
      .get( path )
      .cloned()
      .ok_or_else( ||
      {
        Error ::Fs
        (
          std ::io ::Error ::new
          (
            std ::io ::ErrorKind ::NotFound,
            format!( "File not found: {}", path.display() )
          )
        )
      })
  }

  fn write( &mut self, path: &Path, content: &str ) -> Result< (), Error >
  {
    self.files.insert( path.to_path_buf(), content.to_string() );
    Ok(())
  }

  fn exists( &self, path: &Path ) -> bool
  {
    self.files.contains_key( path )
  }
}

/// Real file system implementation.
///
/// Performs actual disk I/O operations. Use this for production code.
/// For testing, prefer `MemoryFileSystem`.
///
/// # Examples
///
/// ```rust,ignore
/// use genfile_core::{ FileSystem, RealFileSystem };
/// use std::path::PathBuf;
///
/// let mut fs = RealFileSystem::new();
///
/// // Actual file I/O
/// fs.write( &PathBuf::from( "output.txt" ), "content" )?;
/// # Ok::<(), genfile_core::Error>(())
/// ```
#[ derive( Debug ) ]
pub struct RealFileSystem;

impl RealFileSystem
{
  /// Creates a new real file system accessor.
  ///
  /// # Returns
  ///
  /// New `RealFileSystem` instance
  ///
  /// # Examples
  ///
  /// ```rust
  /// use genfile_core::RealFileSystem;
  ///
  /// let fs = RealFileSystem::new();
  /// ```
  #[must_use] 
  pub fn new() -> Self
  {
    Self
  }
}

impl Default for RealFileSystem
{
  fn default() -> Self
  {
    Self ::new()
  }
}

impl FileSystem for RealFileSystem
{
  fn read( &self, path: &Path ) -> Result< String, Error >
  {
    std ::fs ::read_to_string( path ).map_err( Error ::from )
  }

  fn write( &mut self, path: &Path, content: &str ) -> Result< (), Error >
  {
    // Create parent directories if they don't exist
    if let Some( parent ) = path.parent()
    {
      std ::fs ::create_dir_all( parent )?;
    }

    std ::fs ::write( path, content ).map_err( Error ::from )
  }

  fn exists( &self, path: &Path ) -> bool
  {
    path.exists()
  }
}
