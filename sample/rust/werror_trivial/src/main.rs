use werror::*;

fn main()
{
  let err1 = Error::new( "Some error" );
  println!( "err1 : {}", err1 );
  // < err1 : Some error
}
