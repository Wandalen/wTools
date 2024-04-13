use super::*;

#[ derive( Debug, PartialEq, the_module::Former ) ]
// #[ derive( Debug, PartialEq, the_module::Former ) ] #[ debug ]
pub struct Struct1< 'a >
{
  pub string_slice_1 : &'a str,
}

//

// include!( "./only_test/string_slice.rs" );
// xxx : uncomment
