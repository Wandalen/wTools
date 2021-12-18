
use former::Former;

#[derive( Debug, PartialEq, Former )]
// #[derive( Former )]
pub struct Struct1< 'a >
{
  pub string_slice_1 : &'a str,
}

//

include!( "./string_slice_only_test.rs" );
