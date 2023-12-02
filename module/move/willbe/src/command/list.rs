/// Internal namespace.
mod private
{
  use crate::*;
  use std::path::PathBuf;
  use std::str::FromStr;
  use { endpoint, path, wtools };

  use wca::{ Args, Props };
  use endpoint::list::{ ListFormat, ListFilter };
  use wtools::error::for_app::Context;
  use wtools::error;

  ///
  /// List workspace packages.
  ///

  pub fn list( ( args, properties ) : ( Args, Props ) ) -> error::Result< () >
  {
    let path_to_workspace : PathBuf = args.get_owned( 0 ).unwrap_or( std::env::current_dir().context( "Workspace list command without subject" )? );
    let path_to_workspace = path::canonicalize( path_to_workspace )?;

    let format = properties.get_owned( "format" ).map( ListFormat::from_str ).transpose()?.unwrap_or_default();
    let filter = properties.get_owned( "filter" ).map( ListFilter::from_str ).transpose()?.unwrap_or_default();

    match endpoint::list( path_to_workspace, format, filter )
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
  orphan use list;
}
