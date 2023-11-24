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
  pub fn workflow_generate( ( _, _ ) : ( Args, Props ) ) -> Result< () >
  {
    endpoint::workflow_generate().context( "Fail to generate workflow" )
  }
}

crate::mod_interface!
{
  /// List packages.
  prelude use workflow_generate;
}

