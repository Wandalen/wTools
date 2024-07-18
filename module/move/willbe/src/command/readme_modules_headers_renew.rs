mod private
{
  use crate::*;
  // use path::AbsolutePath;
  // use error::{ untyped::Error };

  /// Generate headers for workspace members
  // qqq : typed error
  pub fn readme_modules_headers_renew() -> error::untyped::Result< () >
  {
    match action::readme_modules_headers_renew( CrateDir::transitive_try_from::< AbsolutePath >( CurrentPath )? )
    {
      Ok( report ) =>
      {
        println!( "{report}" );
        Ok( () )
      }
      Err( ( report, e ) ) =>
      {
        eprintln!( "{report}" );
        Err( error::untyped::Error::from( e ).context( "Fail to generate modules headers." ) )
        // qqq : use typed error
      }
    }
  }

}

crate::mod_interface!
{
  /// List packages.
  orphan use readme_modules_headers_renew;
}