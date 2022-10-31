/// Internal namespace.
pub( crate ) mod private
{
  use std::path::PathBuf;
  use toml::Value;

  use crate::{ Package, OrderStrategy };

  /// Workspace
  #[ derive( Debug, Clone ) ]
  pub struct Workspace
  {
    path : PathBuf,
  }

  impl TryFrom< PathBuf > for Workspace
  {
    type Error = ();

    fn try_from( path : PathBuf ) -> Result< Self, Self::Error >
    {
      let config_str = std::fs::read_to_string( path.join( "Cargo.toml" ) ).or( Err( () ) )?;
      let toml = config_str.parse::< Value >().or( Err( () ) )?;

      if toml.get( "workspace" ).is_some()
      {
        Ok( Self{ path } )
      }
      else
      {
        Err( () )
      }
    }
  }

  impl Workspace
  {
    /// iterate over packages into workspace
    pub fn packages_iterate( &self, _order : OrderStrategy ) -> impl Iterator< Item = Package >
    {
      // it might be better to move somewhere. reparse it again - isn't good
      let config_str = std::fs::read_to_string( self.path.join( "Cargo.toml" ) ).unwrap();
      let toml = config_str.parse::< Value >().unwrap();

      // iterate over members into workspace
      let packages = toml[ "workspace" ][ "members" ].as_array().unwrap().iter()
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
      });

      packages.into_iter()
    }
  }
}

//

wtools::meta::mod_interface!
{
  prelude use Workspace;
}
