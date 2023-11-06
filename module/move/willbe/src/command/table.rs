mod private
{
  use crate::{ endpoint, wtools };

  use wca::{ Args, Props };
  use wtools::error::Result;
  use anyhow::*;

  ///
  /// Generate table.
  ///
  pub fn generate_table( ( args, _ ) : ( Args, Props ) ) -> Result< () >
  {
    let path = args.get_owned( 0 ).unwrap_or_default();
    dbg!(&path);
    endpoint::create_table( path ).context( "TODO")
  }
}

crate::mod_interface!
{
  /// List packages.
  prelude use generate_table;
}

