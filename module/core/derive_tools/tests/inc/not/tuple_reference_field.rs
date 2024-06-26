use super::*;

#[ allow( dead_code ) ]
#[ derive( the_module::Not ) ]
struct TupleReferenceField< 'a >( &'a bool, u8 );

include!( "./only_test/tuple_reference_field.rs" );
