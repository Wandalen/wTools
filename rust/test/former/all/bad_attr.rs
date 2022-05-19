
#[cfg( feature = "wtools_alias" )]
use wtools_alias::former as former;

use former::Former;

#[derive( Former )]
pub struct Struct1
{
  #[ defaultx( 31 ) ]
  int_1 : i32,
}

fn main()
{}
