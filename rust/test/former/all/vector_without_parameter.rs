
#[cfg( feature = "in_wtools" )]
use wtools::former::Former;
#[cfg( not( feature = "in_wtools" ) )]
use former::Former;

struct Vec
{
}

#[derive( Former )]
pub struct Struct1
{
  pub string_slice_1 : Vec<>,
}

fn main()
{
}
