mod private
{
  use std::path::PathBuf;
  use crate::*;

  use { action, wtools };

  use wca::{ Args, Props };
  use wtools::error::{ for_app::Context, Result };

  ///
  ///
  ///

  pub fn publish_diff( args : Args ) -> Result< () >
  {
    let package : PathBuf = args.get_owned( 0 ).unwrap_or( std::env::current_dir()? );

    Ok( () )
  }
}

//

crate::mod_interface!
{
  orphan use publish_diff;
}
