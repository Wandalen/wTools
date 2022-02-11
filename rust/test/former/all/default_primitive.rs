
use wtest_basic::dependencies::*;

#[cfg( feature = "in_wtools" )]
use wtools::meta::*;
#[cfg( not( feature = "in_wtools" ) )]
use meta_tools::*;

#[cfg( feature = "in_wtools" )]
use wtools::former::Former;
#[cfg( not( feature = "in_wtools" ) )]
use former::Former;

use std::collections::HashMap;
use std::collections::HashSet;

#[derive( Debug, PartialEq, Former )]
pub struct Struct1
{
  #[ default( 31 ) ]
  pub int_1 : i32,
  #[ default( "abc" ) ]
  string_1 : String,
  #[ default( 31 ) ]
  int_optional_1 : Option< i32 >,
  #[ default( "abc" ) ]
  string_optional_1 : Option< String >,

  vec_1 : Vec< String >,
  hashmap_strings_1 : HashMap< String, String >,
  hashset_strings_1 : HashSet< String >,
}

//

fn test_complex() -> anyhow::Result< () >
{

  let command = Struct1::former().form();

  let expected = Struct1
  {
    int_1 : 31,
    string_1 : "abc".to_string(),
    int_optional_1 : Some( 31 ),
    string_optional_1 : Some( "abc".to_string() ),
    vec_1 : vec![],
    hashmap_strings_1 : hmap!{},
    hashset_strings_1 : hset!{},
  };
  assert_eq!( command, expected );

  Ok( () )
}

//

#[ test ]
fn main_test()
{
  test_complex().unwrap();
}

//

// #[derive( Debug, PartialEq, Default )]
// struct UserType
// {
//   int : i32,
//   uint : u32,
// }
//
// #[derive( Debug, PartialEq, Former )]
// struct Struct2
// {
//   user : UserType,
//   string : String,
// }
//
// //
//
// fn test_user_type_with_default() -> anyhow::Result< () >
// {
//   let command = Struct2::former().form();
//
//   let expected = Struct2
//   {
//     user : UserType { int : 0, uint : 0 },
//     string : String::from( "" ),
//   };
//
//   assert_eq!( command, expected );
//
//   Ok( () )
// }
//
// #[ test ]
// fn user_type_test()
// {
//   test_complex().unwrap();
// }
