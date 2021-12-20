
use former::Former;

#[derive( Former )]
pub struct Struct1
{
  #[former( defaultx = 31 )]
  int_1 : i32,
}

fn main()
{}