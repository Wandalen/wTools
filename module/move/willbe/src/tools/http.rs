/// Internal namespace.
pub( crate ) mod private
{
  use crate::*;
  use wtools::error;
  use wtools::error::for_app::Context;
  use ureq::Agent;
  use std::time::Duration;
  use core::fmt::Write;
  use std::io::Read;

  ///
  /// Get data of remote package.
  ///

  pub fn retrieve_bytes< 'a >( name : &'a str, version : &'a str ) -> error::for_app::Result< Vec< u8 > >
  {
    let agent: Agent = ureq::AgentBuilder::new()
    .timeout_read( Duration::from_secs( 5 ) )
    .timeout_write( Duration::from_secs( 5 ) )
    .build();
    let mut buf = String::new();
    write!( &mut buf, "https://static.crates.io/crates/{0}/{0}-{1}.crate", name, version )?;

    let resp = agent.get( &buf[ .. ] ).call().context( "Get data of remote package" )?;

    let len: usize = resp.header( "Content-Length" )
    .unwrap()
    .parse()?;

    let mut bytes: Vec< u8 > = Vec::with_capacity( len );
    resp.into_reader()
    .take( u64::MAX )
    .read_to_end( &mut bytes )?;

    Ok( bytes )
  }
}

//

crate::mod_interface!
{
  orphan use retrieve_bytes;
}
