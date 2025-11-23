/// Define a private namespace for all its items.
mod private
{
  #[ cfg( all( feature = "enabled", not( feature = "no_std" ) ) ) ]
  extern crate std;

  #[ cfg( all( feature = "enabled", not( feature = "no_std" ) ) ) ]
  use std::path::PathBuf;

  #[ cfg( all( feature = "enabled", not( feature = "no_std" ) ) ) ]
  use std::fs;

  #[ cfg( all( feature = "enabled", not( feature = "no_std" ) ) ) ]
  use std::io;

  /// Temporary directory management structure with RAII cleanup.
  ///
  /// Provides path management for temporary directories with configurable
  /// base, prefix, and postfix components. Directories created via `create()`
  /// or `create_all()` are automatically removed when the struct is dropped.
  ///
  /// # Design
  ///
  /// - **Public fields**: Allow direct path manipulation for flexibility
  /// - **RAII cleanup**: Only cleans up directories created by `create()`/`create_all()`
  /// - **Manual paths**: Setting paths directly (without calling create) won't trigger cleanup
  ///
  /// Only available when `enabled` feature is active and `no_std` feature is disabled.
  ///
  /// # Example
  ///
  /// ```no_run
  /// use fs_tools::TempDir;
  /// use std::path::PathBuf;
  ///
  /// let mut temp = TempDir::new();
  /// temp.base_path = std::env::temp_dir();
  /// temp.prefix_path = PathBuf::from( "my_app" );
  /// temp.postfix_path = PathBuf::from( "session_1" );
  ///
  /// // Create the directory (enables RAII cleanup)
  /// let path = temp.create_all().expect( "failed to create" );
  /// assert!( path.is_dir() );
  ///
  /// // Directory is automatically removed when `temp` goes out of scope
  /// ```
  #[ cfg( all( feature = "enabled", not( feature = "no_std" ) ) ) ]
  #[ derive( Debug ) ]
  pub struct TempDir
  {
    /// Base directory path (e.g., `/tmp`).
    pub base_path : PathBuf,
    /// Prefix path component (e.g., `app_name`).
    pub prefix_path : PathBuf,
    /// Postfix path component (e.g., `unique_id`).
    pub postfix_path : PathBuf,
    /// Internal: tracks path created by `create()`/`create_all()` for RAII cleanup.
    /// None means no auto-cleanup; Some(path) means cleanup on drop.
    created_path : Option< PathBuf >,
  }

  #[ cfg( all( feature = "enabled", not( feature = "no_std" ) ) ) ]
  impl Default for TempDir
  {
    fn default() -> Self
    {
      Self::new()
    }
  }

  #[ cfg( all( feature = "enabled", not( feature = "no_std" ) ) ) ]
  impl TempDir
  {
    /// Creates a new `TempDir` instance with empty paths.
    ///
    /// The returned instance has no auto-cleanup enabled. Call `create()` or
    /// `create_all()` to create the directory and enable RAII cleanup.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[cfg(all(feature = "enabled", not(feature = "no_std")))]
    /// # {
    /// use fs_tools::TempDir;
    /// let temp_dir = TempDir::new();
    /// assert!( temp_dir.base_path.as_os_str().is_empty() );
    /// # }
    /// ```
    #[ must_use ]
    pub fn new() -> Self
    {
      Self
      {
        base_path : PathBuf::new(),
        prefix_path : PathBuf::new(),
        postfix_path : PathBuf::new(),
        created_path : None,
      }
    }

    /// Returns the full path by joining base, prefix, and postfix components.
    ///
    /// Empty components are skipped during joining.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[cfg(all(feature = "enabled", not(feature = "no_std")))]
    /// # {
    /// use fs_tools::TempDir;
    /// use std::path::PathBuf;
    ///
    /// let mut temp = TempDir::new();
    /// temp.base_path = PathBuf::from( "/tmp" );
    /// temp.prefix_path = PathBuf::from( "test" );
    /// temp.postfix_path = PathBuf::from( "run_1" );
    ///
    /// assert_eq!( temp.full_path(), PathBuf::from( "/tmp/test/run_1" ) );
    /// # }
    /// ```
    #[ must_use ]
    pub fn full_path( &self ) -> PathBuf
    {
      let mut path = PathBuf::new();

      if !self.base_path.as_os_str().is_empty()
      {
        path.push( &self.base_path );
      }

      if !self.prefix_path.as_os_str().is_empty()
      {
        path.push( &self.prefix_path );
      }

      if !self.postfix_path.as_os_str().is_empty()
      {
        path.push( &self.postfix_path );
      }

      path
    }

    /// Creates the directory at `full_path()`.
    ///
    /// The parent directory must already exist. Use `create_all()` for
    /// recursive creation.
    ///
    /// Enables RAII cleanup: the directory will be removed when this
    /// struct is dropped.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The parent directory doesn't exist
    /// - The directory already exists
    /// - Permission denied
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # #[cfg(all(feature = "enabled", not(feature = "no_std")))]
    /// # {
    /// use fs_tools::TempDir;
    /// use std::path::PathBuf;
    ///
    /// let mut temp = TempDir::new();
    /// temp.base_path = std::env::temp_dir();
    /// temp.prefix_path = PathBuf::from( "my_test" );
    ///
    /// let path = temp.create().expect( "failed to create" );
    /// assert!( path.is_dir() );
    /// // Cleaned up on drop
    /// # }
    /// ```
    pub fn create( &mut self ) -> io::Result< PathBuf >
    {
      let path = self.full_path();
      fs::create_dir( &path )?;
      self.created_path = Some( path.clone() );
      Ok( path )
    }

    /// Creates the directory at `full_path()` and all parent directories.
    ///
    /// This is the recursive version of `create()`. It will create all
    /// missing parent directories.
    ///
    /// Enables RAII cleanup: the created directory (not parents) will be
    /// removed when this struct is dropped.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Permission denied
    /// - Path is not valid
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # #[cfg(all(feature = "enabled", not(feature = "no_std")))]
    /// # {
    /// use fs_tools::TempDir;
    /// use std::path::PathBuf;
    ///
    /// let mut temp = TempDir::new();
    /// temp.base_path = std::env::temp_dir();
    /// temp.prefix_path = PathBuf::from( "deep" );
    /// temp.postfix_path = PathBuf::from( "nested/path" );
    ///
    /// let path = temp.create_all().expect( "failed to create" );
    /// assert!( path.is_dir() );
    /// // Cleaned up on drop
    /// # }
    /// ```
    pub fn create_all( &mut self ) -> io::Result< PathBuf >
    {
      let path = self.full_path();
      fs::create_dir_all( &path )?;
      self.created_path = Some( path.clone() );
      Ok( path )
    }
  }

  #[ cfg( all( feature = "enabled", not( feature = "no_std" ) ) ) ]
  impl Drop for TempDir
  {
    /// Removes the directory if it was created by `create()` or `create_all()`.
    ///
    /// Errors during removal are silently ignored (can't panic in Drop).
    /// If the directory was already removed manually, this is a no-op.
    fn drop( &mut self )
    {
      if let Some( ref path ) = self.created_path
      {
        // Use remove_dir_all to handle non-empty directories
        // Ignore errors - can't panic in Drop
        let _ = fs::remove_dir_all( path );
      }
    }
  }
}

/// Own namespace of the module.
#[ allow( unused_imports ) ]
pub mod own
{
  use super :: *;
  #[ doc( inline ) ]
  pub use orphan :: *;
}

#[ doc( inline ) ]
#[ allow( unused_imports ) ]
pub use own :: *;

/// Shared with parent namespace of the module
#[ allow( unused_imports ) ]
pub mod orphan
{
  use super :: *;
  #[ doc( inline ) ]
  pub use exposed :: *;
}

/// Exposed namespace of the module.
#[ allow( unused_imports ) ]
pub mod exposed
{
  use super :: *;
  #[ doc( inline ) ]
  pub use prelude :: *;
  #[ cfg( all( feature = "enabled", not( feature = "no_std" ) ) ) ]
  #[ doc( inline ) ]
  pub use super::private::TempDir;
}

/// Prelude to use essentials: `use my_module ::prelude :: *`.
#[ allow( unused_imports ) ]
pub mod prelude
{
  use super :: *;
}
