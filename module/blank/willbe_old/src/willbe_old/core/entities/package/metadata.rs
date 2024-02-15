/// Internal namespace.
pub( crate ) mod private
{
  use std::path::PathBuf;

  use cargo_metadata::MetadataCommand;

  use error_tools::{ BasicError, err };

  use crate::Package;

  /// Package metadata
  #[ derive( Debug ) ]
  pub struct PackageMetadata
  {
    package : Package,
    metadata : cargo_metadata::Package,
  }

  impl TryFrom< PathBuf > for PackageMetadata
  {
    type Error = BasicError;

    fn try_from( value : PathBuf ) -> Result< Self, Self::Error >
    {
      let package = Package::try_from( value )?;
      package.try_into()
    }
  }

  impl TryFrom< Package > for PackageMetadata
  {
    type Error = BasicError;

    fn try_from( value : Package ) -> Result< Self, Self::Error >
    {
      let path = value.path().join( "Cargo.toml" );
      let meta = MetadataCommand::new()
      .manifest_path( &path )
      .no_deps()
      .exec()
      .map_err( | _ | err!( "Can not read metadata" ) )?;

      let metadata = meta.packages.iter()
      .find( | p | p.manifest_path == path )
      .ok_or_else( || err!( "Can not parse metadata for current package" ) )?
      .to_owned();
      Ok( Self
      {
        package : value,
        metadata
      })
    }
  }
  impl PackageMetadata
  {
    /// Returns name
    pub fn name( &self ) -> &String
    {
      &self.metadata.name
    }

    /// Returns version
    pub fn version( &self ) -> String
    {
      self.metadata.version.to_string()
    }
  }

  impl PackageMetadata
  {
    /// Returns license from `Cargo.toml`
    pub fn license( &self ) -> Option< String >
    {
      self.metadata.license.to_owned()
    }

    /// Returns path to license file if exists
    pub fn license_file( &self ) -> Option< PathBuf >
    {
      self.metadata.license_file.as_ref().map( | r | r.to_owned().into_std_path_buf() )
    }

    /// Returns path to Readme file
    pub fn readme( &self ) -> Option< PathBuf >
    {
      self.metadata.readme.as_ref().map( | r | r.to_owned().into_std_path_buf() )
    }

    /// Returns url to documentation if it is exists
    pub fn documentation( &self ) -> Option< String  >
    {
      self.metadata.documentation.to_owned()
    }
  }

  impl PackageMetadata
  {
    /// Returns reference to Package
    pub fn as_package( &self ) -> &Package
    {
      &self.package
    }

    /// Returns all metadata
    pub fn all( &self ) -> &cargo_metadata::Package
    {
      &self.metadata
    }
  }

}

//

crate::mod_interface!
{
  prelude use PackageMetadata;
}
