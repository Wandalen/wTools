use super::*;

#[ derive( Debug, Default, PartialEq, component_model::ComponentFrom ) ]
struct TupleStruct( i32, String );

//

include!( "./only_test/component_from_tuple.rs" );