#![ cfg_attr( feature = "no_std", no_std ) ]
#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ doc( html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico" ) ]
#![ doc( html_root_url = "https://docs.rs/wpublisher/" ) ]
// #![ deny( rust_2018_idioms ) ]
// #![ deny( missing_debug_implementations ) ]
// #![ deny( missing_docs ) ]

//!
//! Utility to publish modules on `crates.io` from a command line.
//!

#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/", "Readme.md" ) ) ]

#[ allow( unused_imports ) ]
use ::wpublisher::*;

//

#[ cfg( not( feature = "no_std" ) ) ]
fn main() -> Result< (), wca::Error >
{
  let args = env::args().skip( 1 ).collect::< Vec< String > >();

  let ca = init::ca().perform();

  if args.is_empty()
  {
    eprintln!( "Ambiguity. Did you mean?" );
    ca.perform( ".help" )?;
    std::process::exit( 1 )
  }
  else
  {
    ca.perform( args )
  }
}

#[ cfg( feature = "no_std" ) ]
fn main()
{
}
