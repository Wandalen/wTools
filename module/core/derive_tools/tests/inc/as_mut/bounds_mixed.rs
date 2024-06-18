use core::fmt::Debug;

use derive_tools::AsMut;

#[ allow( dead_code ) ]
#[ derive( AsMut ) ]
struct BoundsMixed< T : ToString, U >( T, U )
where
  U : Debug;

include!( "./only_test/bounds_mixed.rs" );
