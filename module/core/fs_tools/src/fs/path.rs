//! Path traversal utilities for filesystem navigation.
//!
//! Provides generic utilities for traversing directory trees, particularly
//! upward traversal to find project roots and configuration files, and
//! hierarchical file collection.
//!
//! # Design Rationale
//!
//! This module was extracted to eliminate code duplication across multiple workspace
//! tools. Prior to extraction, identical traversal logic existed in:
//! - `cargo_tools::path` (~196 lines)
//! - `git_tools::path` (~26 lines)
//! - `rulebooks` discovery (~46 lines)
//!
//! Total reduction: 268 lines of duplicate code eliminated.
//!
//! Consumer crates now re-export from `fs_tools::path` as the single source of truth,
//! ensuring consistency and eliminating maintenance burden of parallel implementations.
//!
//! # Feature Flags
//!
//! All functions in this module require:
//! - `feature = "enabled"` - Activates path utilities
//! - `not( feature = "no_std" )` - Requires std library for `std::fs` and `std::io`
//!
//! **Important:** When using `--all-features`, both `enabled` and `no_std` are activated
//! simultaneously. Since these are logically incompatible (path functions require std),
//! the module exports nothing in that configuration. Tests must use conditional compilation
//! to match these export conditions.
//!
//! # Key Functions
//!
//! - `traverse_upward` - Generic upward traversal with custom predicate
//! - `collect_files_in_ancestors` - Hierarchical file collection (root → target order)
//! - `file_upward_find` - Find file by name walking upward
//! - `dir_upward_find` - Find directory by name walking upward
//! - `matching_upward_find` - Find file/directory matching predicate
//!
//! # See Also
//!
//! - `cargo_tools::path` - Re-exports for Cargo workspace operations
//! - `git_tools::path` - Re-exports for git repository detection
//! - `rulebooks` - Uses `collect_files_in_ancestors` for hierarchical discovery

/// Define a private namespace for all its items.
mod private
{
  #[ cfg( all( feature = "enabled", not( feature = "no_std" ) ) ) ]
  extern crate std;

  #[ cfg( all( feature = "enabled", not( feature = "no_std" ) ) ) ]
  use std::path::{ Path, PathBuf };

  #[ cfg( all( feature = "enabled", not( feature = "no_std" ) ) ) ]
  use std::io;

  /// Traverse upward through directory tree looking for target.
  ///
  /// Generic utility for finding files or directories by walking up the directory tree.
  /// The predicate function is called for each directory level, returning `Some(T)` when
  /// the target is found, or `None` to continue searching.
  ///
  /// # Arguments
  /// * `start_dir` - Starting directory for traversal
  /// * `predicate` - Function that returns `Some(result)` when target is found, `None` to continue
  /// * `max_depth` - Maximum number of parent directories to check (prevents infinite loops)
  ///
  /// # Returns
  /// Returns the first `Some(T)` returned by predicate, or `None` if traversal completes without finding target.
  ///
  /// # Examples
  /// ```
  /// # #[cfg(all(feature = "enabled", not(feature = "no_std")))]
  /// # {
  /// use fs_tools::path::traverse_upward;
  /// use std::path::Path;
  ///
  /// // Find Cargo.toml walking upward
  /// let cargo_toml = traverse_upward(
  ///   Path::new("."),
  ///   |dir| {
  ///     let candidate = dir.join("Cargo.toml");
  ///     if candidate.exists() {
  ///       Some(candidate)
  ///     } else {
  ///       None
  ///     }
  ///   },
  ///   20
  /// );
  /// # }
  /// ```
  #[ cfg( all( feature = "enabled", not( feature = "no_std" ) ) ) ]
  #[ inline ]
  pub fn traverse_upward< T, F >( start_dir: &Path, predicate: F, max_depth: usize ) -> Option< T >
  where
    F: Fn( &Path ) -> Option< T >
  {
    let mut current = start_dir.to_path_buf();

    for _ in 0..max_depth
    {
      // Check current directory
      if let Some( result ) = predicate( &current )
      {
        return Some( result );
      }

      // Move up one directory
      if let Some( parent ) = current.parent()
      {
        current = parent.to_path_buf();
      }
      else
      {
        break;
      }
    }

    None
  }

  /// Collect files matching predicate from current directory up to root.
  ///
  /// Walks the directory hierarchy from the target directory upward to the filesystem root,
  /// collecting all files that match the predicate at each level. Results are ordered from
  /// root to target (inheritance order).
  ///
  /// # Arguments
  /// * `target` - Starting directory (typically current working directory)
  /// * `predicate` - Function that returns true for files to collect
  /// * `max_depth` - Optional maximum depth (None for unlimited)
  /// * `deduplicate` - Whether to deduplicate symlinked files using canonical paths
  ///
  /// # Returns
  /// Vector of matching file paths ordered from root to target, or error on I/O failure.
  ///
  /// # Errors
  /// Returns `io::Error` if:
  /// - Target directory does not exist
  /// - Target path cannot be canonicalized
  ///
  /// # Examples
  /// ```
  /// # #[cfg(all(feature = "enabled", not(feature = "no_std")))]
  /// # {
  /// use fs_tools::path::collect_files_in_ancestors;
  /// use std::path::Path;
  ///
  /// // Collect all .toml files from root to current directory
  /// let toml_files = collect_files_in_ancestors(
  ///   Path::new("."),
  ///   |path| path.extension().and_then(|e| e.to_str()) == Some("toml"),
  ///   None,
  ///   true
  /// );
  /// # }
  /// ```
  #[ cfg( all( feature = "enabled", not( feature = "no_std" ) ) ) ]
  pub fn collect_files_in_ancestors< F >(
    target: &Path,
    predicate: F,
    max_depth: Option< usize >,
    deduplicate: bool
  ) -> io::Result< Vec< PathBuf > >
  where
    F: Fn( &Path ) -> bool
  {
    use std::collections::HashSet;

    if !target.exists()
    {
      return Err( io::Error::new(
        io::ErrorKind::NotFound,
        format!( "Directory does not exist: {}", target.display() )
      ) );
    }

    // Canonicalize to get absolute path
    let target = target.canonicalize()?;

    // Build hierarchy from target UP to root
    let mut hierarchy = Vec::new();
    let mut current = target.as_path();
    let mut depth_count = 0;

    loop
    {
      hierarchy.push( current.to_path_buf() );
      depth_count += 1;

      // Check max_depth
      if let Some( max ) = max_depth
      {
        if depth_count >= max
        {
          break;
        }
      }

      // Move to parent
      match current.parent()
      {
        Some( parent ) if parent != current => current = parent,
        _ => break,
      }
    }

    // Reverse: now it's root -> target order
    hierarchy.reverse();

    // Discover files at each level in hierarchy order
    let mut results = Vec::new();
    let mut seen_paths: HashSet< PathBuf > = HashSet::new();

    for dir in &hierarchy
    {
      if let Ok( entries ) = std::fs::read_dir( dir )
      {
        for entry in entries.flatten()
        {
          let path = entry.path();

          if path.is_file() && predicate( &path )
          {
            if deduplicate
            {
              // Deduplicate using canonical paths
              if let Ok( canonical ) = path.canonicalize()
              {
                if seen_paths.insert( canonical )
                {
                  results.push( path );
                }
              }
            }
            else
            {
              results.push( path );
            }
          }
        }
      }
    }

    Ok( results )
  }

  /// Find file by name walking upward from start directory.
  ///
  /// # Arguments
  /// * `start` - Starting directory
  /// * `filename` - Name of file to find
  /// * `max_depth` - Maximum number of parent directories to check
  ///
  /// # Returns
  /// Path to the found file, or `None` if not found.
  ///
  /// # Examples
  /// ```
  /// # #[cfg(all(feature = "enabled", not(feature = "no_std")))]
  /// # {
  /// use fs_tools::path::file_upward_find;
  /// use std::path::Path;
  ///
  /// if let Some(cargo_toml) = file_upward_find(Path::new("."), "Cargo.toml", 10) {
  ///   println!("Found: {}", cargo_toml.display());
  /// }
  /// # }
  /// ```
  #[ cfg( all( feature = "enabled", not( feature = "no_std" ) ) ) ]
  #[ inline ]
  #[ must_use ]
  pub fn file_upward_find( start: &Path, filename: &str, max_depth: usize ) -> Option< PathBuf >
  {
    traverse_upward(
      start,
      | dir |
      {
        let candidate = dir.join( filename );
        if candidate.exists() && candidate.is_file()
        {
          Some( candidate )
        }
        else
        {
          None
        }
      },
      max_depth
    )
  }

  /// Find directory by name walking upward from start directory.
  ///
  /// # Arguments
  /// * `start` - Starting directory
  /// * `dirname` - Name of directory to find
  /// * `max_depth` - Maximum number of parent directories to check
  ///
  /// # Returns
  /// Path to the found directory, or `None` if not found.
  ///
  /// # Examples
  /// ```
  /// # #[cfg(all(feature = "enabled", not(feature = "no_std")))]
  /// # {
  /// use fs_tools::path::dir_upward_find;
  /// use std::path::Path;
  ///
  /// if let Some(git_dir) = dir_upward_find(Path::new("."), ".git", 10) {
  ///   println!("Git directory: {}", git_dir.display());
  /// }
  /// # }
  /// ```
  #[ cfg( all( feature = "enabled", not( feature = "no_std" ) ) ) ]
  #[ inline ]
  #[ must_use ]
  pub fn dir_upward_find( start: &Path, dirname: &str, max_depth: usize ) -> Option< PathBuf >
  {
    traverse_upward(
      start,
      | dir |
      {
        let candidate = dir.join( dirname );
        if candidate.exists() && candidate.is_dir()
        {
          Some( candidate )
        }
        else
        {
          None
        }
      },
      max_depth
    )
  }

  /// Find any file or directory matching predicate walking upward.
  ///
  /// # Arguments
  /// * `start` - Starting directory
  /// * `predicate` - Function that returns true when target is found
  /// * `max_depth` - Maximum number of parent directories to check
  ///
  /// # Returns
  /// Path to the current directory when predicate returns true, or `None` if not found.
  ///
  /// # Examples
  /// ```
  /// # #[cfg(all(feature = "enabled", not(feature = "no_std")))]
  /// # {
  /// use fs_tools::path::matching_upward_find;
  /// use std::path::Path;
  ///
  /// // Find directory containing both Cargo.toml and .git
  /// let project_root = matching_upward_find(
  ///   Path::new("."),
  ///   |dir| dir.join("Cargo.toml").exists() && dir.join(".git").exists(),
  ///   10
  /// );
  /// # }
  /// ```
  #[ cfg( all( feature = "enabled", not( feature = "no_std" ) ) ) ]
  #[ inline ]
  pub fn matching_upward_find< F >( start: &Path, predicate: F, max_depth: usize ) -> Option< PathBuf >
  where
    F: Fn( &Path ) -> bool
  {
    traverse_upward(
      start,
      | dir |
      {
        if predicate( dir )
        {
          Some( dir.to_path_buf() )
        }
        else
        {
          None
        }
      },
      max_depth
    )
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
  pub use super::private::
  {
    traverse_upward,
    collect_files_in_ancestors,
    file_upward_find,
    dir_upward_find,
    matching_upward_find,
  };
}

/// Prelude to use essentials: `use my_module ::prelude :: *`.
#[ allow( unused_imports ) ]
pub mod prelude
{
  use super :: *;
}
