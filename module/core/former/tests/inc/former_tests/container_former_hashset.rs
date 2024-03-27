#![ allow( dead_code ) ]

#[ allow( unused_imports ) ]
use super::*;
#[ allow( unused_imports ) ]
use collection_tools::HashSet;

// qqq : zzz : remove #[ cfg( not( feature = "use_alloc" ) ) ]
#[ cfg( not( feature = "use_alloc" ) ) ]
#[ test ]
fn push()
{

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
