/// Internal namespace.
mod private
{
  use std::path::PathBuf;
  use std::str::FromStr;
  use crate::{ endpoint, wtools };

  use wca::{ Args, Props };
  use wtools::error::Result;
  use anyhow::*;
  use crate::endpoint::list::ListFormat;

  ///
  /// List workspace packages.
  ///

  pub fn list( ( args, properties ) : ( Args, Props ) ) -> Result< () >
  {
    let path_to_workspace : PathBuf = args.get_owned( 0 ).unwrap_or( std::env::current_dir().context( "Workspace list command without subject" )? );

    let root_crate = properties.get_owned( "root_module" ).unwrap_or_default();
    let format = properties.get_owned( "format" ).map( ListFormat::from_str ).transpose()?.unwrap_or_default();

    match endpoint::list(path_to_workspace, root_crate, format )
    {
      core::result::Result::Ok( report ) =>
      {
        println!( "{report} ");
      }
      Err(( report, e )) =>
      {
        eprintln!( "{report}" );

        return Err( e.context( "workspace list command" ) );
      }
    }

    Ok( () )
  }
}

//

crate::mod_interface!
{
  /// List workspace packages.
  prelude use list;
}
