use std::fmt::Debug;
use super::*;

#[ allow( dead_code ) ]
#[ the_module::phantom ]
struct BoundsMixed< T: ToString, U >
where
  U: Debug,
{}

include!( "./only_test/bounds_mixed.rs" );