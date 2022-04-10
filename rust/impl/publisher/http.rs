#![ warn( missing_docs ) ]
#![ warn( missing_debug_implementations ) ]

//!
//! Work with crate on `crates.io`.
//!

use ureq::Agent;
use std::time::Duration;
use std::fmt::Write;
use std::io::Read;

///
/// Get data of remote package.
///

pub fn retrieve_bytes<'a>( name : &'a str, version : &'a str ) -> anyhow::Result<Vec<u8>>
{
  let agent: Agent = ureq::AgentBuilder::new()
  .timeout_read( Duration::from_secs( 5 ) )
  .timeout_write( Duration::from_secs( 5 ) )
  .build();
  let mut buf = String::new();
  write!( &mut buf, "https://static.crates.io/crates/{0}/{0}-{1}.crate", name, version )?;

  let resp = agent.get( &buf[ .. ] ).call()?;

  let len: usize = resp.header( "Content-Length" )
  .unwrap()
  .parse()?;

  let mut bytes: Vec<u8> = Vec::with_capacity( len );
  resp.into_reader()
  .take( u64::MAX )
  .read_to_end( &mut bytes )?;
  Ok( bytes )
}
