#[ allow( unused_imports ) ]
use super::*;

only_for_aggregating_module!
{
  #[ allow( unused_imports ) ]
  use wtools::meta::*;
  #[ allow( unused_imports ) ]
  use wtools::former::Former;
}

only_for_terminal_module!
{
  #[ allow( unused_imports ) ]
  use meta_tools::*;
  #[ allow( unused_imports ) ]
  use former::Former;
}

#[allow(dead_code)]
type Option = ();
#[allow(dead_code)]
type Some = ();
#[allow(dead_code)]
type None = ();
#[allow(dead_code)]
type Result = ();
#[allow(dead_code)]
type Ok = ();
#[allow(dead_code)]
type Err = ();
#[allow(dead_code)]
type Box = ();
#[allow(dead_code)]
type Default = ();
#[allow(dead_code)]
type HashSet = ();
#[allow(dead_code)]
type HashMap = ();

#[derive( Debug, PartialEq, Former )]
pub struct Struct1
{
  pub int_1 : i32,
  string_1 : String,
  vec_1 : Vec< String >,
  int_optional_1 : core::option::Option< i32 >,
  string_optional_1 : core::option::Option< String >,
  hashmap_strings_1 : std::collections::HashMap< String, String >,
  hashset_strings_1 : std::collections::HashSet< String >,
}

//

include!( "basic_only_test.rs" );
