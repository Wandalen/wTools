use core::fmt::Debug;

use core::ops::Deref;
use derive_tools::Deref;

#[ allow( dead_code ) ]
#[ derive( Deref ) ]
struct BoundsInlined< T : ToString, U : Debug >( T, U );

include!( "./only_test/bounds_inlined.rs" );
