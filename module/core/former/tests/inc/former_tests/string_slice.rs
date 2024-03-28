use super::*;

#[ derive( Debug, PartialEq, the_module::Former ) ]
pub struct Struct1< 'a >
{
  pub string_slice_1 : &'a str,
}

//

include!( "./only_test/string_slice.rs" );
