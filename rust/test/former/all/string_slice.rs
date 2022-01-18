
#[cfg( feature = "in_wtools" )]
use wtools::former::Former;
#[cfg( not( feature = "in_wtools" ) )]
use former::Former;

#[derive( Debug, PartialEq, Former )]
pub struct Struct1< 'a >
{
  pub string_slice_1 : &'a str,
}

//

include!( "./string_slice_only_test.rs" );
