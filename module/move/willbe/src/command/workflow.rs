mod private
{
  use crate::*;

  use wca::{ Args, Props };
  use wtools::error::{ anyhow::Context, Result };

  ///
  /// Generate table.
  ///
  pub fn workflow_generate( ( _, _ ) : ( Args, Props ) ) -> Result< () >
  {
    endpoint::workflow_generate( &std::env::current_dir()? ).context( "Fail to generate workflow" )
  }
}

crate::mod_interface!
{
  /// List packages.
  prelude use workflow_generate;
}

