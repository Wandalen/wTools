use super::*;
use std::marker::PhantomData;

#[ allow( dead_code ) ]
struct StructNamed< T >
{
  a : String,
  b : i32,
  _phantom : PhantomData< T >,
}

include!( "./only_test/struct_named.rs" );