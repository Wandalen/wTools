use former::Former;

struct Vec
{
  f1 : i32,
}

#[ derive( Former ) ]
pub struct Struct1
{
  f2 : Vec<>,
}

fn main()
{
}
