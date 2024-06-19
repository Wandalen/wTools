use std::fmt::Debug;
use super::*;

#[ allow( dead_code ) ]
#[ the_module::phantom ]
struct BoundsWhere< T, U >
where
  T: ToString,
  U: Debug,
{}

include!( "./only_test/bounds_where.rs" );