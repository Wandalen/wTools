#![ allow( dead_code ) ]

#[ allow( unused_imports ) ]
use super::*;
#[ allow( unused_imports ) ]
use collection_tools::HashSet;

// qqq : zzz : remove #[ cfg( not( feature = "use_alloc" ) ) ] -- done
// #[ cfg( not( feature = "use_alloc" ) ) ]
#[ test ]
fn add()
{

  // explicit with CollectionFormer

  let got : HashSet< String > = the_module
  ::CollectionFormer
  ::< String, former::HashSetDefinition< String, (), HashSet< String >, the_module::ReturnStorage > >
  ::new( former::ReturnStorage )
  .add( "a" )
  .add( "b" )
  .form();
  let exp = collection_tools::hset!
  [
    "a".to_string(),
    "b".to_string(),
  ];
  a_id!( got, exp );

  // explicit with HashSetFormer

  let got : HashSet< String > = the_module::HashSetFormer::< String, (), HashSet< String >, the_module::ReturnStorage >
  ::new( former::ReturnStorage )
  .add( "a" )
  .add( "b" )
  .form();
  let exp = collection_tools::hset!
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
  let exp = collection_tools::hset!
  [
    "a".to_string(),
    "b".to_string(),
  ];
  a_id!( got, exp );

  // with begin_coercing

  let got : HashSet< String > = the_module::HashSetFormer
  ::begin( Some( collection_tools::hset![ "a".to_string() ] ), Some( () ), former::ReturnStorage )
  .add( "b" )
  .form();
  let exp = collection_tools::hset!
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
  let exp = collection_tools::hset!
  [
    "a".to_string(),
    "b".to_string(),
  ];
  a_id!( got, exp );

  //

}

// qqq : zzz : remove #[ cfg( not( feature = "use_alloc" ) ) ] -- done
// #[ cfg( not( feature = "use_alloc" ) ) ]
#[ test ]
fn replace()
{

  let got : HashSet< String > = the_module::HashSetFormer::new( former::ReturnStorage )
  .add( "x" )
  .replace( collection_tools::hset![ "a".to_string(), "b".to_string() ] )
  .form();
  let exp = collection_tools::hset!
  [
    "a".to_string(),
    "b".to_string(),
  ];
  a_id!( got, exp );

}

#[ test ]
fn entity_to()
{

  let got = < HashSet< i32 > as former::EntityToFormer< former::HashSetDefinition< i32, (), HashSet< i32 >, former::ReturnStorage > > >
  ::Former::new( former::ReturnStorage )
  .add( 13 )
  .form();
  let exp = collection_tools::hset![ 13 ];
  a_id!( got, exp );

  let got = < HashSet< i32 > as former::EntityToStorage >::Storage::default();
  let exp =
  <
    HashSet< i32 > as former::EntityToFormer
    <
      former::HashSetDefinition
      <
        i32,
        (),
        HashSet< i32 >,
        former::ReturnStorage,
      >
    >
  >::Former::new( former::ReturnStorage )
  .form();
  a_id!( got, exp );

  let got = < HashSet< i32 > as former::EntityToStorage >::Storage::default();
  let exp =
  <
    HashSet< i32 > as former::EntityToFormer
    <
      < HashSet< i32 > as former::EntityToDefinition< (), HashSet< i32 >, former::ReturnPreformed > >::Definition
    >
  >::Former::new( former::ReturnPreformed )
  .form();
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

#[ test ]
fn subformer()
{

  /// Parameter description.
  #[ derive( Debug, Default, PartialEq, Eq, Hash, the_module::Former ) ]
  pub struct Child
  {
    name : String,
    data : bool,
  }

  /// Parent required for the template.
  #[ derive( Debug, Default, PartialEq, the_module::Former ) ]
  // #[ derive( Debug, Default, PartialEq, the_module::Former ) ] #[ debug ]
  // #[ derive( Debug, Default, PartialEq ) ]
  pub struct Parent
  {
    #[ subform_collection( definition = former::HashSetDefinition ) ]
    children : HashSet< Child >,
  }

  let got = Parent::former()
  .children()
    .add( Child::former().name( "a" ).form() )
    .add( Child::former().name( "b" ).form() )
    .end()
  .form();

  let children = collection_tools::hset!
  [
    Child { name : "a".to_string(), data : false },
    Child { name : "b".to_string(), data : false },
  ];
  let exp = Parent { children };
  a_id!( got, exp );

}
