
fn main()
{
  #[ cfg( not( feature = "no_std" ) ) ]
  {
    use winterval::*;

    let src = 2..5;
    assert_eq!( src.closed(), ( 2, 4 ) );

    let src = 2..=4;
    assert_eq!( src.closed(), ( 2, 4 ) );
  }
}
