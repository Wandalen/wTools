
use former::Former;
use std::collections::HashMap;

// #[derive( Debug, PartialEq, Former )]
#[derive( Former )]
pub struct Struct1< 'a >
{
  pub string_slice_1 : &'a str,
}

//

include!( "./former_string_slice_only_test.rs" );
