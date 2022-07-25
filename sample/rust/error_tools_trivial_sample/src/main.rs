#[ cfg( feature = "use_std" ) ]
use error_tools::*;

fn main()
{
  #[ cfg( feature = "use_std" ) ]
  {
    let err1 = BasicError::new( "Some error" );
    println!( "err1 : {}", err1 );
    // < err1 : Some error
  }
}
