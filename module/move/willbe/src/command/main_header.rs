mod private
{

  use crate::*;
  // use action;
  use error::untyped::{ Error };
  // Explicit import for Result and its variants for pattern matching
  use std::result::Result::{self, Ok, Err};

  /// Generates header to main readme.md file.
  ///
  /// # Errors
  /// qqq: doc
  // qqq : typed error
  pub fn readme_header_renew() -> error::untyped::Result< () >
  {
    match crate::action::main_header::action
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