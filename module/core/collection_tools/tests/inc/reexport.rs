use super::*;

#[ test ]
#[ cfg( any( not( feature = "no_std" ), feature = "use_alloc" ) ) ]
fn vec()
{
  let mut map : the_module::Vec< i32 > = the_module::Vec::new();
  map.push( 1 );
  map.push( 2 );
  let got = map.first().unwrap().clone();
  assert_eq!( got, 1 );
  let got = map.last().unwrap().clone();
  assert_eq!( got, 2 );
}

#[ test ]
#[ cfg( any( not( feature = "no_std" ), feature = "use_alloc" ) ) ]
fn hashmap()
{
  use the_module::HashMap;
  let mut map : HashMap< i32, i32 > = HashMap::new();
  map.insert( 1, 2 );
  let exp = 2;
  let got = *map.get( &1 ).unwrap();
  assert_eq!( exp, got );
}

#[ test ]
#[ cfg( any( not( feature = "no_std" ), feature = "use_alloc" ) ) ]
fn hashset()
{
  let mut map : the_module::HashSet< i32 > = the_module::HashSet::new();
  map.insert( 1 );
  assert_eq!( map.contains( &1 ), true );
  assert_eq!( map.contains( &2 ), false );
}
