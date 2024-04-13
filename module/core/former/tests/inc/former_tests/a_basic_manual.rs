#[ allow( unused_imports ) ]
use super::*;

#[ derive( Debug, PartialEq ) ]
pub struct Struct1
{
  pub int_1 : i32,
}

// = formed

//

include!( "./only_test/basic.rs" );
