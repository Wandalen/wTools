//! Universal workspace-relative path resolution for Rust projects
//!
//! This crate provides consistent, reliable path management regardless of execution context
//! or working directory. It solves common path resolution issues in software projects by
//! leveraging cargo's environment variable injection system.
//!
//! ## problem solved
//!
//! - **execution context dependency**: paths break when code runs from different directories
//! - **environment inconsistency**: different developers have different working directory habits
//! - **testing fragility**: tests fail when run from different locations
//! - **ci/cd brittleness**: automated systems may execute from unexpected directories
//!
//! ## quick start
//!
//! 1. Configure cargo in workspace root `.cargo/config.toml`:
//! ```toml
//! [env]
//! WORKSPACE_PATH = { value = ".", relative = true }
//! ```
//!
//! 2. Use in your code:
//! ```rust
//! use workspace_tools::{ workspace, WorkspaceError };
//!
//! # fn main() -> Result<(), WorkspaceError> {
//! // get workspace instance
//! let ws = workspace()?;
//!
//! // resolve workspace-relative paths
//! let config_path = ws.config_dir().join("app.toml");
//! let data_path = ws.data_dir().join("cache.db");
//! # Ok(())
//! # }
//! ```
//!
//! ## features
//!
//! - **`glob`**: enables pattern-based resource discovery
//! - **`secret_management`**: provides secure configuration file handling utilities

#![ warn( missing_docs ) ]

use std::
{
  env,
  path::{ Path, PathBuf },
};

#[ cfg( feature = "glob" ) ]
use glob::glob;

#[ cfg( feature = "secret_management" ) ]
use std::{ collections::HashMap, fs };

/// workspace path resolution errors
#[ derive( Debug, Clone ) ]
#[ non_exhaustive ]
pub enum WorkspaceError
{
  /// configuration parsing error
  ConfigurationError( String ),
  /// environment variable not found
  EnvironmentVariableMissing( String ),
  /// glob pattern error
  #[ cfg( feature = "glob" ) ]
  GlobError( String ),
  /// io error during file operations
  IoError( String ),
  /// path does not exist
  PathNotFound( PathBuf ),
  /// path is outside workspace boundaries
  PathOutsideWorkspace( PathBuf ),
}

impl core::fmt::Display for WorkspaceError
{
  #[ inline ]
  fn fmt( &self, f : &mut core::fmt::Formatter< '_ > ) -> core::fmt::Result
  {
    match self
    {
      WorkspaceError::ConfigurationError( msg ) =>
        write!( f, "configuration error: {msg}" ),
      WorkspaceError::EnvironmentVariableMissing( var ) =>
        write!( f, "environment variable '{var}' not found. ensure .cargo/config.toml is properly configured with WORKSPACE_PATH" ),
      #[ cfg( feature = "glob" ) ]
      WorkspaceError::GlobError( msg ) =>
        write!( f, "glob pattern error: {msg}" ),
      WorkspaceError::IoError( msg ) =>
        write!( f, "io error: {msg}" ),
      WorkspaceError::PathNotFound( path ) =>
        write!( f, "path not found: {}. ensure the workspace structure is properly initialized", path.display() ),
      WorkspaceError::PathOutsideWorkspace( path ) =>
        write!( f, "path is outside workspace boundaries: {}", path.display() ),
    }
  }
}

impl core::error::Error for WorkspaceError {}

/// result type for workspace operations
pub type Result< T > = core::result::Result< T, WorkspaceError >;

/// workspace path resolver providing centralized access to workspace-relative paths
///
/// the workspace struct encapsulates workspace root detection and provides methods
/// for resolving standard directory paths and joining workspace-relative paths safely.
#[ derive( Debug, Clone ) ]
pub struct Workspace
{
  root : PathBuf,
}

impl Workspace
{
  /// resolve workspace from environment variables
  ///
  /// reads the `WORKSPACE_PATH` environment variable set by cargo configuration
  /// and validates that the workspace root exists.
  ///
  /// # errors
  ///
  /// returns error if:
  /// - `WORKSPACE_PATH` environment variable is not set
  /// - the path specified by `WORKSPACE_PATH` does not exist
  ///
  /// # examples
  ///
  /// ```rust
  /// use workspace_tools::Workspace;
  ///
  /// # std::env::set_var("WORKSPACE_PATH", std::env::current_dir().unwrap());
  /// let workspace = Workspace::resolve()?;
  /// println!("workspace root: {}", workspace.root().display());
  /// # Ok::<(), workspace_tools::WorkspaceError>(())
  /// ```
  /// 
  /// # Errors
  /// 
  /// Returns an error if the workspace path environment variable is not set or the path doesn't exist.
  #[inline]
  pub fn resolve() -> Result< Self >
  {
    let root = Self::get_env_path( "WORKSPACE_PATH" )?;

    if !root.exists()
    {
      return Err( WorkspaceError::PathNotFound( root ) );
    }

    Ok( Self { root } )
  }

  /// resolve workspace with fallback strategies
  ///
  /// tries multiple strategies to resolve workspace root:
  /// 1. environment variable (`WORKSPACE_PATH`)
  /// 2. current working directory
  /// 3. git repository root (if .git directory found)
  ///
  /// # examples
  ///
  /// ```rust
  /// use workspace_tools::Workspace;
  ///
  /// // this will always succeed with some workspace root
  /// let workspace = Workspace::resolve_or_fallback();
  /// ```
  #[must_use]
  #[inline]
  pub fn resolve_or_fallback() -> Self
  {
    Self::resolve()
      .or_else( |_| Self::from_current_dir() )
      .or_else( |_| Self::from_git_root() )
      .unwrap_or_else( |_| Self::from_cwd() )
  }

  /// create workspace from current working directory
  ///
  /// # Errors
  ///
  /// returns error if current directory cannot be accessed
  #[inline]
  pub fn from_current_dir() -> Result< Self >
  {
    let root = env::current_dir()
      .map_err( | e | WorkspaceError::IoError( e.to_string() ) )?;
    Ok( Self { root } )
  }

  /// create workspace from git repository root
  ///
  /// searches upward from current directory for .git directory
  ///
  /// # Errors
  ///
  /// returns error if current directory cannot be accessed or no .git directory found
  #[inline]
  pub fn from_git_root() -> Result< Self >
  {
    let mut current = env::current_dir()
      .map_err( | e | WorkspaceError::IoError( e.to_string() ) )?;

    loop
    {
      if current.join( ".git" ).exists()
      {
        return Ok( Self { root : current } );
      }

      match current.parent()
      {
        Some( parent ) => current = parent.to_path_buf(),
        None => return Err( WorkspaceError::PathNotFound( current ) ),
      }
    }
  }

  /// create workspace from current working directory (infallible)
  ///
  /// this method will not fail - it uses current directory or root as fallback
  #[must_use]
  #[inline]
  pub fn from_cwd() -> Self
  {
    let root = env::current_dir().unwrap_or_else( |_| PathBuf::from( "/" ) );
    Self { root }
  }

  /// get workspace root directory
  #[must_use]
  #[inline]
  pub fn root( &self ) -> &Path
  {
    &self.root
  }

  /// join path components relative to workspace root
  ///
  /// # examples
  ///
  /// ```rust
  /// use workspace_tools::workspace;
  ///
  /// # std::env::set_var("WORKSPACE_PATH", std::env::current_dir().unwrap());
  /// let ws = workspace()?;
  /// let config_file = ws.join("config/app.toml");
  /// # Ok::<(), workspace_tools::WorkspaceError>(())
  /// ```
  #[inline]
  pub fn join< P : AsRef< Path > >( &self, path : P ) -> PathBuf
  {
    self.root.join( path )
  }

  /// get standard configuration directory
  ///
  /// returns `workspace_root/config`
  #[must_use]
  #[inline]
  pub fn config_dir( &self ) -> PathBuf
  {
    self.root.join( "config" )
  }

  /// get standard data directory
  ///
  /// returns `workspace_root/data`
  #[must_use]
  #[inline]
  pub fn data_dir( &self ) -> PathBuf
  {
    self.root.join( "data" )
  }

  /// get standard logs directory
  ///
  /// returns `workspace_root/logs`
  #[must_use]
  #[inline]
  pub fn logs_dir( &self ) -> PathBuf
  {
    self.root.join( "logs" )
  }

  /// get standard documentation directory
  ///
  /// returns `workspace_root/docs`
  #[must_use]
  #[inline]
  pub fn docs_dir( &self ) -> PathBuf
  {
    self.root.join( "docs" )
  }

  /// get standard tests directory
  ///
  /// returns `workspace_root/tests`
  #[must_use]
  #[inline]
  pub fn tests_dir( &self ) -> PathBuf
  {
    self.root.join( "tests" )
  }

  /// get workspace metadata directory
  ///
  /// returns `workspace_root/.workspace`
  #[must_use]
  #[inline]
  pub fn workspace_dir( &self ) -> PathBuf
  {
    self.root.join( ".workspace" )
  }

  /// get path to workspace cargo.toml
  ///
  /// returns `workspace_root/Cargo.toml`
  #[must_use]
  #[inline]
  pub fn cargo_toml( &self ) -> PathBuf
  {
    self.root.join( "Cargo.toml" )
  }

  /// get path to workspace readme
  ///
  /// returns `workspace_root/readme.md`
  #[must_use]
  #[inline]
  pub fn readme( &self ) -> PathBuf
  {
    self.root.join( "readme.md" )
  }

  /// validate workspace structure
  ///
  /// checks that workspace root exists and is accessible
  ///
  /// # Errors
  ///
  /// returns error if workspace root is not accessible or is not a directory
  #[inline]
  pub fn validate( &self ) -> Result< () >
  {
    if !self.root.exists()
    {
      return Err( WorkspaceError::PathNotFound( self.root.clone() ) );
    }

    if !self.root.is_dir()
    {
      return Err( WorkspaceError::ConfigurationError(
        format!( "workspace root is not a directory: {}", self.root.display() )
      ) );
    }

    Ok( () )
  }

  /// check if a path is within workspace boundaries
  ///
  /// # examples
  ///
  /// ```rust
  /// use workspace_tools::workspace;
  ///
  /// # std::env::set_var("WORKSPACE_PATH", std::env::current_dir().unwrap());
  /// let ws = workspace()?;
  /// let config_path = ws.join("config/app.toml");
  ///
  /// assert!(ws.is_workspace_file(&config_path));
  /// assert!(!ws.is_workspace_file("/etc/passwd"));
  /// # Ok::<(), workspace_tools::WorkspaceError>(())
  /// ```
  #[inline]
  pub fn is_workspace_file< P : AsRef< Path > >( &self, path : P ) -> bool
  {
    path.as_ref().starts_with( &self.root )
  }

  /// normalize path for cross-platform compatibility
  ///
  /// resolves symbolic links and canonicalizes the path
  ///
  /// # Errors
  ///
  /// returns error if path cannot be canonicalized or does not exist
  #[inline]
  pub fn normalize_path< P : AsRef< Path > >( &self, path : P ) -> Result< PathBuf >
  {
    let path = self.join( path );
    path.canonicalize()
      .map_err( | e | WorkspaceError::IoError( format!( "failed to normalize path {}: {}", path.display(), e ) ) )
  }

  /// get environment variable as path
  fn get_env_path( key : &str ) -> Result< PathBuf >
  {
    let value = env::var( key )
      .map_err( |_| WorkspaceError::EnvironmentVariableMissing( key.to_string() ) )?;
    Ok( PathBuf::from( value ) )
  }
}

// conditional compilation for optional features

#[ cfg( feature = "glob" ) ]
impl Workspace
{
  /// find files matching a glob pattern within the workspace
  ///
  /// # Errors
  ///
  /// returns error if the glob pattern is invalid or if there are errors reading the filesystem
  ///
  /// # examples
  ///
  /// ```rust
  /// use workspace_tools::workspace;
  ///
  /// # std::env::set_var("WORKSPACE_PATH", std::env::current_dir().unwrap());
  /// let ws = workspace()?;
  ///
  /// // find all rust source files
  /// let rust_files = ws.find_resources("src/**/*.rs")?;
  ///
  /// // find all configuration files
  /// let configs = ws.find_resources("config/**/*.toml")?;
  /// # Ok::<(), workspace_tools::WorkspaceError>(())
  /// ```
  pub fn find_resources( &self, pattern : &str ) -> Result< Vec< PathBuf > >
  {
    let full_pattern = self.join( pattern );
    let pattern_str = full_pattern.to_string_lossy();

    let mut results = Vec::new();

    for entry in glob( &pattern_str )
      .map_err( | e | WorkspaceError::GlobError( e.to_string() ) )?
    {
      match entry
      {
        Ok( path ) => results.push( path ),
        Err( e ) => return Err( WorkspaceError::GlobError( e.to_string() ) ),
      }
    }

    Ok( results )
  }

  /// find configuration file by name
  ///
  /// searches for configuration files in standard locations:
  /// - config/{name}.toml
  /// - config/{name}.yaml
  /// - config/{name}.json
  /// - .{name}.toml (dotfile in workspace root)
  ///
  /// # Errors
  ///
  /// returns error if no configuration file with the given name is found
  ///
  /// # examples
  ///
  /// ```rust
  /// use workspace_tools::workspace;
  ///
  /// # std::env::set_var("WORKSPACE_PATH", std::env::current_dir().unwrap());
  /// let ws = workspace()?;
  ///
  /// // looks for config/database.toml, config/database.yaml, etc.
  /// if let Ok(config_path) = ws.find_config("database") {
  ///     println!("found config at: {}", config_path.display());
  /// }
  /// # Ok::<(), workspace_tools::WorkspaceError>(())
  /// ```
  pub fn find_config( &self, name : &str ) -> Result< PathBuf >
  {
    let candidates = vec!
    [
      self.config_dir().join( format!( "{name}.toml" ) ),
      self.config_dir().join( format!( "{name}.yaml" ) ),
      self.config_dir().join( format!( "{name}.yml" ) ),
      self.config_dir().join( format!( "{name}.json" ) ),
      self.root.join( format!( ".{name}.toml" ) ),
      self.root.join( format!( ".{name}.yaml" ) ),
      self.root.join( format!( ".{name}.yml" ) ),
    ];

    for candidate in candidates
    {
      if candidate.exists()
      {
        return Ok( candidate );
      }
    }

    Err( WorkspaceError::PathNotFound(
      self.config_dir().join( format!( "{name}.toml" ) )
    ) )
  }
}

#[ cfg( feature = "secret_management" ) ]
impl Workspace
{
  /// get secrets directory path
  ///
  /// returns `workspace_root/.secret`
  #[ must_use ]
  pub fn secret_dir( &self ) -> PathBuf
  {
    self.root.join( ".secret" )
  }

  /// get path to secret configuration file
  ///
  /// returns `workspace_root/.secret/{name}`
  #[ must_use ]
  pub fn secret_file( &self, name : &str ) -> PathBuf
  {
    self.secret_dir().join( name )
  }

  /// load secrets from a key-value file
  ///
  /// supports shell script format (KEY=value lines)
  ///
  /// # Errors
  ///
  /// returns error if the file cannot be read or contains invalid format
  ///
  /// # examples
  ///
  /// ```rust
  /// use workspace_tools::workspace;
  ///
  /// # std::env::set_var("WORKSPACE_PATH", std::env::current_dir().unwrap());
  /// let ws = workspace()?;
  ///
  /// // load from .secret/-secrets.sh
  /// match ws.load_secrets_from_file("-secrets.sh") {
  ///     Ok(secrets) => {
  ///         if let Some(api_key) = secrets.get("API_KEY") {
  ///             println!("loaded api key");
  ///         }
  ///     }
  ///     Err(_) => println!("no secrets file found"),
  /// }
  /// # Ok::<(), workspace_tools::WorkspaceError>(())
  /// ```
  pub fn load_secrets_from_file( &self, filename : &str ) -> Result< HashMap< String, String > >
  {
    let secret_file = self.secret_file( filename );

    if !secret_file.exists()
    {
      return Ok( HashMap::new() );
    }

    let content = fs::read_to_string( &secret_file )
      .map_err( | e | WorkspaceError::IoError( format!( "failed to read {}: {}", secret_file.display(), e ) ) )?;

    Ok( Self::parse_key_value_file( &content ) )
  }

  /// load a specific secret key with fallback to environment
  ///
  /// tries to load from secret file first, then falls back to environment variable
  ///
  /// # Errors
  ///
  /// returns error if the key is not found in either the secret file or environment variables
  ///
  /// # examples
  ///
  /// ```rust
  /// use workspace_tools::workspace;
  ///
  /// # std::env::set_var("WORKSPACE_PATH", std::env::current_dir().unwrap());
  /// let ws = workspace()?;
  ///
  /// // looks for API_KEY in .secret/-secrets.sh, then in environment
  /// match ws.load_secret_key("API_KEY", "-secrets.sh") {
  ///     Ok(key) => println!("loaded api key"),
  ///     Err(_) => println!("api key not found"),
  /// }
  /// # Ok::<(), workspace_tools::WorkspaceError>(())
  /// ```
  pub fn load_secret_key( &self, key_name : &str, filename : &str ) -> Result< String >
  {
    // try loading from secret file first
    if let Ok( secrets ) = self.load_secrets_from_file( filename )
    {
      if let Some( value ) = secrets.get( key_name )
      {
        return Ok( value.clone() );
      }
    }

    // fallback to environment variable
    env::var( key_name )
      .map_err( |_| WorkspaceError::ConfigurationError(
        format!(
          "{} not found. please add it to {} or set environment variable",
          key_name,
          self.secret_file( filename ).display()
        )
      ))
  }

  /// parse key-value file content
  ///
  /// supports shell script format with comments and quotes
  fn parse_key_value_file( content : &str ) -> HashMap< String, String >
  {
    let mut secrets = HashMap::new();

    for line in content.lines()
    {
      let line = line.trim();

      // skip empty lines and comments
      if line.is_empty() || line.starts_with( '#' )
      {
        continue;
      }

      // parse KEY=VALUE format
      if let Some( ( key, value ) ) = line.split_once( '=' )
      {
        let key = key.trim();
        let value = value.trim();

        // remove quotes if present
        let value = if ( value.starts_with( '"' ) && value.ends_with( '"' ) ) ||
                       ( value.starts_with( '\'' ) && value.ends_with( '\'' ) )
        {
          &value[ 1..value.len() - 1 ]
        }
        else
        {
          value
        };

        secrets.insert( key.to_string(), value.to_string() );
      }
    }

    secrets
  }
}

/// testing utilities for workspace functionality
#[ cfg( feature = "enabled" ) ]
pub mod testing
{
  use super::*;
  use tempfile::TempDir;

  /// create a temporary workspace for testing
  ///
  /// returns a tuple of (`temp_dir`, workspace) where `temp_dir` must be kept alive
  /// for the duration of the test to prevent the directory from being deleted
  ///
  /// # Panics
  ///
  /// panics if temporary directory creation fails or workspace resolution fails
  ///
  /// # examples
  ///
  /// ```rust
  /// #[cfg(test)]
  /// mod tests {
  ///     use workspace_tools::testing::create_test_workspace;
  ///
  ///     #[test]
  ///     fn test_my_feature() {
  ///         let (_temp_dir, workspace) = create_test_workspace();
  ///
  ///         // test with isolated workspace
  ///         let config = workspace.config_dir().join("test.toml");
  ///         assert!(config.starts_with(workspace.root()));
  ///     }
  /// }
  /// ```
  #[ must_use ]
  #[ inline ]
  pub fn create_test_workspace() -> ( TempDir, Workspace )
  {
    let temp_dir = TempDir::new().unwrap_or_else( | e | panic!( "failed to create temp directory: {e}" ) );
    std::env::set_var( "WORKSPACE_PATH", temp_dir.path() );
    let workspace = Workspace::resolve().unwrap_or_else( | e | panic!( "failed to resolve test workspace: {e}" ) );
    ( temp_dir, workspace )
  }

  /// create test workspace with standard directory structure
  ///
  /// creates a temporary workspace with config/, data/, logs/, docs/, tests/ directories
  ///
  /// # Panics
  ///
  /// panics if temporary directory creation fails or if any standard directory creation fails
  #[ must_use ]
  #[ inline ]
  pub fn create_test_workspace_with_structure() -> ( TempDir, Workspace )
  {
    let ( temp_dir, workspace ) = create_test_workspace();

    // create standard directories
    let base_dirs = vec!
    [
      workspace.config_dir(),
      workspace.data_dir(),
      workspace.logs_dir(),
      workspace.docs_dir(),
      workspace.tests_dir(),
      workspace.workspace_dir(),
    ];

    #[ cfg( feature = "secret_management" ) ]
    let all_dirs = {
      let mut dirs = base_dirs;
      dirs.push( workspace.secret_dir() );
      dirs
    };

    #[ cfg( not( feature = "secret_management" ) ) ]
    let all_dirs = base_dirs;

    for dir in all_dirs
    {
      std::fs::create_dir_all( &dir )
        .unwrap_or_else( | e | panic!( "failed to create directory {}: {}", dir.display(), e ) );
    }

    ( temp_dir, workspace )
  }
}

/// convenience function to get workspace instance
///
/// equivalent to `Workspace::resolve()`
///
/// # Errors
///
/// returns error if workspace resolution fails
///
/// # examples
///
/// ```rust
/// use workspace_tools::workspace;
///
/// # std::env::set_var("WORKSPACE_PATH", std::env::current_dir().unwrap());
/// let ws = workspace()?;
/// let config_dir = ws.config_dir();
/// # Ok::<(), workspace_tools::WorkspaceError>(())
/// ```
#[ inline ]
pub fn workspace() -> Result< Workspace >
{
  Workspace::resolve()
}