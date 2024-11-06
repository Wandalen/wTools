/// Define a private namespace for all its items.
mod private
{
  #[ allow( unused_imports ) ]
  use crate::tool::*;

  use std::
  {
    io::Read,
    fmt::Write,
    time::Duration
  };
  use error::{ untyped::Context };
  use ureq::Agent;

  ///
  /// Get data of remote package.
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
  orphan use download;
}
