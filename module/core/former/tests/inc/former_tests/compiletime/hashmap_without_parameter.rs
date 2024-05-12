use former::Former;

struct HashMap< T >
{
  f1 : T,
}

#[ derive( Former ) ]
pub struct Struct1
{
  f2 : HashMap< i32 >,
}

fn main()
{
}
