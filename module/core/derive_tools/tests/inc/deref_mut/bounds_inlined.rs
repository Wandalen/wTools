use core::fmt::Debug;

use core::ops::{ Deref };
use derive_tools::{ Deref, DerefMut };

#[ allow( dead_code ) ]
#[ derive( Deref, DerefMut ) ]
struct BoundsInlined< T : ToString, U : Debug >( T, U );

include!( "./only_tests/bounds_inlined.rs" );
