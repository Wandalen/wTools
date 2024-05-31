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
  let exp = collection_tools::vec!
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
  let exp = collection_tools::vec!
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
  let exp = collection_tools::vec!
  [
    "a".to_string(),
    "b".to_string(),
  ];
  a_id!( got, exp );

  // with begin_coercing

  let got : Vec< String > = the_module::VectorFormer
  ::begin( Some( collection_tools::vec![ "a".to_string() ] ), Some( () ), former::ReturnStorage )
  .add( "b" )
  .form();
  let exp = collection_tools::vec!
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
  let exp = collection_tools::vec!
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
  .replace( collection_tools::vec![ "a".to_string(), "b".to_string() ] )
  .form();
  let exp = collection_tools::vec!
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
  let got = < Vec< i32 > as former::EntityToFormer< former::VectorDefinition< i32, (), Vec< i32 >, former::ReturnPreformed > > >
  ::Former::new( former::ReturnPreformed )
  .add( 13 )
  .form();
  let exp = collection_tools::vec![ 13 ];
  a_id!( got, exp );

  // qqq : uncomment and make it working
  let got = < Vec< i32 > as former::EntityToStorage >::Storage::default();
  let exp =
  <
    Vec< i32 > as former::EntityToFormer
    <
      former::VectorDefinition
      <
        i32,
        (),
        Vec< i32 >,
        former::ReturnPreformed,
      >
    >
  >::Former::new( former::ReturnPreformed )
  .form();
  a_id!( got, exp );

  let got = < Vec< i32 > as former::EntityToStorage >::Storage::default();
  let exp =
  <
    Vec< i32 > as former::EntityToFormer
    <
      < Vec< i32 > as former::EntityToDefinition< (), Vec< i32 >, former::ReturnPreformed > >::Definition
    >
  >::Former::new( former::ReturnPreformed )
  .form();
  a_id!( got, exp );

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
    #[ subform_collection( definition = former::VectorDefinition ) ]
    children : Vec< Child >,
  }

  let got = Parent::former()
  .children()
    .add( Child::former().name( "a" ).form() )
    .add( Child::former().name( "b" ).form() )
    .end()
  .form();

  let children = collection_tools::vec!
  [
    Child { name : "a".to_string(), data : false },
    Child { name : "b".to_string(), data : false },
  ];
  let exp = Parent { children };
  a_id!( got, exp );

}
