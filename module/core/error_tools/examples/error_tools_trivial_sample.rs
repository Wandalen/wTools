fn main()
{
  #[ cfg( not( feature = "no_std" ) ) ]
  {
    use error_tools::*;
    let err1 = BasicError::new( "Some error" );
    println!( "err1 : {}", err1 );
    // < err1 : Some error
  }
}
