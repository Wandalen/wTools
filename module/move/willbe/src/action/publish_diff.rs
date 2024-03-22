/// Internal namespace.
mod private
{
  use crate::*;

  use std::path::PathBuf;
  use crates_tools::CrateArchive;

  use _path::AbsolutePath;
  use wtools::error::for_app::Result;
  use diff::{ DiffReport, crate_diff };

  /// Options for `publish_diff` command
  #[ derive( Debug, former::Former ) ]
  pub struct PublishDiffOptions
  {
    path : PathBuf,
    keep_archive : Option< PathBuf >,
  }

  /// Return the differences between a local and remote package versions.
  #[ cfg_attr( feature = "tracing", tracing::instrument ) ]
  pub fn publish_diff( o : PublishDiffOptions ) -> Result< DiffReport >
  {
    let path = AbsolutePath::try_from( o.path )?;
    let dir = CrateDir::try_from( path )?;

    let package = package::Package::try_from( dir.clone() )?;
    let name = &package.name()?;
    let version = &package.version()?;

    _ = cargo::pack( cargo::PackOptions::former().path( dir.as_ref() ).dry( false ).form() )?;
    let l = CrateArchive::read( packed_crate::local_path( name, version, dir )? )?;
    let r = CrateArchive::download_crates_io( name, version ).unwrap();

    if let Some( out_path ) = o.keep_archive
    {
      _ = std::fs::create_dir_all( &out_path );
      for path in r.list()
      {
        let local_path = out_path.join( path );
        let folder = local_path.parent().unwrap();
        _ = std::fs::create_dir_all( folder );

        let content = r.content_bytes( path ).unwrap();

        std::fs::write( local_path, content )?;
      }
    }

    Ok( crate_diff( &l, &r ) )
  }
}

//

crate::mod_interface!
{
  orphan use PublishDiffOptions;
  /// Publishes the difference between the local and published versions of a package.
  orphan use publish_diff;
}
