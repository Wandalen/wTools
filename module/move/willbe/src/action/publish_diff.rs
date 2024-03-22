/// Internal namespace.
mod private
{
  use crate::*;

  use std::path::PathBuf;
  use crates_tools::CrateArchive;

  use _path::AbsolutePath;
  use wtools::error::for_app::Result;
  use diff::{ DiffReport, crate_diff };

  /// Return the differences between a local and remote package versions.
  #[ cfg_attr( feature = "tracing", tracing::instrument ) ]
  pub fn publish_diff( path : PathBuf ) -> Result< DiffReport >
  {
    let path = AbsolutePath::try_from( path )?;
    let dir = CrateDir::try_from( path )?;

    let package = package::Package::try_from( dir.clone() )?;
    let name = &package.name()?;
    let version = &package.version()?;

    _ = cargo::pack( cargo::PackOptions::former().path( dir.as_ref() ).dry( false ).form() )?;
    let l = CrateArchive::read( packed_crate::local_path( name, version, dir )? )?;
    let r = CrateArchive::download_crates_io( name, version ).unwrap();

    Ok( crate_diff( &l, &r ) )
  }
}

//

crate::mod_interface!
{
  /// Publishes the difference between the local and published versions of a package.
  orphan use publish_diff;
}
