#![ cfg_attr( feature = "no_std", no_std ) ]
#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ doc( html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico" ) ]
#![ doc( html_root_url = "https://docs.rs/wcensor/latest/wcensor/" ) ]
// #![ deny( rust_2018_idioms ) ]
// #![ deny( missing_debug_implementations ) ]
// #![ deny( missing_docs ) ]

//!
//! Utility to operate files from a command line.
//!

#[ cfg( not( feature = "no_std" ) ) ]
use std::env;
#[ allow( unused_imports ) ]
use ::wcensor::*;

#[ cfg( not( feature = "no_std" ) ) ]
fn main()
{

  let instruction = instruction::parse_from_splits( env::args().skip( 1 ) );
  println!( "{:?}", instruction );

  // let splits : Vec< &str > = "23cd23def".split( &[ "23", "e" ][ .. ] ).collect();
  // dbg!( &splits );

  // let splits : Vec< &str > = ".ab . cd efg"
  // .split_whitespace()
  // .flat_map( | e | e.split( "." ) )
  // .filter( | e | e != &"" )
  // .collect();
  // dbg!( &splits );

}

#[ cfg( feature = "no_std" ) ]
fn main()
{
}
