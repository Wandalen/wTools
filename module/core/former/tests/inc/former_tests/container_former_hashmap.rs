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
  ::new( former::ReturnStorage )
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
  ::new( former::ReturnStorage )
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

  let got : HashMap< String, String > = the_module::HashMapSubformer::new( former::ReturnStorage )
  .add( ( "a".into(), "x".into() ) )
  .add( ( "b".into(), "y".into() ) )
  .form();
  let exp = hmap!
  [
    "a".to_string() => "x".to_string(),
    "b".to_string() => "y".to_string(),
  ];
  a_id!( got, exp );

  // with begin

  let got : HashMap< String, String > = the_module::HashMapSubformer
  ::begin( Some( hmap![ "a".to_string() => "x".to_string() ] ), Some( () ), former::ReturnStorage )
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

  let got : HashMap< String, String > = the_module::HashMapSubformer::new( former::ReturnStorage )
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

#[ test ]
fn element_to_val()
{
  let got = former::ElementToVal::< HashMap< u32, i32 > >::element_to_val( ( 1u32, 13i32 ) );
  let exp = 13i32;
  a_id!( got, exp )
}

#[ test ]
fn val_to_element()
{

  #[ derive( Clone, Copy, Debug, PartialEq ) ]
  struct Val
  {
    key : u32,
    data : i32,
  }

  impl former::ValToElement< HashMap< u32, Val > > for Val
  {
    type Element = ( u32, Val );
    #[ inline( always ) ]
    fn val_to_element( self ) -> Self::Element
    {
      ( self.key, self )
    }
  }

  let got = former::ValToElement::< HashMap< u32, Val > >::val_to_element( Val { key : 1u32, data : 13i32 } );
  let exp = ( 1u32, Val { key : 1u32, data : 13i32 } );
  a_id!( got, exp )

}
