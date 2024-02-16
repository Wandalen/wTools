/// Internal namespace.
pub( crate ) mod private
{
  use std::path::PathBuf;
  use toml::Value;

  use error_tools::{ BasicError, err };

  use crate::{ Package, unique_walk };

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
      .map_err( | _ | err!( "Can not read \"Cargo.toml\"" ) )?;
      let toml = config_str.parse::< Value >()
      .map_err( | _ | err!( "Can not parse \"Cargo.toml\"" ) )?;

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
    pub fn packages( &self ) -> Vec< Package >
    {
      let config_str = std::fs::read_to_string( self.path.join( "Cargo.toml" ) ).unwrap();
      let toml = config_str.parse::< Value >().unwrap();

      // iterate over members into workspace
      toml[ "workspace" ]
      // members can be doesn't setted
      .get( "members" )
      .unwrap_or( &Value::Array( vec![] ) ).as_array()
      .unwrap_or( &vec![] )
      .iter()
      // fold all packages from members
      .fold( vec![], | mut acc, member |
      {
        let packages_paths = unique_walk
        (
          self.path.to_owned(),
          &[ member.as_str().unwrap().to_string() ]
        );

        packages_paths
        .fold( &mut acc, | acc, package_path |
        {
          if let Ok( package ) = Package::try_from( package_path.to_owned() )
          {
            acc.push( package );
          }
          // workspaces into workspace
          else if let Ok( workspace ) = Workspace::try_from( package_path )
          {
            acc.extend( workspace.packages() );
          }
          acc
        });
        acc
      })
    }

    /// iterate over packages into workspace
    pub fn packages_iterate( &self ) -> impl Iterator< Item = Package >
    {
      self.packages().into_iter()
    }
  }
}

//

crate::mod_interface!
{
  prelude use Workspace;
}
