//! Universal workspace-relative path resolution for Rust projects
//!
//! This crate provides consistent, reliable path management regardless of execution context
//! or working directory. It solves common path resolution issues in software projects by
//! leveraging cargo's environment variable injection system.
//!
//! ## problem solved
//!
//! - **execution context dependency** : paths break when code runs from different directories
//! - **environment inconsistency** : different developers have different working directory habits
//! - **testing fragility** : tests fail when run from different locations
//! - **ci/cd brittleness** : automated systems may execute from unexpected directories
//!
//! ## quick start
//!
//! 1. Configure cargo in workspace root `.cargo/config.toml` :
//! ```toml
//! [env]
//! WORKSPACE_PATH = { value = ".", relative = true }
//! ```
//!
//! 2. Use in your code :
//! ```rust
//! use workspace_tools::{ workspace, WorkspaceError };
//!
//! # fn main() -> Result< (), WorkspaceError >
//! # {
//! // get workspace instance
//! let ws = workspace()?;
//!
//! // resolve workspace-relative paths
//! let config_path = ws.config_dir().join( "app.toml" );
//! let data_path = ws.data_dir().join( "cache.db" );
//! # Ok( () )
//! # }
//! ```
//!
//! ## features
//!
//! - **`glob`** : enables pattern-based resource discovery
//! - **`secrets`** : provides secure configuration file handling utilities
//! - **`secure`** : enables memory-safe secret handling with the secrecy crate
//! - **`serde`** : provides configuration loading with serde support
//! - **`validation`** : enables configuration validation with JSON Schema
//!
//! ## security best practices
//!
//! when using the `secure` feature for secret management :
//!
//! - secrets are wrapped in `SecretString` types that prevent accidental exposure
//! - debug output automatically redacts secret values
//! - secrets require explicit `expose_secret()` calls for access
//! - use the `SecretInjectable` trait for automatic configuration injection
//! - validate secret strength with `validate_secret()` method
//! - secrets are zeroized from memory when dropped

#![ warn( missing_docs ) ]

use std ::
{
  env,
  path :: { Path, PathBuf },
};

use std ::collections ::HashMap;

#[ cfg( feature = "glob" ) ]
use glob ::glob;

#[ cfg( feature = "secrets" ) ]
use std ::fs;

#[ cfg( feature = "validation" ) ]
use jsonschema ::Validator;

#[ cfg( feature = "validation" ) ]
use schemars ::JsonSchema;

#[ cfg( feature = "secure" ) ]
use secrecy :: { SecretString, ExposeSecret };


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
  CargoError( String ),
  /// toml parsing error
  TomlError( String ),
  /// serde deserialization error
  #[ cfg( feature = "serde" ) ]
  SerdeError( String ),
  /// config validation error
  #[ cfg( feature = "validation" ) ]
  ValidationError( String ),
  /// secret validation error
  #[ cfg( feature = "secure" ) ]
  SecretValidationError( String ),
  /// secret injection error
  #[ cfg( feature = "secure" ) ]
  SecretInjectionError( String ),
}

impl core::fmt::Display for WorkspaceError
{
  #[ inline ]
  #[ allow( clippy::elidable_lifetime_names ) ]
  fn fmt< 'a >( &self, f: &mut core::fmt::Formatter< 'a > ) -> core::fmt::Result
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
  WorkspaceError::CargoError( msg ) =>
  write!( f, "cargo metadata error: {msg}" ),
  WorkspaceError::TomlError( msg ) =>
  write!( f, "toml parsing error: {msg}" ),
   #[ cfg( feature = "serde" ) ]
   WorkspaceError::SerdeError( msg ) =>
  write!( f, "serde error: {msg}" ),
   #[ cfg( feature = "validation" ) ]
   WorkspaceError::ValidationError( msg ) =>
  write!( f, "config validation error: {msg}" ),
   #[ cfg( feature = "secure" ) ]
   WorkspaceError::SecretValidationError( msg ) =>
  write!( f, "secret validation error: {msg}" ),
   #[ cfg( feature = "secure" ) ]
   WorkspaceError::SecretInjectionError( msg ) =>
  write!( f, "secret injection error: {msg}" ),
  }
  }
}

impl core ::error ::Error for WorkspaceError {}

/// result type for workspace operations
pub type Result< T > = core ::result ::Result< T, WorkspaceError >;

/// trait for types that support automatic secret injection
///
/// configuration types can implement this trait to enable automatic
/// secret injection from workspace secret files
#[ cfg( feature = "secure" ) ]
pub trait SecretInjectable
{
  /// inject a secret value for the given key
  ///
  /// # Errors
  ///
  /// returns error if the key is not recognized or injection fails
  fn inject_secret( &mut self, key: &str, value: String ) -> Result< () >;

  /// validate all injected secrets meet security requirements
  ///
  /// # Errors
  ///
  /// returns error if any secret fails validation
  fn validate_secrets( &self ) -> Result< () >;
}

/// workspace path resolver providing centralized access to workspace-relative paths
///
/// the workspace struct encapsulates workspace root detection and provides methods
/// for resolving standard directory paths and joining workspace-relative paths safely.
#[ derive( Debug, Clone ) ]
pub struct Workspace
{
  root: PathBuf,
}

impl Workspace
{
  /// create workspace from a given root path
  ///
  /// # Arguments
  ///
  /// * `root` - the root directory path for the workspace
  ///
  /// # Examples
  ///
  /// ```rust
  /// use workspace_tools ::Workspace;
  /// use std ::path ::PathBuf;
  ///
  /// let workspace = Workspace ::new( PathBuf ::from( "/path/to/workspace" ) );
  /// ```
  #[ must_use ]
  #[ inline ]
  pub fn new< P: Into< PathBuf > >( root: P ) -> Self
  {
  Self { root: root.into() }
  }

  /// resolve workspace from environment variables
  ///
  /// reads the `WORKSPACE_PATH` environment variable set by cargo configuration
  /// and validates that the workspace root exists.
  ///
  /// # errors
  ///
  /// returns error if :
  /// - `WORKSPACE_PATH` environment variable is not set
  /// - the path specified by `WORKSPACE_PATH` does not exist
  ///
  /// # examples
  ///
  /// ```rust
  /// # fn main() -> Result< (), workspace_tools ::WorkspaceError > {
  /// use workspace_tools ::Workspace;
  ///
  /// # std ::env ::set_var( "WORKSPACE_PATH", std ::env ::current_dir().unwrap() );
  /// let workspace = Workspace ::resolve()?;
  /// println!( "workspace root: {}", workspace.root().display() );
  /// # Ok(())
  /// # }
  /// ```
  /// 
  /// # Errors
  /// 
  /// Returns an error if the workspace path environment variable is not set or the path doesn't exist.
  #[ inline ]
  pub fn resolve() -> Result< Self >
  {
  let root = Self ::get_env_path( "WORKSPACE_PATH" )?;

  if !root.exists()
  {
   return Err( WorkspaceError::PathNotFound( root ) );
  }

  Ok( Self { root } )
  }

  /// resolve workspace with fallback strategies
  ///
  /// tries multiple strategies to resolve workspace root :
  /// 1. cargo workspace detection (if `cargo_integration` feature enabled)
  /// 2. environment variable (`WORKSPACE_PATH`)
  /// 3. current working directory
  /// 4. git repository root (if .git directory found)
  ///
  /// # examples
  ///
  /// ```rust
  /// use workspace_tools ::Workspace;
  ///
  /// // this will always succeed with some workspace root
  /// let workspace = Workspace ::resolve_or_fallback();
  /// ```
  #[ must_use ]
  #[ inline ]
  pub fn resolve_or_fallback() -> Self
  {
  {
   Self ::from_cargo_workspace()
  .or_else( |_| Self ::resolve() )
  .or_else( |_| Self ::from_current_dir() )
  .or_else( |_| Self ::from_git_root() )
  .unwrap_or_else( |_| Self ::from_cwd() )
  }
  }

  /// create workspace from current working directory
  ///
  /// # Errors
  ///
  /// returns error if current directory cannot be accessed
  #[ inline ]
  pub fn from_current_dir() -> Result< Self >
  {
  let root = env ::current_dir()
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
  #[ inline ]
  pub fn from_git_root() -> Result< Self >
  {
  let mut current = env ::current_dir()
   .map_err( | e | WorkspaceError::IoError( e.to_string() ) )?;

  loop
  {
   if current.join( ".git" ).exists()
   {
  return Ok( Self { root: current } );
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
  #[ must_use ]
  #[ inline ]
  pub fn from_cwd() -> Self
  {
  let root = env ::current_dir().unwrap_or_else( |_| PathBuf ::from( "/" ) );
  Self { root }
  }

  /// get workspace root directory
  #[ must_use ]
  #[ inline ]
  pub fn root( &self ) -> &Path
  {
  &self.root
  }

  /// join path components relative to workspace root
  ///
  /// # examples
  ///
  /// ```rust
  /// # fn main() -> Result< (), workspace_tools ::WorkspaceError > {
  /// use workspace_tools ::workspace;
  ///
  /// # std ::env ::set_var( "WORKSPACE_PATH", std ::env ::current_dir().unwrap() );
  /// let ws = workspace()?;
  /// let config_file = ws.join( "config/app.toml" );
  /// # Ok(())
  /// # }
  /// ```
  #[ inline ]
  pub fn join< P: AsRef< Path > >( &self, path: P ) -> PathBuf
  {
  self.root.join( path )
  }

  /// get standard configuration directory
  ///
  /// returns `workspace_root/config`
  #[ must_use ]
  #[ inline ]
  pub fn config_dir( &self ) -> PathBuf
  {
  self.root.join( "config" )
  }

  /// get standard data directory
  ///
  /// returns `workspace_root/data`
  #[ must_use ]
  #[ inline ]
  pub fn data_dir( &self ) -> PathBuf
  {
  self.root.join( "data" )
  }

  /// get standard logs directory
  ///
  /// returns `workspace_root/logs`
  #[ must_use ]
  #[ inline ]
  pub fn logs_dir( &self ) -> PathBuf
  {
  self.root.join( "logs" )
  }

  /// get standard documentation directory
  ///
  /// returns `workspace_root/docs`
  #[ must_use ]
  #[ inline ]
  pub fn docs_dir( &self ) -> PathBuf
  {
  self.root.join( "docs" )
  }

  /// get standard tests directory
  ///
  /// returns `workspace_root/tests`
  #[ must_use ]
  #[ inline ]
  pub fn tests_dir( &self ) -> PathBuf
  {
  self.root.join( "tests" )
  }

  /// get workspace metadata directory
  ///
  /// returns `workspace_root/.workspace`
  #[ must_use ]
  #[ inline ]
  pub fn workspace_dir( &self ) -> PathBuf
  {
  self.root.join( ".workspace" )
  }

  /// get path to workspace cargo.toml
  ///
  /// returns `workspace_root/Cargo.toml`
  #[ must_use ]
  #[ inline ]
  pub fn cargo_toml( &self ) -> PathBuf
  {
  self.root.join( "Cargo.toml" )
  }

  /// get path to workspace readme
  ///
  /// returns `workspace_root/readme.md`
  #[ must_use ]
  #[ inline ]
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
  #[ inline ]
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
  /// # fn main() -> Result< (), workspace_tools ::WorkspaceError > {
  /// use workspace_tools ::workspace;
  ///
  /// # std ::env ::set_var( "WORKSPACE_PATH", std ::env ::current_dir().unwrap() );
  /// let ws = workspace()?;
  /// let config_path = ws.join( "config/app.toml" );
  ///
  /// assert!( ws.is_workspace_file( &config_path ) );
  /// assert!( !ws.is_workspace_file( "/etc/passwd" ) );
  /// # Ok(())
  /// # }
  /// ```
  #[ inline ]
  pub fn is_workspace_file< P: AsRef< Path > >( &self, path: P ) -> bool
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
  #[ inline ]
  pub fn normalize_path< P: AsRef< Path > >( &self, path: P ) -> Result< PathBuf >
  {
  let path = self.join( path );
  path.canonicalize()
   .map_err( | e | WorkspaceError::IoError( format!( "failed to normalize path {} : {}", path.display(), e ) ) )
  }

  /// get environment variable as path
  fn get_env_path( key: &str ) -> Result< PathBuf >
  {
  let value = env ::var( key )
   .map_err( |_| WorkspaceError::EnvironmentVariableMissing( key.to_string() ) )?;
  Ok( PathBuf ::from( value ) )
  }

  /// find configuration file by name
  ///
  /// searches for configuration files in standard locations :
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
  /// # fn main() -> Result< (), workspace_tools ::WorkspaceError > {
  /// use workspace_tools ::workspace;
  ///
  /// # std ::env ::set_var( "WORKSPACE_PATH", std ::env ::current_dir().unwrap() );
  /// let ws = workspace()?;
  ///
  /// // looks for config/database.toml, config/database.yaml, etc.
  /// if let Ok( config_path ) = ws.find_config( "database" )
  /// {
  ///     println!( "found config at: {}", config_path.display() );
  /// }
  /// # Ok(())
  /// # }
  /// ```
  pub fn find_config( &self, name: &str ) -> Result< PathBuf >
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

// cargo integration types and implementations
/// cargo metadata information for workspace
#[ derive( Debug, Clone ) ]
pub struct CargoMetadata
{
  /// root directory of the cargo workspace
  pub workspace_root: PathBuf,
  /// list of workspace member packages
  pub members: Vec< CargoPackage >,
  /// workspace-level dependencies
  pub workspace_dependencies: HashMap< String, String >,
}

/// information about a cargo package within a workspace
#[ derive( Debug, Clone ) ]
pub struct CargoPackage
{
  /// package name
  pub name: String,
  /// package version
  pub version: String,
  /// path to the package's Cargo.toml
  pub manifest_path: PathBuf,
  /// root directory of the package
  pub package_root: PathBuf,
}

// serde integration types
#[ cfg( feature = "serde" ) ]
/// trait for configuration types that can be merged
pub trait ConfigMerge: Sized
{
  /// merge this configuration with another, returning the merged result
  #[ must_use ]
  fn merge( self, other: Self ) -> Self;
}

#[ cfg( feature = "serde" ) ]
/// workspace-aware serde deserializer
#[ derive( Debug ) ]
pub struct WorkspaceDeserializer< 'ws >
{
  /// reference to workspace for path resolution
  pub workspace: &'ws Workspace,
}

#[ cfg( feature = "serde" ) ]
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
  /// # fn main() -> Result< (), workspace_tools ::WorkspaceError > {
  /// use workspace_tools ::workspace;
  ///
  /// # std ::env ::set_var( "WORKSPACE_PATH", std ::env ::current_dir().unwrap() );
  /// let ws = workspace()?;
  ///
  /// // find all rust source files
  /// let rust_files = ws.find_resources( "src/**/*.rs" )?;
  ///
  /// // find all configuration files
  /// let configs = ws.find_resources( "config/**/*.toml" )?;
  /// # Ok(())
  /// # }
  /// ```
  pub fn find_resources( &self, pattern: &str ) -> Result< Vec< PathBuf > >
  {
  let full_pattern = self.join( pattern );
  let pattern_str = full_pattern.to_string_lossy();

  let mut results = Vec ::new();

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

}

#[ cfg( feature = "secrets" ) ]
impl Workspace
{
  /// get secrets directory path
  ///
  /// returns `workspace_root/secret`
  #[ must_use ]
  pub fn secret_dir( &self ) -> PathBuf
  {
  self.root.join( "secret" )
  }

  /// get path to secret configuration file
  ///
  /// returns `workspace_root/secret/{name}`
  #[ must_use ]
  pub fn secret_file( &self, name: &str ) -> PathBuf
  {
  self.secret_dir().join( name )
  }

  /// load secrets from a file in the workspace secrets directory
  ///
  /// supports shell script format (KEY=value lines) and loads secrets from filenames
  /// within the workspace `secret/` directory
  ///
  /// # Path Resolution
  ///
  /// Files are resolved as: `workspace_root/secret/{filename}`
  ///
  /// **Important** : This method expects a filename, not a path. If you need to load
  /// from a path, use `load_secrets_from_path()` instead.
  ///
  /// # examples
  ///
  /// ```rust
  /// # fn main() -> Result< (), workspace_tools ::WorkspaceError > {
  /// use workspace_tools ::workspace;
  ///
  /// # std ::env ::set_var( "WORKSPACE_PATH", std ::env ::current_dir().unwrap() );
  /// let ws = workspace()?;
  ///
  /// // ✅ Correct usage - simple filenames only
  /// // let secrets = ws.load_secrets_from_file( "-secrets.sh" )?;      // -> secret/-secrets.sh
  /// // let dev = ws.load_secrets_from_file( "development.env" )?;      // -> secret/development.env
  ///
  /// // ❌ Common mistake - using paths (will emit warning)
  /// // let secrets = ws.load_secrets_from_file( "config/secrets.env" )?; // DON'T DO THIS
  ///
  /// // ✅ For paths, use the path-specific method instead
  /// // let path_secrets = ws.load_secrets_from_path( "config/secrets.env" )?; // -> workspace/config/secrets.env
  /// # Ok(())
  /// # }
  /// ```
  ///
  /// # Errors
  ///
  /// returns error if the file cannot be read, doesn't exist, or contains invalid format
  pub fn load_secrets_from_file( &self, filename: &str ) -> Result< HashMap< String, String > >
  {
  // validate parameter doesn't look like a path
  if filename.contains( '/' ) || filename.contains( '\\' )
  {
   eprintln!(
  "⚠️  Warning: '{filename}' contains path separators. Use load_secrets_from_path() for paths."
 );
  }

  let secret_file = self.secret_file( filename );

  if !secret_file.exists()
  {
   // enhanced error: provide context about what files are available
   let available = self.list_secrets_files().unwrap_or_default();
   let suggestion = if available.is_empty()
   {
  format!( "\nNo files found in secrets directory: {}", self.secret_dir().display() )
  }
   else
   {
  format!( "\nAvailable files: {}", available.join( ", " ) )
 };

   return Err( WorkspaceError::ConfigurationError(
  format!(
   "Secrets file '{}' not found at {}.{}",
   filename,
   secret_file.display(),
   suggestion
 )
 ) );
  }

  let content = fs ::read_to_string( &secret_file )
   .map_err( | e | WorkspaceError::IoError( format!( "failed to read {} : {}", secret_file.display(), e ) ) )?;

  Ok( Self ::parse_key_value_file( &content ) )
  }

  /// load a specific secret key with fallback to environment
  ///
  /// tries to load from secret file first, then falls back to environment variable
  /// this method uses filename-based resolution (looks in secret/ directory)
  ///
  /// # Path Resolution
  ///
  /// Files are resolved as: `workspace_root/secret/{filename}`
  ///
  /// # Fallback Strategy
  ///
  /// 1. First attempts to load from secrets file
  /// 2. If key not found in file or file doesn't exist, checks environment variables
  /// 3. If neither source contains the key, returns error
  ///
  /// # examples
  ///
  /// ```rust
  /// # fn main() -> Result< (), workspace_tools ::WorkspaceError > {
  /// use workspace_tools ::workspace;
  ///
  /// # std ::env ::set_var( "WORKSPACE_PATH", std ::env ::current_dir().unwrap() );
  /// let ws = workspace()?;
  ///
  /// // ✅ Correct usage - filename only
  /// match ws.load_secret_key( "API_KEY", "-secrets.sh" )  // -> secret/-secrets.sh
  /// {
  ///     Ok( key ) => println!( "loaded api key from file or environment" ),
  ///     Err( e ) => println!( "api key not found: {}", e ),
  /// }
  ///
  /// // ❌ Common mistake - using paths (will emit warning)
  /// // let key = ws.load_secret_key( "API_KEY", "config/secrets.env" )?; // DON'T DO THIS
  /// # Ok(())
  /// # }
  /// ```
  ///
  /// # Errors
  ///
  /// returns error if the key is not found in either the secret file or environment variables
  pub fn load_secret_key( &self, key_name: &str, filename: &str ) -> Result< String >
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
  env ::var( key_name )
   .map_err( |_| WorkspaceError::ConfigurationError(
  format!(
   "{} not found in secrets file '{}' (resolved to: {}) or environment variables",
   key_name,
   filename,
   self.secret_file( filename ).display()
 )
 ))
  }

  /// parse key-value file content
  ///
  /// supports multiple formats :
  /// - shell script format with comments and quotes
  /// - export statements: `export KEY=VALUE`
  /// - standard dotenv format: `KEY=VALUE`
  /// - mixed formats in same file
  fn parse_key_value_file( content: &str ) -> HashMap< String, String >
  {
  let mut secrets = HashMap ::new();

  for line in content.lines()
  {
   let line = line.trim();

   // skip empty lines and comments
   if line.is_empty() || line.starts_with( '#' )
   {
  continue;
  }

   // handle export statements by stripping 'export ' prefix
   let processed_line = if line.starts_with( "export " )
   {
  line.strip_prefix( "export " ).unwrap_or( line ).trim()
  }
   else
   {
  line
 };

   // parse KEY=VALUE format
   if let Some( ( key, value ) ) = processed_line.split_once( '=' )
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

  /// list available secrets files in the secrets directory
  ///
  /// returns vector of filenames (not full paths) found in secret/ directory
  ///
  /// # examples
  ///
  /// ```rust
  /// # fn main() -> Result< (), workspace_tools ::WorkspaceError > {
  /// use workspace_tools ::workspace;
  ///
  /// # std ::env ::set_var( "WORKSPACE_PATH", std ::env ::current_dir().unwrap() );
  /// let ws = workspace()?;
  /// let files = ws.list_secrets_files()?;
  /// println!( "Available secret files: {:?}", files );
  /// # Ok(())
  /// # }
  /// ```
  ///
  /// # Errors
  ///
  /// returns error if the secrets directory cannot be read
  pub fn list_secrets_files( &self ) -> Result< Vec< String > >
  {
  let secret_dir = self.secret_dir();

  if !secret_dir.exists()
  {
   return Ok( Vec ::new() );
  }

  let entries = fs ::read_dir( &secret_dir )
   .map_err( | e | WorkspaceError::IoError( format!( "failed to read secrets directory {} : {}", secret_dir.display(), e ) ) )?;

  let mut files = Vec ::new();

  for entry in entries
  {
   let entry = entry
  .map_err( | e | WorkspaceError::IoError( format!( "failed to read directory entry: {e}" ) ) )?;

   let path = entry.path();

   if path.is_file()
   {
  if let Some( filename ) = path.file_name()
  {
   if let Some( filename_str ) = filename.to_str()
   {
  files.push( filename_str.to_string() );
  }
  }
  }
  }

  files.sort();
  Ok( files )
  }

  /// check if a secrets file exists
  ///
  /// returns true if the file exists in the secrets directory
  ///
  /// # examples
  ///
  /// ```rust
  /// # fn main() -> Result< (), workspace_tools ::WorkspaceError > {
  /// use workspace_tools ::workspace;
  ///
  /// # std ::env ::set_var( "WORKSPACE_PATH", std ::env ::current_dir().unwrap() );
  /// let ws = workspace()?;
  ///
  /// if ws.secrets_file_exists( "-secrets.sh" )
  /// {
  ///     println!( "secrets file found" );
  /// }
  /// # Ok(())
  /// # }
  /// ```
  #[ must_use ]
  pub fn secrets_file_exists( &self, secret_file_name: &str ) -> bool
  {
  self.secret_file( secret_file_name ).exists()
  }

  /// get resolved path for secrets file (for debugging)
  ///
  /// returns the full path where the secrets file would be located
  ///
  /// # examples
  ///
  /// ```rust
  /// # fn main() -> Result< (), workspace_tools ::WorkspaceError > {
  /// use workspace_tools ::workspace;
  ///
  /// # std ::env ::set_var( "WORKSPACE_PATH", std ::env ::current_dir().unwrap() );
  /// let ws = workspace()?;
  /// let path = ws.resolve_secrets_path( "-secrets.sh" );
  /// println!( "Secrets file would be at: {}", path.display() );
  /// # Ok(())
  /// # }
  /// ```
  #[ must_use ]
  pub fn resolve_secrets_path( &self, secret_file_name: &str ) -> PathBuf
  {
  self.secret_file( secret_file_name )
  }

  /// load secrets from workspace-relative path
  ///
  /// loads secrets from a file specified as a path relative to the workspace root
  /// use this method when you need to load secrets from custom locations
  ///
  /// # Path Resolution
  ///
  /// Files are resolved as: `workspace_root/{relative_path}`
  ///
  /// # examples
  ///
  /// ```rust
  /// # fn main() -> Result< (), workspace_tools ::WorkspaceError > {
  /// use workspace_tools ::workspace;
  ///
  /// # std ::env ::set_var( "WORKSPACE_PATH", std ::env ::current_dir().unwrap() );
  /// let ws = workspace()?;
  ///
  /// // load from config/secrets.env (workspace_root/config/secrets.env)
  /// // let secrets = ws.load_secrets_from_path( "config/secrets.env" )?;
  ///
  /// // load from nested directory
  /// // let nested = ws.load_secrets_from_path( "lib/project/secret/api.env" )?;
  /// # Ok(())
  /// # }
  /// ```
  ///
  /// # Errors
  ///
  /// returns error if the file cannot be read, doesn't exist, or contains invalid format
  pub fn load_secrets_from_path( &self, relative_path: &str ) -> Result< HashMap< String, String > >
  {
  let secret_file = self.join( relative_path );

  if !secret_file.exists()
  {
   return Err( WorkspaceError::ConfigurationError(
  format!(
   "Secrets file not found at path: {} (resolved to: {})",
   relative_path,
   secret_file.display()
 )
 ) );
  }

  let content = fs ::read_to_string( &secret_file )
   .map_err( | e | WorkspaceError::IoError( format!( "failed to read {} : {}", secret_file.display(), e ) ) )?;

  Ok( Self ::parse_key_value_file( &content ) )
  }

  /// load secrets from absolute path
  ///
  /// loads secrets from a file specified as an absolute filesystem path
  /// use this method when you need to load secrets from locations outside the workspace
  ///
  /// # examples
  ///
  /// ```rust
  /// # fn main() -> Result< (), workspace_tools ::WorkspaceError > {
  /// use workspace_tools ::workspace;
  /// use std ::path ::Path;
  ///
  /// # std ::env ::set_var( "WORKSPACE_PATH", std ::env ::current_dir().unwrap() );
  /// let ws = workspace()?;
  ///
  /// // load from absolute path
  /// let absolute_path = Path ::new( "/etc/secrets/production.env" );
  /// // let secrets = ws.load_secrets_from_absolute_path( absolute_path )?;
  /// # Ok(())
  /// # }
  /// ```
  ///
  /// # Errors
  ///
  /// returns error if the file cannot be read, doesn't exist, or contains invalid format
  pub fn load_secrets_from_absolute_path( &self, absolute_path: &Path ) -> Result< HashMap< String, String > >
  {
  if !absolute_path.exists()
  {
   return Err( WorkspaceError::ConfigurationError(
  format!(
   "Secrets file not found at absolute path: {}",
   absolute_path.display()
 )
 ) );
  }

  let content = fs ::read_to_string( absolute_path )
   .map_err( | e | WorkspaceError::IoError( format!( "failed to read {} : {}", absolute_path.display(), e ) ) )?;

  Ok( Self ::parse_key_value_file( &content ) )
  }

  /// load secrets with verbose debug information
  ///
  /// provides detailed path resolution and validation information for debugging
  /// use this method when troubleshooting secret loading issues
  ///
  /// # examples
  ///
  /// ```rust
  /// # fn main() -> Result< (), workspace_tools ::WorkspaceError > {
  /// use workspace_tools ::workspace;
  ///
  /// # std ::env ::set_var( "WORKSPACE_PATH", std ::env ::current_dir().unwrap() );
  /// let ws = workspace()?;
  ///
  /// // load with debug output
  /// match ws.load_secrets_with_debug( "-secrets.sh" )
  /// {
  ///     Ok( secrets ) => println!( "Loaded {} secrets", secrets.len() ),
  ///     Err( e ) => println!( "Failed to load secrets: {}", e ),
  /// }
  /// # Ok(())
  /// # }
  /// ```
  ///
  /// # Errors
  ///
  /// returns error if the file cannot be read, doesn't exist, or contains invalid format
  pub fn load_secrets_with_debug( &self, secret_file_name: &str ) -> Result< HashMap< String, String > >
  {
  println!( "🔍 Debug: Loading secrets with detailed information" );
  println!( "   Parameter: '{secret_file_name}'" );

  // check for path-like parameter
  if secret_file_name.contains( '/' ) || secret_file_name.contains( '\\' )
  {
   println!( "   ⚠️  Parameter contains path separators - consider using load_secrets_from_path()" );
  }

  let secret_file = self.secret_file( secret_file_name );
  println!( "   Resolved path: {}", secret_file.display() );
  println!( "   File exists: {}", secret_file.exists() );

  // show available files for context
  match self.list_secrets_files()
  {
   Ok( files ) =>
   {
  if files.is_empty()
  {
   println!( "   Available files: none (secrets directory: {})", self.secret_dir().display() );
  }
  else
  {
   println!( "   Available files: {}", files.join( ", " ) );
  }
  }
   Err( e ) => println!( "   Could not list available files: {e}" ),
  }

  // attempt to load normally
  match self.load_secrets_from_file( secret_file_name )
  {
   Ok( secrets ) =>
   {
  println!( "   ✅ Successfully loaded {} secrets", secrets.len() );
  for key in secrets.keys()
  {
   println!( "      - {key}" );
  }
  Ok( secrets )
  }
   Err( e ) =>
   {
  println!( "   ❌ Failed to load secrets: {e}" );
  Err( e )
  }
  }
  }
}

#[ cfg( feature = "secure" ) ]
impl Workspace
{
  /// load secrets from a file in the workspace secrets directory with memory-safe handling
  ///
  /// returns secrets as `SecretString` types for enhanced security
  /// supports shell script format (KEY=value lines) and loads secrets from filenames
  /// within the workspace `secret/` directory
  ///
  /// # Path Resolution
  ///
  /// Files are resolved as: `workspace_root/secret/{filename}`
  ///
  /// **Important** : This method expects a filename, not a path. If you need to load
  /// from a path, use `load_secrets_from_path_secure()` instead.
  ///
  /// # examples
  ///
  /// ```rust
  /// # fn main() -> Result< (), workspace_tools ::WorkspaceError > {
  /// use workspace_tools ::workspace;
  /// use secrecy ::ExposeSecret;
  ///
  /// # std ::env ::set_var( "WORKSPACE_PATH", std ::env ::current_dir().unwrap() );
  /// let ws = workspace()?;
  ///
  /// // ✅ Correct usage - simple filenames only
  /// // let secrets = ws.load_secrets_secure( "-secrets.sh" )?;         // -> secret/-secrets.sh
  /// // let dev = ws.load_secrets_secure( "development.env" )?;         // -> secret/development.env
  ///
  /// // Access secret values (requires explicit expose_secret() call)
  /// // if let Some( api_key ) = secrets.get( "API_KEY" )
  /// // {
  /// //     println!( "loaded api key: {}", api_key.expose_secret() );
  /// // }
  ///
  /// // ❌ Common mistake - using paths (will emit warning)
  /// // let secrets = ws.load_secrets_secure( "config/secrets.env" )?; // DON'T DO THIS
  ///
  /// // ✅ For paths, use the path-specific method instead
  /// // let path_secrets = ws.load_secrets_from_path_secure( "config/secrets.env" )?;
  /// # Ok(())
  /// # }
  /// ```
  ///
  /// # Errors
  ///
  /// returns error if the file cannot be read, doesn't exist, or contains invalid format
  pub fn load_secrets_secure( &self, filename: &str ) -> Result< HashMap< String, SecretString > >
  {
  // validate parameter doesn't look like a path
  if filename.contains( '/' ) || filename.contains( '\\' )
  {
   eprintln!(
  "⚠️  Warning: '{filename}' contains path separators. Use load_secrets_from_path() for paths."
 );
  }

  let secret_file = self.secret_file( filename );

  if !secret_file.exists()
  {
   // enhanced error: provide context about what files are available
   let available = self.list_secrets_files().unwrap_or_default();
   let suggestion = if available.is_empty()
   {
  format!( "\nNo files found in secrets directory: {}", self.secret_dir().display() )
  }
   else
   {
  format!( "\nAvailable files: {}", available.join( ", " ) )
 };

   return Err( WorkspaceError::ConfigurationError(
  format!(
   "Secrets file '{}' not found at {}.{}",
   filename,
   secret_file.display(),
   suggestion
 )
 ) );
  }

  let content = fs ::read_to_string( &secret_file )
   .map_err( | e | WorkspaceError::IoError( format!( "failed to read {} : {}", secret_file.display(), e ) ) )?;

  let parsed = Self ::parse_key_value_file( &content );
  let mut secure_secrets = HashMap ::new();

  for ( key, value ) in parsed
  {
  secure_secrets.insert( key, SecretString ::new( value ) );
  }

  Ok( secure_secrets )
  }

  /// load a specific secret key with memory-safe handling and fallback to environment
  ///
  /// tries to load from secret file first, then falls back to environment variable
  /// returns `SecretString` for enhanced security
  ///
  /// # Errors
  ///
  /// returns error if the key is not found in either the secret file or environment variables
  ///
  /// # examples
  ///
  /// ```rust
  /// # fn main() -> Result< (), workspace_tools ::WorkspaceError > {
  /// use workspace_tools ::workspace;
  /// use secrecy ::ExposeSecret;
  ///
  /// # std ::env ::set_var( "WORKSPACE_PATH", std ::env ::current_dir().unwrap() );
  /// let ws = workspace()?;
  ///
  /// // looks for API_KEY in secret/-secrets.sh, then in environment
  /// match ws.load_secret_key_secure( "API_KEY", "-secrets.sh" )
  /// {
  ///     Ok( key ) => println!( "loaded api key: {}", key.expose_secret() ),
  ///     Err( _ ) => println!( "api key not found" ),
  /// }
  /// # Ok(())
  /// # }
  /// ```
  pub fn load_secret_key_secure( &self, key_name: &str, filename: &str ) -> Result< SecretString >
  {
  // try loading from secret file first
  if let Ok( secrets ) = self.load_secrets_secure( filename )
  {
   if let Some( value ) = secrets.get( key_name )
   {
  return Ok( value.clone() );
  }
  }

  // fallback to environment variable
  match env ::var( key_name )
  {
   Ok( value ) => Ok( SecretString ::new( value ) ),
   Err( _ ) => Err( WorkspaceError::ConfigurationError(
  format!(
   "{} not found in secrets file '{}' (resolved to: {}) or environment variables",
   key_name,
   filename,
   self.secret_file( filename ).display()
 )
 ))
  }
  }

  /// get environment variable as `SecretString` for memory-safe handling
  ///
  /// # examples
  ///
  /// ```rust
  /// # fn main() -> Result< (), workspace_tools ::WorkspaceError > {
  /// use workspace_tools ::workspace;
  /// use secrecy ::ExposeSecret;
  ///
  /// # std ::env ::set_var( "WORKSPACE_PATH", std ::env ::current_dir().unwrap() );
  /// let ws = workspace()?;
  ///
  /// if let Some( token ) = ws.env_secret( "GITHUB_TOKEN" )
  /// {
  ///     println!( "using secure token: {}", token.expose_secret() );
  /// }
  /// # Ok(())
  /// # }
  /// ```
  #[ must_use ]
  pub fn env_secret( &self, key: &str ) -> Option< SecretString >
  {
  env ::var( key ).ok().map( SecretString ::new )
  }

  /// validate secret strength and security requirements
  ///
  /// checks for common security issues like weak passwords, common patterns, etc.
  ///
  /// # Errors
  ///
  /// returns error if the secret does not meet minimum security requirements
  ///
  /// # examples
  ///
  /// ```rust
  /// # fn main() -> Result< (), workspace_tools ::WorkspaceError > {
  /// use workspace_tools ::workspace;
  ///
  /// # std ::env ::set_var( "WORKSPACE_PATH", std ::env ::current_dir().unwrap() );
  /// let ws = workspace()?;
  ///
  /// // this will fail - too weak
  /// assert!( ws.validate_secret( "123" ).is_err() );
  ///
  /// // this will pass - strong secret
  /// assert!( ws.validate_secret( "super-strong-secret-2024!" ).is_ok() );
  /// # Ok(())
  /// # }
  /// ```
  pub fn validate_secret( &self, secret: &str ) -> Result< () >
  {
  if secret.len() < 8
  {
   return Err( WorkspaceError::SecretValidationError( 
  "secret must be at least 8 characters long".to_string() 
 ) );
  }

  if secret == "123" || secret == "password" || secret == "secret" || secret.to_lowercase() == "test"
  {
   return Err( WorkspaceError::SecretValidationError( 
  "secret is too weak or uses common patterns".to_string() 
 ) );
  }

  // check for reasonable complexity (at least some variety)
  let has_letter = secret.chars().any( char ::is_alphabetic );
  let has_number = secret.chars().any( char ::is_numeric );
  let has_special = secret.chars().any( | c | !c.is_alphanumeric() );

  if !( has_letter || has_number || has_special )
  {
   return Err( WorkspaceError::SecretValidationError( 
  "secret should contain letters, numbers, or special characters".to_string() 
 ) );
  }

  Ok( () )
  }

  /// load configuration with automatic secret injection
  ///
  /// replaces `${VAR_NAME}` placeholders in configuration with values from secret files
  ///
  /// # Errors
  ///
  /// returns error if configuration file cannot be read or secret injection fails
  ///
  /// # examples
  ///
  /// ```rust,no_run
  /// # fn main() -> Result< (), workspace_tools ::WorkspaceError > {
  /// use workspace_tools ::workspace;
  ///
  /// # std ::env ::set_var( "WORKSPACE_PATH", std ::env ::current_dir().unwrap() );
  /// let ws = workspace()?;
  ///
  /// // loads config.toml and replaces ${SECRET} with values from secrets.sh
  /// let config = ws.load_config_with_secret_injection( "config.toml", "secrets.sh" )?;
  /// # Ok(())
  /// # }
  /// ```
  pub fn load_config_with_secret_injection( &self, config_file: &str, secret_file: &str ) -> Result< String >
  {
  // load the configuration file
  let config_path = self.join( config_file );
  let config_content = std ::fs ::read_to_string( &config_path )
   .map_err( | e | WorkspaceError::IoError( format!( "failed to read config {} : {}", config_path.display(), e ) ) )?;

  // load secrets securely
  let secrets = self.load_secrets_secure( secret_file )?;

  // perform template substitution
  let mut result = config_content;
  for ( key, secret_value ) in secrets
  {
   let placeholder = format!( "${{{key}}}" );
   let replacement = secret_value.expose_secret();
   result = result.replace( &placeholder, replacement );
  }

  // check for unresolved placeholders
  if result.contains( "${" )
  {
   return Err( WorkspaceError::SecretInjectionError(
  "configuration contains unresolved placeholders - check secret file completeness".to_string()
 ) );
  }

  Ok( result )
  }

  /// load configuration with automatic secret injection using `SecretInjectable` trait
  ///
  /// loads secrets from file and injects them into the configuration type
  ///
  /// # Errors
  ///
  /// returns error if secret loading or injection fails
  ///
  /// # examples
  ///
  /// ```rust,no_run
  /// # fn main() -> Result< (), workspace_tools ::WorkspaceError > {
  /// # #[ cfg(feature = "secure") ] {
  /// use workspace_tools :: { workspace, SecretInjectable };
  ///
  /// #[ derive(Debug) ]
  /// struct AppConfig {
  ///     database_url: String,
  ///     api_key: String,
  /// }
  ///
  /// impl SecretInjectable for AppConfig
  /// {
  ///   fn inject_secret(&mut self, key: &str, value: String) -> workspace_tools ::Result< () >
  ///   {
  ///     match key
  ///     {
  ///             "DATABASE_URL" => self.database_url = value,
  ///             "API_KEY" => self.api_key = value,
  ///             _ => return Err(workspace_tools ::WorkspaceError::SecretInjectionError(
  ///                 format!("unknown secret key: {}", key)
  /// )),
  /// }
  ///         Ok(())
  /// }
  ///
  ///     fn validate_secrets( &self ) -> workspace_tools ::Result< () > {
  ///  if self.api_key.is_empty() {
  ///             return Err(workspace_tools ::WorkspaceError::SecretValidationError(
  ///                 "api_key cannot be empty".to_string()
  /// ));
  /// }
  ///         Ok(())
  /// }
  /// }
  ///
  /// # std ::env ::set_var( "WORKSPACE_PATH", std ::env ::current_dir().unwrap() );
  /// let ws = workspace()?;
  /// let mut config = AppConfig { database_url: String ::new(), api_key: String ::new() };
  ///
  /// // config gets secrets injected from secret/-config.sh
  /// config = ws.load_config_with_secrets( config, "-config.sh" )?;
  /// # }
  /// # Ok(())
  /// # }
  /// ```
  pub fn load_config_with_secrets< T: SecretInjectable >( &self, mut config: T, secret_file: &str ) -> Result< T >
  {
  // load secrets securely
  let secrets = self.load_secrets_secure( secret_file )?;

  // inject each secret into the configuration
  for ( key, secret_value ) in secrets
  {
   config.inject_secret( &key, secret_value.expose_secret().to_string() )?;
  }

  // validate the final configuration
  config.validate_secrets()?;

  Ok( config )
  }

  /// load secrets from workspace-relative path with memory-safe handling
  ///
  /// loads secrets from a file specified as a path relative to the workspace root
  /// returns secrets as `SecretString` types for enhanced security
  ///
  /// # Path Resolution
  ///
  /// Files are resolved as: `workspace_root/{relative_path}`
  ///
  /// # examples
  ///
  /// ```rust
  /// # fn main() -> Result< (), workspace_tools ::WorkspaceError > {
  /// use workspace_tools ::workspace;
  /// use secrecy ::ExposeSecret;
  ///
  /// # std ::env ::set_var( "WORKSPACE_PATH", std ::env ::current_dir().unwrap() );
  /// let ws = workspace()?;
  ///
  /// // load from config/secrets.env (workspace_root/config/secrets.env)
  /// // let secrets = ws.load_secrets_from_path_secure( "config/secrets.env" )?;
  /// # Ok(())
  /// # }
  /// ```
  ///
  /// # Errors
  ///
  /// returns error if the file cannot be read, doesn't exist, or contains invalid format
  pub fn load_secrets_from_path_secure( &self, relative_path: &str ) -> Result< HashMap< String, SecretString > >
  {
  let secrets = self.load_secrets_from_path( relative_path )?;
  let mut secure_secrets = HashMap ::new();

  for ( key, value ) in secrets
  {
  secure_secrets.insert( key, SecretString ::new( value ) );
  }

  Ok( secure_secrets )
  }

  /// load secrets from absolute path with memory-safe handling
  ///
  /// loads secrets from a file specified as an absolute filesystem path
  /// returns secrets as `SecretString` types for enhanced security
  ///
  /// # examples
  ///
  /// ```rust
  /// # fn main() -> Result< (), workspace_tools ::WorkspaceError > {
  /// use workspace_tools ::workspace;
  /// use secrecy ::ExposeSecret;
  /// use std ::path ::Path;
  ///
  /// # std ::env ::set_var( "WORKSPACE_PATH", std ::env ::current_dir().unwrap() );
  /// let ws = workspace()?;
  ///
  /// // load from absolute path
  /// // let absolute_path = Path ::new( "/etc/secrets/production.env" );
  /// // let secrets = ws.load_secrets_from_absolute_path_secure( absolute_path )?;
  /// # Ok(())
  /// # }
  /// ```
  ///
  /// # Errors
  ///
  /// returns error if the file cannot be read, doesn't exist, or contains invalid format
  pub fn load_secrets_from_absolute_path_secure( &self, absolute_path: &Path ) -> Result< HashMap< String, SecretString > >
  {
  let secrets = self.load_secrets_from_absolute_path( absolute_path )?;
  let mut secure_secrets = HashMap ::new();

  for ( key, value ) in secrets
  {
  secure_secrets.insert( key, SecretString ::new( value ) );
  }

  Ok( secure_secrets )
  }

  /// load secrets with verbose debug information and memory-safe handling
  ///
  /// provides detailed path resolution and validation information for debugging
  /// returns secrets as `SecretString` types for enhanced security
  ///
  /// # examples
  ///
  /// ```rust
  /// # fn main() -> Result< (), workspace_tools ::WorkspaceError > {
  /// use workspace_tools ::workspace;
  /// use secrecy ::ExposeSecret;
  ///
  /// # std ::env ::set_var( "WORKSPACE_PATH", std ::env ::current_dir().unwrap() );
  /// let ws = workspace()?;
  ///
  /// // load with debug output
  /// match ws.load_secrets_with_debug_secure( "-secrets.sh" )
  /// {
  ///     Ok( secrets ) => println!( "Loaded {} secrets", secrets.len() ),
  ///     Err( e ) => println!( "Failed to load secrets: {}", e ),
  /// }
  /// # Ok(())
  /// # }
  /// ```
  ///
  /// # Errors
  ///
  /// returns error if the file cannot be read, doesn't exist, or contains invalid format
  pub fn load_secrets_with_debug_secure( &self, secret_file_name: &str ) -> Result< HashMap< String, SecretString > >
  {
  let secrets = self.load_secrets_with_debug( secret_file_name )?;
  let mut secure_secrets = HashMap ::new();

  for ( key, value ) in secrets
  {
  secure_secrets.insert( key, SecretString ::new( value ) );
  }

  Ok( secure_secrets )
  }

}

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
  /// # fn main() -> Result< (), workspace_tools ::WorkspaceError > {
  /// use workspace_tools ::Workspace;
  ///
  /// let workspace = Workspace ::from_cargo_workspace()?;
  /// println!( "cargo workspace root: {}", workspace.root().display() );
  /// # Ok(())
  /// # }
  /// ```
  pub fn from_cargo_workspace() -> Result< Self >
  {
  let workspace_root = Self ::find_cargo_workspace()?;
  Ok( Self { root: workspace_root } )
  }

  /// create workspace from specific cargo.toml path
  ///
  /// # Errors
  ///
  /// returns error if the manifest path does not exist or cannot be parsed
  pub fn from_cargo_manifest< P: AsRef< Path > >( manifest_path: P ) -> Result< Self >
  {
  let manifest_path = manifest_path.as_ref();
  
  if !manifest_path.exists()
  {
   return Err( WorkspaceError::PathNotFound( manifest_path.to_path_buf() ) );
  }

  let workspace_root = if manifest_path.file_name() == Some( std ::ffi ::OsStr ::new( "Cargo.toml" ) )
  {
   manifest_path.parent()
  .ok_or_else( || WorkspaceError::ConfigurationError( "invalid manifest path".to_string() ) )?
  .to_path_buf()
  }
  else
  {
   manifest_path.to_path_buf()
 };

  Ok( Self { root: workspace_root } )
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
  let metadata = cargo_metadata ::MetadataCommand ::new()
   .manifest_path( &cargo_toml )
   .exec()
   .map_err( | e | WorkspaceError::CargoError( e.to_string() ) )?;

  let mut members = Vec ::new();
  let mut workspace_dependencies = HashMap ::new();

  // extract workspace member information
  for package in metadata.workspace_packages()
  {
   members.push( CargoPackage {
  name: package.name.clone(),
  version: package.version.to_string(),
  manifest_path: package.manifest_path.clone().into(),
  package_root: package.manifest_path
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
   workspace_root: metadata.workspace_root.into(),
   members,
   workspace_dependencies,
 } )
  }

  /// check if this workspace is a cargo workspace
  #[ must_use ]
  pub fn is_cargo_workspace( &self ) -> bool
  {
  let cargo_toml = self.cargo_toml();
  
  if !cargo_toml.exists()
  {
   return false;
  }

  // check if Cargo.toml contains workspace section
  if let Ok( content ) = std ::fs ::read_to_string( &cargo_toml )
  {
   if let Ok( parsed ) = toml ::from_str :: < toml ::Value >( &content )
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
  let mut current = std ::env ::current_dir()
   .map_err( | e | WorkspaceError::IoError( e.to_string() ) )?;

  loop
  {
   let manifest = current.join( "Cargo.toml" );
   if manifest.exists()
   {
  let content = std ::fs ::read_to_string( &manifest )
   .map_err( | e | WorkspaceError::IoError( e.to_string() ) )?;
  
  let parsed: toml ::Value = toml ::from_str( &content )
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

#[ cfg( feature = "serde" ) ]
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
  /// use workspace_tools ::workspace;
  /// use serde ::Deserialize;
  ///
  /// #[ derive( Deserialize ) ]
  /// struct AppConfig
  /// {
  ///     name: String,
  ///     port: u16,
  /// }
  ///
  /// # fn main() -> Result< (), workspace_tools ::WorkspaceError > {
  /// let ws = workspace()?;
  /// // looks for config/app.toml, config/app.yaml, config/app.json
  /// let config: AppConfig = ws.load_config( "app" )?;
  /// # Ok(())
  /// # }
  /// ```
  pub fn load_config< T >( &self, name: &str ) -> Result< T >
  where
  T: serde ::de ::DeserializeOwned,
  {
  let config_path = self.find_config( name )?;
  self.load_config_from( config_path )
  }

  /// load configuration from specific file
  ///
  /// # Errors
  ///
  /// returns error if file cannot be read or deserialized
  pub fn load_config_from< T, P >( &self, path: P ) -> Result< T >
  where
  T: serde ::de ::DeserializeOwned,
  P: AsRef< Path >,
  {
  let path = path.as_ref();
  let content = std ::fs ::read_to_string( path )
   .map_err( | e | WorkspaceError::IoError( format!( "failed to read {} : {}", path.display(), e ) ) )?;

  let extension = path.extension()
   .and_then( | ext | ext.to_str() )
   .unwrap_or( "toml" );

  match extension
  {
   "toml" => toml ::from_str( &content )
  .map_err( | e | WorkspaceError::SerdeError( format!( "toml deserialization error: {e}" ) ) ),
   "json" => serde_json ::from_str( &content )
  .map_err( | e | WorkspaceError::SerdeError( format!( "json deserialization error: {e}" ) ) ),
   "yaml" | "yml" => serde_yaml ::from_str( &content )
  .map_err( | e | WorkspaceError::SerdeError( format!( "yaml deserialization error: {e}" ) ) ),
   _ => Err( WorkspaceError::ConfigurationError( format!( "unsupported config format: {extension}" ) ) ),
  }
  }

  /// save configuration with format matching the original
  ///
  /// # Errors
  ///
  /// returns error if configuration cannot be serialized or written to file
  pub fn save_config< T >( &self, name: &str, config: &T ) -> Result< () >
  where
  T: serde ::Serialize,
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
  pub fn save_config_to< T, P >( &self, path: P, config: &T ) -> Result< () >
  where
  T: serde ::Serialize,
  P: AsRef< Path >,
  {
  let path = path.as_ref();
  let extension = path.extension()
   .and_then( | ext | ext.to_str() )
   .unwrap_or( "toml" );

  let content = match extension
  {
   "toml" => toml ::to_string_pretty( config )
  .map_err( | e | WorkspaceError::SerdeError( format!( "toml serialization error: {e}" ) ) )?,
   "json" => serde_json ::to_string_pretty( config )
  .map_err( | e | WorkspaceError::SerdeError( format!( "json serialization error: {e}" ) ) )?,
   "yaml" | "yml" => serde_yaml ::to_string( config )
  .map_err( | e | WorkspaceError::SerdeError( format!( "yaml serialization error: {e}" ) ) )?,
   _ => return Err( WorkspaceError::ConfigurationError( format!( "unsupported config format: {extension}" ) ) ),
 };

  // ensure parent directory exists
  if let Some( parent ) = path.parent()
  {
   std ::fs ::create_dir_all( parent )
  .map_err( | e | WorkspaceError::IoError( format!( "failed to create directory {} : {}", parent.display(), e ) ) )?;
  }

  // atomic write using temporary file
  let temp_path = path.with_extension( format!( "{extension}.tmp" ) );
  std ::fs ::write( &temp_path, content )
   .map_err( | e | WorkspaceError::IoError( format!( "failed to write temporary file {} : {}", temp_path.display(), e ) ) )?;
  
  std ::fs ::rename( &temp_path, path )
   .map_err( | e | WorkspaceError::IoError( format!( "failed to rename {} to {} : {}", temp_path.display(), path.display(), e ) ) )?;

  Ok( () )
  }

  /// load and merge multiple configuration layers
  ///
  /// # Errors
  ///
  /// returns error if any configuration file cannot be loaded or merged
  pub fn load_config_layered< T >( &self, names: &[ &str ] ) -> Result< T >
  where
  T: serde ::de ::DeserializeOwned + ConfigMerge,
  {
  let mut result: Option< T > = None;

  for name in names
  {
   if let Ok( config ) = self.load_config :: < T >( name )
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
  pub fn update_config< T, U >( &self, name: &str, updates: U ) -> Result< T >
  where
  T: serde ::de ::DeserializeOwned + serde ::Serialize,
  U: serde ::Serialize,
  {
  // load existing configuration
  let existing: T = self.load_config( name )?;
  
  // serialize both to json for merging
  let existing_json = serde_json ::to_value( &existing )
   .map_err( | e | WorkspaceError::SerdeError( format!( "failed to serialize existing config: {e}" ) ) )?;
  
  let updates_json = serde_json ::to_value( updates )
   .map_err( | e | WorkspaceError::SerdeError( format!( "failed to serialize updates: {e}" ) ) )?;

  // merge json objects
  let merged = Self ::merge_json_objects( existing_json, updates_json )?;
  
  // deserialize back to target type
  let merged_config: T = serde_json ::from_value( merged )
   .map_err( | e | WorkspaceError::SerdeError( format!( "failed to deserialize merged config: {e}" ) ) )?;
  
  // save updated configuration
  self.save_config( name, &merged_config )?;
  
  Ok( merged_config )
  }

  /// merge two json objects recursively
  fn merge_json_objects( mut base: serde_json ::Value, updates: serde_json ::Value ) -> Result< serde_json ::Value >
  {
  match ( &mut base, updates )
  {
   ( serde_json ::Value ::Object( ref mut base_map ), serde_json ::Value ::Object( updates_map ) ) =>
   {
  for ( key, value ) in updates_map
  {
   match base_map.get_mut( &key )
   {
  Some( existing ) if existing.is_object() && value.is_object() =>
  {
   *existing = Self ::merge_json_objects( existing.clone(), value )?;
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

#[ cfg( feature = "serde" ) ]
impl serde ::Serialize for WorkspacePath
{
  fn serialize< S >( &self, serializer: S ) -> core ::result ::Result< S ::Ok, S ::Error >
  where
  S: serde ::Serializer,
  {
  self.0.serialize( serializer )
  }
}

#[ cfg( feature = "serde" ) ]
impl< 'de > serde ::Deserialize< 'de > for WorkspacePath
{
  fn deserialize< D >( deserializer: D ) -> core ::result ::Result< Self, D ::Error >
  where
  D: serde ::Deserializer< 'de >,
  {
  let path = PathBuf ::deserialize( deserializer )?;
  Ok( WorkspacePath( path ) )
  }
}

#[ cfg( feature = "validation" ) ]
impl Workspace
{
  /// load and validate configuration against a json schema
  ///
  /// # Errors
  ///
  /// returns error if configuration cannot be loaded, schema is invalid, or validation fails
  ///
  /// # examples
  ///
  /// ```rust,no_run
  /// use workspace_tools ::workspace;
  /// use serde :: { Deserialize };
  /// use schemars ::JsonSchema;
  ///
  /// #[ derive( Deserialize, JsonSchema ) ]
  /// struct AppConfig
  /// {
  ///     name: String,
  ///     port: u16,
  /// }
  ///
  /// # fn main() -> Result< (), workspace_tools ::WorkspaceError > {
  /// let ws = workspace()?;
  /// let config: AppConfig = ws.load_config_with_validation( "app" )?;
  /// # Ok(())
  /// # }
  /// ```
  pub fn load_config_with_validation< T >( &self, name: &str ) -> Result< T >
  where
  T: serde ::de ::DeserializeOwned + JsonSchema,
  {
  // generate schema from type
  let schema = schemars ::schema_for!( T );
  let schema_json = serde_json ::to_value( &schema )
   .map_err( | e | WorkspaceError::ValidationError( format!( "failed to serialize schema: {e}" ) ) )?;
  
  // compile schema for validation
  let compiled_schema = Validator ::new( &schema_json )
   .map_err( | e | WorkspaceError::ValidationError( format!( "failed to compile schema: {e}" ) ) )?;
  
  self.load_config_with_schema( name, &compiled_schema )
  }
  
  /// load and validate configuration against a provided json schema
  ///
  /// # Errors
  ///
  /// returns error if configuration cannot be loaded or validation fails
  pub fn load_config_with_schema< T >( &self, name: &str, schema: &Validator ) -> Result< T >
  where
  T: serde ::de ::DeserializeOwned,
  {
  let config_path = self.find_config( name )?;
  self.load_config_from_with_schema( config_path, schema )
  }
  
  /// load and validate configuration from specific file with schema
  ///
  /// # Errors
  ///
  /// returns error if file cannot be read, parsed, or validated
  pub fn load_config_from_with_schema< T, P >( &self, path: P, schema: &Validator ) -> Result< T >
  where
  T: serde ::de ::DeserializeOwned,
  P: AsRef< Path >,
  {
  let path = path.as_ref();
  let content = std ::fs ::read_to_string( path )
   .map_err( | e | WorkspaceError::IoError( format!( "failed to read {} : {}", path.display(), e ) ) )?;

  let extension = path.extension()
   .and_then( | ext | ext.to_str() )
   .unwrap_or( "toml" );

  // parse to json value first for validation
  let json_value = match extension
  {
   "toml" =>
   {
  let toml_value: toml ::Value = toml ::from_str( &content )
   .map_err( | e | WorkspaceError::SerdeError( format!( "toml parsing error: {e}" ) ) )?;
  serde_json ::to_value( toml_value )
   .map_err( | e | WorkspaceError::SerdeError( format!( "toml to json conversion error: {e}" ) ) )?
  }
   "json" => serde_json ::from_str( &content )
  .map_err( | e | WorkspaceError::SerdeError( format!( "json parsing error: {e}" ) ) )?,
   "yaml" | "yml" =>
   {
  let yaml_value: serde_yaml ::Value = serde_yaml ::from_str( &content )
   .map_err( | e | WorkspaceError::SerdeError( format!( "yaml parsing error: {e}" ) ) )?;
  serde_json ::to_value( yaml_value )
   .map_err( | e | WorkspaceError::SerdeError( format!( "yaml to json conversion error: {e}" ) ) )?
  }
   _ => return Err( WorkspaceError::ConfigurationError( format!( "unsupported config format: {extension}" ) ) ),
 };
  
  // validate against schema
  if let Err( validation_errors ) = schema.validate( &json_value )
  {
   let errors: Vec< String > = validation_errors
  .map( | error | format!( "{} : {}", error.instance_path, error ) )
  .collect();
   return Err( WorkspaceError::ValidationError( format!( "validation failed: {}", errors.join( "; " ) ) ) );
  }
  
  // if validation passes, deserialize to target type
  serde_json ::from_value( json_value )
   .map_err( | e | WorkspaceError::SerdeError( format!( "deserialization error: {e}" ) ) )
  }
  
  /// validate configuration content against schema without loading
  ///
  /// # Errors
  ///
  /// returns error if content cannot be parsed or validation fails
  pub fn validate_config_content( content: &str, schema: &Validator, format: &str ) -> Result< () >
  {
  // parse content to json value
  let json_value = match format
  {
   "toml" =>
   {
  let toml_value: toml ::Value = toml ::from_str( content )
   .map_err( | e | WorkspaceError::SerdeError( format!( "toml parsing error: {e}" ) ) )?;
  serde_json ::to_value( toml_value )
   .map_err( | e | WorkspaceError::SerdeError( format!( "toml to json conversion error: {e}" ) ) )?
  }
   "json" => serde_json ::from_str( content )
  .map_err( | e | WorkspaceError::SerdeError( format!( "json parsing error: {e}" ) ) )?,
   "yaml" | "yml" =>
   {
  let yaml_value: serde_yaml ::Value = serde_yaml ::from_str( content )
   .map_err( | e | WorkspaceError::SerdeError( format!( "yaml parsing error: {e}" ) ) )?;
  serde_json ::to_value( yaml_value )
   .map_err( | e | WorkspaceError::SerdeError( format!( "yaml to json conversion error: {e}" ) ) )?
  }
   _ => return Err( WorkspaceError::ConfigurationError( format!( "unsupported config format: {format}" ) ) ),
 };
  
  // validate against schema
  if let Err( validation_errors ) = schema.validate( &json_value )
  {
   let errors: Vec< String > = validation_errors
  .map( | error | format!( "{} : {}", error.instance_path, error ) )
  .collect();
   return Err( WorkspaceError::ValidationError( format!( "validation failed: {}", errors.join( "; " ) ) ) );
  }
  
  Ok( () )
  }
}

/// testing utilities for workspace functionality
#[ cfg( feature = "testing" ) ]
pub mod testing
{
  use super ::Workspace;
  use tempfile ::TempDir;

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
  /// #[ cfg( test ) ]
  /// mod tests
  /// {
  ///     use workspace_tools ::testing ::create_test_workspace;
  ///
  ///     #[ test ]
  ///     fn test_my_feature()
  ///     {
  ///         let ( _temp_dir, workspace ) = create_test_workspace();
  ///
  ///         // test with isolated workspace
  ///         let config = workspace.config_dir().join( "test.toml" );
  ///         assert!( config.starts_with( workspace.root() ) );
  /// }
  /// }
  /// ```
  #[ must_use ]
  #[ inline ]
  pub fn create_test_workspace() -> ( TempDir, Workspace )
  {
  let temp_dir = TempDir ::new().unwrap_or_else( | e | panic!( "failed to create temp directory: {e}" ) );
  std ::env ::set_var( "WORKSPACE_PATH", temp_dir.path() );
  let workspace = Workspace ::resolve().unwrap_or_else( | e | panic!( "failed to resolve test workspace: {e}" ) );
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

  #[ cfg( feature = "secrets" ) ]
  let all_dirs = {
   let mut dirs = base_dirs;
   dirs.push( workspace.secret_dir() );
   dirs
 };

  #[ cfg( not( feature = "secrets" ) ) ]
  let all_dirs = base_dirs;

  for dir in all_dirs
  {
   std ::fs ::create_dir_all( &dir )
  .unwrap_or_else( | e | panic!( "failed to create directory {} : {}", dir.display(), e ) );
  }

  ( temp_dir, workspace )
  }
}

/// convenience function to get workspace instance
///
/// equivalent to `Workspace ::resolve()`
///
/// # Errors
///
/// returns error if workspace resolution fails
///
/// # examples
///
/// ```rust
/// # fn main() -> Result< (), workspace_tools ::WorkspaceError > {
/// use workspace_tools ::workspace;
///
/// # std ::env ::set_var( "WORKSPACE_PATH", std ::env ::current_dir().unwrap() );
/// let ws = workspace()?;
/// let config_dir = ws.config_dir();
/// # Ok(())
/// # }
/// ```
#[ inline ]
pub fn workspace() -> Result< Workspace >
{
  Workspace ::resolve()
}