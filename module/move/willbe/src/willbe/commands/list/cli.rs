/// Internal namespace.
mod private
{
  use crate::list::endpoint;

  use wca::{ Args, Props };
  use wtools::error::{ Result, err };

  ///
  /// List packages.
  ///

  pub fn list( ( args, _ ) : ( Args, Props ) ) -> Result< () >
  {
    let patterns = args.get_owned( 0 ).unwrap_or_default();

    endpoint::list( patterns )
  }

  ///
  /// List workspace packages.
  ///

  pub fn workspace_list( ( args, properties ) : ( Args, Props ) ) -> Result< () >
  {
    let path_to_workspace = args.get_owned( 0 ).unwrap_or_default();

    let root_crate = properties.get_owned( "root_module" ).unwrap_or_default();
    let list_type = properties.get_owned( "type" ).unwrap_or( "tree" );

    if list_type != "tree" && list_type != "topsort" {
      return Err(err!( format!( "Unknown option 'type:{}'", list_type ) ) );
    }

    endpoint::workspace_list( path_to_workspace, root_crate, list_type )
  }
}

//

crate::mod_interface!
{
  /// List packages.
  prelude use list;
  /// List workspace packages.
  prelude use workspace_list;
}
