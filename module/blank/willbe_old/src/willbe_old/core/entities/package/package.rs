/// Internal namespace.
pub( crate ) mod private
{
  use std::path::PathBuf;
  use toml::Value;

  use error_tools::{ BasicError, err };

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
      .map_err( | _ | err!( "Can not read \"Cargo.toml\"" ) )?;
      let toml = config_str.parse::< Value >()
      .map_err( | _ | err!( "Can not parse \"Cargo.toml\"" ) )?;

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
  }
}

//

crate::mod_interface!
{
  prelude use Package;
}
