mod private
{
  use crate::*;
  use action;
  use _path::AbsolutePath;
  use error_tools::Result;
  use wtools::error::anyhow::Error;

  /// Generates header to main Readme.md file.
  pub fn readme_header_renew() -> Result< () >
  {
    match action::readme_header_renew( AbsolutePath::try_from( std::env::current_dir()? )? )
    {
      Ok( report ) => 
      { 
        println!( "{report}" );
        Ok( () )
      }
      Err( ( report, e ) ) =>
      {
        eprintln!( "{report}" );
        Err( Error::from( e ).context( "Fail to generate main header." ) )
      }
    }
  }
}

crate::mod_interface!
{
  /// Generate header.
  orphan use readme_header_renew;
}