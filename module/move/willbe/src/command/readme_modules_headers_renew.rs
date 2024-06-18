mod private
{
  use crate::*;
  // use path::AbsolutePath;
  use error::{ untyped::Error, Result };

  /// Generate headers for workspace members
  pub fn readme_modules_headers_renew() -> Result< () >
  {
    // match action::readme_modules_headers_renew( AbsolutePath::try_from( std::env::current_dir()? )? )
    match action::readme_modules_headers_renew( CrateDir::try_from( std::env::current_dir()? )? )
    {
      Ok( report ) =>
      {
        println!( "{report}" );
        Ok( () )
      }
      Err( ( report, e ) ) =>
      {
        eprintln!( "{report}" );
        Err( Error::from( e ).context( "Fail to generate modules headers." ) )
      }
    }
  }

}

crate::mod_interface!
{
  /// List packages.
  orphan use readme_modules_headers_renew;
}