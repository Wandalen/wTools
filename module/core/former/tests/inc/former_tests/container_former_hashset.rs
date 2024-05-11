#![ allow( dead_code ) ]

#[ allow( unused_imports ) ]
use super::*;
#[ allow( unused_imports ) ]
use collection_tools::HashSet;

// qqq : xxx : remove #[ cfg( not( feature = "use_alloc" ) ) ]
#[ cfg( not( feature = "use_alloc" ) ) ]
#[ test ]
fn add()
{

  // expliccit with ContainerFormer

  let got : HashSet< String > = the_module
  ::ContainerFormer
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

  // expliccit with HashSetFormer

  let got : HashSet< String > = the_module::HashSetFormer::< String, (), HashSet< String >, the_module::ReturnStorage >
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

  // compact with HashSetFormer

  let got : HashSet< String > = the_module::HashSetFormer::new( former::ReturnStorage )
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

  let got : HashSet< String > = the_module::HashSetFormer
  ::begin( Some( hset![ "a".to_string() ] ), Some( () ), former::ReturnStorage )
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

// qqq : xxx : remove #[ cfg( not( feature = "use_alloc" ) ) ]
#[ cfg( not( feature = "use_alloc" ) ) ]
#[ test ]
fn replace()
{

  let got : HashSet< String > = the_module::HashSetFormer::new( former::ReturnStorage )
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

#[ test ]
fn entry_to_val()
{
  let got = former::EntryToVal::< HashSet< i32 > >::entry_to_val( 13i32 );
  let exp = 13i32;
  a_id!( got, exp )
}

#[ test ]
fn val_to_entry()
{
  let got = former::ValToEntry::< HashSet< i32 > >::val_to_entry( 13i32 );
  let exp = 13i32;
  a_id!( got, exp )
}
