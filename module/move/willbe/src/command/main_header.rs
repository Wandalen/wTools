mod private
{
  use crate::*;
  use action;
  use _path::AbsolutePath;
  use error_tools::{ for_app::Context, Result };

  /// Generates header to main Readme.md file.
  pub fn readme_header_renew() -> Result< () >
  {
    action::readme_header_renew( AbsolutePath::try_from( std::env::current_dir()? )? ).context( "Fail to create table" )
  }
}

crate::mod_interface!
{
  /// Generate header.
  orphan use readme_header_renew;
}