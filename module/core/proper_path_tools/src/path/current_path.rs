/// Internal namespace.
mod private
{

  use crate::*;
  #[ cfg( not( feature = "no_std" ) ) ]
  use std::env;

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
            format!( "Cant convert to utf8 {}", err ),
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

}

crate::mod_interface!
{
  exposed use CurrentPath;
}
