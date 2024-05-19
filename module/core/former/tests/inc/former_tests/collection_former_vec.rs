// #![ allow( dead_code ) ]

use super::*;
#[ allow( unused_imports ) ]
use collection_tools::Vec;

//

#[ test ]
fn add()
{

  // expliccit with CollectionFormer

  let got : Vec< String > = the_module
  ::CollectionFormer
  ::< String, former::VectorDefinition< String, (), Vec< String >, the_module::ReturnStorage > >
  ::new( former::ReturnStorage )
  .add( "a" )
  .add( "b" )
  .form();
  let exp = vec!
  [
    "a".to_string(),
    "b".to_string(),
  ];
  a_id!( got, exp );

  // expliccit with VectorFormer

  let got : Vec< String > = the_module::VectorFormer::< String, (), Vec< String >, the_module::ReturnStorage >
  ::new( former::ReturnStorage )
  .add( "a" )
  .add( "b" )
  .form();
  let exp = vec!
  [
    "a".to_string(),
    "b".to_string(),
  ];
  a_id!( got, exp );

  // compact with VectorFormer

  let got : Vec< String > = the_module::VectorFormer::new( former::ReturnStorage )
  .add( "a" )
  .add( "b" )
  .form();
  let exp = vec!
  [
    "a".to_string(),
    "b".to_string(),
  ];
  a_id!( got, exp );

  // with begin_coercing

  let got : Vec< String > = the_module::VectorFormer
  ::begin( Some( vec![ "a".to_string() ] ), Some( () ), former::ReturnStorage )
  .add( "b" )
  .form();
  let exp = vec!
  [
    "a".to_string(),
    "b".to_string(),
  ];
  a_id!( got, exp );

  // with help of ext

  use the_module::VecExt;
  let got : Vec< String > = Vec::former()
  .add( "a" )
  .add( "b" )
  .form();
  let exp = vec!
  [
    "a".to_string(),
    "b".to_string(),
  ];
  a_id!( got, exp );

  //

}

//

#[ test ]
fn replace()
{

  let got : Vec< String > = the_module::VectorFormer::new( former::ReturnStorage )
  .add( "x" )
  .replace( vec![ "a".to_string(), "b".to_string() ] )
  .form();
  let exp = vec!
  [
    "a".to_string(),
    "b".to_string(),
  ];
  a_id!( got, exp );

}

//

// qqq : make similar test for all collections
#[ test ]
fn entity_to()
{

  // qqq : uncomment and make it working
  // let got = < Vec< i32 > as former::EntityToFormer< former::VectorDefinition< i32, (), Vec< i32 >, former::ReturnPreformed > > >
  // ::Former::new( former::ReturnPreformed )
  // .add( 13 )
  // .form();
  // let exp = vec![ 13 ];
  // a_id!( got, exp );

// xxx
// qqq : uncomment and make it working
//   let got = < Vec< i32 > as former::EntityToStorage >::Storage::default();
//   let exp =
//   <
//     Vec< i32 > as former::EntityToFormer
//     <
//       Vec< i32 >FormerDefinition< (), Vec< i32 >, former::ReturnPreformed >
//     >
//   >::Former::new( former::ReturnPreformed );
//   a_id!( got.int_1, exp.storage.int_1 );
//
//   let got = < Vec< i32 > as former::EntityToStorage >::Storage::default();
//   let exp =
//   <
//     Vec< i32 > as former::EntityToFormer
//     <
//       < Vec< i32 > as former::EntityToDefinition< (), Vec< i32 >, former::ReturnPreformed > >::Definition
//     >
//   >::Former::new( former::ReturnPreformed );
//   a_id!( got.int_1, exp.storage.int_1 );

}

#[ test ]
fn entry_to_val()
{
  let got = former::EntryToVal::< Vec< i32 > >::entry_to_val( 13i32 );
  let exp = 13i32;
  a_id!( got, exp )
}

#[ test ]
fn val_to_entry()
{
  let got = former::ValToEntry::< Vec< i32 > >::val_to_entry( 13i32 );
  let exp = 13i32;
  a_id!( got, exp )
}
