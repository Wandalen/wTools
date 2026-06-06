/// Define a private namespace for all its items.
#[ allow( clippy ::std_instead_of_alloc, clippy ::std_instead_of_core ) ]
mod private
{

  use crate :: *;

  /// Md's extension for workspace
  pub trait PackageMdExtension
  {
  /// Package name
  /// # Errors
  fn name( &self ) -> Result< &str, package ::PackageError >;

  /// Stability
  /// # Errors
  fn stability( &self ) -> Result< action ::readme_health_table_renew ::Stability, package ::PackageError >;

  /// Repository
  /// # Errors
  fn repository( &self ) -> Result< Option< String >, package ::PackageError >;

  /// Discord url
  /// # Errors
  fn discord_url( &self ) -> Result< Option< String >, package ::PackageError >;
 }
  // fix clippy
  impl  package ::Package< '_ >
  {
  /// Package name
  /// # Errors
  ///
  /// # Panics
  pub fn name( &self ) -> Result< &str, package ::PackageError >
  {
   match self
   {
  Self ::Manifest( manifest ) =>
  {
   // let data = manifest.data.as_ref().ok_or_else( || PackageError ::Manifest( ManifestError ::EmptyManifestData ) )?;
   let data = &manifest.data;

   // Unwrap safely because of the `Package` type guarantee
   // Ok( data[ "package" ][ "name" ].as_str().unwrap().to_string() )
   Result ::Ok( data[ "package" ][ "name" ].as_str().unwrap() )
 }
  Self ::WorkspacePackageRef( package ) =>
  {
   Result ::Ok( package.name() )
 }
 }
 }

  /// Stability
  ///
  /// # Errors
  pub fn stability( &self ) -> Result< action ::readme_health_table_renew ::Stability, package ::PackageError >
  {
   match self
   {
  Self ::Manifest( _ ) =>
  {
   // Unwrap safely because of the `Package` type guarantee
   Result ::Ok
   (
  self.package_metadata()
  .and_then( | m | m.get( "stability" ) )
  .and_then( | s | s.as_str() )
  .and_then( | s | s.parse :: < action ::readme_health_table_renew ::Stability >().ok() )
  .unwrap_or( action ::readme_health_table_renew ::Stability ::Experimental )
 )
 }
  Self ::WorkspacePackageRef( package ) =>
  {
   Result ::Ok
   (
  package
  .metadata()[ "stability" ]
  .as_str()
  .and_then( | s | s.parse :: < action ::readme_health_table_renew ::Stability >().ok() )
  .unwrap_or( action ::readme_health_table_renew ::Stability ::Experimental)
 )
 }
 }
 }

  /// Repository
  ///
  /// # Errors
  pub fn repository( &self ) -> Result< Option< String >, package ::PackageError >
  {
   match self
   {
  Self ::Manifest( manifest ) =>
  {
   // let data = manifest.data.as_ref().ok_or_else( || PackageError ::Manifest( ManifestError ::EmptyManifestData ) )?;
   let data = &manifest.data;

   // Unwrap safely because of the `Package` type guarantee
   Result ::Ok
   (
  data[ "package" ]
  .get( "repository" )
  .and_then( | r | r.as_str() )
  .map( std ::string ::ToString ::to_string )
 )
 }
  Self ::WorkspacePackageRef( package ) =>
  {
   Result ::Ok( package.repository().cloned() )
 }
 }
 }

  /// Discord url
  ///
  /// # Errors
  pub fn discord_url( &self ) -> Result< Option< String >, package ::PackageError >
  {
   match self
   {
  Self ::Manifest( _ ) =>
  {
   // let data = manifest.data.as_ref().ok_or_else( || PackageError ::Manifest( ManifestError ::EmptyManifestData ) )?;
   Result ::Ok
   (
  self.package_metadata()
  .and_then( | m | m.get( "discord_url" ) )
  .and_then( | url | url.as_str() )
  .map( std ::string ::ToString ::to_string )
 )
 }
  Self ::WorkspacePackageRef( package ) =>
  {
   Result ::Ok( package.metadata()[ "discord_url" ].as_str().map( std ::string ::ToString ::to_string ) )
 }
 }
 }

  fn package_metadata( &self ) -> Option< &toml_edit ::Item >
  {
   match self 
   {
  package ::Package ::Manifest( manifest ) =>
  {
   let data = &manifest.data;

   data[ "package" ]
   .get( "metadata" )
 }
  package ::Package ::WorkspacePackageRef( _ ) =>
  {
   None
 }
 }
 }
 }
}


crate ::mod_interface!
{
  own use PackageMdExtension;
}
