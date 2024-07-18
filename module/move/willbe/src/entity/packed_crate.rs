mod private
{
  use crate::*;

  use std::
  {
    io::Read,
    fmt::Write,
    time::Duration,
    path::PathBuf,
  };
  use error::{ untyped::Context };
  use ureq::Agent;

  /// Returns the local path of a packed `.crate` file based on its name, version, and manifest path.
  ///
  /// # Args :
  /// - `name` - the name of the package.
  /// - `version` - the version of the package.
  /// - `manifest_file` - path to the package `Cargo.toml` file.
  ///
  /// # Returns :
  /// The local packed `.crate` file of the package
  // qqq : typed error
  pub fn local_path< 'a >( name : &'a str, version : &'a str, crate_dir : CrateDir ) -> error::untyped::Result< PathBuf >
  {
    let buf = format!( "package/{0}-{1}.crate", name, version );
    let workspace = Workspace::try_from( crate_dir )?;

    let mut local_package_path = PathBuf::new();
    local_package_path.push( workspace.target_directory() );
    local_package_path.push( buf );

    Ok( local_package_path )
  }

  ///
  /// Get data of remote package from crates.io.
  ///
  // qqq : typed error
  pub fn download< 'a >( name : &'a str, version : &'a str ) -> error::untyped::Result< Vec< u8 > >
  {
    let agent : Agent = ureq::AgentBuilder::new()
    .timeout_read( Duration::from_secs( 5 ) )
    .timeout_write( Duration::from_secs( 5 ) )
    .build();
    let mut buf = String::new();
    write!( &mut buf, "https://static.crates.io/crates/{0}/{0}-{1}.crate", name, version )?;

    let resp = agent.get( &buf[ .. ] ).call().context( "Get data of remote package" )?;

    let len : usize = resp.header( "Content-Length" )
    .unwrap()
    .parse()?;

    let mut bytes : Vec< u8 > = Vec::with_capacity( len );
    resp.into_reader()
    .take( u64::MAX )
    .read_to_end( &mut bytes )?;

    Ok( bytes )
  }

}

//

crate::mod_interface!
{

  own use local_path;
  own use download;

}
