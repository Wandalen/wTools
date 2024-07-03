/// Internal namespace.
pub( crate ) mod private
{
  use crate::*;

  /// Md's extension for workspace
  pub trait PackageMdExtension
  {
    /// Package name
    fn name( &self ) -> Result< &str, package::PackageError >;

    /// Stability
    fn stability( &self ) -> Result< action::readme_health_table_renew::Stability, package::PackageError >;

    /// Repository
    fn repository( &self ) -> Result< Option< String >, package::PackageError >;

    /// Discord url
    fn discord_url( &self ) -> Result< Option< String >, package::PackageError >;
  }

  impl < 'a > package::Package< 'a >
  {
    /// Package name
    pub fn name( &self ) -> Result< &str, package::PackageError >
    {
      match self
      {
        Self::Manifest( manifest ) =>
        {
          // let data = manifest.data.as_ref().ok_or_else( || PackageError::Manifest( ManifestError::EmptyManifestData ) )?;
          let data = &manifest.data;

          // Unwrap safely because of the `Package` type guarantee
          // Ok( data[ "package" ][ "name" ].as_str().unwrap().to_string() )
          Ok( data[ "package" ][ "name" ].as_str().unwrap() )
        }
        Self::WorkspacePackageRef( package ) =>
        {
          Ok( package.name() )
        }
      }
    }

    /// Stability
    pub fn stability( &self ) -> Result< action::readme_health_table_renew::Stability, package::PackageError >
    {
      // aaa : for Petro : bad : first of all it should be in trait. also there is duplicated code
      // aaa : done
      // aaa : for Petro : review other similar places
      // aaa : done
      match self
      {
        Self::Manifest( _ ) =>
        {
          // Unwrap safely because of the `Package` type guarantee
          Ok
          (
            self.package_metadata()
            .and_then( | m | m.get( "stability" ) )
            .and_then( | s | s.as_str() )
            .and_then( | s | s.parse::< action::readme_health_table_renew::Stability >().ok() )
            .unwrap_or( action::readme_health_table_renew::Stability::Experimental )
          )
        }
        Self::WorkspacePackageRef( package ) =>
        {
          Ok
          (
            package
            .metadata()[ "stability" ]
            .as_str()
            .and_then( | s | s.parse::< action::readme_health_table_renew::Stability >().ok() )
            .unwrap_or( action::readme_health_table_renew::Stability::Experimental)
          )
        }
      }
    }

    /// Repository
    pub fn repository( &self ) -> Result< Option< String >, package::PackageError >
    {
      match self
      {
        Self::Manifest( manifest ) =>
        {
          // let data = manifest.data.as_ref().ok_or_else( || PackageError::Manifest( ManifestError::EmptyManifestData ) )?;
          let data = &manifest.data;

          // Unwrap safely because of the `Package` type guarantee
          Ok
          (
            data[ "package" ]
            .get( "repository" )
            .and_then( | r | r.as_str() )
            .map( | r | r.to_string())
          )
        }
        Self::WorkspacePackageRef( package ) =>
        {
          Ok( package.repository().cloned() )
        }
      }
    }

    /// Discord url
    pub fn discord_url( &self ) -> Result< Option< String >, package::PackageError >
    {
      match self
      {
        Self::Manifest( _ ) =>
        {
          // let data = manifest.data.as_ref().ok_or_else( || PackageError::Manifest( ManifestError::EmptyManifestData ) )?;
          Ok
          (
            self.package_metadata()
            .and_then( | m | m.get( "discord_url" ) )
            .and_then( | url | url.as_str() )
            .map( | r | r.to_string() )
          )
        }
        Self::WorkspacePackageRef( package ) =>
        {
          Ok( package.metadata()[ "discord_url" ].as_str().map( | url | url.to_string() ) )
        }
      }
    }

    fn package_metadata( &self ) -> Option< &toml_edit::Item >
    {
      match self {
        package::Package::Manifest( manifest ) =>
        {
          let data = &manifest.data;

          data[ "package" ]
          .get( "metadata" )
        }
        package::Package::WorkspacePackageRef(_) =>
        {
          None
        }
      }
    }
  }
}


crate::mod_interface!
{
  own use PackageMdExtension;
}
