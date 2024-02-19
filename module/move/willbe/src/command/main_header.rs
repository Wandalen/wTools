mod private
{
	use error_tools::{ for_app::Context, Result };
	use crate::endpoint;

	/// Generates header to main Readme.md file.
  pub fn main_header_generate( ( _, _ ) : ( wca::Args, wca::Props ) ) -> Result< () >
	{
		endpoint::generate_main_header( &std::env::current_dir()? ).context( "Fail to create table" )
	}
}

crate::mod_interface!
{
  /// Generate header.
  prelude use main_header_generate;
}