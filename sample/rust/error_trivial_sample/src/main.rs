#[ cfg( feature = "use_std" ) ]
use werror::*;

fn main()
{
  #[ cfg( feature = "use_std" ) ]
  {
    let err1 = Error::new( "Some error" );
    println!( "err1 : {}", err1 );
    // < err1 : Some error
  }
}
