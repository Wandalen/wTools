#[ allow( unused_imports ) ]
use super::*;

only_for_wtools!
{
  #[ allow( unused_imports ) ]
  use wtools::meta::*;
  #[ allow( unused_imports ) ]
  use wtools::former::Former;
}

only_for_local_module!
{
  #[ allow( unused_imports ) ]
  use meta_tools::*;
  #[ allow( unused_imports ) ]
  use former::Former;
}

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
