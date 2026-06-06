/// Define a private namespace for all its items.
#[ allow( clippy ::std_instead_of_alloc, clippy ::std_instead_of_core ) ]
mod private
{

  use crate :: *;

  use std ::
  {
  io :: { self, Read },
  fs,
 };
  use error ::
  {
  typed ::Error,
  untyped :: { format_err },
 };

  /// Represents errors related to manifest data processing.
  #[ derive( Debug, Error ) ]
  pub enum  ManifestError
  {
  /// Manifest data not loaded.
  #[ error( "Manifest data not loaded." ) ]
  EmptyManifestData,
  /// Cannot find the specified tag in the TOML file.
  #[ error( "Cannot find tag {0} in toml file." ) ]
  CannotFindValue( String ),
  /// Try to read or write
  #[ error( "Io operation with manifest failed. Details: {0}" ) ]
  Io( #[ from ] io ::Error ),
  /// It was expected to be a package, but it wasn't
  #[ error( "Is not a package" ) ]
  NotAPackage,
  /// It was expected to be a package, but it wasn't
  #[ error( "Invalid value `{0}` in manifest file." ) ]
  InvalidValue( String ),
 }

  ///
  /// Hold manifest data.
  ///
  #[ derive( Debug, Clone ) ]
  pub struct Manifest
  {
  /// Path to `Cargo.toml`
  // pub manifest_file: AbsolutePath,
  pub manifest_file: ManifestFile,
  /// Strict type of `Cargo.toml` manifest.
  pub data: toml_edit ::Document,
  // pub data: Option< toml_edit ::Document >,
 }

  impl TryFrom< ManifestFile > for Manifest
  {
  type Error = ManifestError;

  fn try_from( manifest_file: ManifestFile ) -> Result< Self, Self ::Error >
  {

   let read = fs ::read_to_string( &manifest_file )?;
   let data = read.parse :: < toml_edit ::Document >()
   .map_err( | e | io ::Error ::new( io ::ErrorKind ::InvalidData, e ) )?;

   Result ::Ok
   (
  Manifest
  {
   manifest_file,
   data,
 }
 )
 }
 }

  impl TryFrom< CrateDir > for Manifest
  {
  type Error = ManifestError;

  fn try_from( src: CrateDir ) -> Result< Self, Self ::Error >
  {
   Self ::try_from( src.manifest_file() )
 }
 }

  impl Manifest
  {
  /// Returns a mutable reference to the TOML document.
  ///
  /// If the TOML document has not been loaded yet, this function will load it
  /// by calling the `load` method. If loading fails, this function will panic.
  ///
  /// # Returns
  ///
  /// A mutable reference to the TOML document.
  pub fn data( &mut self ) -> &mut toml_edit ::Document
  {
   // if self.data.is_none() { self.load().unwrap() }
   // self.data.as_mut().unwrap()
   &mut self.data
 }

  /// Returns path to `Cargo.toml`.
  #[ must_use ]
  pub fn manifest_file( &self ) -> &AbsolutePath
  {
   &self.manifest_file
 }

  /// Path to directory where `Cargo.toml` located.
  /// # Panics
  #[ must_use ]
  pub fn crate_dir( &self ) -> CrateDir
  {
   self.manifest_file.parent().unwrap().try_into().unwrap()
   // CrateDir( self.manifest_file.parent().unwrap() )
 }

  /// Store manifest.
  /// # Errors
  pub fn store( &self ) -> io ::Result< () >
  {
   fs ::write( &self.manifest_file, self.data.to_string() )?;

   std ::io ::Result ::Ok( () )
 }

  /// Check that the current manifest is the manifest of the package (can also be a virtual workspace).
  #[ must_use ]
  pub fn package_is( &self ) -> bool
  {
   // let data = self.data.as_ref().ok_or_else( || ManifestError ::EmptyManifestData )?;
   let data = &self.data;
   data.get( "package" ).is_some() && data[ "package" ].get( "name" ).is_some()
 }

  /// Check that module is local.
  /// The package is defined as local if the `publish` field is set to `false` or the registers are specified.
  #[ must_use ]
  pub fn local_is( &self ) -> bool
  {
   // let data = self.data.as_ref().ok_or_else( || ManifestError ::EmptyManifestData )?;
   let data = &self.data;
   if data.get( "package" ).is_some() && data[ "package" ].get( "name" ).is_some()
   {
  let remote = data[ "package" ].get( "publish" ).is_none()
  || data[ "package" ][ "publish" ].as_bool().unwrap_or( true );

  return !remote;
 }
   true
 }

  /// Resolves workspace-inherited field value.
  ///
  /// When a package uses workspace inheritance (e.g., `version.workspace = true`),
  /// this function locates the workspace root `Cargo.toml` and extracts the value
  /// from `[workspace.package]` section.
  ///
  /// # Arguments
  ///
  /// * `field_name` - Name of the field to resolve (e.g., "version", "edition", "license")
  ///
  /// # Returns
  ///
  /// Returns the resolved field value as a string if found.
  ///
  /// # Errors
  ///
  /// Returns error if:
  /// - Cannot locate workspace root manifest
  /// - Cannot read workspace manifest file
  /// - Field not found in workspace.package section
  /// - Field value is not a string
  ///
  /// # Panics
  ///
  /// Panics if parent directory traversal encounters an invalid state (should not occur in practice).
  pub fn resolve_workspace_field( &self, field_name: &str ) -> Result< String, ManifestError >
  {
   // Find workspace root by traversing up from current manifest
   let mut current_dir = self.manifest_file.parent()
   .ok_or_else( || ManifestError ::Io( io ::Error ::new( io ::ErrorKind ::NotFound, "Cannot find parent directory" ) ) )?;

   // Search for workspace Cargo.toml by going up directories
   loop
   {
  let workspace_manifest = current_dir.join( "Cargo.toml" )?;

  if workspace_manifest.exists()
  {
   let workspace_content = fs ::read_to_string( workspace_manifest )?;
   let workspace_doc = workspace_content.parse :: < toml_edit ::Document >()
   .map_err( | e | io ::Error ::new( io ::ErrorKind ::InvalidData, e ) )?;

   // Check if this is a workspace manifest
   if let Some( workspace ) = workspace_doc.get( "workspace" )
   {
    // Try to get field from [workspace.package]
    if let Some( workspace_package ) = workspace.get( "package" )
    {
     if let Some( field_value ) = workspace_package.get( field_name )
     {
      if let Some( value_str ) = field_value.as_str()
      {
       return Result ::Ok( value_str.to_string() );
      }
     }
    }
   }
  }

  // Move up to parent directory
  let parent = current_dir.parent();
  if parent.is_none() || parent == Some( current_dir )
  {
   // Reached filesystem root without finding workspace
   break;
  }
  current_dir = parent.unwrap();
 }

   Err( ManifestError ::CannotFindValue( format!( "workspace.package.{field_name}" ) ) )
 }

  /// Gets package version, handling both direct and workspace-inherited values.
  ///
  /// This function properly resolves the version field whether it's:
  /// - A direct string value: `version = "0.2.0"`
  /// - Workspace-inherited: `version.workspace = true`
  ///
  /// # Returns
  ///
  /// Returns the version string if found.
  ///
  /// # Errors
  ///
  /// Returns error if:
  /// - Version field is missing
  /// - Version field has invalid format
  /// - Workspace inheritance is declared but cannot be resolved
  pub fn version( &self ) -> Result< String, ManifestError >
  {
   let data = &self.data;

   // First, try to get version as a direct string
   if let Some( version_str ) = data.get( "package" )
   .and_then( | p | p.get( "version" ) )
   .and_then( | v | v.as_str() )
   {
  return Result ::Ok( version_str.to_string() );
 }

   // Check if version uses workspace inheritance
   if let Some( version_value ) = data.get( "package" ).and_then( | p | p.get( "version" ) )
   {
  if let Some( version_table ) = version_value.as_table()
  {
   if version_table.get( "workspace" ).and_then( toml_edit::Item::as_bool ) == Some( true )
   {
    // Resolve from workspace
    return self.resolve_workspace_field( "version" );
   }
  }
 }

   // Version field is missing or has invalid format
   Err( ManifestError ::CannotFindValue( "package.version".to_string() ) )
 }
 }

  /// Retrieves the repository URL of a package from its `Cargo.toml` file.
  /// # Errors
  pub fn repo_url( crate_dir: &CrateDir ) -> error ::untyped ::Result< String >
  {
  let path = crate_dir.clone().manifest_file().inner().inner();
  if path.exists()
  {
   let mut contents = String ::new();
   fs ::File ::open( path )?.read_to_string( &mut contents )?;
   let doc = contents.parse :: < toml_edit ::Document >()?;

   let repo_url = doc
   .get( "package" )
   .and_then( | package | package.get( "repository" ) )
   .and_then( | i | i.as_str() );
   if let Some( repo_url ) = repo_url
   {
  url ::repo_url_extract( repo_url ).ok_or_else( || format_err!( "Fail to extract repository url ") )
 }
   else
   {
  let report = tool ::git ::ls_remote_url( crate_dir.clone().absolute_path() )?;
  url ::repo_url_extract( report.out.trim() ).ok_or_else( || format_err!( "Fail to extract repository url from git remote.") )
 }
 }
  else
  {
   Err( format_err!( "No Cargo.toml found" ) )
 }
 }

}

//

crate ::mod_interface!
{
  exposed use Manifest;
  orphan use ManifestError;
  own use repo_url;
}
