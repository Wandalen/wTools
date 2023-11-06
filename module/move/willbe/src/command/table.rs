mod private
{
  use crate::{ endpoint, wtools };

  use wca::{ Args, Props };
  use wtools::error::Result;
  use anyhow::*;

  ///
  /// Generate table.
  ///
  pub fn generate_table( ( _, _ ) : ( Args, Props ) ) -> Result< () >
  {
    endpoint::create_table().context( "Fail to create table" )
  }
}

crate::mod_interface!
{
  /// List packages.
  prelude use generate_table;
}

