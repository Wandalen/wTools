#[ allow( clippy::std_instead_of_alloc, clippy::std_instead_of_core ) ]
mod private
{
  #[ allow( clippy::wildcard_imports ) ]
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

  /// Constructs the expected local path for a packed `.crate` file within a target directory.
  ///
  /// This is a utility function that builds a predictable path without verifying
  /// if the file actually exists. It follows the standard Cargo packaging structure.
  ///
  /// # Arguments
  ///
  /// - `name` - The name of the package.
  /// - `version` - The version of the package.
  /// - `target_dir` - The path to the workspace's `target` directory, inside which
  ///   the `package/` subdirectory is expected.
  ///
  /// # Returns
  ///
  /// Returns a `Result` containing a `PathBuf` that points to the expected location of the `.crate` file,
  /// for example: `<target_dir>/package/my_package-0.1.0.crate`.
  ///
  /// # Errors
  ///
  /// This function is currently infallible as it only performs path joining and string formatting.
  /// The `Result` is kept for API consistency.
  // qqq : typed error
  pub fn local_path< 'a >( name : &'a str, version : &'a str, target_dir : &std::path::Path ) -> error::untyped::Result< PathBuf >
  {
    let buf = format!( "package/{name}-{version}.crate" );
    let local_package_path = target_dir.join( buf );
    Ok( local_package_path )

  }

  ///
  /// Get data of remote package from crates.io.
  ///
  /// # Errors
  /// qqq: doc
  ///
  /// # Panics
  /// qqq: doc
  // qqq : typed error
  pub fn download< 'a >( name : &'a str, version : &'a str ) -> error::untyped::Result< Vec< u8 > >
  {
    let agent : Agent = ureq::AgentBuilder::new()
    .timeout_read( Duration::from_secs( 5 ) )
    .timeout_write( Duration::from_secs( 5 ) )
    .build();
    let mut buf = String::new();
    write!( &mut buf, "https://static.crates.io/crates/{name}/{name}-{version}.crate" )?;

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
