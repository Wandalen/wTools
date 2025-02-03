use super::*;

mod mod1
{
  use super::*;

  #[ derive( Debug, PartialEq, Eq, the_module::New ) ]
  pub struct Struct1( pub i32, pub bool );

}

include!( "./only_test/multiple_unnamed.rs" );
