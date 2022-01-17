
#[cfg( feature = "in_wtools" )]
use wtools::former::Former;
#[cfg( not( feature = "in_wtools" ) )]
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
