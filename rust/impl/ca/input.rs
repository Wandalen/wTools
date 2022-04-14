
#![allow(dead_code)]

use std::io;
use std::io::Write;

//

pub fn ask( request : &str ) -> String
{
  let mut response = String::new();
  print!( "{} : ", request );
  io::stdout().flush().ok();
  io::stdin().read_line( &mut response ).ok();
  response.trim().to_string()
}

//
