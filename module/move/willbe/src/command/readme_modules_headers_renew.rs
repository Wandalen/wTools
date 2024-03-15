mod private
{
  use crate::*;
  use path::AbsolutePath;
  use wtools::error::{ for_app::Context, Result };

  /// Generate headers for workspace members
  pub fn readme_modules_headers_renew() -> Result< () >
  {
    action::readme_modules_headers_renew( AbsolutePath::try_from( std::env::current_dir()? )? ).context( "Fail to generate headers" )
  }

}

crate::mod_interface!
{
  /// List packages.
  orphan use readme_modules_headers_renew;
}