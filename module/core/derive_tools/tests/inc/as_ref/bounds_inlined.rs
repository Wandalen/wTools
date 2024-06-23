use core::fmt::Debug;

use derive_tools::AsRef;

#[ allow( dead_code ) ]
#[ derive( AsRef ) ]
struct BoundsInlined< T : ToString, U : Debug >( T, U );

include!( "./only_test/bounds_inlined.rs" );
