use super::*;

#[ allow( dead_code ) ]
#[ derive( the_module::Index ) ]
#[ debug ]
enum Enum<T> 
{
  Nothing,
  #[ index ]
  IndexVector( Vec<T> )
}

include!( "./only_test/enum_unnamed.rs" );

