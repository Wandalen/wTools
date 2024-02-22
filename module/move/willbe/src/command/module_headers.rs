mod private
{
	use crate::endpoint;
	use crate::path::AbsolutePath;
	use crate::wtools::error::{ for_app::Context, Result };

	/// Generate headers for workspace members
	pub fn headers_generate(( _, _ ) : (wca::Args, wca::Props ) ) -> Result< () >
	{
		endpoint::generate_modules_headers( AbsolutePath::try_from( std::env::current_dir()? )? ).context( "Fail to generate headers" )
	}

}

crate::mod_interface!
{
  /// List packages.
  orphan use headers_generate;
}