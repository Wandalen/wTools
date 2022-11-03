/// Internal namespace.
pub( crate ) mod private
{
  use std::{ path::PathBuf, process::Command };
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
    pub fn info( &self ) -> PackageInfo
    {
      self.to_owned().into()
    }
  }

  impl Package
  {
    /// Check if the package has a license
    pub fn has_license( &self ) -> bool
    {
      self.path.join( "License" ).exists()
    }

    /// Check if the package has a readme
    pub fn has_readme( &self ) -> bool
    {
      self.path.join( "Readme.md" ).exists()
    }

    /// Check if the package has a documentation
    pub fn has_documentation( &self ) -> bool
    {
      //? How to check it?
      false
    }

    /// Checks if all tests have completed successfully
    pub fn is_tests_passed( &self ) -> bool
    {
      let tests_output = Command::new( "cargo" )
      .current_dir( &self.path )
      .args([ "test", "-q" ])
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
