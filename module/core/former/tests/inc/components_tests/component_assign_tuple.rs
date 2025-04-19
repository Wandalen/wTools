use super::*;
#[ allow( unused_imports ) ]
use former::Assign;

#[ derive( Default, PartialEq, Debug, former::Assign ) ]
struct TupleStruct( i32, String );

//

include!( "./only_test/component_assign_tuple.rs" );
