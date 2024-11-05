use super::*;

mod mod1
{

  #[ derive( Debug, PartialEq, Eq ) ]
  pub struct Struct1( pub i32, pub bool );

  impl Struct1
  {
    #[ inline( always ) ]
    pub fn new( src1 : i32, src2 : bool ) -> Self
    {
      Self( src1, src2 )
    }
  }

}

include!( "./only_test/multiple_unnamed.rs" );
