/// Internal namespace.
mod private
{
  use crate::*;

  use { endpoint, wtools };

  use std::
  {
    str::FromStr,
    path::PathBuf,
    collections::HashSet,
  };

  use wca::{ Args, Props };
  use wtools::error::{ for_app::Context, Result };

  use path::AbsolutePath;
  use endpoint::{ list as l, list::{ ListFormat, ListArgs } };

  ///
  /// List workspace packages.
  ///

  pub fn list( ( args, properties ) : ( Args, Props ) ) -> Result< () >
  {
    let path_to_workspace : PathBuf = args.get_owned( 0 ).unwrap_or( std::env::current_dir().context( "Workspace list command without subject" )? );
    let path_to_workspace = AbsolutePath::try_from( path_to_workspace )?;

    let format = properties.get_owned( "format" ).map( ListFormat::from_str ).transpose()?.unwrap_or_default();

    let with_local = properties.get_owned( "with_local" ).unwrap_or( true );
    let with_remote = properties.get_owned( "with_remote" ).unwrap_or( false );

    let with_primary = properties.get_owned( "with_primary" ).unwrap_or( true );
    let with_dev = properties.get_owned( "with_dev" ).unwrap_or( false );
    let with_build = properties.get_owned( "with_build" ).unwrap_or( false );

    let crate_dir = CrateDir::try_from( path_to_workspace )?;

    let mut sources = HashSet::new();
    if with_local { sources.insert( l::DependencySource::Local ); }
    if with_remote { sources.insert( l::DependencySource::Remote ); }

    let mut categories = HashSet::new();
    if with_primary { categories.insert( l::DependencyCategory::Primary ); }
    if with_dev { categories.insert( l::DependencyCategory::Dev ); }
    if with_build { categories.insert( l::DependencyCategory::Build ); }

    let args = ListArgs::former()
    .path_to_manifest( crate_dir )
    .format( format )
    .dependency_sources( sources )
    .dependency_categories( categories )
    .form();

    match endpoint::list( args )
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
