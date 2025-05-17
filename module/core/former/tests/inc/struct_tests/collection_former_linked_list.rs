// #![ allow( dead_code ) ]

use super::*;
#[ allow( unused_imports ) ]
use collection_tools::LinkedList;

//

#[ test ]
fn add()
{

  // explicit with CollectionFormer

  let got : LinkedList< String > = the_module
  ::CollectionFormer
  ::< String, former::LinkedListDefinition< String, (), LinkedList< String >, the_module::ReturnStorage > >
  ::new( former::ReturnStorage )
  .add( "a" )
  .add( "b" )
  .form();
  let exp = collection_tools::llist!
  [
    "a".to_string(),
    "b".to_string(),
  ];
  a_id!( got, exp );

  // explicit with LinkedListFormer

  let got : LinkedList< String > = the_module::LinkedListFormer::< String, (), LinkedList< String >, the_module::ReturnStorage >
  ::new( former::ReturnStorage )
  .add( "a" )
  .add( "b" )
  .form();
  let exp = collection_tools::llist!
  [
    "a".to_string(),
    "b".to_string(),
  ];
  a_id!( got, exp );

  // compact with Former

  let got : LinkedList< String > = the_module::LinkedListFormer::new( former::ReturnStorage )
  .add( "a" )
  .add( "b" )
  .form();
  let exp = collection_tools::llist!
  [
    "a".to_string(),
    "b".to_string(),
  ];
  a_id!( got, exp );

  // with begin_coercing

  let got : LinkedList< String > = the_module::LinkedListFormer
  ::begin( Some( collection_tools::llist![ "a".to_string() ] ), Some( () ), former::ReturnStorage )
  .add( "b" )
  .form();
  let exp = collection_tools::llist!
  [
    "a".to_string(),
    "b".to_string(),
  ];
  a_id!( got, exp );

  // with help of ext

  use the_module::LinkedListExt;
  let got : LinkedList< String > = LinkedList::former()
  .add( "a" )
  .add( "b" )
  .form();
  let exp = collection_tools::llist!
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

  let got : LinkedList< String > = the_module::LinkedListFormer::new( former::ReturnStorage )
  .add( "x" )
  .replace( collection_tools::llist![ "a".to_string(), "b".to_string() ] )
  .form();
  let exp = collection_tools::llist!
  [
    "a".to_string(),
    "b".to_string(),
  ];
  a_id!( got, exp );

}

//

#[ test ]
fn entity_to()
{

  let got = < LinkedList< i32 > as former::EntityToFormer< former::LinkedListDefinition< i32, (), LinkedList< i32 >, former::ReturnPreformed > > >
  ::Former::new( former::ReturnPreformed )
  .add( 13 )
  .form();
  let exp = collection_tools::llist![ 13 ];
  a_id!( got, exp );

  // qqq : uncomment and make it working
  let got = < LinkedList< i32 > as former::EntityToStorage >::Storage::default();
  let exp =
  <
    LinkedList< i32 > as former::EntityToFormer
    <
      former::LinkedListDefinition
      <
        i32,
        (),
        LinkedList< i32 >,
        former::ReturnPreformed,
      >
    >
  >::Former::new( former::ReturnPreformed )
  .form();
  a_id!( got, exp );

  let got = < LinkedList< i32 > as former::EntityToStorage >::Storage::default();
  let exp =
  <
    LinkedList< i32 > as former::EntityToFormer
    <
      < LinkedList< i32 > as former::EntityToDefinition< (), LinkedList< i32 >, former::ReturnPreformed > >::Definition
    >
  >::Former::new( former::ReturnPreformed )
  .form();
  a_id!( got, exp );

}

#[ test ]
fn entry_to_val()
{
  let got = former::EntryToVal::< LinkedList< i32 > >::entry_to_val( 13 );
  let exp = 13i32;
  a_id!( got, exp )
}

#[ test ]
fn val_to_entry()
{
  let got = former::ValToEntry::< LinkedList< i32 > >::val_to_entry( 13 );
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
    #[ subform_collection( definition = former::LinkedListDefinition ) ]
    children : LinkedList< Child >,
  }

  let got = Parent::former()
  .children()
    .add( Child::former().name( "a" ).form() )
    .add( Child::former().name( "b" ).form() )
    .end()
  .form();

  let children = collection_tools::llist!
  [
    Child { name : "a".to_string(), data : false },
    Child { name : "b".to_string(), data : false },
  ];
  let exp = Parent { children };
  a_id!( got, exp );

}
