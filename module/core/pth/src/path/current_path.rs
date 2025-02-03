/// Define a private namespace for all its items.
mod private
{

  #[ allow( clippy::wildcard_imports ) ]
  use crate::*;
  #[ cfg( not( feature = "no_std" ) ) ]
  use std::
  {
    env,
    io,
  };

  /// Symbolize current path.
  #[ derive( Clone, Copy, Debug, Default, PartialEq, Eq ) ]
  pub struct CurrentPath;

  #[ cfg( feature = "path_utf8" ) ]
  #[ cfg( not( feature = "no_std" ) ) ]
  impl TryFrom< CurrentPath > for Utf8PathBuf
  {
    #[ cfg( not( feature = "no_std" ) ) ]
    type Error = std::io::Error;

    #[ inline ]
    fn try_from( src : CurrentPath ) -> Result< Self, Self::Error >
    {
      Utf8PathBuf::try_from( PathBuf::try_from( src )? )
      .map_err
      (
        | err |
        {
          #[ cfg( not( feature = "no_std" ) ) ]
          std::io::Error::new
          (
            std::io::ErrorKind::NotFound,
            format!( "Cant convert to utf8 {err}" ),
          )
        }
      )
    }
  }

  #[ cfg( not( feature = "no_std" ) ) ]
  impl TryFrom< CurrentPath > for PathBuf
  {
    #[ cfg( not( feature = "no_std" ) ) ]
    type Error = std::io::Error;

    #[ inline ]
    fn try_from( _ : CurrentPath ) -> Result< Self, Self::Error >
    {
      env::current_dir()
    }
  }

  #[ cfg( not( feature = "no_std" ) ) ]
  impl TryFrom< CurrentPath > for AbsolutePath
  {
    #[ cfg( not( feature = "no_std" ) ) ]
    type Error = std::io::Error;

    #[ inline ]
    fn try_from( src : CurrentPath ) -> Result< Self, Self::Error >
    {
      AbsolutePath::try_from( PathBuf::try_from( src )? )
    }
  }

  impl TryIntoPath for &CurrentPath
  {
    fn try_into_path( self ) -> Result< PathBuf, io::Error >
    {
      env::current_dir()
    }
  }

  impl TryIntoPath for CurrentPath
  {
    fn try_into_path( self ) -> Result< PathBuf, io::Error >
    {
      env::current_dir()
    }
  }

  impl< 'a > TryIntoCowPath< 'a > for CurrentPath
  {
    fn try_into_cow_path( self ) -> Result< Cow<'a, Path>, io::Error >
    {
      let current_dir = env::current_dir()?;
      Ok( Cow::Owned( current_dir ) )
    }
  }

  impl< 'a > TryIntoCowPath< 'a > for &CurrentPath
  {
    fn try_into_cow_path( self ) -> Result< Cow<'a, Path>, io::Error >
    {
      TryIntoCowPath::try_into_cow_path( *self )
    }
  }

}

crate::mod_interface!
{
  exposed use CurrentPath;
}
