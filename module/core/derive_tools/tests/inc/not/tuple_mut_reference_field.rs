use super::*;

#[ allow( dead_code ) ]
#[ derive( the_module::Not ) ]
struct TupleMutReferenceField< 'a >( &'a mut bool, u8 );

include!( "./only_test/tuple_mut_reference_field.rs" );
