
use former::Options;

// #[ derive( Debug ) ]
// #[ derive( Former ) ]
// #[ perform( fn split( self ) -> SplitIterator< 'a > ) ]
#[ Options ]
fn SomeOptions()
{

  int1 : i32;
  #[ default( 13 ) ]
  int2 : i32;

  #[ default( 13 ) ]
  pub fn f2( self ) -> i32;
  fn f3( self ) -> i32;

  #[ method ]
  fn f1( self ) -> i32
  {
    13
  };

  fn f2( self ) -> i32
  {
    13
  };

}

//

fn basic() -> anyhow::Result< () >
{

  // let options = SomeOptions { int1 : 31 };
  // let got = options.f1();
  // let exp = 13;
  // assert_eq!( got, exp );

  // let options = SomeOptions { int1 : 31 };
  // let got = options.int1();
  // let exp = 31;
  // assert_eq!( got, exp );

  Ok( () )
}

//

#[ test ]
fn main_test() -> anyhow::Result< () >
{
  basic()?;
  Ok( () )
}
