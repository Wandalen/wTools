#![ allow( dead_code ) ]

#[ allow( unused_imports ) ]
use super::*;
#[ allow( unused_imports ) ]
use collection_tools::HashMap;

// qqq : zzz : remove #[ cfg( not( feature = "use_alloc" ) ) ]
#[ cfg( not( feature = "use_alloc" ) ) ]
#[ test ]
fn add()
{

  // expliccit with ContainerSubformer

  let got : HashMap< String, String > = the_module
  ::ContainerSubformer
  ::< ( String, String ), former::HashMapDefinition< String, String, (), HashMap< String, String >, the_module::ReturnStorage > >
  ::new_precise( former::ReturnStorage )
  .add( ( "a".into(), "x".into() ) )
  .add( ( "b".into(), "y".into() ) )
  .form();
  let exp = hmap!
  [
    "a".to_string() => "x".to_string(),
    "b".to_string() => "y".to_string(),
  ];
  a_id!( got, exp );

  // expliccit with HashMapSubformer

  let got : HashMap< String, String > = the_module::HashMapSubformer::< String, String, (), HashMap< String, String >, the_module::ReturnStorage >
  ::new_precise( former::ReturnStorage )
  .add( ( "a".into(), "x".into() ) )
  .add( ( "b".into(), "y".into() ) )
  .form();
  let exp = hmap!
  [
    "a".to_string() => "x".to_string(),
    "b".to_string() => "y".to_string(),
  ];
  a_id!( got, exp );

  // compact with HashMapSubformer

  let got : HashMap< String, String > = the_module::HashMapSubformer::new_precise( former::ReturnStorage )
  .add( ( "a".into(), "x".into() ) )
  .add( ( "b".into(), "y".into() ) )
  .form();
  let exp = hmap!
  [
    "a".to_string() => "x".to_string(),
    "b".to_string() => "y".to_string(),
  ];
  a_id!( got, exp );

  // with begin_precise

  let got : HashMap< String, String > = the_module::HashMapSubformer
  ::begin_precise( Some( hmap![ "a".to_string() => "x".to_string() ] ), Some( () ), former::ReturnStorage )
  .add( ( "b".into(), "y".into() ) )
  .form();
  let exp = hmap!
  [
    "a".to_string() => "x".to_string(),
    "b".to_string() => "y".to_string(),
  ];
  a_id!( got, exp );

  // with help of ext

  use the_module::HashMapExt;
  let got : HashMap< String, String > = HashMap::former()
  .add( ( "a".into(), "x".into() ) )
  .add( ( "b".into(), "y".into() ) )
  .form();
  let exp = hmap!
  [
    "a".to_string() => "x".to_string(),
    "b".to_string() => "y".to_string(),
  ];
  a_id!( got, exp );

  //

}

// qqq : zzz : remove #[ cfg( not( feature = "use_alloc" ) ) ]
#[ cfg( not( feature = "use_alloc" ) ) ]
#[ test ]
fn replace()
{

  let got : HashMap< String, String > = the_module::HashMapSubformer::new_precise( former::ReturnStorage )
  .add( ( "x".to_string(), "y".to_string() ) )
  .replace( hmap![ "a".to_string() => "x".to_string(), "b".to_string() => "y".to_string(), ] )
  .form();
  let exp = hmap!
  [
    "a".to_string() => "x".to_string(),
    "b".to_string() => "y".to_string(),
  ];
  a_id!( got, exp );

}

// xxx
// #[ test ]
// fn entity_to()
// {
//
//   let got = < Vec< i32 > as former::EntityToFormer< former::VectorDefinition< i32, (), Vec< i32 >, former::ReturnPreformed > > >
//   ::Former::new_precise( former::ReturnPreformed )
//   .add( 13 )
//   .form();
//   let exp = vec![ 13 ];
//   a_id!( got, exp );
//
// }
