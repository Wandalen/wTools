mod private
{
  use error_tools::{ for_app::Context, Result };
  use crate::endpoint;
  use crate::path::AbsolutePath;

  /// Aggregates two commands: `generate_modules_headers` & `generate_main_header`
  pub fn readme_headers_generate( ( _, _ ) : ( wca::Args, wca::Props ) ) -> Result< () >
  {
    let absolute_path = AbsolutePath::try_from( std::env::current_dir()? )?;
    endpoint::generate_modules_headers( absolute_path.clone() ).context( "Fail to generate header`s" )?;
    endpoint::generate_main_header( absolute_path ).context( "Fail ti generate main header" )
  }
}

crate::mod_interface!
{
  /// Generate header's.
  exposed use readme_headers_generate;
}