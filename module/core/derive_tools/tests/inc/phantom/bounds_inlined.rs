use std::fmt::Debug;
use super::*;

#[ allow( dead_code ) ]
#[ the_module::phantom ]
struct BoundsInlined< T: ToString, U: Debug > {}

include!( "./only_test/bounds_inlined.rs" );