#![ deny( missing_docs ) ]

#[ allow( unused_imports ) ]
use super::*;

// xxx : qqq : make that working

// use collection_tools::HashMap;
//
// type Key = &'static str;
// type Value = &'static str;
//
// #[ derive( Debug, PartialEq, former::Former ) ]
// pub struct Struct1( #[ subform_collection ] HashMap< Key, Value > );
//
// impl Struct1
// {
//   pub fn get( &self, key : Key ) -> Option< &Value >
//   {
//     self.0.get( key )
//   }
// }
//
// #[ test ]
// fn example()
// {
//   // form a key-value store
//   let instance = Struct1::former()
//   .map()
//     .add( ( "first", "Value1" ) )
//     .add( ( "second", "Value2" ) )
//     .end()
//   .form();
//
//   // now it is a read-only storage with pre-configured data
//   assert_eq!( Some( &"Value1" ), instance.get( "first" ) );
// }
