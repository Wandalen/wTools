use core::fmt::Debug;

use derive_tools::InnerFrom;

#[ allow( dead_code ) ]
#[ derive( InnerFrom ) ]
struct BoundsMixed< T : ToString, U >( T, U )
where
  U : Debug;

include!( "./only_test/bounds_mixed.rs" );
