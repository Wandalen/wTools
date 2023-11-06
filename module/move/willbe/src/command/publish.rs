/// Internal namespace.
mod private
{
  use crate::{ endpoint, wtools };

  use crate::tools::bool::*;
  use wca::{ Args, Props };
  use wtools::error::Result;
  use anyhow::*;

  ///
  /// Publish package.
  ///

  pub fn publish( ( args, properties ) : ( Args, Props ) ) -> Result< () >
  {
    let patterns = args.get_owned( 0 ).unwrap_or_default();
    let dry = properties.get_owned( "dry" ).map( | dry : String | dry.to_bool_like() ).unwrap_or_else( || BoolLike::True ).into();

    endpoint::publish( patterns, dry ).context( "publish command" )
  }

  ///
  /// Publish packages from workspace.
  ///

  pub fn workspace_publish( ( args, properties ) : ( Args, Props ) ) -> Result< () >
  {
    let path_to_workspace = args.get_owned( 0 ).unwrap_or_default();
    let dry = properties.get_owned( "dry" ).map( | dry : String | dry.to_bool_like() ).unwrap_or_else( || BoolLike::True ).into();

    endpoint::workspace_publish( path_to_workspace, dry ).context( "workspace publish command" )
  }
}

//

crate::mod_interface!
{
  /// List packages.
  prelude use publish;
  /// List workspace packages.
  prelude use workspace_publish;
}
