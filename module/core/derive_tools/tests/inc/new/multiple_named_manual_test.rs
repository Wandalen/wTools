use super::*;

mod mod1
{

  #[ derive( Debug, PartialEq, Eq ) ]
  pub struct Struct1
  {
    pub a : i32,
    pub b : bool,
  }

  impl Struct1
  {
    #[ inline( always ) ]
    pub fn new( a : i32, b : bool ) -> Self
    {
      Self{ a, b }
    }
  }

}

include!( "./only_test/multiple_named.rs" );
