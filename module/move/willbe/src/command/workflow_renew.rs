mod private
{
  use crate::*;

  use wca::{ Args, Props };
  use wtools::error::{ anyhow::Context, Result };

  ///
  /// Generate table.
  ///
  pub fn workflow_renew( ( _, _ ) : ( Args, Props ) ) -> Result< () >
  {
    action::workflow_renew( &std::env::current_dir()? ).context( "Fail to generate workflow" )
  }
}

crate::mod_interface!
{
  /// List packages.
  exposed use workflow_renew;
}

