use super::*;

mod mod1
{

  #[ derive( Debug, Clone, Copy, PartialEq ) ]
  pub struct Struct1( pub bool );

  impl Struct1
  {
    #[ inline( always ) ]
    pub fn new( src : bool ) -> Self
    {
      Self( src )
    }
  }

}

include!( "./only_test/basic.rs" );
