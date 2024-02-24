mod private
{ 
  use crate::*;
  
  use wca::{ Args, Props };
  use wtools::error::{ anyhow::Context, Result };
  
  ///
  /// Create new workspace.
  ///
  pub fn workspace_new( ( _, _ ) : ( Args, Props ) ) -> Result< () > 
  { 
    endpoint::workspace_new( &std::env::current_dir()? ).context( "Fail to workspace" ) 
  } 
}

crate::mod_interface! 
{
  /// List packages.
  prelude use workspace_new;
}

