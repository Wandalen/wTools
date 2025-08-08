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

#[ cfg( feature = "cargo_integration" ) ]
use std::collections::HashMap;

#[ cfg( feature = "glob" ) ]
use glob::glob;

#[ cfg( feature = "secret_management" ) ]
use std::fs;

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
  /// cargo metadata error
  #[ cfg( feature = "cargo_integration" ) ]
  CargoError( String ),
  /// toml parsing error
  #[ cfg( feature = "cargo_integration" ) ]
  TomlError( String ),
  /// serde deserialization error
  #[ cfg( feature = "serde_integration" ) ]
  SerdeError( String ),
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
      #[ cfg( feature = "cargo_integration" ) ]
      WorkspaceError::CargoError( msg ) =>
        write!( f, "cargo metadata error: {msg}" ),
      #[ cfg( feature = "cargo_integration" ) ]
      WorkspaceError::TomlError( msg ) =>
        write!( f, "toml parsing error: {msg}" ),
      #[ cfg( feature = "serde_integration" ) ]
      WorkspaceError::SerdeError( msg ) =>
        write!( f, "serde error: {msg}" ),
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
  /// 1. cargo workspace detection (if `cargo_integration` feature enabled)
  /// 2. environment variable (`WORKSPACE_PATH`)
  /// 3. current working directory
  /// 4. git repository root (if .git directory found)
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
    #[ cfg( feature = "cargo_integration" ) ]
    {
      Self::from_cargo_workspace()
        .or_else( |_| Self::resolve() )
        .or_else( |_| Self::from_current_dir() )
        .or_else( |_| Self::from_git_root() )
        .unwrap_or_else( |_| Self::from_cwd() )
    }
    
    #[ cfg( not( feature = "cargo_integration" ) ) ]
    {
      Self::resolve()
        .or_else( |_| Self::from_current_dir() )
        .or_else( |_| Self::from_git_root() )
        .unwrap_or_else( |_| Self::from_cwd() )
    }
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

// cargo integration types and implementations
#[ cfg( feature = "cargo_integration" ) ]
/// cargo metadata information for workspace
#[ derive( Debug, Clone ) ]
pub struct CargoMetadata
{
  /// root directory of the cargo workspace
  pub workspace_root : PathBuf,
  /// list of workspace member packages
  pub members : Vec< CargoPackage >,
  /// workspace-level dependencies
  pub workspace_dependencies : HashMap< String, String >,
}

#[ cfg( feature = "cargo_integration" ) ]
/// information about a cargo package within a workspace
#[ derive( Debug, Clone ) ]
pub struct CargoPackage
{
  /// package name
  pub name : String,
  /// package version
  pub version : String,
  /// path to the package's Cargo.toml
  pub manifest_path : PathBuf,
  /// root directory of the package
  pub package_root : PathBuf,
}

// serde integration types
#[ cfg( feature = "serde_integration" ) ]
/// trait for configuration types that can be merged
pub trait ConfigMerge : Sized
{
  /// merge this configuration with another, returning the merged result
  #[must_use]
  fn merge( self, other : Self ) -> Self;
}

#[ cfg( feature = "serde_integration" ) ]
/// workspace-aware serde deserializer
#[ derive( Debug ) ]
pub struct WorkspaceDeserializer< 'ws >
{
  /// reference to workspace for path resolution
  pub workspace : &'ws Workspace,
}

#[ cfg( feature = "serde_integration" ) ]
/// custom serde field for workspace-relative paths
#[ derive( Debug, Clone, PartialEq ) ]
pub struct WorkspacePath( pub PathBuf );

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

#[ cfg( feature = "cargo_integration" ) ]
impl Workspace
{
  /// create workspace from cargo workspace root (auto-detected)
  ///
  /// traverses up directory tree looking for `Cargo.toml` with `[workspace]` section
  /// or workspace member that references a workspace root
  ///
  /// # Errors
  ///
  /// returns error if no cargo workspace is found or if cargo.toml cannot be parsed
  ///
  /// # examples
  ///
  /// ```rust
  /// use workspace_tools::Workspace;
  ///
  /// let workspace = Workspace::from_cargo_workspace()?;
  /// println!("cargo workspace root: {}", workspace.root().display());
  /// # Ok::<(), workspace_tools::WorkspaceError>(())
  /// ```
  pub fn from_cargo_workspace() -> Result< Self >
  {
    let workspace_root = Self::find_cargo_workspace()?;
    Ok( Self { root : workspace_root } )
  }

  /// create workspace from specific cargo.toml path
  ///
  /// # Errors
  ///
  /// returns error if the manifest path does not exist or cannot be parsed
  pub fn from_cargo_manifest< P : AsRef< Path > >( manifest_path : P ) -> Result< Self >
  {
    let manifest_path = manifest_path.as_ref();
    
    if !manifest_path.exists()
    {
      return Err( WorkspaceError::PathNotFound( manifest_path.to_path_buf() ) );
    }

    let workspace_root = if manifest_path.file_name() == Some( std::ffi::OsStr::new( "Cargo.toml" ) )
    {
      manifest_path.parent()
        .ok_or_else( || WorkspaceError::ConfigurationError( "invalid manifest path".to_string() ) )?
        .to_path_buf()
    }
    else
    {
      manifest_path.to_path_buf()
    };

    Ok( Self { root : workspace_root } )
  }

  /// get cargo metadata for this workspace
  ///
  /// # Errors
  ///
  /// returns error if cargo metadata command fails or workspace is not a cargo workspace
  pub fn cargo_metadata( &self ) -> Result< CargoMetadata >
  {
    let cargo_toml = self.cargo_toml();
    
    if !cargo_toml.exists()
    {
      return Err( WorkspaceError::CargoError( "not a cargo workspace".to_string() ) );
    }

    // use cargo_metadata crate for robust metadata extraction
    let metadata = cargo_metadata::MetadataCommand::new()
      .manifest_path( &cargo_toml )
      .exec()
      .map_err( | e | WorkspaceError::CargoError( e.to_string() ) )?;

    let mut members = Vec::new();
    let mut workspace_dependencies = HashMap::new();

    // extract workspace member information
    for package in metadata.workspace_packages()
    {
      members.push( CargoPackage {
        name : package.name.clone(),
        version : package.version.to_string(),
        manifest_path : package.manifest_path.clone().into(),
        package_root : package.manifest_path
          .parent()
          .unwrap_or( &package.manifest_path )
          .into(),
      } );
    }

    // extract workspace dependencies if available
    if let Some( deps ) = metadata.workspace_metadata.get( "dependencies" )
    {
      if let Some( deps_map ) = deps.as_object()
      {
        for ( name, version ) in deps_map
        {
          if let Some( version_str ) = version.as_str()
          {
            workspace_dependencies.insert( name.clone(), version_str.to_string() );
          }
        }
      }
    }

    Ok( CargoMetadata {
      workspace_root : metadata.workspace_root.into(),
      members,
      workspace_dependencies,
    } )
  }

  /// check if this workspace is a cargo workspace
  #[must_use]
  pub fn is_cargo_workspace( &self ) -> bool
  {
    let cargo_toml = self.cargo_toml();
    
    if !cargo_toml.exists()
    {
      return false;
    }

    // check if Cargo.toml contains workspace section
    if let Ok( content ) = std::fs::read_to_string( &cargo_toml )
    {
      if let Ok( parsed ) = toml::from_str::< toml::Value >( &content )
      {
        return parsed.get( "workspace" ).is_some();
      }
    }

    false
  }

  /// get workspace members (if cargo workspace)
  ///
  /// # Errors
  ///
  /// returns error if not a cargo workspace or cargo metadata fails
  pub fn workspace_members( &self ) -> Result< Vec< PathBuf > >
  {
    let metadata = self.cargo_metadata()?;
    Ok( metadata.members.into_iter().map( | pkg | pkg.package_root ).collect() )
  }

  /// find cargo workspace root by traversing up directory tree
  fn find_cargo_workspace() -> Result< PathBuf >
  {
    let mut current = std::env::current_dir()
      .map_err( | e | WorkspaceError::IoError( e.to_string() ) )?;

    loop
    {
      let manifest = current.join( "Cargo.toml" );
      if manifest.exists()
      {
        let content = std::fs::read_to_string( &manifest )
          .map_err( | e | WorkspaceError::IoError( e.to_string() ) )?;
        
        let parsed : toml::Value = toml::from_str( &content )
          .map_err( | e | WorkspaceError::TomlError( e.to_string() ) )?;

        // check if this is a workspace root
        if parsed.get( "workspace" ).is_some()
        {
          return Ok( current );
        }

        // check if this is a workspace member pointing to a parent workspace
        if let Some( package ) = parsed.get( "package" )
        {
          if package.get( "workspace" ).is_some()
          {
            // continue searching upward for the actual workspace root
          }
        }
      }

      match current.parent()
      {
        Some( parent ) => current = parent.to_path_buf(),
        None => return Err( WorkspaceError::PathNotFound( current ) ),
      }
    }
  }
}

#[ cfg( feature = "serde_integration" ) ]
impl Workspace
{
  /// load configuration with automatic format detection
  ///
  /// # Errors
  ///
  /// returns error if configuration file is not found or cannot be deserialized
  ///
  /// # examples
  ///
  /// ```rust,no_run
  /// use workspace_tools::workspace;
  /// use serde::Deserialize;
  ///
  /// #[derive(Deserialize)]
  /// struct AppConfig {
  ///     name: String,
  ///     port: u16,
  /// }
  ///
  /// let ws = workspace()?;
  /// // looks for config/app.toml, config/app.yaml, config/app.json
  /// let config: AppConfig = ws.load_config("app")?;
  /// # Ok::<(), workspace_tools::WorkspaceError>(())
  /// ```
  pub fn load_config< T >( &self, name : &str ) -> Result< T >
  where
    T : serde::de::DeserializeOwned,
  {
    let config_path = self.find_config( name )?;
    self.load_config_from( config_path )
  }

  /// load configuration from specific file
  ///
  /// # Errors
  ///
  /// returns error if file cannot be read or deserialized
  pub fn load_config_from< T, P >( &self, path : P ) -> Result< T >
  where
    T : serde::de::DeserializeOwned,
    P : AsRef< Path >,
  {
    let path = path.as_ref();
    let content = std::fs::read_to_string( path )
      .map_err( | e | WorkspaceError::IoError( format!( "failed to read {}: {}", path.display(), e ) ) )?;

    let extension = path.extension()
      .and_then( | ext | ext.to_str() )
      .unwrap_or( "toml" );

    match extension
    {
      "toml" => toml::from_str( &content )
        .map_err( | e | WorkspaceError::SerdeError( format!( "toml deserialization error: {e}" ) ) ),
      "json" => serde_json::from_str( &content )
        .map_err( | e | WorkspaceError::SerdeError( format!( "json deserialization error: {e}" ) ) ),
      "yaml" | "yml" => serde_yaml::from_str( &content )
        .map_err( | e | WorkspaceError::SerdeError( format!( "yaml deserialization error: {e}" ) ) ),
      _ => Err( WorkspaceError::ConfigurationError( format!( "unsupported config format: {extension}" ) ) ),
    }
  }

  /// save configuration with format matching the original
  ///
  /// # Errors
  ///
  /// returns error if configuration cannot be serialized or written to file
  pub fn save_config< T >( &self, name : &str, config : &T ) -> Result< () >
  where
    T : serde::Serialize,
  {
    let config_path = self.find_config( name )
      .or_else( |_| Ok( self.config_dir().join( format!( "{name}.toml" ) ) ) )?;
    
    self.save_config_to( config_path, config )
  }

  /// save configuration to specific file with format detection
  ///
  /// # Errors
  ///
  /// returns error if configuration cannot be serialized or written to file
  pub fn save_config_to< T, P >( &self, path : P, config : &T ) -> Result< () >
  where
    T : serde::Serialize,
    P : AsRef< Path >,
  {
    let path = path.as_ref();
    let extension = path.extension()
      .and_then( | ext | ext.to_str() )
      .unwrap_or( "toml" );

    let content = match extension
    {
      "toml" => toml::to_string_pretty( config )
        .map_err( | e | WorkspaceError::SerdeError( format!( "toml serialization error: {e}" ) ) )?,
      "json" => serde_json::to_string_pretty( config )
        .map_err( | e | WorkspaceError::SerdeError( format!( "json serialization error: {e}" ) ) )?,
      "yaml" | "yml" => serde_yaml::to_string( config )
        .map_err( | e | WorkspaceError::SerdeError( format!( "yaml serialization error: {e}" ) ) )?,
      _ => return Err( WorkspaceError::ConfigurationError( format!( "unsupported config format: {extension}" ) ) ),
    };

    // ensure parent directory exists
    if let Some( parent ) = path.parent()
    {
      std::fs::create_dir_all( parent )
        .map_err( | e | WorkspaceError::IoError( format!( "failed to create directory {}: {}", parent.display(), e ) ) )?;
    }

    // atomic write using temporary file
    let temp_path = path.with_extension( format!( "{extension}.tmp" ) );
    std::fs::write( &temp_path, content )
      .map_err( | e | WorkspaceError::IoError( format!( "failed to write temporary file {}: {}", temp_path.display(), e ) ) )?;
    
    std::fs::rename( &temp_path, path )
      .map_err( | e | WorkspaceError::IoError( format!( "failed to rename {} to {}: {}", temp_path.display(), path.display(), e ) ) )?;

    Ok( () )
  }

  /// load and merge multiple configuration layers
  ///
  /// # Errors
  ///
  /// returns error if any configuration file cannot be loaded or merged
  pub fn load_config_layered< T >( &self, names : &[ &str ] ) -> Result< T >
  where
    T : serde::de::DeserializeOwned + ConfigMerge,
  {
    let mut result : Option< T > = None;

    for name in names
    {
      if let Ok( config ) = self.load_config::< T >( name )
      {
        result = Some( match result
        {
          Some( existing ) => existing.merge( config ),
          None => config,
        } );
      }
    }

    result.ok_or_else( || WorkspaceError::ConfigurationError( "no configuration files found".to_string() ) )
  }

  /// update configuration partially
  ///
  /// # Errors
  ///
  /// returns error if configuration cannot be loaded, updated, or saved
  pub fn update_config< T, U >( &self, name : &str, updates : U ) -> Result< T >
  where
    T : serde::de::DeserializeOwned + serde::Serialize,
    U : serde::Serialize,
  {
    // load existing configuration
    let existing : T = self.load_config( name )?;
    
    // serialize both to json for merging
    let existing_json = serde_json::to_value( &existing )
      .map_err( | e | WorkspaceError::SerdeError( format!( "failed to serialize existing config: {e}" ) ) )?;
    
    let updates_json = serde_json::to_value( updates )
      .map_err( | e | WorkspaceError::SerdeError( format!( "failed to serialize updates: {e}" ) ) )?;

    // merge json objects
    let merged = Self::merge_json_objects( existing_json, updates_json )?;
    
    // deserialize back to target type
    let merged_config : T = serde_json::from_value( merged )
      .map_err( | e | WorkspaceError::SerdeError( format!( "failed to deserialize merged config: {e}" ) ) )?;
    
    // save updated configuration
    self.save_config( name, &merged_config )?;
    
    Ok( merged_config )
  }

  /// merge two json objects recursively
  fn merge_json_objects( mut base : serde_json::Value, updates : serde_json::Value ) -> Result< serde_json::Value >
  {
    match ( &mut base, updates )
    {
      ( serde_json::Value::Object( ref mut base_map ), serde_json::Value::Object( updates_map ) ) =>
      {
        for ( key, value ) in updates_map
        {
          match base_map.get_mut( &key )
          {
            Some( existing ) if existing.is_object() && value.is_object() =>
            {
              *existing = Self::merge_json_objects( existing.clone(), value )?;
            }
            _ =>
            {
              base_map.insert( key, value );
            }
          }
        }
      }
      ( _, updates_value ) =>
      {
        base = updates_value;
      }
    }
    
    Ok( base )
  }
}

#[ cfg( feature = "serde_integration" ) ]
impl serde::Serialize for WorkspacePath
{
  fn serialize< S >( &self, serializer : S ) -> core::result::Result< S::Ok, S::Error >
  where
    S : serde::Serializer,
  {
    self.0.serialize( serializer )
  }
}

#[ cfg( feature = "serde_integration" ) ]
impl< 'de > serde::Deserialize< 'de > for WorkspacePath
{
  fn deserialize< D >( deserializer : D ) -> core::result::Result< Self, D::Error >
  where
    D : serde::Deserializer< 'de >,
  {
    let path = PathBuf::deserialize( deserializer )?;
    Ok( WorkspacePath( path ) )
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