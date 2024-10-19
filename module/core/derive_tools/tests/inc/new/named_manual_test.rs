use super::*;

mod mod1
{

  #[ derive( Debug, PartialEq, Eq ) ]
  pub struct Struct1
  {
    pub a : i32,
  }

  impl Struct1
  {
    #[ inline( always ) ]
    pub fn new( src : i32 ) -> Self
    {
      Self{ a : src }
    }
  }

}

include!( "./only_test/named.rs" );
