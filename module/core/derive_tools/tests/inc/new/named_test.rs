use super::*;

mod mod1
{
  use super::*;

  #[ derive( Debug, PartialEq, Eq, the_module::New ) ]
  pub struct Struct1
  {
    pub a : i32,
  }

}

include!( "./only_test/named.rs" );
