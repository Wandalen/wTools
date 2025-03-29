#![ allow( dead_code ) ]
#![ allow( non_camel_case_types ) ]
#![ allow( non_snake_case ) ]

#[ allow( unused_imports ) ]
use super::*;

// #[ allow( dead_code ) ]
// type Option = ();
// #[ allow( dead_code ) ]
// type Some = ();
// #[ allow( dead_code ) ]
// type None = ();
// #[ allow( dead_code ) ]
// type Result = ();
// #[ allow( dead_code ) ]
// type Ok = ();
// #[ allow( dead_code ) ]
// type Err = ();
// #[ allow( dead_code ) ]
// type Box = ();
// #[ allow( dead_code ) ]
// type Default = ();
// #[ allow( dead_code ) ]
// type HashSet = ();
// #[ allow( dead_code ) ]
// type HashMap = ();

// pub mod core {}
// pub mod std {}
// pub mod marker {}

pub struct core{}
pub struct std{}
pub struct marker{}
pub struct CloneAny{}
pub struct Context{}
pub struct Formed{}
pub struct OnEnd{}
pub struct Option{}
pub struct None{}
pub struct Some{}
pub struct Into{}
pub struct From{}
pub struct Default{}
pub struct Vec{}
pub struct HashSet{}
pub struct HashMap{}

pub fn std(){}
pub fn marker(){}
pub fn CloneAny(){}
pub fn Context(){}
pub fn Formed(){}
pub fn OnEnd(){}
pub fn Option(){}
pub fn None(){}
pub fn Some(){}
pub fn Into(){}
pub fn From(){}
pub fn Default(){}
pub fn Vec(){}
pub fn HashSet(){}
pub fn HashMap(){}

// // #[ derive( Clone ) ]
// #[ derive( Clone, the_module::Former ) ]
// #[ debug ]
// pub struct core
// {
//   inner : ::std::sync::Arc< ::core::cell::RefCell< dyn ::core::convert::AsRef< i32 > > >,
//   i : ::std::option::Option< i32 >,
// }

#[ derive( PartialEq, Debug, the_module::Former ) ]
// #[ debug ]
pub struct Struct1
{
  vec_1 : collection_tools::Vec< String >,
  hashmap_1 : collection_tools::HashMap< String, String >,
  hashset_1 : collection_tools::HashSet< String >,
  // inner : ::std::sync::Arc< ::core::cell::RefCell< dyn ::core::convert::AsRef< i32 > > >,
  i : ::core::option::Option< i32 >,
}

#[ test ]
fn test_vector()
{

  // test.case( "vector : construction" );

  let command = Struct1::former()
  .vec_1( ::collection_tools::vec![ "ghi".to_string(), "klm".to_string() ] )
  // .inner()
  .form()
  ;
  // dbg!( &command );

  let expected = Struct1
  {
    vec_1 : ::collection_tools::vec![ "ghi".to_string(), "klm".to_string() ],
    hashmap_1 : ::collection_tools::hmap!{},
    hashset_1 : ::collection_tools::hset!{},
    // inner : ::std::sync::Arc::new( ::core::cell::RefCell::new( &0 ) ),
    i : ::core::option::Option::None,
  };
  a_id!( command, expected );
}
