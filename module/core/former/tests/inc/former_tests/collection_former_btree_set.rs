#![ allow( dead_code ) ]

#[ allow( unused_imports ) ]
use super::*;
#[ allow( unused_imports ) ]
use collection_tools::BTreeSet;

#[ test ]
fn add()
{

  // explicit with CollectionFormer

  let got : BTreeSet< String > = the_module
  ::CollectionFormer
  ::< String, former::BTreeSetDefinition< String, (), BTreeSet< String >, the_module::ReturnStorage > >
  ::new( former::ReturnStorage )
  .add( "a" )
  .add( "b" )
  .form();
  let exp = collection_tools::bset!
  [
    "a".to_string(),
    "b".to_string(),
  ];
  a_id!( got, exp );

  // explicit with BTreeSetFormer

  let got : BTreeSet< String > = the_module::BTreeSetFormer::< String, (), BTreeSet< String >, the_module::ReturnStorage >
  ::new( former::ReturnStorage )
  .add( "a" )
  .add( "b" )
  .form();
  let exp = collection_tools::bset!
  [
    "a".to_string(),
    "b".to_string(),
  ];
  a_id!( got, exp );

  // compact with BTreeSetFormer

  let got : BTreeSet< String > = the_module::BTreeSetFormer::new( former::ReturnStorage )
  .add( "a" )
  .add( "b" )
  .form();
  let exp = collection_tools::bset!
  [
    "a".to_string(),
    "b".to_string(),
  ];
  a_id!( got, exp );

  // with begin_coercing

  let got : BTreeSet< String > = the_module::BTreeSetFormer
  ::begin( Some( collection_tools::bset![ "a".to_string() ] ), Some( () ), former::ReturnStorage )
  .add( "b" )
  .form();
  let exp = collection_tools::bset!
  [
    "a".to_string(),
    "b".to_string(),
  ];
  a_id!( got, exp );

  // with help of ext

  use the_module::BTreeSetExt;
  let got : BTreeSet< String > = BTreeSet::former()
  .add( "a" )
  .add( "b" )
  .form();
  let exp = collection_tools::bset!
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

  let got : BTreeSet< String > = the_module::BTreeSetFormer::new( former::ReturnStorage )
  .add( "x" )
  .replace( collection_tools::bset![ "a".to_string(), "b".to_string() ] )
  .form();
  let exp = collection_tools::bset!
  [
    "a".to_string(),
    "b".to_string(),
  ];
  a_id!( got, exp );

}

#[ test ]
fn entity_to()
{

  let got = < BTreeSet< i32 > as former::EntityToFormer< former::BTreeSetDefinition< i32, (), BTreeSet< i32 >, former::ReturnStorage > > >
  ::Former::new( former::ReturnStorage )
  .add( 13 )
  .form();
  let exp = collection_tools::bset![ 13 ];
  a_id!( got, exp );

  let got = < BTreeSet< i32 > as former::EntityToStorage >::Storage::default();
  let exp =
  <
    BTreeSet< i32 > as former::EntityToFormer
    <
      former::BTreeSetDefinition
      <
        i32,
        (),
        BTreeSet< i32 >,
        former::ReturnStorage,
      >
    >
  >::Former::new( former::ReturnStorage )
  .form();
  a_id!( got, exp );

  let got = < BTreeSet< i32 > as former::EntityToStorage >::Storage::default();
  let exp =
  <
    BTreeSet< i32 > as former::EntityToFormer
    <
      < BTreeSet< i32 > as former::EntityToDefinition< (), BTreeSet< i32 >, former::ReturnPreformed > >::Definition
    >
  >::Former::new( former::ReturnPreformed )
  .form();
  a_id!( got, exp );

}

#[ test ]
fn entry_to_val()
{
  let got = former::EntryToVal::< BTreeSet< i32 > >::entry_to_val( 13i32 );
  let exp = 13i32;
  a_id!( got, exp )
}

#[ test ]
fn val_to_entry()
{
  let got = former::ValToEntry::< BTreeSet< i32 > >::val_to_entry( 13i32 );
  let exp = 13i32;
  a_id!( got, exp )
}

#[ test ]
fn subformer()
{

  /// Parameter description.
  #[ derive( Debug, Default, PartialEq, Eq, PartialOrd, Ord, the_module::Former ) ]
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
    #[ subform_collection( definition = former::BTreeSetDefinition ) ]
    children : BTreeSet< Child >,
  }

  let got = Parent::former()
  .children()
    .add( Child::former().name( "a" ).form() )
    .add( Child::former().name( "b" ).form() )
    .end()
  .form();

  let children = collection_tools::bset!
  [
    Child { name : "a".to_string(), data : false },
    Child { name : "b".to_string(), data : false },
  ];
  let exp = Parent { children };
  a_id!( got, exp );

}
