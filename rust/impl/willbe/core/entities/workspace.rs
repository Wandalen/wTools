/// Internal namespace.
pub( crate ) mod private
{
  use std::path::PathBuf;
  use toml::Value;

  use wtools::{ BasicError, err };

  use crate::{ Package, OrderStrategy };

  /// Workspace
  #[ derive( Debug, Clone ) ]
  pub struct Workspace
  {
    path : PathBuf,
  }

  impl TryFrom< PathBuf > for Workspace
  {
    type Error = BasicError;

    fn try_from( path : PathBuf ) -> Result< Self, Self::Error >
    {
      let config_str = std::fs::read_to_string( path.join( "Cargo.toml" ) )
      .or( Err( err!( "Can not read \"Cargo.toml\"" ) ) )?;
      let toml = config_str.parse::< Value >()
      .or( Err( err!( "Can not parse \"Cargo.toml\"" ) ) )?;

      if toml.get( "workspace" ).is_some()
      {
        Ok( Self{ path } )
      }
      else
      {
        Err( err!( "\"workspace\" into \"Cargo.toml\" not found" ) )
      }
    }
  }

  impl Workspace
  {
    /// Gets list of packages into workspace
    pub fn packages( &self, _order : OrderStrategy ) -> Vec< Package >
    {
      let config_str = std::fs::read_to_string( self.path.join( "Cargo.toml" ) ).unwrap();
      let toml = config_str.parse::< Value >().unwrap();

      // iterate over members into workspace
      toml[ "workspace" ][ "members" ].as_array().unwrap_or( &vec![] ).iter()
      // fold all packages from members
      .fold( vec![], | mut acc, member |
      {
        let packages_paths = glob::glob
        (
          &format!( "{sp}/{mp}", sp = self.path.display(), mp = member.as_str().unwrap() )
        ).unwrap();

        packages_paths.filter_map( Result::ok )
        .fold( &mut acc, | acc, package_path |
        {
          if let Ok( package ) = Package::try_from( package_path )
          {
            acc.push( package );
          }
          acc
        });
        acc
      })
    }

    /// iterate over packages into workspace
    pub fn packages_iterate( &self, order : OrderStrategy ) -> impl Iterator< Item = Package >
    {
      self.packages( order ).into_iter()
    }
  }
}

//

wtools::meta::mod_interface!
{
  prelude use Workspace;
}
