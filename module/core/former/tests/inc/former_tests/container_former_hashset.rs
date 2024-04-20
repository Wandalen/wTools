#![ allow( dead_code ) ]

#[ allow( unused_imports ) ]
use super::*;
#[ allow( unused_imports ) ]
use collection_tools::HashSet;

// qqq : zzz : remove #[ cfg( not( feature = "use_alloc" ) ) ]
#[ cfg( not( feature = "use_alloc" ) ) ]
#[ test ]
fn add()
{

  // expliccit with ContainerSubformer

  let got : HashSet< String > = the_module
  ::ContainerSubformer
  ::< String, former::HashSetDefinition< String, (), HashSet< String >, the_module::ReturnStorage > >
  ::new( former::ReturnStorage )
  .add( "a" )
  .add( "b" )
  .form();
  let exp = hset!
  [
    "a".to_string(),
    "b".to_string(),
  ];
  a_id!( got, exp );

  // expliccit with HashSetSubformer

  let got : HashSet< String > = the_module::HashSetSubformer::< String, (), HashSet< String >, the_module::ReturnStorage >
  ::new( former::ReturnStorage )
  .add( "a" )
  .add( "b" )
  .form();
  let exp = hset!
  [
    "a".to_string(),
    "b".to_string(),
  ];
  a_id!( got, exp );

  // compact with HashSetSubformer

  let got : HashSet< String > = the_module::HashSetSubformer::new( former::ReturnStorage )
  .add( "a" )
  .add( "b" )
  .form();
  let exp = hset!
  [
    "a".to_string(),
    "b".to_string(),
  ];
  a_id!( got, exp );

  // with begin_coercing

  let got : HashSet< String > = the_module::HashSetSubformer
  ::begin_coercing( Some( hset![ "a".to_string() ] ), Some( () ), former::ReturnStorage )
  .add( "b" )
  .form();
  let exp = hset!
  [
    "a".to_string(),
    "b".to_string(),
  ];
  a_id!( got, exp );

  // with help of ext

  use the_module::HashSetExt;
  let got : HashSet< String > = HashSet::former()
  .add( "a" )
  .add( "b" )
  .form();
  let exp = hset!
  [
    "a".to_string(),
    "b".to_string(),
  ];
  a_id!( got, exp );

  //

}

// qqq : zzz : remove #[ cfg( not( feature = "use_alloc" ) ) ]
#[ cfg( not( feature = "use_alloc" ) ) ]
#[ test ]
fn replace()
{

  let got : HashSet< String > = the_module::HashSetSubformer::new( former::ReturnStorage )
  .add( "x" )
  .replace( hset![ "a".to_string(), "b".to_string() ] )
  .form();
  let exp = hset!
  [
    "a".to_string(),
    "b".to_string(),
  ];
  a_id!( got, exp );

}
