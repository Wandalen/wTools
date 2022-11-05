/// Internal namespace.
pub( crate ) mod private
{
  use std::{ path::PathBuf, process::Command };
  use cargo_metadata::MetadataCommand;
  use toml::Value;

  use wtools::{ BasicError, err };

  use crate::PackageInfo;

  /// Package
  #[ derive( Debug, Clone ) ]
  pub struct Package
  {
    path : PathBuf,
  }

  impl TryFrom< PathBuf > for Package
  {
    type Error = BasicError;

    fn try_from( path : PathBuf ) -> Result< Self, Self::Error >
    {
      let config_str = std::fs::read_to_string( path.join( "Cargo.toml" ) )
      .or( Err( err!( "Can not read \"Cargo.toml\"" ) ) )?;
      let toml = config_str.parse::< Value >()
      .or( Err( err!( "Can not parse \"Cargo.toml\"" ) ) )?;

      if toml.get( "package" ).is_some()
      {
        Ok( Self{ path } )
      }
      else
      {
        Err( err!( "\"package\" into \"Cargo.toml\" not found" ) )
      }
    }
  }

  impl Package
  {
    /// Gets path of package
    pub fn path( &self ) -> &PathBuf
    {
      &self.path
    }

    /// Gets info about package
    pub fn info( &self ) -> cargo_metadata::Package
    {
      // self.to_owned().into()
      let meta = MetadataCommand::new()
      .manifest_path( self.path.join( "Cargo.toml" ).to_owned() )
      .no_deps()
      .exec().unwrap();

      meta.packages.iter()
      .find( | p | p.manifest_path == self.path.join( "Cargo.toml" ) ).unwrap()
      .to_owned()
    }
  }

  impl Package
  {
    /// Check if the package has a license
    pub fn has_license( &self ) -> bool
    {
      let info = self.info();
      info.license.is_some()
      ||
      info.license_file.is_some()
    }

    /// Check if the package has a readme
    pub fn has_readme( &self ) -> bool
    {
      self.info().readme.is_some()
    }

    /// Check if the package has a documentation
    pub fn has_documentation( &self ) -> bool
    {
      self.info().documentation.is_some()
    }

    /// Checks if all tests have completed successfully
    pub fn is_tests_passed( &self ) -> bool
    {
      let tests_output = Command::new( "cargo" )
      .current_dir( &self.path )
      .args([ "test" ])
      .output().unwrap();

      tests_output.status.success()
    }
  }
}

//

wtools::meta::mod_interface!
{
  prelude use Package;
}
