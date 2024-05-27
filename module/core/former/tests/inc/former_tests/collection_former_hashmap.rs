#![ allow( dead_code ) ]

#[ allow( unused_imports ) ]
use super::*;
#[ allow( unused_imports ) ]
use collection_tools::HashMap;

// qqq : zzz : remove #[ cfg( not( feature = "use_alloc" ) ) ] -- done
// #[ cfg( not( feature = "use_alloc" ) ) ]
#[ test ]
fn add()
{

  // expliccit with CollectionFormer

  let got : HashMap< String, String > = the_module
  ::CollectionFormer
  ::< ( String, String ), former::HashMapDefinition< String, String, (), HashMap< String, String >, the_module::ReturnStorage > >
  ::new( former::ReturnStorage )
  .add( ( "a".into(), "x".into() ) )
  .add( ( "b".into(), "y".into() ) )
  .form();
  let exp = collection_tools::hmap!
  [
    "a".to_string() => "x".to_string(),
    "b".to_string() => "y".to_string(),
  ];
  a_id!( got, exp );

  // expliccit with HashMapFormer

  let got : HashMap< String, String > = the_module::HashMapFormer::< String, String, (), HashMap< String, String >, the_module::ReturnStorage >
  ::new( former::ReturnStorage )
  .add( ( "a".into(), "x".into() ) )
  .add( ( "b".into(), "y".into() ) )
  .form();
  let exp = collection_tools::hmap!
  [
    "a".to_string() => "x".to_string(),
    "b".to_string() => "y".to_string(),
  ];
  a_id!( got, exp );

  // compact with HashMapFormer

  let got : HashMap< String, String > = the_module::HashMapFormer::new( former::ReturnStorage )
  .add( ( "a".into(), "x".into() ) )
  .add( ( "b".into(), "y".into() ) )
  .form();
  let exp = collection_tools::hmap!
  [
    "a".to_string() => "x".to_string(),
    "b".to_string() => "y".to_string(),
  ];
  a_id!( got, exp );

  // with begin

  let got : HashMap< String, String > = the_module::HashMapFormer
  ::begin( Some( collection_tools::hmap![ "a".to_string() => "x".to_string() ] ), Some( () ), former::ReturnStorage )
  .add( ( "b".into(), "y".into() ) )
  .form();
  let exp = collection_tools::hmap!
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
  let exp = collection_tools::hmap!
  [
    "a".to_string() => "x".to_string(),
    "b".to_string() => "y".to_string(),
  ];
  a_id!( got, exp );

  //

}

// qqq : zzz : remove #[ cfg( not( feature = "use_alloc" ) ) ] -- done
// #[ cfg( not( feature = "use_alloc" ) ) ]
#[ test ]
fn replace()
{

  let got : HashMap< String, String > = the_module::HashMapFormer::new( former::ReturnStorage )
  .add( ( "x".to_string(), "y".to_string() ) )
  .replace( collection_tools::hmap![ "a".to_string() => "x".to_string(), "b".to_string() => "y".to_string(), ] )
  .form();
  let exp = collection_tools::hmap!
  [
    "a".to_string() => "x".to_string(),
    "b".to_string() => "y".to_string(),
  ];
  a_id!( got, exp );

}

#[ test ]
fn entity_to()
{

  let got = < HashMap< i32, i32 > as former::EntityToFormer< former::HashMapDefinition< i32, i32, (), HashMap< i32, i32 >, former::ReturnStorage > > >
  ::Former::new( former::ReturnStorage )
  .add( ( 13, 14 ) )
  .form();
  let exp = collection_tools::hmap![ 13 => 14 ];
  a_id!( got, exp );

  let got = < HashMap< i32, i32 > as former::EntityToStorage >::Storage::default();
  let exp =
  <
    HashMap< i32, i32 > as former::EntityToFormer
    <
      former::HashMapDefinition
      <
        i32,
        i32,
        (),
        HashMap< i32, i32 >,
        former::ReturnStorage,
      >
    >
  >::Former::new( former::ReturnStorage )
  .form();
  a_id!( got, exp );

  let got = < HashMap< i32, i32 > as former::EntityToStorage >::Storage::default();
  let exp =
  <
    HashMap< i32, i32 > as former::EntityToFormer
    <
      < HashMap< i32, i32 > as former::EntityToDefinition< (), HashMap< i32, i32 >, former::ReturnPreformed > >::Definition
    >
  >::Former::new( former::ReturnPreformed )
  .form();
  a_id!( got, exp );

}

#[ test ]
fn entry_to_val()
{
  let got = former::EntryToVal::< HashMap< u32, i32 > >::entry_to_val( ( 1u32, 13i32 ) );
  let exp = 13i32;
  a_id!( got, exp )
}

#[ test ]
fn val_to_entry()
{

  #[ derive( Clone, Copy, Debug, PartialEq ) ]
  struct Val
  {
    key : u32,
    data : i32,
  }

  impl former::ValToEntry< HashMap< u32, Val > > for Val
  {
    type Entry = ( u32, Val );
    #[ inline( always ) ]
    fn val_to_entry( self ) -> Self::Entry
    {
      ( self.key, self )
    }
  }

  let got = former::ValToEntry::< HashMap< u32, Val > >::val_to_entry( Val { key : 1u32, data : 13i32 } );
  let exp = ( 1u32, Val { key : 1u32, data : 13i32 } );
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
    #[ subform_collection( definition = former::HashMapDefinition ) ]
    children : HashMap< u32, Child >,
  }

  let got = Parent::former()
  .children()
    .add( ( 0, Child::former().name( "a" ).form() ) )
    .add( ( 1, Child::former().name( "b" ).form() ) )
    .end()
  .form();

  let children = collection_tools::hmap!
  [
    0 => Child { name : "a".to_string(), data : false },
    1 => Child { name : "b".to_string(), data : false },
  ];
  let exp = Parent { children };
  a_id!( got, exp );

}
