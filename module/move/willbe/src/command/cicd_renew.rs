mod private
{
  use crate::*;

  use wtools::error::{ anyhow::Context, Result };

  ///
  /// Generate table.
  ///
  pub fn cicd_renew() -> Result< () >
  {
    action::cicd_renew( &std::env::current_dir()? ).context( "Fail to generate workflow" )
  }
}

crate::mod_interface!
{
  /// List packages.
  exposed use cicd_renew;
}

