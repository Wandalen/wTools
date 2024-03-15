/// Internal namespace.
mod private
{
  use crate::*;

  use { action, wtools };

  use std::
  {
    str::FromStr,
    path::PathBuf,
    collections::HashSet,
  };

  use wca::{ Args, Props };
  use wtools::error::{ for_app::Context, Result };

  use path::AbsolutePath;
  use action::{ list as l, list::{ ListFormat, ListOptions } };
  use former::Former;

  #[ derive( Former ) ]
  struct ListProperties
  {
    #[ default( ListFormat::Tree ) ]
    format : ListFormat,

    #[ default( false ) ]
    with_version : bool,
    #[ default( false ) ]
    with_path : bool,

    #[ default( true ) ]
    with_local : bool,
    #[ default( false ) ]
    with_remote : bool,

    #[ default( true ) ]
    with_primary : bool,
    #[ default( false ) ]
    with_dev : bool,
    #[ default( false ) ]
    with_build : bool,
  }

  ///
  /// List workspace packages.
  ///

  pub fn list( args : Args, properties : Props ) -> Result< () >
  {
    let path_to_workspace : PathBuf = args.get_owned( 0 ).unwrap_or( std::env::current_dir().context( "Workspace list command without subject" )? );
    let path_to_workspace = AbsolutePath::try_from( path_to_workspace )?;

    let ListProperties { format, with_version, with_path, with_local, with_remote, with_primary, with_dev, with_build } = ListProperties::try_from( properties )?;

    let crate_dir = CrateDir::try_from( path_to_workspace )?;

    let mut additional_info = HashSet::new();
    if with_version { additional_info.insert( l::PackageAdditionalInfo::Version ); }
    if with_path { additional_info.insert( l::PackageAdditionalInfo::Path ); }

    let mut sources = HashSet::new();
    if with_local { sources.insert( l::DependencySource::Local ); }
    if with_remote { sources.insert( l::DependencySource::Remote ); }

    let mut categories = HashSet::new();
    if with_primary { categories.insert( l::DependencyCategory::Primary ); }
    if with_dev { categories.insert( l::DependencyCategory::Dev ); }
    if with_build { categories.insert( l::DependencyCategory::Build ); }

    let args = ListOptions::former()
    .path_to_manifest( crate_dir )
    .format( format )
    .info( additional_info )
    .dependency_sources( sources )
    .dependency_categories( categories )
    .form();

    match action::list( args )
    {
      Ok( report ) =>
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

  impl TryFrom< Props > for ListProperties
  {
    type Error = wtools::error::for_app::Error;
    fn try_from( value : Props ) -> Result< Self, Self::Error >
    {
      let mut this = Self::former();

      this = if let Some( v ) = value.get_owned( "format" ).map( ListFormat::from_str ) { this.format( v? ) } else { this };
      this = if let Some( v ) = value.get_owned( "with_version" ) { this.with_version::< bool >( v ) } else { this };
      this = if let Some( v ) = value.get_owned( "with_path" ) { this.with_path::< bool >( v ) } else { this };
      this = if let Some( v ) = value.get_owned( "with_local" ) { this.with_local::< bool >( v ) } else { this };
      this = if let Some( v ) = value.get_owned( "with_remote" ) { this.with_remote::< bool >( v ) } else { this };
      this = if let Some( v ) = value.get_owned( "with_primary" ) { this.with_primary::< bool >( v ) } else { this };
      this = if let Some( v ) = value.get_owned( "with_dev" ) { this.with_dev::< bool >( v ) } else { this };
      this = if let Some( v ) = value.get_owned( "with_build" ) { this.with_build::< bool >( v ) } else { this };

      Ok( this.form() )
    }
  }

}

//

crate::mod_interface!
{
  /// List workspace packages.
  orphan use list;
}
