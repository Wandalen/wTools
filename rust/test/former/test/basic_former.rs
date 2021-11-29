
// #[cfg( feature = "all" )]
// use wtools::former;
// use wtools::former::Former;
// #[cfg( not( feature = "all" ) )]
// use former_derive::Former;
use former::Former;

#[derive( Debug, PartialEq, Former )]
pub struct Command
{
  pub int_1 : i32,
  string_1 : String,
  vec_1 : Vec< String >,
  hashmap_strings_1 : std::collections::HashMap< String, String >,
  int_optional_1 : core::option::Option< i32 >,
  string_optional_1 : Option< String >,
}

//

include!( "basic_test.rs" );
