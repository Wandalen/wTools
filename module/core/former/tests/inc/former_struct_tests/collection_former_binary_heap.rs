#![ allow( dead_code ) ]

#[ allow( unused_imports ) ]
use super::*;
#[ allow( unused_imports ) ]
use collection_tools::BinaryHeap;

#[ test ]
fn add()
{

  // explicit with CollectionFormer

  let got : BinaryHeap< String > = the_module
  ::CollectionFormer
  ::< String, former::BinaryHeapDefinition< String, (), BinaryHeap< String >, the_module::ReturnStorage > >
  ::new( former::ReturnStorage )
  .add( "a" )
  .add( "b" )
  .form();
  let exp = collection_tools::heap!
  [
    "a".to_string(),
    "b".to_string(),
  ];
  a_id!( got.into_sorted_vec(), exp.into_sorted_vec() );

  // explicit with BinaryHeapFormer

  let got : BinaryHeap< String > = the_module::BinaryHeapFormer::< String, (), BinaryHeap< String >, the_module::ReturnStorage >
  ::new( former::ReturnStorage )
  .add( "a" )
  .add( "b" )
  .form();
  let exp = collection_tools::heap!
  [
    "a".to_string(),
    "b".to_string(),
  ];
  a_id!( got.into_sorted_vec(), exp.into_sorted_vec() );

  // compact with BinaryHeapFormer

  let got : BinaryHeap< String > = the_module::BinaryHeapFormer::new( former::ReturnStorage )
  .add( "a" )
  .add( "b" )
  .form();
  let exp = collection_tools::heap!
  [
    "a".to_string(),
    "b".to_string(),
  ];
  a_id!( got.into_sorted_vec(), exp.into_sorted_vec() );

  // with begin_coercing

  let got : BinaryHeap< String > = the_module::BinaryHeapFormer
  ::begin( Some( collection_tools::heap![ "a".to_string() ] ), Some( () ), former::ReturnStorage )
  .add( "b" )
  .form();
  let exp = collection_tools::heap!
  [
    "a".to_string(),
    "b".to_string(),
  ];
  a_id!( got.into_sorted_vec(), exp.into_sorted_vec() );

  // with help of ext

  use the_module::BinaryHeapExt;
  let got : BinaryHeap< String > = BinaryHeap::former()
  .add( "a" )
  .add( "b" )
  .form();
  let exp = collection_tools::heap!
  [
    "a".to_string(),
    "b".to_string(),
  ];
  a_id!( got.into_sorted_vec(), exp.into_sorted_vec() );

  //

}

// qqq : zzz : remove #[ cfg( not( feature = "use_alloc" ) ) ] -- done
// #[ cfg( not( feature = "use_alloc" ) ) ]
#[ test ]
fn replace()
{

  let got : BinaryHeap< String > = the_module::BinaryHeapFormer::new( former::ReturnStorage )
  .add( "x" )
  .replace( collection_tools::heap![ "a".to_string(), "b".to_string() ] )
  .form();
  let exp = collection_tools::heap!
  [
    "a".to_string(),
    "b".to_string(),
  ];
  a_id!( got.into_sorted_vec(), exp.into_sorted_vec() );

}

#[ test ]
fn entity_to()
{

  let got = < BinaryHeap< i32 > as former::EntityToFormer< former::BinaryHeapDefinition< i32, (), BinaryHeap< i32 >, former::ReturnStorage > > >
  ::Former::new( former::ReturnStorage )
  .add( 13 )
  .form();
  let exp = collection_tools::heap![ 13 ];
  a_id!( got.into_sorted_vec(), exp.into_sorted_vec() );

  let got = < BinaryHeap< i32 > as former::EntityToStorage >::Storage::default();
  let exp =
  <
    BinaryHeap< i32 > as former::EntityToFormer
    <
      former::BinaryHeapDefinition
      <
        i32,
        (),
        BinaryHeap< i32 >,
        former::ReturnStorage,
      >
    >
  >::Former::new( former::ReturnStorage )
  .form();
  a_id!( got.into_sorted_vec(), exp.into_sorted_vec() );

  let got = < BinaryHeap< i32 > as former::EntityToStorage >::Storage::default();
  let exp =
  <
    BinaryHeap< i32 > as former::EntityToFormer
    <
      < BinaryHeap< i32 > as former::EntityToDefinition< (), BinaryHeap< i32 >, former::ReturnPreformed > >::Definition
    >
  >::Former::new( former::ReturnPreformed )
  .form();
  a_id!( got.into_sorted_vec(), exp.into_sorted_vec() );

}

#[ test ]
fn entry_to_val()
{
  let got = former::EntryToVal::< BinaryHeap< i32 > >::entry_to_val( 13i32 );
  let exp = 13i32;
  a_id!( got, exp )
}

#[ test ]
fn val_to_entry()
{
  let got = former::ValToEntry::< BinaryHeap< i32 > >::val_to_entry( 13i32 );
  let exp = 13i32;
  a_id!( got, exp )
}

#[ test ]
fn subformer()
{

  /// Parameter description.
  #[ derive( Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, the_module::Former ) ]
  pub struct Child
  {
    name : String,
    data : bool,
  }

  /// Parent required for the template.
  #[ derive( Debug, Default, the_module::Former ) ]
  // #[ derive( Debug, Default, PartialEq, the_module::Former ) ] #[ debug ]
  // #[ derive( Debug, Default, PartialEq ) ]
  pub struct Parent
  {
    #[ subform_collection( definition = former::BinaryHeapDefinition ) ]
    children : BinaryHeap< Child >,
  }

  impl PartialEq< Parent > for Parent
  {
    fn eq( &self, other : &Parent ) -> bool
    {
      self.children.clone().into_sorted_vec() == other.children.clone().into_sorted_vec()
    }
  }

  let got = Parent::former()
  .children()
    .add( Child::former().name( "a" ).form() )
    .add( Child::former().name( "b" ).form() )
    .end()
  .form();

  let children = collection_tools::heap!
  [
    Child { name : "a".to_string(), data : false },
    Child { name : "b".to_string(), data : false },
  ];
  let exp = Parent { children };
  a_id!( got, exp );

}
