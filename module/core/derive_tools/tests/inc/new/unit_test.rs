use super::*;

mod mod1
{
  use super::*;

  #[ derive( Debug, Clone, Copy, PartialEq, the_module::New ) ]
  pub struct Struct1;

}

include!( "./only_test/unit.rs" );
