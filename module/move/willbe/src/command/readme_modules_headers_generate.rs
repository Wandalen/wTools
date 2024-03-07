mod private
{
  use crate::*;
  use path::AbsolutePath;
  use wtools::error::{ for_app::Context, Result };

  /// Generate headers for workspace members
  pub fn readme_modules_headers_generate( ( _, _ ) : ( wca::Args, wca::Props ) ) -> Result< () >
  {
    endpoint::readme_modules_headers_generate( AbsolutePath::try_from( std::env::current_dir()? )? ).context( "Fail to generate headers" )
  }

}

crate::mod_interface!
{
  /// List packages.
  orphan use readme_modules_headers_generate;
}