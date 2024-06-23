use core::fmt::Debug;

use derive_tools::AsMut;

#[ allow( dead_code ) ]
#[ derive( AsMut ) ]
struct BoundsInlined< T : ToString, U : Debug >( T, U );

include!( "./only_test/bounds_inlined.rs" );
