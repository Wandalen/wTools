use std::path::PathBuf;
use willbe::*;

fn to_asset_path( path : PathBuf ) -> PathBuf
{
  let mut out = PathBuf::from( "rust/test/willbe/_asset" );
  out.push( path );
  out
}

#[ cfg( feature = "use_std" ) ]
mod tests;
