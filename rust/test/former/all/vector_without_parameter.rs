#[cfg( feature = "wtools_alias" )]
use wtools_alias::former as former;

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
