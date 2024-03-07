mod private
{
  use error_tools::{ for_app::Context, Result };
  use crate::endpoint;
  use crate::path::AbsolutePath;

  /// Generates header to main Readme.md file.
  pub fn main_header_generate( ( _, _ ) : ( wca::Args, wca::Props ) ) -> Result< () >
  {
    endpoint::readme_header_generate( AbsolutePath::try_from( std::env::current_dir()? )? ).context( "Fail to create table" )
  }
}

crate::mod_interface!
{
  /// Generate header.
  exposed use main_header_generate;
}