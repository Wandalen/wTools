mod private
{
  use crate::*;

  use wtools::error::{ for_app::Context, Result };

  ///
  /// Generate table.
  ///
  pub fn readme_health_table_renew( ( _, _ ) : ( wca::Args, wca::Props ) ) -> Result< () >
  {
    action::readme_health_table_renew( &std::env::current_dir()? ).context( "Fail to create table" )
  }
}

crate::mod_interface!
{
  /// List packages.
  orphan use readme_health_table_renew;
}
