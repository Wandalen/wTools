#[ allow( unused_imports ) ]
use super::*;

#[ derive( Debug, PartialEq, the_module::Former ) ]
pub struct Struct1
{
  pub int_1 : i32,
  string_1 : String,
  int_optional_1 : core::option::Option< i32 >,
  string_optional_1 : Option< String >,
}

//

// xxx : uncomment
// include!( "./only_test/primitives.rs" );
