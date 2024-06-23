use core::fmt::Debug;

use derive_tools::AsRef;

#[ allow( dead_code ) ]
#[ derive( AsRef ) ]
struct BoundsMixed< T : ToString, U >( T, U )
where
  U : Debug;

include!( "./only_test/bounds_mixed.rs" );
