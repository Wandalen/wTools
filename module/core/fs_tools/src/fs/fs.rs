/// Define a private namespace for all its items.
mod private
{
  #[ cfg( all( feature = "enabled", not( feature = "no_std" ) ) ) ]
  extern crate std;

  #[ cfg( all( feature = "enabled", not( feature = "no_std" ) ) ) ]
  use std::path::PathBuf;

  /// Temporary directory management structure.
  ///
  /// Provides path management for temporary directories with configurable
  /// base, prefix, and postfix components.
  ///
  /// Only available when `enabled` feature is active and `no_std` feature is disabled.
  #[ cfg( all( feature = "enabled", not( feature = "no_std" ) ) ) ]
  #[ derive( Debug ) ]
  pub struct TempDir
  {
    /// Base directory path.
    pub base_path : PathBuf,
    /// Prefix path component.
    pub prefix_path : PathBuf,
    /// Postfix path component.
    pub postfix_path : PathBuf,
  }

  #[ cfg( all( feature = "enabled", not( feature = "no_std" ) ) ) ]
  impl TempDir
  {
    /// Creates a new TempDir instance with empty paths.
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
    pub fn new() -> Self
    {
      Self
      {
        base_path : PathBuf::new(),
        prefix_path : PathBuf::new(),
        postfix_path : PathBuf::new(),
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
