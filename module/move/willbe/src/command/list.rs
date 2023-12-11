/// Internal namespace.
mod private
{
  use crate::*;
  use std::path::PathBuf;
  use std::str::FromStr;
  use { endpoint, wtools };

  use wca::{ Args, Props };
  use endpoint::list::{ ListFormat, ListFilter };
  use wtools::error::for_app::Context;
  use wtools::error;
  use path::AbsolutePath;

  ///
  /// List workspace packages.
  ///

  pub fn list( ( args, properties ) : ( Args, Props ) ) -> error::Result< () >
  {
    let path_to_workspace : PathBuf = args.get_owned( 0 ).unwrap_or( std::env::current_dir().context( "Workspace list command without subject" )? );
    let path_to_workspace = AbsolutePath::try_from( path_to_workspace )?;

    let format = properties.get_owned( "format" ).map( ListFormat::from_str ).transpose()?.unwrap_or_default();
    let filter = properties.get_owned( "filter" ).map( ListFilter::from_str ).transpose()?.unwrap_or_default();

    let crate_dir = CrateDir::try_from( path_to_workspace )?;

    match endpoint::list( crate_dir, format, filter )
    {
      core::result::Result::Ok( report ) =>
      {
        println!( "{report}" );
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
