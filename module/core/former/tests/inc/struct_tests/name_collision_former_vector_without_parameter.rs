#![allow(dead_code)]

use super::*;
use the_module::Former;

pub mod core {}
pub mod std {}
pub mod marker {}
pub trait CloneAny {}
pub trait Context {}
pub trait Formed {}
pub trait OnEnd {}
pub struct None {}
pub struct Some {}

#[derive(Debug, PartialEq)]
struct Vec {
  f1: i32,
}

// xxx : Re-enable when trailing comma issue is fully fixed in macro_tools::generic_params::decompose

// #[derive(Debug, PartialEq, Former)]

#[derive(Debug, PartialEq)]
pub struct Struct1 {
  f2: Vec,
  i: ::std::option::Option<i32>,
}

tests_impls! {

  // Name conflict is not a problem.
  fn basic()
  {

    let got = Struct1::former().f2( Vec { f1 : 3 } ).form();
    let expected = Struct1 { f2 : Vec { f1 : 3 }, i : ::std::option::Option::None };
    a_id!( got, expected );

  }

}

//

// xxx : Re-enable when trailing comma issue is fully fixed in macro_tools::generic_params::decompose
// tests_index! {
//   basic,
// }
