#[cfg( feature = "wtools_alias" )]
use wtools::former as former;

use former::Former;

struct HashMap< T >
{
  f1 : T,
}

#[derive( Former )]
pub struct Struct1
{
  pub string_slice_1 : HashMap< i32 >,
}

fn main()
{
}
