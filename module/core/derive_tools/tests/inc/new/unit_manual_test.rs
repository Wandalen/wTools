use super::*;

mod mod1
{

  #[ derive( Debug, Clone, Copy, PartialEq ) ]
  pub struct Struct1;

  impl Struct1
  {
    #[ inline( always ) ]
    pub fn new() -> Self
    {
      Self
    }
  }

}

include!( "./only_test/unit.rs" );
