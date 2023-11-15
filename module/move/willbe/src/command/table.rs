mod private
{
  use crate::
  { 
    endpoint,
    wtools, 
  };

  use wca::
  { 
    Args, 
    Props,
  };
  
  use wtools::error::Result;
  use anyhow::*;

  ///
  /// Generate table.
  ///
  pub fn table_generate( ( _, _ ) : ( Args, Props ) ) -> Result< () >
  {
    endpoint::table_create().context( "Fail to create table" )
  }
}

crate::mod_interface!
{
  /// List packages.
  prelude use table_generate;
}

