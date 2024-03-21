mod private
{
  use std::path::PathBuf;
  use crate::*;

  use { wtools };
  use crates_tools::CrateArchive;

  use wca::Args;
  use wtools::error::Result;
  use crate::_path::AbsolutePath;

  ///
  ///
  ///

  pub fn publish_diff( args : Args ) -> Result< () >
  {
    let path : PathBuf = args.get_owned( 0 ).unwrap_or( std::env::current_dir()? );

    let path = AbsolutePath::try_from( path )?;
    let dir = CrateDir::try_from( path )?;

    let package = package::Package::try_from( dir.clone() )?;
    let name = &package.name()?;
    let version = &package.version()?;

    _ = cargo::pack( cargo::PackOptions::former().path( dir.as_ref() ).dry( false ).form() )?;
    let l = CrateArchive::read( packed_crate::local_path( name, version, dir )? )?;
    let r = CrateArchive::download_crates_io( name, version ).unwrap();

    println!( "{}", package::crate_diff( &l, &r ) );

    Ok( () )
  }
}

//

crate::mod_interface!
{
  orphan use publish_diff;
}
