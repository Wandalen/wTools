use core::fmt::Debug;

use core::ops::Deref;
use derive_tools::Deref;

#[ allow( dead_code ) ]
#[ derive( Deref ) ]
struct BoundsMixed< T : ToString, U >( T, U )
where
  U : Debug;

include!( "./only_test/bounds_mixed.rs" );
