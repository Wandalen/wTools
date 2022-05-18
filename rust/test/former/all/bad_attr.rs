use super::*;

#[cfg( feature = "in_wtools" )]
use wtools::former::Former;
#[cfg( not( feature = "in_wtools" ) )]
use former::Former;

#[derive( Former )]
pub struct Struct1
{
  #[ defaultx( 31 ) ]
  int_1 : i32,
}

fn main()
{}
