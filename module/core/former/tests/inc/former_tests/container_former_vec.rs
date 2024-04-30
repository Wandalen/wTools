// #![ allow( dead_code ) ]

use super::*;
#[ allow( unused_imports ) ]
use collection_tools::Vec;

//

#[ test ]
fn add()
{

  // expliccit with ContainerSubformer

  let got : Vec< String > = the_module
  ::ContainerSubformer
  ::< String, former::VectorDefinition< String, (), Vec< String >, the_module::ReturnStorage > >
  ::new_precise( former::ReturnStorage )
  .add( "a" )
  .add( "b" )
  .form();
  let exp = vec!
  [
    "a".to_string(),
    "b".to_string(),
  ];
  a_id!( got, exp );

  // expliccit with VectorSubformer

  let got : Vec< String > = the_module::VectorSubformer::< String, (), Vec< String >, the_module::ReturnStorage >
  ::new_precise( former::ReturnStorage )
  .add( "a" )
  .add( "b" )
  .form();
  let exp = vec!
  [
    "a".to_string(),
    "b".to_string(),
  ];
  a_id!( got, exp );

  // compact with VectorSubformer

  let got : Vec< String > = the_module::VectorSubformer::new_precise( former::ReturnStorage )
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

  let got : Vec< String > = the_module::VectorSubformer
  ::begin_precise( Some( vec![ "a".to_string() ] ), Some( () ), former::ReturnStorage )
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

  let got : Vec< String > = the_module::VectorSubformer::new_precise( former::ReturnStorage )
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

// qqq : make similar test for all containers
#[ test ]
fn entity_to()
{

  // qqq : uncomment and make it working
  // let got = < Vec< i32 > as former::EntityToFormer< former::VectorDefinition< i32, (), Vec< i32 >, former::ReturnPreformed > > >
  // ::Former::new_precise( former::ReturnPreformed )
  // .add( 13 )
  // .form();
  // let exp = vec![ 13 ];
  // a_id!( got, exp );

// qqq : uncomment and make it working
//   let got = < Vec< i32 > as former::EntityToStorage >::Storage::default();
//   let exp =
//   <
//     Vec< i32 > as former::EntityToFormer
//     <
//       Vec< i32 >FormerDefinition< (), Vec< i32 >, former::ReturnPreformed >
//     >
//   >::Former::new_precise( former::ReturnPreformed );
//   a_id!( got.int_1, exp.storage.int_1 );
//
//   let got = < Vec< i32 > as former::EntityToStorage >::Storage::default();
//   let exp =
//   <
//     Vec< i32 > as former::EntityToFormer
//     <
//       < Vec< i32 > as former::EntityToDefinition< (), Vec< i32 >, former::ReturnPreformed > >::Definition
//     >
//   >::Former::new_precise( former::ReturnPreformed );
//   a_id!( got.int_1, exp.storage.int_1 );

}
