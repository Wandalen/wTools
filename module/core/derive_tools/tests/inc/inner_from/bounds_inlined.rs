use core::fmt::Debug;

use derive_tools::InnerFrom;

#[ allow( dead_code ) ]
#[ derive( InnerFrom ) ]
struct BoundsInlined< T : ToString, U : Debug >( T, U );

include!( "./only_test/bounds_inlined.rs" );
