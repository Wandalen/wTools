/// Internal namespace.
mod private
{
  use crate::*;

  use std::
  {
    str::FromStr,
    path::PathBuf,
  };
  use wca::VerifiedCommand;
  use error::{ untyped::Context, Result };
  use collection::HashSet;

  use action::
  {
    list as l,
    list::{ ListFormat, ListOptions },
  };
  use former::Former;

  #[ derive( Former ) ]
  struct ListProperties
  {
    #[ former( default = ListFormat::Tree ) ]
    format : ListFormat,

    #[ former( default = false ) ]
    with_version : bool,
    #[ former( default = false ) ]
    with_path : bool,

    #[ former( default = true ) ]
    with_local : bool,
    #[ former( default = false ) ]
    with_remote : bool,

    #[ former( default = true ) ]
    with_primary : bool,
    #[ former( default = false ) ]
    with_dev : bool,
    #[ former( default = false ) ]
    with_build : bool,
  }

  ///
  /// List workspace packages.
  ///

  pub fn list( o : VerifiedCommand ) -> Result< () >
  {
    let path_to_workspace : PathBuf = o.args
    .get_owned( 0 )
    .unwrap_or( std::env::current_dir().context( "Workspace list command without subject" )? );
    // let path_to_workspace = AbsolutePath::try_from( fs::canonicalize( path_to_workspace )? )?;

    let ListProperties { format, with_version, with_path, with_local, with_remote, with_primary, with_dev, with_build } = o.props.try_into()?;

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

    let o = ListOptions::former()
    .path_to_manifest( crate_dir )
    .format( format )
    .info( additional_info )
    .dependency_sources( sources )
    .dependency_categories( categories )
    .form();

    match action::list( o )
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

  impl TryFrom< wca::Props > for ListProperties
  {
    type Error = error::untyped::Error;
    fn try_from( value : wca::Props ) -> Result< Self, Self::Error >
    {
      let mut this = Self::former();

      this = if let Some( v ) = value
      .get_owned( "format" )
      .map( ListFormat::from_str ) { this.format( v? ) } else { this };

      this = if let Some( v ) = value
      .get_owned( "with_version" ) { this.with_version::< bool >( v ) } else { this };
      this = if let Some( v ) = value
      .get_owned( "with_path" ) { this.with_path::< bool >( v ) } else { this };
      this = if let Some( v ) = value
      .get_owned( "with_local" ) { this.with_local::< bool >( v ) } else { this };
      this = if let Some( v ) = value
      .get_owned( "with_remote" ) { this.with_remote::< bool >( v ) } else { this };
      this = if let Some( v ) = value
      .get_owned( "with_primary" ) { this.with_primary::< bool >( v ) } else { this };
      this = if let Some( v ) = value
      .get_owned( "with_dev" ) { this.with_dev::< bool >( v ) } else { this };
      this = if let Some( v ) = value
      .get_owned( "with_build" ) { this.with_build::< bool >( v ) } else { this };

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
