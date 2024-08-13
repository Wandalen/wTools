use super::*;

#[ allow( dead_code ) ]
#[ derive( the_module::Not ) ]
struct NamedReferenceField< 'a >
{
  a : &'a bool,
  b : u8,
}

include!( "only_test/named_reference_field.rs" );
