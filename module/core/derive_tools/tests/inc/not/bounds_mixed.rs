use std::fmt::Debug;
use core::ops::Not;
use super::*;

#[ allow( dead_code ) ]
#[ derive( the_module::Not ) ]
struct BoundsMixed< T : ToString + Not< Output = T >, U >
where
  U : Debug + Not< Output = U >,
{
  a: T,
  b: U,
}

include!( "./only_test/bounds_mixed.rs" );
