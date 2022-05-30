#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ warn( rust_2018_idioms ) ]
#![ warn( missing_debug_implementations ) ]
#![ warn( missing_docs ) ]

//!
//! Utility to operate files from a command line.
//!

// #![feature( pattern )]

use std::env;
use wcensor::*;
// use wtools::former::Former;

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
