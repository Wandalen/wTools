mod private
{
  use crate::*;
  use action;
  use error::untyped::{ Error, Result };

  /// Generates header to main Readme.md file.
  pub fn readme_header_renew() -> Result< () >
  {
    match action::readme_header_renew
    (
      CrateDir::transitive_try_from::< AbsolutePath >( CurrentPath )?
    )
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