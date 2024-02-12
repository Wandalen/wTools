mod private
{
  use crate::*;

  use wtools::error::{ for_app::Context, Result };

  ///
  /// Generate table.
  ///
  pub fn table_generate( ( _, _ ) : ( wca::Args, wca::Props ) ) -> Result< () >
  {
    endpoint::table_create( &std::env::current_dir()? ).context( "Fail to create table" )
  }
}

crate::mod_interface!
{
  /// List packages.
  orphan use table_generate;
}
