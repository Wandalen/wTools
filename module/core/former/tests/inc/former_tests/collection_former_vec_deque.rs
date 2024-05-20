// #![ allow( dead_code ) ]

use super::*;
#[ allow( unused_imports ) ]
use collection_tools::VecDeque;

//

#[ test ]
fn add()
{

  // explicit with CollectionFormer

  let got : VecDeque< String > = the_module
  ::CollectionFormer
  ::< String, former::VecDequeDefinition< String, (), VecDeque< String >, the_module::ReturnStorage > >
  ::new( former::ReturnStorage )
  .add( "a" )
  .add( "b" )
  .form();
  let exp = collection_tools::vecd!
  [
    "a".to_string(),
    "b".to_string(),
  ];
  a_id!( got, exp );

  // explicit with VecDequeFormer

  let got : VecDeque< String > = the_module::VecDequeFormer::< String, (), VecDeque< String >, the_module::ReturnStorage >
  ::new( former::ReturnStorage )
  .add( "a" )
  .add( "b" )
  .form();
  let exp = collection_tools::vecd!
  [
    "a".to_string(),
    "b".to_string(),
  ];
  a_id!( got, exp );

  // compact with VecDequeFormer

  let got : VecDeque< String > = the_module::VecDequeFormer::new( former::ReturnStorage )
  .add( "a" )
  .add( "b" )
  .form();
  let exp = collection_tools::vecd!
  [
    "a".to_string(),
    "b".to_string(),
  ];
  a_id!( got, exp );

  // with begin_coercing

  let got : VecDeque< String > = the_module::VecDequeFormer
  ::begin( Some( collection_tools::vecd![ "a".to_string() ] ), Some( () ), former::ReturnStorage )
  .add( "b" )
  .form();
  let exp = collection_tools::vecd!
  [
    "a".to_string(),
    "b".to_string(),
  ];
  a_id!( got, exp );

  // with help of ext

  use the_module::VecDequeExt;
  let got : VecDeque< String > = VecDeque::former()
  .add( "a" )
  .add( "b" )
  .form();
  let exp = collection_tools::vecd!
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

  let got : VecDeque< String > = the_module::VecDequeFormer::new( former::ReturnStorage )
  .add( "x" )
  .replace( collection_tools::vecd![ "a".to_string(), "b".to_string() ] )
  .form();
  let exp = collection_tools::vecd!
  [
    "a".to_string(),
    "b".to_string(),
  ];
  a_id!( got, exp );

}

//

// qqq : make similar test for all collections -- done
#[ test ]
fn entity_to()
{

  // qqq : uncomment and make it working -- done
  let got = < VecDeque< i32 > as former::EntityToFormer< former::VecDequeDefinition< i32, (), VecDeque< i32 >, former::ReturnStorage > > >
  ::Former::new( former::ReturnStorage )
  .add( 13 )
  .form();
  let exp = collection_tools::vecd![ 13 ];
  a_id!( got, exp );

  // qqq : uncomment and make it working
  let got = < VecDeque< i32 > as former::EntityToStorage >::Storage::default();
  let exp =
  <
    VecDeque< i32 > as former::EntityToFormer
    <
      former::VecDequeDefinition
      <
        i32,
        (),
        VecDeque< i32 >,
        former::ReturnStorage,
      >
    >
  >::Former::new( former::ReturnStorage )
  .form();
  a_id!( got, exp );

  let got = < VecDeque< i32 > as former::EntityToStorage >::Storage::default();
  let exp =
  <
    VecDeque< i32 > as former::EntityToFormer
    <
      < VecDeque< i32 > as former::EntityToDefinition< (), VecDeque< i32 >, former::ReturnPreformed > >::Definition
    >
  >::Former::new( former::ReturnPreformed )
  .form();
  a_id!( got, exp );

}

#[ test ]
fn entry_to_val()
{
  let got = former::EntryToVal::< VecDeque< i32 > >::entry_to_val( 13 );
  let exp = 13;
  a_id!( got, exp )
}

#[ test ]
fn val_to_entry()
{
  let got = former::ValToEntry::< VecDeque< i32 > >::val_to_entry( 13 );
  let exp = 13;
  a_id!( got, exp )
}

#[ test ]
fn subformer()
{

  /// Parameter description.
  #[ derive( Debug, Default, PartialEq, the_module::Former ) ]
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
    #[ subform_collection( definition = former::VecDequeDefinition ) ]
    children : VecDeque< Child >,
  }

  let got = Parent::former()
  .children()
    .add( Child::former().name( "a" ).form() )
    .add( Child::former().name( "b" ).form() )
    .end()
  .form();

  let children = collection_tools::vecd!
  [
    Child { name : "a".to_string(), data : false },
    Child { name : "b".to_string(), data : false },
  ];
  let exp = Parent { children };
  a_id!( got, exp );

}
