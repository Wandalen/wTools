use super::*;

#[ derive( Debug, Default, PartialEq, former::ComponentFrom ) ]
struct TupleStruct( i32, String );

//

include!( "./only_test/component_from_tuple.rs" );