use core::fmt::Debug;

use core::ops::{ Deref };
use derive_tools::{ Deref, DerefMut };

#[ allow( dead_code ) ]
#[ derive( Deref, DerefMut ) ]
struct BoundsMixed< T : ToString, U >( T, U )
where
  U : Debug;

include!( "./only_tests/bounds_mixed.rs" );
