#![ cfg_attr( feature = "no_std", no_std ) ]
#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ doc( html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico" ) ]
#![ doc( html_root_url = "https://docs.rs/wpublisher/" ) ]
// #![ deny( rust_2018_idioms ) ]
// #![ deny( missing_debug_implementations ) ]
// #![ deny( missing_docs ) ]

//!
//! ___.
//!

#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/", "Readme.md" ) ) ]

#[ allow( unused_imports ) ]
use ::willbe_old::*;

//

#[ cfg( not( feature = "no_std" ) ) ]
fn main() -> error_tools::Result< () >
{
  let args = std::env::args().skip( 1 ).collect::< Vec< String > >();

  let ca = wca::CommandsAggregator::former()
  .grammar( commands::grammar_form() )
  .executor( commands::executor_form() )
  .perform();

  Ok( ca.perform( if args.is_empty() { "".to_owned() } else { args.join( " " ) + " .end" } )? )
}

#[ cfg( feature = "no_std" ) ]
fn main() {}
